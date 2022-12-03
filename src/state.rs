use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Message {
    pub id:Uint128,
    pub owner:Addr,
    pub topic: String,
    pub message: String
}
//message stored by id
pub const CURRENT_ID: Item<u128> = Item::new("current_id");
//key durrent_id, value message
pub const MESSAGES: Map<u128, Message> = Map::new("messages");
