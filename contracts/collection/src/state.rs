use cw721_base::Extension;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;
use cw_utils::{Expiration, Scheduled};
use cw_storage_plus::{Map};
use crate::msg::{SaleInfo, Royalty};

    pub enabled: bool
}



pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const SALE_KEY: &str = "sale";
pub const SALE: Map<String, SaleInfo> = Map::new(SALE_KEY);
// pub const PRICE_KEY: &str = "price";
// pub const PRICE: Map<u32, Uint128> = Map::new(PRICE_KEY);

