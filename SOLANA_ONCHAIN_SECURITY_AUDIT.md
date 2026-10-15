# 🛡️ Relatório Oficial de Auditoria de Segurança On-Chain (Solana / Anchor)

**Arquitetura:** Graph Enterprise & Solana DevSecOps Agent
**Repositório Analisado:** SlipPay2
**Data de Execução:** 2026-10-15T23:05:36.349Z

## 📋 Escopo da Análise Estática e Metodologia
O presente relatório documenta a proveniência, o escopo e os resultados detalhados da análise estática aplicada aos contratos inteligentes e bibliotecas Rust em conformidade com as taxonomias de segurança:
- **CWE-862:** Missing Authorization / Verificação de Signatários e Autoridade em Contas Anchor (#[derive(Accounts)]).
- **CWE-190:** Integer Overflow / Underflow em Operações Aritméticas On-Chain.

## 🔍 Resultados Detalhados por Ficheiro

### 1. `crates/ast-engine/ast.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 2. `crates/ast-engine/context.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 3. `crates/ast-engine/evaluator.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 4. `crates/ast-engine/examples/test.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 5. `crates/ast-engine/lib.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 6. `crates/ast-engine/parser.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 7. `crates/ast-engine/src/ast.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 8. `crates/ast-engine/src/context.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 9. `crates/ast-engine/src/evaluator.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 10. `crates/ast-engine/src/lib.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 11. `crates/ast-engine/src/parser.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 12. `crates/policy-engine/src/execution.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 13. `crates/policy-engine/src/lib.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 14. `crates/policy-engine/src/policy.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 15. `crates/policy-engine/src/rules.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 16. `crates/policy-engine/src/validator.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 17. `crates/pq-security/src/dilithium.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 18. `crates/pq-security/src/entropy.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 19. `crates/pq-security/src/hybrid.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 20. `crates/pq-security/src/key_rotation.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 21. `crates/pq-security/src/kyber.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 22. `crates/pq-security/src/lib.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 23. `crates/pq-security/src/wallet.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 24. `crates/risk-engine/src/anomaly.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 25. `crates/risk-engine/src/fraud.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 26. `crates/risk-engine/src/lib.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 27. `crates/risk-engine/src/sanctions.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 28. `crates/risk-engine/src/scoring.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 29. `crates/risk-engine/src/velocity.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 30. `crates/solana-engine/src/lib.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 31. `crates/storage-engine/src/lib.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 32. `programs/solana_sandbox_counter/src/lib.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Sim
- **Status CWE-862 (Autoridade / Signer):** Pass (Restrições de autoridade/signer identificadas no Anchor)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 33. `src/adapters/database/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 34. `src/adapters/external/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 35. `src/adapters/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 36. `src/adapters/stellar/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 37. `src/config/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 38. `src/domain/invoice/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 39. `src/domain/merchant/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 40. `src/domain/payment/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 41. `src/domain/settlement/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 42. `src/dto/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 43. `src/errors/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 44. `src/interfaces/api/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 45. `src/interfaces/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 46. `src/interfaces/webhook/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 47. `src/lib.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 48. `src/main.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 49. `src/models.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 50. `src/observability/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 51. `src/repositories/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 52. `src/routes/orders.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 53. `src/routes.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 54. `src/services/api.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 55. `src/services/db.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 56. `src/services/ingestion/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 57. `src/services/listener.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 58. `src/services/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 59. `src/services/orchestration/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 60. `src/services/reconciler.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 61. `src/services/reconciliation/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 62. `src/services/repository/charge_repository.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 63. `src/services/repository/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 64. `src/services/repository/payment_repository.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 65. `src/services/security/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 66. `src/utils/mod.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Pass (Sem operações aritméticas críticas ou uso de checked)

### 67. `src/utils/stellar.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 68. `src/webhook.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

### 69. `tests/integration.rs`
- **Contém Structs Anchor (`#[derive(Accounts)]`):** Não
- **Status CWE-862 (Autoridade / Signer):** Pass (Validado / Sem structs Anchor críticas)
- **Status CWE-190 (Aritmética Segura):** Warning / Review Required (Operações aritméticas sem checked_* detetadas)
- **Observações / Achados:**
  - CWE-190: Operações aritméticas nativas sem métodos checked_* ou saturating_*.

