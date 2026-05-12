SlipPay

SlipPay é um gateway de pagamentos nativo da Stellar focado em infraestrutura Pix-to-Stellar e liquidação blockchain para e-commerce, SaaS e pagamentos digitais.

O projeto está sendo desenvolvido como um processador de pagamentos não custodial, leve e modular, com foco em:

integração com Stellar

reconciliação baseada em memo

confirmação on-chain

webhooks

persistência de estados

arquitetura limpa

compatibilidade com e-commerce

manutenção simplificada

infraestrutura modular em Rust



---

Status Atual

MVP em desenvolvimento

Atualmente o SlipPay opera sobre:

Stellar Testnet

Backend em Rust

SQLite para persistência

API em Actix Web

Estrutura de polling Horizon

Listener modular

Reconciliador de pagamentos


A infraestrutura principal já está funcional e modularizada.


---

Funcionalidades Implementadas

Pagamentos

criação de cobranças

geração de memo único

persistência de pagamentos

rastreamento de status

lógica de conversão BRL → XLM


Infraestrutura Blockchain

integração com Stellar Horizon

listener de pagamentos

reconciliador baseado em memo

estrutura para verificação de transações

fluxo de confirmação on-chain


Arquitetura Backend

arquitetura modular em Rust

separação de camadas

camada de repositório

serviço de reconciliação

serviço de webhook

serviço de listener

persistência de estados


Webhooks

estrutura de eventos

preparação para retries

base para idempotência

fluxo de confirmação automática



---

Arquitetura

buyer
↓
SlipPay API
├── SQLite Persistence
├── Stellar Listener
├── Reconciler
├── Webhook Dispatcher
↓
Stellar Network


---

Stack Tecnológica

Backend

Rust

Actix Web

Tokio

Rusqlite

Serde


Blockchain

Stellar Testnet

Horizon API

Reconciliação por Memo


Banco de Dados

SQLite



---

Estrutura do Projeto

src/
├── main.rs
├── db.rs
├── stellar.rs
├── reconciler.rs
├── listener.rs
├── webhook.rs
├── models.rs
├── repository/
└── routes/


---

Execução Local

Requisitos

Rust

Cargo

SQLite

Linux ou Termux


Inicialização

O projeto pode ser compilado e executado localmente utilizando Cargo.

O servidor roda na porta 8081.


---

Funcionalidades em Desenvolvimento

polling real da Horizon

reconciliação automática

processamento idempotente

verificação real de transações Stellar

assinatura segura de webhooks

sistema de retries

persistência avançada de estados



---

Roadmap

Próximos Passos

suporte a USDC

integração Pix

dashboard merchant

SDK de checkout

billing recorrente

suporte multi-wallet

reconciliador de produção



---

Princípios do Projeto

SlipPay está sendo desenvolvido com foco em:

arquitetura limpa

liquidação determinística

serviços modulares

baixo consumo de infraestrutura

compatibilidade com e-commerce

manutenção simplificada

fluxo não custodial



---

Aviso

SlipPay ainda é um MVP experimental em desenvolvimento ativo.

A implementação atual roda em Stellar Testnet e não deve ser utilizada em produção financeira real neste estágio.


---

Licença

MIT