pub mod file;
pub mod pda;
pub use ::idl::*;

#[cfg(test)]
mod tests {
    use crate::idl::IdlType;
    use std::str::FromStr;

    #[test]
    fn multidimensional_array() {
        assert_eq!(
            IdlType::from_str("[[u8;16];32]").unwrap(),
            IdlType::Array(Box::new(IdlType::Array(Box::new(IdlType::U8), 16)), 32)
        );
    }

    #[test]
    fn array() {
        assert_eq!(
            IdlType::from_str("[Pubkey;16]").unwrap(),
            IdlType::Array(Box::new(IdlType::PublicKey), 16)
        );
    }

    #[test]
    fn array_with_underscored_length() {
        assert_eq!(
            IdlType::from_str("[u8;50_000]").unwrap(),
            IdlType::Array(Box::new(IdlType::U8), 50000)
        );
    }

    #[test]
    fn option() {
        assert_eq!(
            IdlType::from_str("Option<bool>").unwrap(),
            IdlType::Option(Box::new(IdlType::Bool))
        )
    }

    #[test]
    fn vector() {
        assert_eq!(
            IdlType::from_str("Vec<bool>").unwrap(),
            IdlType::Vec(Box::new(IdlType::Bool))
        )
    }
}
