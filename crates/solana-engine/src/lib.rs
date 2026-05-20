use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use thiserror::Error;

/// 🚨 Dicionário de Erros Criptográficos Contextualizados para a Solana
#[derive(Error, Debug, serde::Serialize)]
#[serde(tag = "error_type", content = "context", rename_all = "snake_case")]
pub enum SolanaEngineError {
    #[error("A string fornecida '{input}' não está no formato Base58 válido da rede Solana.")]
    InvalidBase58Format { input: String },

    #[error("Tamanho de chave pública inválido. A Solana exige 32 bytes decodificados, mas foram recebidos {received} bytes.")]
    InvalidPubkeyLength { received: usize },

    #[error("Assinatura criptográfica inválida ou corrompida. Uma assinatura Solana precisa ter exatamente 64 bytes após a decodificação Base58.")]
    InvalidSignatureStructure,
}

#[derive(serde::Serialize)]
pub struct SolanaValidationResult {
    pub is_valid_signature: bool,
    pub tx_fee_lamports: u64,
    pub client_pubkey: String,
    pub amount_in_sol: f64,
}

pub struct SolanaVerifier;

impl SolanaVerifier {
    /// Valida chaves e assinaturas extraindo o contexto exato em caso de falha
    pub fn verify_payment_intent(
        wallet_str: &str, 
        amount: Decimal,
        mock_signature_str: &str
    ) -> Result<SolanaValidationResult, SolanaEngineError> {
        
        // 1. Tenta decodificar o Base58 capturando a string original se falhar
        let decoded_pubkey = bs58::decode(wallet_str)
            .into_vec()
            .map_err(|_| SolanaEngineError::InvalidBase58Format {
                input: wallet_str.to_string(),
            })?;

        // 2. Verifica o tamanho exato dos bytes da Pubkey
        if decoded_pubkey.len() != 32 {
            return Err(SolanaEngineError::InvalidPubkeyLength {
                received: decoded_pubkey.len(),
            });
        }

        // 3. Validação contextual da assinatura Base58 da Solana
        let is_valid_sig = if !mock_signature_str.is_empty() {
            if let Ok(decoded_sig) = bs58::decode(mock_signature_str).into_vec() {
                if decoded_sig.len() == 64 {
                    true
                } else {
                    // Retorna erro se a estrutura de bytes estiver errada
                    return Err(SolanaEngineError::InvalidSignatureStructure);
                }
            } else {
                return Err(SolanaEngineError::InvalidBase58Format {
                    input: mock_signature_str.to_string(),
                });
            }
        } else {
            false
        };

        let amount_f64 = amount.to_f64().unwrap_or(0.0);
        let tx_fee_lamports = 5000;

        Ok(SolanaValidationResult {
            is_valid_signature: is_valid_sig,
            tx_fee_lamports,
            client_pubkey: wallet_str.to_string(),
            amount_in_sol: amount_f64,
        })
    }
}

