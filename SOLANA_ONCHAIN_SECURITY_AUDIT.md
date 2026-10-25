# 🛡️ Relatório Oficial de Auditoria de Segurança On-Chain (Solana / Anchor)

**Arquitetura:** Graph Enterprise & Solana DevSecOps Agent
**Repositório:** SlipPay2
**Data de Execução:** 2026-10-15T22:53:24.861Z

## 📋 Escopo da Análise Estática
Ficheiros .rs escaneados no ecossistema Anchor:
- `crates/ast-engine/ast.rs`
- `crates/ast-engine/context.rs`
- `crates/ast-engine/evaluator.rs`
- `crates/ast-engine/examples/test.rs`
- `crates/ast-engine/lib.rs`
- `crates/ast-engine/parser.rs`
- `crates/ast-engine/src/ast.rs`
- `crates/ast-engine/src/context.rs`
- `crates/ast-engine/src/evaluator.rs`
- `crates/ast-engine/src/lib.rs`
- `crates/ast-engine/src/parser.rs`
- `crates/policy-engine/src/execution.rs`
- `crates/policy-engine/src/lib.rs`
- `crates/policy-engine/src/policy.rs`
- `crates/policy-engine/src/rules.rs`
- `crates/policy-engine/src/validator.rs`
- `crates/pq-security/src/dilithium.rs`
- `crates/pq-security/src/entropy.rs`
- `crates/pq-security/src/hybrid.rs`
- `crates/pq-security/src/key_rotation.rs`
- `crates/pq-security/src/kyber.rs`
- `crates/pq-security/src/lib.rs`
- `crates/pq-security/src/wallet.rs`
- `crates/risk-engine/src/anomaly.rs`
- `crates/risk-engine/src/fraud.rs`
- `crates/risk-engine/src/lib.rs`
- `crates/risk-engine/src/sanctions.rs`
- `crates/risk-engine/src/scoring.rs`
- `crates/risk-engine/src/velocity.rs`
- `crates/solana-engine/src/lib.rs`
- `crates/storage-engine/src/lib.rs`
- `programs/solana_sandbox_counter/src/lib.rs`
- `src/adapters/database/mod.rs`
- `src/adapters/external/mod.rs`
- `src/adapters/mod.rs`
- `src/adapters/stellar/mod.rs`
- `src/config/mod.rs`
- `src/domain/invoice/mod.rs`
- `src/domain/merchant/mod.rs`
- `src/domain/payment/mod.rs`
- `src/domain/settlement/mod.rs`
- `src/dto/mod.rs`
- `src/errors/mod.rs`
- `src/interfaces/api/mod.rs`
- `src/interfaces/mod.rs`
- `src/interfaces/webhook/mod.rs`
- `src/lib.rs`
- `src/main.rs`
- `src/models.rs`
- `src/observability/mod.rs`
- `src/repositories/mod.rs`
- `src/routes/orders.rs`
- `src/routes.rs`
- `src/services/api.rs`
- `src/services/db.rs`
- `src/services/ingestion/mod.rs`
- `src/services/listener.rs`
- `src/services/mod.rs`
- `src/services/orchestration/mod.rs`
- `src/services/reconciler.rs`
- `src/services/reconciliation/mod.rs`
- `src/services/repository/charge_repository.rs`
- `src/services/repository/mod.rs`
- `src/services/repository/payment_repository.rs`
- `src/services/security/mod.rs`
- `src/utils/mod.rs`
- `src/utils/stellar.rs`
- `src/webhook.rs`
- `tests/integration.rs`

## 🔍 Diagnóstico de Vulnerabilidades e Vetores Solana

### 1. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/ast-engine/evaluator.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/ast-engine/evaluator.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 2. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/ast-engine/examples/test.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/ast-engine/examples/test.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 3. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/ast-engine/parser.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/ast-engine/parser.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 4. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/ast-engine/src/ast.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/ast-engine/src/ast.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 5. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/ast-engine/src/evaluator.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/ast-engine/src/evaluator.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 6. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/ast-engine/src/lib.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/ast-engine/src/lib.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 7. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/ast-engine/src/parser.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/ast-engine/src/parser.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 8. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/policy-engine/src/execution.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/policy-engine/src/execution.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 9. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/policy-engine/src/lib.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/policy-engine/src/lib.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 10. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/policy-engine/src/policy.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/policy-engine/src/policy.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 11. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/policy-engine/src/rules.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/policy-engine/src/rules.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 12. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/policy-engine/src/validator.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/policy-engine/src/validator.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 13. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/pq-security/src/dilithium.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/pq-security/src/dilithium.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 14. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/pq-security/src/hybrid.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/pq-security/src/hybrid.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 15. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/pq-security/src/kyber.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/pq-security/src/kyber.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 16. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/pq-security/src/lib.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/pq-security/src/lib.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 17. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/pq-security/src/wallet.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/pq-security/src/wallet.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 18. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/risk-engine/src/anomaly.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/risk-engine/src/anomaly.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 19. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/risk-engine/src/fraud.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/risk-engine/src/fraud.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 20. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/risk-engine/src/lib.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/risk-engine/src/lib.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 21. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/risk-engine/src/sanctions.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/risk-engine/src/sanctions.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 22. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/risk-engine/src/scoring.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/risk-engine/src/scoring.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 23. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/risk-engine/src/velocity.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/risk-engine/src/velocity.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 24. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/solana-engine/src/lib.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/solana-engine/src/lib.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 25. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `crates/storage-engine/src/lib.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em crates/storage-engine/src/lib.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 26. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `programs/solana_sandbox_counter/src/lib.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em programs/solana_sandbox_counter/src/lib.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 27. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `src/lib.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em src/lib.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 28. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `src/main.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em src/main.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 29. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `src/routes/orders.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em src/routes/orders.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 30. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `src/routes.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em src/routes.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 31. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `src/services/api.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em src/services/api.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 32. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `src/services/db.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em src/services/db.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 33. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `src/services/repository/charge_repository.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em src/services/repository/charge_repository.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 34. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `src/services/repository/payment_repository.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em src/services/repository/payment_repository.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 35. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `src/utils/stellar.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em src/utils/stellar.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 36. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `src/webhook.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em src/webhook.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

### 37. [MEDIUM] CWE-190: Integer Overflow / Underflow Risk
- **Ficheiro:** `tests/integration.rs`
- **Descrição:** Uso de operações aritméticas diretas sem métodos checked_* ou saturating_* em tests/integration.rs.
- **Remediação Recomendada:** Substituir operações aritméticas nativas por .checked_add(), .checked_sub(), ou garantir ativação de overflow-checks no Cargo.toml.

