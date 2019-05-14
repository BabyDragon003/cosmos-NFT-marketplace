use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
    pub collection_code_id: u64,
    pub cw721_base_code_id: u64,
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const COLLECTIONS_KEY: &str = "collections";
pub const COLLECTIONS: Map<u32, CollectionRecord> = Map::new(COLLECTIONS_KEY);
