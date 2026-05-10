# SlipPay 🚀

Gateway de pagamentos blockchain construído sobre a rede Stellar.

O SlipPay permite criar cobranças em BRL, gerar pagamentos em XLM, monitorar transações on-chain e confirmar pagamentos automaticamente utilizando memos hash na Stellar Testnet.

---

# 🌐 MVP Online

Acesso público temporário via Cloudflare Tunnel.

---

# ✨ Funcionalidades

- ✅ Criação de cobranças
- ✅ Conversão BRL → XLM
- ✅ Geração automática de memo hash
- ✅ Listener Stellar em tempo real
- ✅ Confirmação automática de pagamentos
- ✅ API REST
- ✅ Frontend Web
- ✅ Persistência SQLite
- ✅ Integração Stellar Testnet
- ✅ Cloudflare Tunnel
- ✅ GitHub Sync automático
- ✅ Precisão financeira usando Decimal

---

# 🏗️ Arquitetura

```text
Frontend (HTML/CSS/JS)
        ↓
Actix-Web API (Rust)
        ↓
SQLite
        ↓
Stellar Horizon API
        ↓
Blockchain Stellar Testnet
