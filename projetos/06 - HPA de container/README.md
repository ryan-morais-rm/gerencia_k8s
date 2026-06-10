# Projeto — Autoscaling Horizontal de Containers no Docker (HPA Simplificado)

## Tema do Projeto

Desenvolvimento de um sistema de autoscaling horizontal para containers Docker, inspirado no funcionamento do HPA (Horizontal Pod Autoscaler) do **Kubernetes**.

---

# Objetivo do Projeto

O objetivo deste projeto é desenvolver uma solução automatizada capaz de monitorar o consumo de recursos de containers Docker e realizar escalabilidade horizontal automática baseada em métricas de utilização de CPU e memória.

A aplicação deverá atuar de forma semelhante ao HPA (Horizontal Pod Autoscaler) do **Kubernetes**, monitorando continuamente containers específicos e criando ou removendo réplicas automaticamente de acordo com regras definidas de consumo.

Os containers que poderão sofrer escalabilidade deverão ser identificados através de labels (tags) Docker, como por exemplo:

```yaml
autoscale.enable=true
autoscale.min=1
autoscale.max=5
autoscale.cpu=70
autoscale.memory=80
app=fin
```

Com base nessas informações, o sistema deverá:

* monitorar continuamente containers elegíveis;
* obter métricas de CPU e memória;
* detectar sobrecarga;
* criar novas réplicas automaticamente;
* remover réplicas quando houver baixa utilização;
* manter limites mínimos e máximos de instâncias;
* registrar eventos de scaling.

O projeto busca proporcionar aos alunos uma compreensão prática sobre:

* autoscaling horizontal;
* orquestração de containers;
* monitoramento de recursos;
* observabilidade;
* automação operacional;
* balanceamento de carga;
* tolerância a falhas;
* elasticidade computacional.

Além disso, os alunos irão explorar conceitos semelhantes aos utilizados por plataformas modernas como:

* **Kubernetes**;
* **Docker Swarm**;
* **Nomad**;
* **OpenShift**.

---

# Competências Esperadas

Ao final do projeto, espera-se que os alunos sejam capazes de:

* compreender conceitos de autoscaling horizontal;
* monitorar consumo de CPU e memória;
* consumir métricas de containers Docker;
* automatizar criação de containers;
* implementar políticas de scaling;
* compreender elasticidade computacional;
* trabalhar com automação de infraestrutura;
* implementar monitoramento contínuo;
* utilizar APIs do Docker.

---

# Requisitos Mínimos

O projeto deverá obrigatoriamente:

1. Monitorar containers Docker;
2. Selecionar containers através de labels;
3. Obter métricas de CPU;
4. Obter métricas de memória;
5. Implementar scaling horizontal automático;
6. Criar novas réplicas automaticamente;
7. Remover réplicas automaticamente;
8. Respeitar limites mínimo e máximo;
9. Registrar eventos de scaling;
10. Funcionar em ambiente Linux com Docker.

---

# Exemplo de Labels Esperadas

Os containers elegíveis poderão possuir:

```yaml
labels:
  - autoscale.enable=true
  - autoscale.min=1
  - autoscale.max=5
  - autoscale.cpu=70
  - autoscale.memory=80
  - app=fin
```

---

# Exemplo de Funcionamento Esperado

## Cenário Inicial

Container:

* fin-app-1

CPU:

* 85%

Threshold:

* 70%

---

## Resultado esperado

O sistema deverá:

* detectar sobrecarga;
* criar nova réplica:
  * fin-app-2

---

## Cenário posterior

Carga reduzida:

* 25%

O sistema poderá:

* remover réplicas excedentes;
* retornar à quantidade mínima configurada.

---

# Sugestão de Implementação

## Integração com Docker API

Uma das principais sugestões é utilizar o Docker Socket:

```text
/var/run/docker.sock
```

permitindo comunicação direta com o Docker Engine.

---

# Exemplo de montagem do Docker Socket

```yaml
volumes:
  - /var/run/docker.sock:/var/run/docker.sock
```

---

