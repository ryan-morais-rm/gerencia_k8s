use std::fs;
use std::path::Path;
use std::process::Command;
use nix::mount::{mount, umount, MsFlags};
use rusqlite::{Connection, Result as SqlResult};

pub fn baixar_ou_atualizar_imagem(repo_url: &str, caminho_destino: &str) {
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
        println!("Clonando rootfs a partir de: {}", repo_url);
        let status = Command::new("git").arg("clone").arg(repo_url).arg(caminho_destino).status().unwrap();
        if status.success() { println!("Imagem base baixada com sucesso!"); }
    }
}

pub fn preparar_pastas_overlay(name: &str) {
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

pub fn montar_overlay_interno(name: &str) -> String {
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

pub fn init_db() -> SqlResult<Connection> {
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

pub fn configurar_cgroup(name: &str, pid: u32, memory_mb: Option<u32>) {
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

pub fn montar_volume(volume_arg: &str, merged_path: &str) {
    let partes: Vec<&str> = volume_arg.split(':').collect();
    if partes.len() != 2 {
        eprintln!("Erro: O volume deve estar no formato host_path:container_path");
        return;
    }

    let host_path = partes[0];
    let container_path = partes[1].trim_start_matches('/'); 
    let target_path = format!("{}/{}", merged_path, container_path);

    if !Path::new(host_path).exists() {
        eprintln!("Erro: Caminho de volume no host não existe: {}", host_path);
        return;
    }

    if !Path::new(&target_path).exists() {
        fs::create_dir_all(&target_path).expect("Falha ao criar diretório de montagem no container");
    }

    mount(
        Some(host_path),
        target_path.as_str(),
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&str>,
    ).expect("Falha ao montar volume bind");

    println!("Volume montado: {} -> {}", host_path, target_path);
}

pub fn desmontar_volume(path: &str) {
    let _ = umount(path);
}