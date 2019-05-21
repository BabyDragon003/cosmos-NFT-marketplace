use cw721_base::Extension;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
pub struct Config {
    pub owner: Addr,
    pub cw721_address: Option<Addr>,
    pub max_tokens: u32,
    pub name: String,
    pub symbol: String,
    pub unused_token_id: u32,
    pub maximum_royalty_fee: u32,
    pub royalties: Vec<Royalty>,
    pub uri: String,
    pub enabled: bool
}



pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const SALE_KEY: &str = "sale";
pub const SALE: Map<String, SaleInfo> = Map::new(SALE_KEY);
// pub const PRICE_KEY: &str = "price";
// pub const PRICE: Map<u32, Uint128> = Map::new(PRICE_KEY);

