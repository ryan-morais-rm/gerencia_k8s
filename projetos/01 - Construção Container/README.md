# Instruções de Uso
- Baixar imagem base: 
  * `sudo -E ./target/debug/nayr pull --repo <URL_DO_GIT_COM_ROOTFS>`
- Executar um container:
  * `sudo -E ./target/debug/nayr run --name <NOME> --memory <MB> --exec "<COMANDO>"`
- Listar container:
  * `sudo -E ./target/debug/nayr ps`
- Remover um container e liberar recurso:
  * `sudo -E ./target/debug/nayr rm --name <NOME>`

# Limitações Conhecidas
- Chroot vs Pivot Root: A aplicação utiliza chroot por motivos de prototipagem. Em um ambiente de produção real, seria mandatório o uso de pivot_root para evitar vulnerabilidades de "jailbreak".

- Falta de Namespace de Rede: O processo dentro do container ainda compartilha a interface de rede do Host.

- Dependência do Sudo: Alterações no cwd (current working directory) pelo comando sudo exigem a flag -E ou caminhos absolutos no código para referenciar adequadamente a pasta images/base.

# Dificuldades Encontradas
- Cgroups e Manipulação do Sysfs: Entender e interagir com os arquivos puros do Cgroups v2 diretamente na pasta /sys/fs/cgroup/ (ex: memory.max e cgroup.procs) foi um desafio, pois qualquer escrita falha ou formato incorreto é silenciosamente ignorado ou barrado pelo Kernel.

- Problemas de Linker (Compilação): Conectar bibliotecas Rust com C (como o SQLite nativo do host) gerou erros de linker (ld returned 1 exit status), resolvido de forma estática via features bundled no Cargo.

- Montagens Vazando para o Host: Inicialmente, montar o OverlayFS causava anomalias no sistema anfitrião. A solução exigiu compreender a aplicação profunda da flag de montagem MS_PRIVATE e MS_REC aplicadas recursivamente no novo namespace.

# Aprendizados Obtidos
- Desmistificação de Containers: Containers não são entidades ou máquinas virtuais; são puramente processos comuns limitados de forma cirúrgica pelo Kernel Linux através de abstrações (Namespaces e Cgroups).

- Sistema de Arquivos em Camadas: Domínio do funcionamento do OverlayFS e como diretórios como upper e workdir trabalham em conjunto para garantir transparência nas mutações de arquivos.

- Segurança Memory-Safe: A utilização da linguagem Rust demonstrou ser uma vantagem enorme frente à linguagem C. Interagir com syscalls inseguras do Kernel manteve uma alta rastreabilidade de erros sem causar vazamentos de memória (memory leaks) ou segmentation faults.