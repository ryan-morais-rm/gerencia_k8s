use std::fs;
use std::path::Path;
use std::process::Command;
use clap::{Parser, Subcommand};
use nix::mount::{mount, MsFlags};
use nix::sched::{unshare, CloneFlags};
use rusqlite::{Connection, Result as SqlResult};

#[derive(Parser)]
#[command(name = "nayr")]
#[command(about = "Um runtime de container simplificado escrito em Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Pull {
        #[arg(short, long)]
        repo: String,
    },
    Run {
        #[arg(short, long)]
        name: String,
        #[arg(short, long, default_value = "/bin/sh")]
        exec: String,
        #[arg(short, long)]
        memory: Option<u32>,
    },
    Ps,
    #[command(hide = true)]
    Child {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        exec: String,
    },
    Rm {
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    if !nix::unistd::Uid::current().is_root() {
        eprintln!("Erro: O 'nayr' precisa ser executado com privilégios de superusuário (sudo).");
        std::process::exit(1);
    }

    let cli = Cli::parse();

    match &cli.command {
        Commands::Pull { repo } => {
            let destino = "images/base";
            println!("Iniciando gerenciamento da imagem base...");
            baixar_ou_atualizar_imagem(repo, destino);
        }
        Commands::Run { name, exec, memory } => {
            println!("Criando infraestrutura e Namespaces para '{}'...", name);

            preparar_pastas_overlay(name);

            unshare(CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUTS)
                .expect("Falha ao criar novos namespaces (unshare)");

            let mut child_proc = Command::new("/proc/self/exe")
                .arg("child")
                .arg("--name")
                .arg(name)
                .arg("--exec")
                .arg(exec)
                .spawn()
                .expect("Falha ao reexecutar o binário em modo child");

            let child_pid = child_proc.id();
            
            configurar_cgroup(name, child_pid, *memory);

            if let Ok(conn) = init_db() {
                conn.execute(
                    "INSERT OR REPLACE INTO containers (name, pid, status, command) VALUES (?1, ?2, ?3, ?4)",
                    (name, child_pid, "Running", exec),
                ).ok();
            }
            match child_proc.wait() {
                Ok(status) => {
                    println!("\nContentor '{}' finalizado com status: {}", name, status);
                    if let Ok(conn) = init_db() {
                        conn.execute("UPDATE containers SET status = 'Exited' WHERE name = ?1", [name]).ok();
                    }
                }
                Err(e) => eprintln!("Erro ao aguardar o contentor: {}", e),
            }
        }
        Commands::Child { name, exec } => {
            nix::mount::mount(
                None::<&str>,
                "/",
                None::<&str>,
                nix::mount::MsFlags::MS_PRIVATE | nix::mount::MsFlags::MS_REC,
                None::<&str>,
            ).expect("Falha ao privar a montagem");

            let merged_path = montar_overlay_interno(name);

            nix::unistd::sethostname(name).expect("Falha ao definir hostname");

            let proc_path = format!("{}/proc", merged_path);
            if Path::new(&proc_path).exists() {
                nix::mount::mount(
                    Some("proc"),
                    Path::new(&proc_path),
                    Some("proc"),
                    nix::mount::MsFlags::empty(),
                    None::<&str>,
                ).ok(); 
            }

            nix::unistd::chroot(merged_path.as_str()).expect("Falha ao aplicar chroot");
            std::env::set_current_dir("/").expect("Falha ao mudar para a raiz do container");

            let mut partes_comando = exec.split_whitespace();
            let programa = partes_comando.next().unwrap_or("/bin/sh");
            let argumentos: Vec<&str> = partes_comando.collect();

            println!("Executando processo interno: {} {:?}", programa, argumentos);
            
            let mut final_cmd = Command::new(programa)
                .args(&argumentos)
                .spawn()
                .expect("Erro: Não foi possível executar o binário dentro do jail.");

            final_cmd.wait().expect("Falha ao aguardar o processo interno terminar");
        }
        Commands::Ps => {
            if let Ok(conn) = init_db() {
                let mut stmt = conn.prepare("SELECT name, pid, status, command FROM containers").unwrap();
                let container_iter = stmt.query_map([], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, u32>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                    ))
                }).unwrap();

                println!("{:<15} {:<10} {:<15} {}", "NOME", "PID", "STATUS", "COMANDO");
                println!("{:-<60}", ""); 
                
                for c in container_iter {
                    let (name, pid, status, cmd) = c.unwrap();
                    println!("{:<15} {:<10} {:<15} {}", name, pid, status, cmd);
                }
            } else {
                eprintln!("Erro ao aceder ao banco de dados dos contentores.");
            }
        }
        Commands::Rm { name } => {
            println!("A iniciar a limpeza do contentor '{}'...", name);
            
            let upper_dir = format!("overlays/{}/upper", name);
            let work_dir = format!("overlays/{}/work", name);
            let overlay_parent = format!("overlays/{}", name);
            let merged_dir = format!("containers/{}", name);

            if Path::new(&overlay_parent).exists() {
                if let Err(e) = fs::remove_dir_all(&overlay_parent) {
                    eprintln!("Aviso: Falha ao remover pastas do overlay: {}", e);
                } else {
                    println!("- Camadas de escrita (upper/work) removidas.");
                }
            }

            if Path::new(&merged_dir).exists() {
                if let Err(e) = fs::remove_dir_all(&merged_dir) {
                    eprintln!("Aviso: Falha ao remover a pasta raiz do contentor: {}", e);
                } else {
                    println!("- Pasta de montagem removida.");
                }
            }

            let cgroup_dir = format!("/sys/fs/cgroup/nayr_{}", name);
            if Path::new(&cgroup_dir).exists() {
                if let Err(e) = fs::remove_dir(&cgroup_dir) {
                    eprintln!("Aviso: Falha ao remover cgroup: {}", e);
                } else {
                    println!("- Limites de recursos (Cgroups) removidos.");
                }
            }

            if let Ok(conn) = init_db() {
                let linhas_apagadas = conn.execute("DELETE FROM containers WHERE name = ?1", [name]).unwrap_or(0);
                if linhas_apagadas > 0 {
                    println!("- Registo do banco de dados removido.");
                } else {
                    println!("- O contentor não constava no banco de dados.");
                }
            }
            
            println!("Contentor removido com sucesso!");
        }
    }
}

