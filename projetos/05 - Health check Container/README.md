# Projeto — Monitoramento e Auto-Recovery de Containers com Health Check

## Tema do Projeto

Desenvolvimento de um sistema automatizado de monitoramento de containers Docker capaz de verificar a saúde de aplicações em execução e realizar ações automáticas de recuperação em caso de falhas.

---

# Objetivo do Projeto

O objetivo deste projeto é desenvolver uma solução baseada em containers capaz de monitorar continuamente a saúde de outros containers Docker e executar ações automáticas de recuperação quando forem identificadas falhas ou indisponibilidades.

A aplicação deverá atuar como um sistema de supervisão, realizando health checks periódicos nos containers cadastrados e validando se as aplicações continuam operando corretamente.

Caso algum problema seja identificado, o sistema deverá:

* detectar a falha;
* registrar o evento;
* reiniciar automaticamente o container afetado;
* monitorar o retorno da aplicação;
* registrar histórico das ocorrências.

O projeto busca explorar conceitos fundamentais relacionados a:

* health checks;
* alta disponibilidade;
* automação operacional;
* observabilidade;
* gerenciamento de containers;
* auto-healing;
* tolerância a falhas;
* supervisão de serviços.

Além disso, os alunos irão compreender conceitos utilizados em plataformas modernas como:

* **Kubernetes**;
* **Docker Swarm**;
* **systemd**;
* **Monit**;
* **supervisord**.

---

# Competências Esperadas

Ao final do projeto, espera-se que os alunos sejam capazes de:

* compreender conceitos de health check;
* monitorar containers Docker;
* automatizar recuperação de serviços;
* utilizar a API Docker;
* implementar mecanismos de supervisão;
* compreender tolerância a falhas;
* trabalhar com automação operacional;
* registrar logs e eventos;
* implementar observabilidade básica.

---

# Requisitos Mínimos

O projeto deverá obrigatoriamente:

1. Monitorar containers Docker;
2. Permitir cadastro de containers monitorados;
3. Realizar health checks periódicos;
4. Detectar indisponibilidade do serviço;
5. Reiniciar automaticamente containers com falha;
6. Registrar logs das ocorrências;
7. Monitorar continuamente o retorno do serviço;
8. Funcionar em ambiente Linux com Docker;
9. Possuir configuração simples;
10. Exibir status atual dos containers monitorados.

---

# Exemplos de Estratégias de Health Check

Os alunos poderão implementar verificações como:

* teste HTTP;
* conexão TCP;
* verificação de processo;
* teste de porta;
* execução de comando interno;
* validação de resposta da aplicação;
* consumo de CPU/memória;
* status do Docker HealthCheck.

---

# Exemplo de Configuração

```yaml
containers:
  - name: app-fin
    healthcheck:
      type: http
      url: http://app-fin:8080/health
      interval: 30

  - name: nginx
    healthcheck:
      type: tcp
      host: nginx
      port: 80
```

---

# Fluxo Esperado

## 1. Monitoramento periódico

A aplicação realiza:

* consultas;
* testes;
* verificações de status.

---

## 2. Falha detectada

Exemplo:

```text
Health check falhou para container app-fin
```

---

## 3. Recuperação automática

Fluxo:

* Parando container
* Reiniciando container
* Aguardando inicialização
* Executando novo health check

---

## 4. Retorno do serviço

Exemplo:

* Container app-fin recuperado com sucesso

---

# Sugestão de Implementação

## Integração com Docker API

Uma das principais sugestões é utilizar o socket Docker:

* /var/run/docker.sock

permitindo comunicação direta com o Docker Engine.

---

# Exemplo de montagem do Docker Socket

```yaml
volumes:
  - /var/run/docker.sock:/var/run/docker.sock
```

---

# Possibilidades Utilizando Docker Socket

Os alunos poderão:

* listar containers;
* obter status;
* reiniciar containers;
* consultar health status;
* acompanhar eventos do Docker;
* verificar logs.

---

# Estratégias de Detecção

## 1. HTTP Health Check

Exemplo:

* GET /health

---

## 2. TCP Check

Validar:

* porta aberta;
* resposta TCP.

---

## 3. Docker Native HealthCheck

Consultar:

* docker inspect

---

## 4. Process Check

Verificar:

* PID ativo;
* processo principal.

# Requisitos Opcionais (Bônus)

Os grupos poderão implementar:

* dashboard web;
* métricas Prometheus;
* integração Grafana;
* notificações:
  * Telegram;
  * Discord;
  * Email;
* múltiplos tipos de health checks;
* rollback automático;
* retry inteligente;
* limite de reinicializações;
* análise de logs;
* persistência SQLite;
* cluster distribuído;
* monitoramento multi-host;
* escalabilidade automática.

---

# Sugestões Técnicas Avançadas

## Auto-Healing Inteligente

Evitar:

* loop infinito de restart;
* restart excessivo.

---

## Circuit Breaker

Bloquear temporariamente:

* containers instáveis.

---

## Graceful Recovery

Aguardar:

* tempo de inicialização;
* warm-up da aplicação.

---

# Sugestão de Estrutura do Projeto

```text
project/
├── monitor/
├── healthchecks/
├── recovery/
├── logs/
├── config/
├── database/
├── docker/
├── main.py
└── README.md
```

---

# Modelo de Avaliação


| Critério                     | Peso |
| ------------------------------- | ------ |
| Funcionamento do health check | 25%  |
| Recuperação automática     | 25%  |
| Integração com Docker API   | 15%  |
| Confiabilidade da solução   | 10%  |
| Organização do código      | 10%  |
| Documentação                | 10%  |
| Funcionalidades extras        | 5%   |

---

# Forma de Entrega

## Cada grupo deverá entregar:

### 1. Código-fonte

Disponibilizado em repositório Git.

### 2. README.md contendo

* arquitetura da solução;
* tipos de health checks;
* funcionamento da recuperação;
* instruções de uso;
* limitações conhecidas.

E também:

* problemas encontrados;
* aprendizados obtidos.

### 3. Vídeo demonstrativo

Entre 5 e 15 minutos contendo:

* falha simulada;
* detecção automática;
* restart do container;
* recuperação do serviço.

# Sugestões de Expansão do Projeto

Os grupos mais avançados poderão evoluir o projeto para:

* criar sistema semelhante ao auto-healing do **Kubernetes**;
* implementar monitoramento distribuído;
* suportar múltiplos hosts Docker;
* criar dashboard observabilidade;
* integrar Prometheus e Grafana;
* implementar inteligência para análise de falhas;
* adicionar machine learning para previsão de falhas;
* criar sistema completo de gerenciamento de containers.

Isso permitirá aos alunos compreender profundamente como plataformas modernas realizam monitoramento, tolerância a falhas e recuperação automática em ambientes distribuídos e containerizados.
