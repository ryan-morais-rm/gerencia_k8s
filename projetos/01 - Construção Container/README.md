# Projeto — Construção de Containers Utilizando Recursos Nativos do Linux

## Tema do Projeto

Implementação de um mecanismo simplificado de containers utilizando recursos nativos do Linux, sem uso de plataformas prontas como Docker, Podman ou LXC.

## Objetivo do Projeto

O objetivo deste projeto é desenvolver um sistema simplificado de criação e gerenciamento de containers utilizando tecnologias nativas do sistema operacional Linux, explorando conceitos fundamentais de virtualização em nível de sistema operacional.

Os alunos deverão implementar um script ou aplicação capaz de criar ambientes isolados utilizando recursos como:

* namespaces;
* chroot;
* sistemas de arquivos em camadas (OverlayFS/UnionFS — citado como OnionFS no projeto);
* gerenciamento básico de processos;
* isolamento de recursos do sistema.

A solução deverá permitir que uma imagem base seja obtida a partir de um repositório Git, servindo como camada inicial do container. Durante a criação do container, o usuário deverá informar qual será o processo principal executado dentro do ambiente isolado.

Como recurso adicional, os alunos poderão utilizar um banco de dados SQLite para armazenar informações relacionadas aos containers criados, como:

* nome;
* PID principal;
* status;
* diretório raiz;
* data de criação;
* processo executado;
* portas ou configurações adicionais.

O projeto busca proporcionar uma compreensão prática de como tecnologias modernas de containers funcionam internamente, explorando os mesmos fundamentos utilizados por ferramentas como Docker, containerd e LXC.

## Competências Esperadas

Ao final do projeto, espera-se que os alunos sejam capazes de:

* compreender o funcionamento interno de containers;
* utilizar namespaces para isolamento de processos;
* manipular sistemas de arquivos isolados;
* utilizar OverlayFS/UnionFS para construção de camadas;
* compreender o papel do processo PID 1 em containers;
* gerenciar processos no Linux;
* automatizar tarefas utilizando scripts;
* compreender como runtimes de containers funcionam;
* aplicar conceitos de segurança e isolamento.

## Requisitos Mínimos

O projeto deverá obrigatoriamente:

1. Criar ambiente isolado utilizando namespaces;
2. Utilizar`chroot` para isolamento do sistema de arquivos;
3. Utilizar OverlayFS/UnionFS para criar camada de escrita do container;
4. Permitir informar o processo principal do container;
5. Baixar ou atualizar a camada base via Git;
6. Executar o processo dentro do ambiente isolado;
7. Permitir iniciar e remover containers;
8. Exibir containers em execução;
9. Registrar informações básicas do container;
10. Funcionar em ambiente Linux.

## Requisitos Opcionais (Bônus)

Os grupos poderão implementar funcionalidades extras, como:

* isolamento de rede utilizando network namespace;
* limitação de recursos com cgroups;
* mapeamento de portas;
* shell interativo no container;
* persistência de volumes;
* logs do container;
* snapshots;
* suporte a múltiplas imagens base;
* criação de comandos semelhantes ao Docker:
  * `run`
  * `ps`
  * `stop`
  * `rm`
  * `exec`
* interface web;
* monitoramento de containers;
* suporte a usuários não-root.

## Sugestões de Estrutura do Projeto

Estrutura básica sugerida

```text
project/
├── images/
├── containers/
├── overlays/
├── database/
├── scripts/
├── logs/
├── main.sh
├── container.db
└── README.md
```

## Modelo de Avaliação

Critérios de Avaliação


| Critério                            | Peso |
| -------------------------------------- | ------ |
| Funcionamento do container           | 25%  |
| Uso correto de namespaces            | 20%  |
| Uso correto de OverlayFS/UnionFS     | 15%  |
| Organização e qualidade do código | 10%  |
| Documentação do projeto            | 10%  |
| Apresentação prática              | 10%  |
| Funcionalidades extras               | 10%  |

asdasd


# Forma de Entrega

## Entrega obrigatória

Cada grupo deverá entregar:

### 1. Código-fonte

Disponibilizado em repositório Git.

### 2. README.md contendo:

* descrição do projeto;
* arquitetura da solução;
* instruções de instalação;
* instruções de uso;
* tecnologias utilizadas;
* limitações conhecidas;
* exemplos de execução.
Além disso, também:
* dificuldades encontradas;
* aprendizados obtidos.

### 3. Vídeo demonstrativo

Entre 5 e 15 minutos contendo:

* explicação da arquitetura;
* demonstração prática;
* criação e execução de containers;
* funcionamento do isolamento.