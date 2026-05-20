use pqcrypto_kyber::kyber1024::*;
use pqcrypto_traits::kem::PublicKey as _;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KyberError {
    #[error("Falha ao desencapsular o segredo: chaves ou texto cifrado corrompidos")]
    DecapsulationFailed,
}

/// Estrutura contendo o par de chaves pós-quânticas Kyber-1024
pub struct KyberKeyPair {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

/// Gera um novo par de chaves Kyber-1024 de segurança máxima (nível 5 do NIST)
pub fn generate_keypair() -> KyberKeyPair {
    let (public_key, secret_key) = keypair();
    KyberKeyPair { public_key, secret_key }
}

/// Encapsula um segredo compartilhado usando a chave pública do destinatário.
/// Retorna uma tupla contendo o (Segredo Compartilhado, Texto Cifrado)
pub fn encapsulate_secret(public_key: &PublicKey) -> (SharedSecret, Ciphertext) {
    let (shared_secret, ciphertext) = encapsulate(public_key);
    (shared_secret, ciphertext)
}

/// Desencapsula o texto cifrado usando a chave privada correspondente para recuperar o segredo compartilhado.
pub fn decapsulate_secret(ciphertext: &Ciphertext, secret_key: &SecretKey) -> Result<SharedSecret, KyberError> {
    // Na versão estável da pqcrypto, decapsulate retorna diretamente o SharedSecret.
    // Em caso de falha grave na biblioteca C subjacente, ela entra em pânico ou falha de forma opaca.
    // Retornamos Ok embrulhado para manter a assinatura segura e compatível com o ecossistema do SlipPay.
    let shared_secret = decapsulate(ciphertext, secret_key);
    Ok(shared_secret)
}

/// Converte a chave pública em bytes puros para transmissão ou armazenamento
pub fn public_key_to_bytes(public_key: &PublicKey) -> Vec<u8> {
    public_key.as_bytes().to_vec()
}

/// Reconstrói a chave pública a partir de bytes brutos
pub fn public_key_from_bytes(bytes: &[u8]) -> Result<PublicKey, anyhow::Error> {
    PublicKey::from_bytes(bytes).map_err(|e| anyhow::anyhow!("Chave pública Kyber inválida: {:?}", e))
}

