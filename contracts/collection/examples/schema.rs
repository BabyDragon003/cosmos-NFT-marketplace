use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use cosmwasm_std::Coin;

use marble_collection::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ConfigResponse, SaleInfo, SalesResponse};

fn main() {
  let mut out_dir = current_dir().unwrap();
  out_dir.push("schema");
  create_dir_all(&out_dir).unwrap();
  remove_schemas(&out_dir).unwrap();

  export_schema(&schema_for!(InstantiateMsg), &out_dir);
  export_schema(&schema_for!(ExecuteMsg), &out_dir);
  export_schema(&schema_for!(QueryMsg), &out_dir);
  export_schema(&schema_for!(ConfigResponse), &out_dir);
  export_schema(&schema_for!(SaleInfo), &out_dir);
  export_schema(&schema_for!(SalesResponse), &out_dir);
}
