use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const STATE_KEY: &[u8] = b"state";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {}
