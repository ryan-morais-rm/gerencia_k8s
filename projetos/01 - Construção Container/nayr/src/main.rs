use clap::{Parser, Subcommand};
use nix::mount::{mount, MsFlags};
use nix::sched::{unshare, CloneFlags};
use std::fs;
use std::path::Path;
use std::process::Command;

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
    },
    #[command(hide = true)]
    Child {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        exec: String,
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
        Commands::Run { name, exec } => {
            println!("Configurando o ambiente para o container '{}'...", name);
            
            let merged_dir = configurar_e_montar_overlay(name);

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

            match child_proc.wait() {
                Ok(status) => println!("\nContainer '{}' finalizado com status: {}", name, status),
                Err(e) => eprintln!("Erro ao aguardar o container: {}", e),
            }
        }
        Commands::Child { name, exec } => {
            println!("Container isolado via Namespaces. Configurando montagens privadas...");

            nix::mount::mount(
                None::<&str>,
                "/",
                None::<&str>,
                nix::mount::MsFlags::MS_PRIVATE | nix::mount::MsFlags::MS_REC,
                None::<&str>,
            ).expect("Falha ao definir propagação de montagem como privada");

            let merged_path = format!("containers/{}", name);

            nix::unistd::sethostname(name).expect("Falha ao definir hostname do container");

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
            // --------------------------------------------------

            nix::unistd::chroot(merged_path.as_str()).expect("Falha ao aplicar chroot");
            std::env::set_current_dir("/").expect("Falha ao mudar para a raiz do container");

            println!("Executando processo interno: {}\n", exec);
            
            let mut final_cmd = Command::new(exec)
                .spawn()
                .expect("Erro: Não foi possível executar o binário dentro do jail.");

            final_cmd.wait().expect("Falha ao aguardar o processo interno terminar");
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
        
        let status = Command::new("git")
            .arg("-C")
            .arg(caminho_destino)
            .arg("pull")
            .status()
            .expect("Falha ao executar o comando git pull");

        if status.success() {
            println!("Imagem base atualizada com sucesso!");
        } else {
            println!("Erro ao tentar atualizar o repositório.");
        }
    } else {
        println!("🛰️ Clonando rootfs a partir de: {}", repo_url);
        
        let status = Command::new("git")
            .arg("clone")
            .arg(repo_url)
            .arg(caminho_destino)
            .status()
            .expect("Falha ao executar o comando git clone");

        if status.success() {
            println!("Imagem base baixada com sucesso em '{}'!", caminho_destino);
        } else {
            println!("Erro ao tentar clonar o repositório. Verifique a URL.");
        }
    }
}

fn configurar_e_montar_overlay(name: &str) -> String {
    let lower_dir = "images/base";
    let upper_dir = format!("overlays/{}/upper", name);
    let work_dir = format!("overlays/{}/work", name);
    let merged_dir = format!("containers/{}", name);

    if !Path::new(lower_dir).exists() {
        eprintln!("Erro: Imagem base não encontrada em '{}'. Rode 'nayr pull' primeiro.", lower_dir);
        std::process::exit(1);
    }

    fs::create_dir_all(&upper_dir).unwrap();
    fs::create_dir_all(&work_dir).unwrap();
    fs::create_dir_all(&merged_dir).unwrap();

    let options = format!(
        "lowerdir={},upperdir={},workdir={}",
        lower_dir, upper_dir, work_dir
    );

    println!("Montando sistema de arquivos em camadas (OverlayFS)...");
    
    mount(
        Some("overlay"),
        Path::new(&merged_dir),
        Some("overlay"),
        MsFlags::empty(),
        Some(options.as_str()),
    )
    .expect("Falha ao montar o OverlayFS. Certifique-se de estar usando caminhos compatíveis.");

    merged_dir
}