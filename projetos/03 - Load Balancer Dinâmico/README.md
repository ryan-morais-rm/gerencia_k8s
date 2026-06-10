# Projeto — Load Balancer Dinâmico com Auto-Discovery de Containers no Docker

## Tema do Projeto

Desenvolvimento de um sistema de balanceamento de carga dinâmico utilizando **NGINX** com descoberta automática de containers Docker através de labels (tags) e integração com a API do Docker via socket Unix.

## Objetivo do Projeto

O objetivo deste projeto é desenvolver uma solução de load balancing baseada em containers, utilizando o **NGINX** como proxy reverso e balanceador de carga, capaz de descobrir automaticamente aplicações Docker em execução através de labels configuradas nos containers.

A solução deverá monitorar continuamente o ambiente Docker e identificar containers elegíveis para balanceamento utilizando labels ou tags como:

> Obs. Quando me referir a **label**, na verdade estou também me referindo a **tags**. No docker é possível atribuir várias tags no mesmo container, e podemos utilizá-las para selecionar e categorizar os containers.

```text
lb.enable=true
lb.port=9000
app=fin
```

Com base nessas informações, o sistema deverá:

* identificar automaticamente novos containers;
* descobrir IP e porta dos serviços;
* gerar dinamicamente a configuração do NGINX;
* atualizar o balanceamento automaticamente;
* remover containers indisponíveis;
* realizar reload do NGINX sem interrupção do serviço.

O projeto também deverá explorar a integração direta com a API do Docker através do socket Unix:

```bash
/var/run/docker.sock
```

permitindo que o próprio container do load balancer consulte eventos, containers ativos, labels e redes Docker em tempo real.

O projeto busca proporcionar aos alunos uma compreensão prática sobre:

* service discovery;
* reverse proxy;
* integração com API Docker;
* automação de infraestrutura;
* microsserviços;
* comunicação entre containers;
* balanceamento de carga dinâmico;
* arquiteturas modernas cloud-native.

Além disso, os alunos irão explorar conceitos semelhantes aos utilizados por ferramentas como:

* **Traefik**;
* **HAProxy**;
* **NGINX**;
* **Kubernetes**;
* **Docker Swarm**.

## Competências Esperadas

Ao final do projeto, espera-se que os alunos sejam capazes de:

* compreender o funcionamento de load balancers;
* configurar e automatizar o NGINX;
* utilizar labels em containers Docker;
* consumir a API do Docker;
* utilizar o socket Docker Unix;
* implementar service discovery;
* automatizar geração de configuração;
* trabalhar com redes Docker;
* compreender arquiteturas distribuídas;
* implementar reload dinâmico de serviços.


# Requisitos Mínimos

O projeto deverá obrigatoriamente:

1. Executar o NGINX em container;
2. Descobrir containers automaticamente;
3. Filtrar containers através de labels;
4. Consumir informações da API Docker;
5. Identificar IP e porta do container;
6. Gerar configuração dinâmica do NGINX;
7. Implementar balanceamento de carga;
8. Atualizar configuração automaticamente;
9. Recarregar o NGINX dinamicamente;
10. Remover containers indisponíveis do balanceamento;
11. Funcionar em ambiente Linux com Docker.

## Exemplo de Labels/tags Esperadas

Os containers das aplicações poderão possuir:
```yaml
tags:
  - lb.enable=true
  - lb.port=9000
  - app=fin
```

## Exemplo de Funcionamento Esperado

### Containers ativos

```text
fin-app-1
fin-app-2
fin-app-3
```

Todos contendo:

```text
lb.enable=true
lb.port=9000
app=fin
```

---

### Configuração gerada automaticamente

```text
upstream fin_backend {
    server 172.18.0.2:9000;
    server 172.18.0.3:9000;
    server 172.18.0.4:9000;
}

server {
    listen 80;

    location / {
        proxy_pass http://fin_backend;
    }
}
```

## Sugestão de Implementação — API Docker via Docker Socket

### Uso do socket Docker

Uma das principais sugestões deste projeto é utilizar o socket Unix do Docker:

```bash
/var/run/docker.sock
```

Esse socket permite que aplicações consultem diretamente a API interna do Docker sem necessidade de CLI externa.

---

## Exemplo de montagem do socket no container

```yaml
volumes:
  - /var/run/docker.sock:/var/run/docker.sock
```

