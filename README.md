🚀 SlipPay — Ficha Técnica + README

Visão Geral

O SlipPay é um gateway de pagamentos baseado na blockchain Stellar, desenvolvido em Rust com foco em:

segurança

processamento assíncrono

confirmação automática de pagamentos

arquitetura incremental

compatibilidade com Stellar Testnet/Mainnet

persistência local com SQLite

integração futura com frontend e APIs externas


O projeto foi desenvolvido inicialmente em ambiente Android via Termux, utilizando Rust nativo.


---

🧠 Objetivo do Projeto

O objetivo do SlipPay é fornecer:

criação de pedidos de pagamento

geração automática de memos criptográficos

listener blockchain em tempo real

confirmação automática de transações Stellar

base arquitetural para um gateway de pagamento profissional



---

🏗 Arquitetura Atual

Backend

Tecnologia principal:

Rust


Framework HTTP:

Actix Web


Banco de dados:

SQLite


Blockchain:

Stellar Testnet


Listener:

Horizon API


Assinatura criptográfica:

HMAC SHA-256


Formato de memo:

Memo Hash


Persistência:

SQLite local



---

🔐 Segurança Implementada

HMAC SHA-256

Todas as requisições de criação de pedidos utilizam:

HMAC SHA-256

assinatura obrigatória

validação server-side


Header obrigatório:

x-signature


---

Memo Hash

O sistema utiliza:

memo_type=hash

hash SHA-256

compatível com wallets Stellar profissionais


Benefícios:

maior segurança

menor risco de colisão

compatibilidade com Horizon

prevenção de memos simples previsíveis



---

📦 Funcionalidades Já Implementadas

✅ API HTTP

GET /

Retorna homepage simples do SlipPay.


---

POST /orders

Cria um pedido de pagamento.

Entrada:

{
  "valor_brl": 100
}

Resposta:

{
  "id": "uuid",
  "valor_brl": 100,
  "valor_xlm": 20,
  "memo": "hash",
  "payment_uri": "stellar:...",
  "status": "pending"
}


---

GET /orders

Lista todos os pedidos.

Exemplo:

[
  {
    "id": "uuid",
    "status": "confirmed",
    "valor_brl": 100,
    "tx_hash": "stellar_tx"
  }
]


---

🌌 Listener Blockchain

Funcionamento

O SlipPay possui um listener assíncrono que:

consulta Horizon API

monitora transações da conta Stellar

detecta memos automaticamente

confirma pagamentos

salva hash da transação



---

Cursor Persistente

O sistema implementa:

paging_token persistente

prevenção de replay

leitura incremental

consumo contínuo de eventos


Tabela:

listener_state

Campos:

id
paging_token


---

💾 Banco de Dados

orders

CREATE TABLE orders (
    id TEXT PRIMARY KEY,
    valor_brl REAL NOT NULL,
    valor_xlm REAL NOT NULL,
    memo TEXT NOT NULL,
    tx_hash TEXT,
    status TEXT NOT NULL
)


---

listener_state

CREATE TABLE listener_state (
    id INTEGER PRIMARY KEY,
    paging_token TEXT
)


---

🔄 Fluxo Atual do Sistema

1. Cliente cria pedido

Cliente -> POST /orders


---

2. Backend gera

UUID

memo hash

payment URI

registro SQLite



---

3. Cliente envia pagamento Stellar

Utilizando:

destination

amount

memo hash



---

4. Listener detecta transação

Consulta Horizon:

/accounts/{account}/transactions


---

5. Sistema confirma pedido

Atualiza:

status = confirmed

E salva:

tx_hash


---

🧪 Testes Realizados

Testes de API

Realizados com:

curl


---

Testes blockchain

Realizados via:

Stellar Laboratory

Stellar Testnet



---

Testes confirmados

✅ criação de pedidos

✅ assinatura HMAC

✅ persistência SQLite

✅ geração de memos

✅ geração de payment URI

✅ listener blockchain

✅ confirmação automática

✅ prevenção de replay

✅ cursor incremental

✅ memo hash Base64 -> HEX


---

📂 Estrutura Atual do Projeto

slippay-rust/
│
├── Cargo.toml
├── Cargo.lock
├── README.md
├── slippay.db
├── index.html
│
├── src/
│   └── main.rs
│
└── target/


---

📚 Dependências Rust

Principais crates

actix-web
rusqlite
serde
serde_json
uuid
reqwest
tokio
sha2
hmac
hex
base64


---

⚙️ Ambiente de Desenvolvimento

Plataforma atual

Android

Termux



---

Compilador

cargo run


---

🌐 Endpoints Disponíveis

Homepage

GET /


---

Criar pedido

POST /orders


---

Listar pedidos

GET /orders


---

🔐 Exemplo de Assinatura HMAC

PAYLOAD='{"valor_brl":100}'

SIG=$(echo -n $PAYLOAD | openssl dgst -sha256 -hmac "super-secret-key" | sed 's/^.* //')


---

🚀 Exemplo de Criação de Pedido

curl -X POST http://127.0.0.1:8081/orders \
-H "x-signature: $SIG" \
-H "Content-Type: application/json" \
-d "$PAYLOAD"


---

🌌 Testnet Stellar

Conta utilizada:

GB4TW32HFZEQMTS67U33D6GD36ZHTMEPAVFOIEPWXWY5QYFQDE3PC7QT


---

🧭 Roadmap Futuro

Próximas implementações

✅ QR Code

✅ Dashboard Web

✅ Atualização em tempo real

✅ Deploy VPS

✅ Dockerização

✅ Mainnet Stellar

✅ Webhooks

✅ Multi-wallet

✅ Conversão BRL/XLM real

✅ Rate provider

✅ JWT auth

✅ Painel administrativo

✅ Logs estruturados

✅ Websocket

✅ Anti-fraude de amount

✅ Validação de asset

✅ Multi-moeda


---

🛡 Considerações Técnicas

O SlipPay já implementa conceitos utilizados em sistemas profissionais:

blockchain watchers

payment processors

event listeners

incremental consumers

replay protection

hash memos

HMAC authentication

asynchronous architecture



---

📌 Estado Atual do Projeto

Backend

✅ funcional


---

Blockchain Listener

✅ funcional


---

Persistência

✅ funcional


---

Segurança

✅ funcional


---

Frontend

🚧 em desenvolvimento


---

Mainnet

🚧 futura implementação


---

👨‍💻 Projeto

Projeto desenvolvido como base para um gateway de pagamento blockchain focado em Stellar.

Arquitetura voltada para:

escalabilidade

segurança

modularidade

integração futura enterprise



---

📄 Licença

Projeto em desenvolvimento experimental.

Uso educacional e arquitetural.

## 📦 Instalação

Clone o repositório:
```bash
git clone https://github.com/mrcoantonioconceicao-ctrl/SlipPay2.git
cd SlipPay2


