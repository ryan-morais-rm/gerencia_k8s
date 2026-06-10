# Projeto — Atualização Automática de Containers Baseada em Novas Versões de Imagens Docker

## Tema do Projeto

Desenvolvimento de um sistema automatizado capaz de monitorar imagens Docker com tag `latest`, detectar novas versões publicadas em um registry e atualizar automaticamente os containers em execução.

---

# Objetivo do Projeto

O objetivo deste projeto é desenvolver uma solução automatizada capaz de monitorar periodicamente imagens Docker publicadas em registries de containers e identificar alterações em imagens que utilizam a tag: [**latest**]

Quando uma nova versão da imagem for detectada no registry, o sistema deverá:

* identificar a atualização;
* baixar a nova imagem;
* interromper o container antigo;
* remover o container desatualizado;
* criar um novo container utilizando a imagem atualizada;
* restaurar a execução da aplicação automaticamente.

O projeto busca explorar conceitos fundamentais relacionados a:

* gerenciamento de containers;
* automação de infraestrutura;
* versionamento de imagens;
* registries Docker;
* CI/CD;
* rolling updates;
* atualização contínua;
* observabilidade e automação operacional.

Os alunos deverão compreender como plataformas modernas realizam atualização automatizada de workloads containerizados, explorando conceitos semelhantes aos utilizados por ferramentas como:

* **Watchtower**;
* **Kubernetes**;
* **Docker Swarm**;
* **Argo CD**.

---

# Competências Esperadas

Ao final do projeto, espera-se que os alunos sejam capazes de:

* compreender o funcionamento de registries Docker;
* monitorar alterações em imagens;
* automatizar atualização de containers;
* utilizar a API do Docker;
* trabalhar com automação operacional;
* compreender estratégias de atualização contínua;
* manipular imagens Docker;
* automatizar processos de deploy;
* implementar mecanismos de monitoramento.

---

# Requisitos Mínimos

O projeto deverá obrigatoriamente:

1. Monitorar periodicamente uma imagem Docker;
2. Trabalhar com imagens utilizando tag`latest`;
3. Verificar se houve alteração da imagem no registry;
4. Fazer download automático da nova imagem;
5. Identificar containers utilizando a imagem antiga;
6. Parar o container atual;
7. Remover o container antigo;
8. Criar novo container utilizando a nova imagem;
9. Restaurar a execução automaticamente;
10. Funcionar em ambiente Linux com Docker.

---

# Exemplo de Funcionamento Esperado

## Cenário Inicial

Container em execução:

```bash
docker run -d --name app nginx:latest
```

---

## Fluxo esperado

### 1. Verificação periódica

A aplicação verifica:

* digest da imagem;
* ID da imagem;
* data de atualização;
* manifesto remoto.

---

### 2. Nova versão detectada

Caso o registry possua nova versão:

```text
Nova imagem detectada para nginx:latest
```

---

### 3. Atualização automática

Fluxo:

* Parando container antigo
* Removendo container
* Baixando nova imagem
* Criando novo container
* Container atualizado com sucesso

---

# Sugestão de Implementação

## Estratégias possíveis

Os alunos poderão utilizar:

### 1. Docker CLI

Exemplo:

```bash
docker pull nginx:latest
docker inspect
docker ps
```

---

### 2. API Docker (Recomendado)

Utilizar:

* /var/run/docker.sock
  permitindo acesso à API Docker diretamente do container de gerenciamento.

---

# Exemplo de montagem do Docker Socket

```yaml
volumes:
  - /var/run/docker.sock:/var/run/docker.sock
```

---

# Estratégias de Verificação

## 1. Comparação de digest

Verificar:

* SHA256 local;
* SHA256 remoto.

---

## 2. Docker pull periódico

Executar:

* docker pull imagem:latest

e verificar se houve alteração.

---

## 3. Consulta ao Registry API

Consultar:

* manifestos;
* tags;
* timestamps;
* digest remoto.

# Requisitos Opcionais (Bônus)

Os grupos poderão implementar:

* rolling update;
* health check antes da troca;
* rollback automático;
* dashboard web;
* logs persistentes;
* atualização paralela;
* suporte a múltiplos containers;
* notificações:
  * email;
  * Telegram;
  * Discord;
* métricas Prometheus;
* integração Grafana;
* suporte OCI Registry;
* atualização baseada em semver;
* política de atualização;
* múltiplos registries.

---

# Possíveis Estratégias Avançadas

## Rolling Update

Atualizar:

* sem downtime;
* mantendo disponibilidade.

---

## Blue/Green Deployment

Criar novo container antes de remover o antigo.

---

## Canary Update

Atualização parcial gradual.

---

# Sugestão de Estrutura do Projeto

```text
project/
├── updater/
├── scheduler/
├── logs/
├── config/
├── docker/
├── main.py
├── docker-compose.yml
└── README.md
```

---

# Modelo de Avaliação


| Critério                             | Peso |
| --------------------------------------- | ------ |
| Detecção correta de atualização   | 25%  |
| Atualização automática funcionando | 25%  |
| Integração com Docker API           | 15%  |
| Confiabilidade da automação         | 10%  |
| Organização do código              | 10%  |
| Documentação                        | 10%  |
| Funcionalidades extras                | 5%   |

---

# Forma de Entrega

## Cada grupo deverá entregar:

### 1. Código-fonte

Disponibilizado em repositório Git.

### 2. README.md contendo

* arquitetura da solução;
* funcionamento da detecção;
* estratégias de atualização;
* instruções de execução;
* limitações conhecidas.

E  também:

* dificuldades encontradas;
* aprendizados obtidos.

### 3. Vídeo demonstrativo

Entre 5 e 15 minutos contendo:

* detecção da nova imagem;
* atualização automática;
* substituição do container;
* logs do processo.

# Sugestões de Expansão do Projeto

Os grupos mais avançados poderão evoluir o projeto para:

* criar ferramenta semelhante ao **Watchtower**;
* implementar dashboard web;
* suportar múltiplos hosts Docker;
* integrar com Kubernetes;
* implementar políticas inteligentes de atualização;
* adicionar observabilidade;
* criar sistema de rollback avançado;
* integrar notificações e auditoria.

Isso permitirá aos alunos compreender profundamente como plataformas modernas realizam atualização automatizada, gerenciamento contínuo e orquestração de containers em ambientes cloud-native.
