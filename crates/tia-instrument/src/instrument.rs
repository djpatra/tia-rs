use serde::{Deserialize, Serialize};


pub struct Asset

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstrumentKind<AssetKey> {
    Spot,
    Pertual(PerpetualContract<AssetKey>),
    Option(OptionContract<AssetKey>),
    Future(FutureContract<AssetKey>),
}

impl<AssetKey> InstrumentKind<AssetKey> {}
