use pqcrypto_dilithium::dilithium5::*;
use pqcrypto_traits::sign::DetachedSignature as _;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DilithiumError {
    #[error("Falha na validação da assinatura digital pós-quântica")]
    InvalidSignature,
}

/// Estrutura contendo o par de chaves pós-quânticas Dilithium5 (nível máximo de segurança)
pub struct DilithiumKeyPair {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

/// Gera um novo par de chaves Dilithium5 para assinaturas digitais
pub fn generate_keypair() -> DilithiumKeyPair {
    let (public_key, secret_key) = keypair();
    DilithiumKeyPair { public_key, secret_key }
}

/// Assina uma mensagem digital (payload/transação) usando a chave secreta Dilithium
pub fn sign_message(message: &[u8], secret_key: &SecretKey) -> DetachedSignature {
    detached_sign(message, secret_key)
}

/// Verifica a assinatura digital de uma mensagem usando a chave pública correspondente
pub fn verify_signature(
    message: &[u8],
    signature: &DetachedSignature,
    public_key: &PublicKey,
) -> Result<(), DilithiumError> {
    verify_detached_signature(signature, message, public_key)
        .map_err(|_| DilithiumError::InvalidSignature)
}

/// Converte a assinatura digital em bytes para trafegar em cabeçalhos HTTP ou JSON
pub fn signature_to_bytes(signature: &DetachedSignature) -> Vec<u8> {
    signature.as_bytes().to_vec()
}

/// Reconstrói a estrutura de assinatura a partir de bytes puros
pub fn signature_from_bytes(bytes: &[u8]) -> Result<DetachedSignature, anyhow::Error> {
    DetachedSignature::from_bytes(bytes).map_err(|e| anyhow::anyhow!("Assinatura Dilithium inválida: {:?}", e))
}

