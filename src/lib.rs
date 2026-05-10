// src/lib.rs

// Importa o módulo de banco de dados (src/db.rs)
pub mod db;

// Aqui você pode adicionar outros módulos conforme for criando
// pub mod charges;
// pub mod conversion;

// Funções utilitárias globais podem ficar aqui também.
// Exemplo: conversão BRL → XLM
use rust_decimal::Decimal;

pub fn brl_to_xlm(valor_brl: Decimal, taxa: Decimal) -> Decimal {
    valor_brl / taxa
}

// ----------------------
// Testes automatizados
// ----------------------
#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_conversao_brl_para_xlm() {
        let resultado = brl_to_xlm(dec!(100), dec!(2));
        assert_eq!(resultado, dec!(50));
    }
}
