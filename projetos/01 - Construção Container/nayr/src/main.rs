use std::fs::{self, OpenOptions};
use std::path::Path;
use std::process::{Command, Stdio};
use nix::unistd::Pid; 
use nix::sys::signal::{kill, Signal};
use nix::sched::{unshare, CloneFlags};
use clap::{Parser, Subcommand};

mod utils;
use crate::utils::{baixar_ou_atualizar_imagem, preparar_pastas_overlay, montar_overlay_interno, init_db, configurar_cgroup};

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
    Start {
        #[arg(short, long)]
        name: String,
    },
    Stop {
        #[arg(short, long)]
        name: String,
    },
    Logs {
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

            let log_path = format!("logs/{}/container.log", name);
            let log_dir = Path::new(&log_path).parent().unwrap();
            fs::create_dir_all(log_dir).expect("Falha ao criar diretório de logs"); 

            let log_file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&log_path)
                .expect("Falha ao abrir o ficheiro de log");

            let log_file_err = log_file.try_clone().expect("Falha ao clonar descritor do log");

            unshare(CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUTS)
                .expect("Falha ao criar novos namespaces (unshare)");

            let mut child_proc = Command::new("/proc/self/exe")
                .arg("child")
                .arg("--name")
                .arg(name)
                .arg("--exec")
                .arg(exec)
                .stdout(Stdio::from(log_file))
                .stderr(Stdio::from(log_file_err))
                .spawn()
                .expect("Falha ao reexecutar o binário em modo child");

            let child_pid = child_proc.id();

            println!("Processo em execução! Utilize 'nayr logs --name {}' para visualizar as saídas.", name);
            
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
        },
        Commands::Start { name } => {
            println!("A preparar para iniciar o contentor '{}'...", name);
            let mut stored_cmd = String::new();

            if let Ok(conn) = init_db() {
                let mut stmt = conn.prepare("SELECT status, command FROM containers WHERE name = ?1").unwrap();
                let mut rows = stmt.query([name]).unwrap();

                if let Ok(Some(row)) = rows.next() {
                    let status: String = row.get(0).unwrap();
                    stored_cmd = row.get(1).unwrap();

                    if status == "Running" {
                        println!("O contentor '{}' já se encontra em execução.", name);
                        return;
                    }
                } else {
                    println!("Erro: Contentor '{}' não encontrado na base de dados.", name);
                    return;
                }
            } else {
                eprintln!("Erro ao aceder ao banco de dados.");
                return;
            }

            println!("A retomar a infraestrutura e Namespaces...");
            
            preparar_pastas_overlay(name);

            let log_path = format!("logs/{}/container.log", name);
            let log_file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&log_path)
                .expect("Falha ao abrir/criar o ficheiro de log");
            let log_file_err = log_file.try_clone().expect("Falha ao clonar descritor do log");

            unshare(CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUTS)
                .expect("Falha ao criar novos namespaces (unshare)");

            let mut child_proc = Command::new("/proc/self/exe")
                .arg("child")
                .arg("--name")
                .arg(name)
                .arg("--exec")
                .arg(&stored_cmd)
                .stdout(Stdio::from(log_file))
                .stderr(Stdio::from(log_file_err))
                .spawn()
                .expect("Falha ao reexecutar o binário em modo child");

            let child_pid = child_proc.id();

            println!("Processo retomado! Utilize 'nayr logs --name {}' para visualizar as saídas.", name);

            configurar_cgroup(name, child_pid, None);

            if let Ok(conn) = init_db() {
                conn.execute(
                    "UPDATE containers SET pid = ?1, status = 'Running' WHERE name = ?2",
                    (child_pid, name),
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
        },
        Commands::Stop { name } => {
            println!("A tentar parar o contentor '{}'...", name);
            if let Ok(conn) = init_db() {
                let mut stmt = conn.prepare("SELECT pid, status FROM containers WHERE name = ?1").unwrap();
                let mut rows = stmt.query([name]).unwrap();

                if let Ok(Some(row)) = rows.next() {
                    let pid: u32 = row.get(0).unwrap();
                    let status: String = row.get(1).unwrap();

                    if status == "Running" {
                        let nix_pid = Pid::from_raw(pid as i32);
                        
                        match kill(nix_pid, Signal::SIGTERM) {
                            Ok(_) => {
                                println!("Sinal SIGTERM enviado ao processo (PID: {}).", pid);
                                conn.execute("UPDATE containers SET status = 'Exited' WHERE name = ?1", [name]).unwrap();
                            }
                            Err(e) => {
                                eprintln!("Aviso: Não foi possível parar o processo {}. Ele já pode ter sido encerrado. (Erro: {})", pid, e);
                                conn.execute("UPDATE containers SET status = 'Exited' WHERE name = ?1", [name]).unwrap();
                            }
                        }
                    } else {
                        println!("O contentor '{}' já se encontra parado (Status: {}).", name, status);
                    }
                } else {
                    println!("Erro: Contentor '{}' não encontrado no banco de dados.", name);
                }
            } else {
                eprintln!("Erro ao aceder ao banco de dados.");
            }
        },
        Commands::Logs { name } => {
            let log_path = format!("logs/{}/container.log", name);
            if Path::new(&log_path).exists() {
                match fs::read_to_string(&log_path) {
                    Ok(conteudo) => {
                        println!("=== Logs do Contentor '{}' ===", name);
                        print!("{}", conteudo);
                        println!("================================");
                    }
                    Err(e) => eprintln!("Erro ao ler o ficheiro de logs: {}", e),
                }
            } else {
                println!("Nenhum ficheiro de log encontrado para o contentor '{}'.", name);
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
            let log_parent = format!("logs/{}", name);
            let merged_dir = format!("containers/{}", name);

            if Path::new(&overlay_parent).exists() {
                if let Err(e) = fs::remove_dir_all(&overlay_parent) {
                    eprintln!("Aviso: Falha ao remover pastas do overlay: {}", e);
                } else {
                    println!("- Camadas de escrita (upper/work) removidas.");
                }
            }
            
            if Path::new(&log_parent).exists() {
                if let Err(e) = fs::remove_dir_all(&overlay_parent) {
                    eprintln!("Aviso: Falha ao remover pastas do log: {}", e);
                } else {
                    println!("- Camadas de log removidas.");
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