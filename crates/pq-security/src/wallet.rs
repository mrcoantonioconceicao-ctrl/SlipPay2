use ed25519_dalek::SigningKey;
use pqcrypto_kyber::kyber1024::PublicKey as KyberPublic;
use pqcrypto_dilithium::dilithium5::PublicKey as DilithiumPublic;
use rand::Rng; // Importação essencial para habilitar o método .fill() em geradores de números aleatórios
use crate::kyber;
use crate::dilithium;

/// Vault completo contendo todas as chaves privadas e públicas do nó/carteira SlipPay
pub struct PqWalletVault {
    pub classic_signing: SigningKey,
    pub kyber_keys: kyber::KyberKeyPair,
    pub dilithium_keys: dilithium::DilithiumKeyPair,
}

/// Estrutura pública da carteira usada para compartilhar identidades na rede Stellar/SlipPay
pub struct PqWalletPublicIdentity {
    pub stellar_classic_key: ed25519_dalek::VerifyingKey,
    pub kyber_pubkey: KyberPublic,
    pub dilithium_pubkey: DilithiumPublic,
}

impl PqWalletVault {
    /// Gera uma carteira totalmente nova, blindada com chaves clássicas e pós-quânticas
    pub fn generate_new() -> Self {
        // Gera 32 bytes de entropia segura usando a crate 'rand'
        let mut entropy = [0u8; 32];
        rand::thread_rng().fill(&mut entropy);
        
        // Inicializa a SigningKey a partir dos bytes de entropia gerados
        let classic_signing = SigningKey::from_bytes(&entropy);
        
        PqWalletVault {
            classic_signing,
            kyber_keys: kyber::generate_keypair(),
            dilithium_keys: dilithium::generate_keypair(),
        }
    }

    /// Extrai com segurança a identidade pública correspondente a esta carteira
    pub fn to_public_identity(&self) -> PqWalletPublicIdentity {
        PqWalletPublicIdentity {
            stellar_classic_key: self.classic_signing.verifying_key(),
            kyber_pubkey: self.kyber_keys.public_key.clone(),
            dilithium_pubkey: self.dilithium_keys.public_key.clone(),
        }
    }
}