## Possibilidades Utilizando Docker Socket

Os alunos poderão:

* listar containers ativos;
* obter labels;
* identificar redes;
* descobrir IPs;
* monitorar eventos do Docker;
* detectar start/stop de containers;
* atualizar automaticamente o balanceamento.

---

# Estratégias de Descoberta de Containers

## 1. Polling periódico

Consultar periodicamente:

```bash
docker ps
docker inspect
```

## 2. Docker Events API (Recomendado)

Monitorar eventos em tempo real:

* container start;
* stop;
* destroy;
* die.

---
# Fluxo de Funcionamento

## 1. Monitorar Docker

A aplicação monitora:

* containers ativos;
* eventos do Docker;
* labels.


## 2. Filtrar containers elegíveis

Selecionar:

```text
lb.enable=true
```


## 3. Obter informações

Extrair:

* IP;
* porta;
* nome da aplicação;
* status.

---

## 4. Gerar configuração NGINX

Criar:

* upstreams;
* rotas;
* regras.


## 5. Recarregar NGINX

Aplicar:

```bash
nginx -s reload
```

# Sugestões de Discussões Técnicas

Os grupos poderão analisar:

* riscos de segurança do docker.sock;
* isolamento entre containers;
* privilégios excessivos;
* alternativas ao socket Docker;
* impacto de reload dinâmico;
* service discovery em larga escala.

---

# Requisitos Opcionais (Bônus)

Os grupos poderão implementar:

* HTTPS automático;
* health check;
* múltiplos upstreams;
* dashboard web;
* métricas Prometheus;
* integração Grafana;
* failover automático;
* sticky session;
* cache reverso;
* autenticação;
* suporte a múltiplos hosts Docker;
* suporte OCI;
* integração com **Kubernetes**.

---

# Modelo de Avaliação


| Critério                         | Peso |
| ----------------------------------- | ------ |
| Auto-discovery funcionando        | 25%  |
| Integração com API Docker       | 20%  |
| Configuração dinâmica do NGINX | 20%  |
| Balanceamento funcionando         | 15%  |
| Organização do código          | 10%  |
| Documentação                    | 5%   |
| Funcionalidades extras            | 5%   |

---

# Forma de Entrega

## Cada grupo deverá entregar:

### 1. Código-fonte

Disponibilizado em repositório Git.

---

### 2. README.md contendo

* arquitetura da solução;
* instruções de instalação;
* labels suportadas;
* explicação do Docker Socket;
* exemplos de execução.
e também:
* dificuldades encontradas;
* aprendizados obtidos.

### 3. Vídeo demonstrativo

Entre 5 e 15 minutos contendo:

* descoberta automática;
* atualização dinâmica;
* subida/remoção de containers;
* funcionamento do balanceamento.

# Perguntas Sobre o Projeto

1. O que é service discovery?
2. Qual a função de um load balancer?
3. Como o **NGINX** realiza balanceamento?
4. O que são labels no Docker?
5. Como a API do Docker funciona?
6. Qual a função do arquivo:

* /var/run/docker.sock

7. Quais riscos de segurança existem ao expor o docker.sock para containers?
8. Como o Docker fornece IP para containers?
9. O que é upstream no NGINX?
10. Como evitar downtime durante reload do NGINX?
11. Como ferramentas como **Traefik** utilizam auto-discovery?
12. Como implementar health checks automáticos?
13. Como detectar containers indisponíveis?
14. Qual a diferença entre reverse proxy e ingress controller?
15. Como seria possível expandir o projeto para múltiplos hosts?
16. Qual a relação deste projeto com plataformas como **Kubernetes**?
17. Como o balanceamento impacta escalabilidade e disponibilidade?
18. Como reduzir privilégios ao utilizar Docker Socket?

---

# Sugestões de Expansão do Projeto

Os grupos mais avançados poderão evoluir o projeto para:

* criar um mini ingress controller;
* implementar métricas observabilidade;
* suportar autoscaling;
* integrar com Kubernetes;
* implementar certificados automáticos;
* desenvolver um proxy reverso semelhante ao **Traefik**;
* criar painel administrativo em tempo real;
* suportar ambientes distribuídos.

Isso permitirá aos alunos compreender profundamente como arquiteturas modernas realizam descoberta automática de serviços, balanceamento dinâmico e automação de infraestrutura em ambientes containerizados.