# Sugestão de Implementação — Coleta de CPU e Memória

## Opção 1 — Docker Stats (Mais simples)

Os alunos poderão utilizar:

```text
docker stats
```

Exemplo:

```text
docker stats --no-stream
```

## Opção 2 — Docker API (Recomendado)

Consultar métricas diretamente via API Docker utilizando:

```text
/containers/{id}/stats
```

---

# Exemplo de Informações Disponíveis

A API poderá fornecer:

* uso de CPU;
* limite de CPU;
* uso de memória;
* limite de memória;
* IO;
* rede;
* PIDs.

---

# Exemplo Simplificado de Métrica

```json
{
  "memory_usage": "512MB",
  "memory_limit": "1GB",
  "cpu_percent": "82%"
}
```

---

# Estratégias de Scaling

## Scale Out

Criar novos containers quando:

* CPU > 70%

ou:

* MEM > 80%

---

## Scale In

Remover containers quando:

* CPU < 20%

durante determinado período.

---

# Sugestões Técnicas Importantes

## Evitar Scaling Excessivo

Os alunos deverão implementar:

* cooldown;
* janela de observação;
* média móvel;
* limite de frequência.

---

# Sugestões de Balanceamento

O projeto poderá ser integrado com:

* **NGINX**;
* **HAProxy**;
* projeto anterior de auto-discovery.

---

# Linguagens Sugeridas

* Python;
* Go;
* Node.js.

---

# Bibliotecas Úteis

## Python

* `docker`
* `psutil`
* `schedule`

## Go

* Docker SDK
* Prometheus client

---

# Requisitos Opcionais (Bônus)

Os grupos poderão implementar:

* integração com load balancer;
* dashboard web;
* métricas Prometheus;
* integração Grafana;
* autoscaling baseado em requisições;
* cooldown inteligente;
* suporte multi-host;
* persistência SQLite;
* notificações;
* escalabilidade preditiva;
* suporte a GPU;
* algoritmos avançados de scaling.

---

# Possíveis Estratégias Avançadas

## Média móvel

Evitar scaling baseado em picos instantâneos.

---

## Histerese

Evitar:

* scale in/out contínuo.

---

## Warm-Up

Aguardar:

* inicialização da nova réplica antes de considerar saudável.

---

# Sugestão de Estrutura do Projeto

```text
project/
├── autoscaler/
├── metrics/
├── scaling/
├── logs/
├── config/
├── database/
├── docker/
├── main.py
└── README.md
```

# Modelo de Avaliação


| Critério                    | Peso |
| ------------------------------ | ------ |
| Coleta correta de métricas  | 20%  |
| Funcionamento do autoscaling | 25%  |
| Integração com Docker API  | 20%  |
| Estratégia de scaling       | 10%  |
| Organização do código     | 10%  |
| Documentação               | 10%  |
| Funcionalidades extras       | 5%   |

---

# Forma de Entrega

## Cada grupo deverá entregar:

### 1. Código-fonte

Disponibilizado em repositório Git.

### 2. README.md contendo

* arquitetura da solução;
* política de scaling;
* labels suportadas;
* estratégia de coleta de métricas;
* instruções de execução.

e também:

* dificuldades encontradas;
* aprendizados obtidos.

### 3. Vídeo demonstrativo

Entre 5 e 15 minutos contendo:

* aumento de carga;
* criação automática de réplicas;
* redução de carga;
* remoção automática de réplicas;
* exibição das métricas.

# Sugestões de Expansão do Projeto

Os grupos mais avançados poderão evoluir o projeto para:

* criar sistema semelhante ao HPA do **Kubernetes**;
* implementar integração com Prometheus;
* criar dashboard observabilidade;
* integrar com load balancer dinâmico;
* implementar autoscaling distribuído;
* adicionar IA para previsão de carga;
* suportar múltiplos hosts Docker;
* criar mini orquestrador de containers.

Isso permitirá aos alunos compreender profundamente como plataformas modernas realizam elasticidade automática, gerenciamento de carga e orquestração de aplicações containerizadas em ambientes distribuídos.
