use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "nayr")]
#[command(about = "Um runtime de container simplificado escrito em Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Baixa ou atualiza a imagem base de um sistema de arquivos via Git
    Pull {
        #[arg(short, long)]
        repo: String,
    },
    // Cria e executa um ambiente isolado (Container)
    Run {
        #[arg(short, long)]
        name: String,
        #[arg(short, long, default_value = "/bin/sh")]
        exec: String,
    },
}

fn main() {
    // Garante que o programa está rodando com privilégios de Root (Sudo)
    if !nix::unistd::Uid::current().is_root() {
        eprintln!("Erro: O 'nayr' precisa ser executado com privilégios de superusuário (sudo).");
        std::process::exit(1);
    }

    let cli = Cli::parse();

    match &cli.command {
        Commands::Pull { repo } => {
            let destino = "images/base";
            println!("📥 Iniciando gerenciamento da imagem base...");
            baixar_ou_atualizar_imagem(repo, destino);
        }
        Commands::Run { name, exec } => {
            println!("🚀 Preparando para rodar o container '{}' executando '{}'...", name, exec);
            // Próximas fases: Aqui entrará a lógica do OverlayFS e Namespaces
        }
    }
}

/// Função responsável pelo Requisito Mínimo: "Baixar ou atualizar a camada base via Git"
fn baixar_ou_atualizar_imagem(repo_url: &str, caminho_destino: &str) {
    // Garante que o diretório pai (../images) exista
    if let Some(parent) = Path::new(caminho_destino).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Falha ao criar o diretório 'images/'");
        }
    }

    // Verifica se a imagem base já foi baixada anteriormente
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