fn baixar_ou_atualizar_imagem(repo_url: &str, caminho_destino: &str) {
    if let Some(parent) = Path::new(caminho_destino).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Falha ao criar o diretório 'images/'");
        }
    }

    if Path::new(caminho_destino).join(".git").exists() {
        println!("Imagem detectada em '{}'. Atualizando via 'git pull'...", caminho_destino);
        let status = Command::new("git").arg("-C").arg(caminho_destino).arg("pull").status().unwrap();
        if status.success() { println!("Imagem base atualizada com sucesso!"); }
    } else {
        println!("🛰️ Clonando rootfs a partir de: {}", repo_url);
        let status = Command::new("git").arg("clone").arg(repo_url).arg(caminho_destino).status().unwrap();
        if status.success() { println!("Imagem base baixada com sucesso!"); }
    }
}

fn preparar_pastas_overlay(name: &str) {
    let lower_dir = "images/base";
    let upper_dir = format!("overlays/{}/upper", name);
    let work_dir = format!("overlays/{}/work", name);
    let merged_dir = format!("containers/{}", name);

    if !Path::new(lower_dir).exists() {
        eprintln!("Erro: Imagem base não encontrada. Rode 'nayr pull' primeiro.");
        std::process::exit(1);
    }

    fs::create_dir_all(&upper_dir).unwrap();
    fs::create_dir_all(&work_dir).unwrap();
    fs::create_dir_all(&merged_dir).unwrap();
}

fn montar_overlay_interno(name: &str) -> String {
    let lower_dir = "images/base";
    let upper_dir = format!("overlays/{}/upper", name);
    let work_dir = format!("overlays/{}/work", name);
    let merged_dir = format!("containers/{}", name);

    let options = format!(
        "lowerdir={},upperdir={},workdir={}",
        lower_dir, upper_dir, work_dir
    );

    println!("Montando sistema de arquivos (OverlayFS) dentro do namespace...");
    
    mount(
        Some("overlay"),
        Path::new(&merged_dir),
        Some("overlay"),
        MsFlags::empty(),
        Some(options.as_str()),
    ).expect("Falha ao montar o OverlayFS isolado.");

    merged_dir
}

fn init_db() -> SqlResult<Connection> {
    let conn = Connection::open("container.db")?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS containers (
            name TEXT PRIMARY KEY,
            pid INTEGER,
            status TEXT,
            command TEXT
        )",
        (),
    )?;
    
    Ok(conn)
}

fn configurar_cgroup(name: &str, pid: u32, memory_mb: Option<u32>) {
    let cgroup_path = format!("/sys/fs/cgroup/nayr_{}", name);

    if !Path::new(&cgroup_path).exists() {
        fs::create_dir_all(&cgroup_path).expect("Falha ao criar diretório do cgroup");
    }

    if let Some(mem) = memory_mb {
        let mem_bytes = mem * 1024 * 1024; 
        let mem_path = format!("{}/memory.max", cgroup_path);
        
        fs::write(&mem_path, mem_bytes.to_string())
            .unwrap_or_else(|_| eprintln!("Aviso: Falha ao definir limite de memória. O sistema possui Cgroups v2 ativo?"));
            
        println!("Limite de RAM restrito para {} MB.", mem);
    }

    let procs_path = format!("{}/cgroup.procs", cgroup_path);
    fs::write(&procs_path, pid.to_string()).expect("Falha ao registrar processo no cgroup");
}