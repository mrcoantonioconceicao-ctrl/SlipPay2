use crate::dilithium::{self, DilithiumError};
use ed25519_dalek::{Signer, Verifier, SigningKey, VerifyingKey, Signature as EdSignature};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HybridError {
    #[error("Falha na camada clássica (Ed25519) da assinatura híbrida")]
    ClassicVerificationFailed,
    
    #[error("Falha na camada pós-quântica (Dilithium) da assinatura híbrida")]
    QuantumVerificationFailed(#[from] DilithiumError),
}

/// Representação de uma assinatura híbrida unificada para o SlipPay
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HybridSignature {
    pub classic_sig: Vec<u8>,
    pub quantum_sig: Vec<u8>,
}

/// Assina digitalmente um payload de transação de forma híbrida (Ed25519 + Dilithium5)
pub fn sign_hybrid(
    message: &[u8],
    classic_key: &SigningKey,
    quantum_key: &pqcrypto_dilithium::dilithium5::SecretKey,
) -> HybridSignature {
    // 1. Assinatura Clássica
    let ed_sig = classic_key.sign(message);
    
    // 2. Assinatura Pós-Quântica
    let pq_sig = dilithium::sign_message(message, quantum_key);
    let pq_bytes = dilithium::signature_to_bytes(&pq_sig);

    HybridSignature {
        classic_sig: ed_sig.to_bytes().to_vec(),
        quantum_sig: pq_bytes,
    }
}

/// Valida de forma estrita e combinada os dois níveis da assinatura híbrida
pub fn verify_hybrid(
    message: &[u8],
    signature: &HybridSignature,
    classic_public: &VerifyingKey,
    quantum_public: &pqcrypto_dilithium::dilithium5::PublicKey,
) -> Result<(), HybridError> {
    // 1. Valida a assinatura clássica Ed25519
    let ed_sig_bytes: [u8; 64] = signature.classic_sig.as_slice()
        .try_into()
        .map_err(|_| HybridError::ClassicVerificationFailed)?;
        
    let ed_sig = EdSignature::from_bytes(&ed_sig_bytes);
    classic_public.verify(message, &ed_sig)
        .map_err(|_| HybridError::ClassicVerificationFailed)?;

    // 2. Valida a assinatura pós-quântica Dilithium
    let pq_sig = dilithium::signature_from_bytes(&signature.quantum_sig)
        .map_err(|_| HybridError::QuantumVerificationFailed(DilithiumError::InvalidSignature))?;
        
    dilithium::verify_signature(message, &pq_sig, quantum_public)?;

    Ok(())
}

