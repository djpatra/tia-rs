use serde::{Deserialize, Serialize};

/// Unique identifier of a source represented as a 2 byte integer

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
#[serde(rename = "execution", rename_all = "snake_case")]
pub enum ExchangeId {
    Mock,
    BinanceSpot,
}

impl ExchangeId {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExchangeId::Mock => "mock",
            ExchangeId::BinanceSpot => "binance_spot",
        }
    }

    pub fn as_bytes(&self) -> [u8; 2] {
        (*self as u16).to_le_bytes()
    }

    pub fn from_bytes(bytes: [u8; 2]) -> Result<Self, u16> {
        if bytes.is_empty() {
            return Err(u16::MAX);
        }

        match u16::from_le_bytes(bytes) {
            0 => Ok(ExchangeId::Mock),
            1 => Ok(ExchangeId::BinanceSpot),
            other => Err(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_exchange_id() {
        assert_eq!(
            ExchangeId::from_bytes((1_u16).to_le_bytes()).unwrap(),
            ExchangeId::BinanceSpot
        );
    }
}
