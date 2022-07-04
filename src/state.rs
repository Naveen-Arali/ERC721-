use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{Singleton, singleton};
use cw_storage_plus::{Item, Map};
pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct  Tokens{
    pub id:u8,
    pub name:String,
    pub symbol: String,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct  Approve{
    pub owner:Addr,
    pub user:Addr,
    pub id:u8,
    pub app:bool,
}

pub fn config(storage: &mut dyn Storage) -> Singleton<Tokens> {
    singleton(storage, CONFIG_KEY)
}
pub const STATE: Item<State> = Item::new("state");
// pub const TOKEN: Item<Tokens> =Item::new("token");
pub const TOKEN: Map<u64, Tokens> = Map::new("Tokens");
pub const APPROVE:Map<u8,Approve> = Map::new("Approve");
