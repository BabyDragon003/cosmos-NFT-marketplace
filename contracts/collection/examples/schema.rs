use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use cosmwasm_std::Coin;

use marble_collection::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ConfigResponse, SaleInfo, SalesResponse};

fn main() {
  let mut out_dir = current_dir().unwrap();
