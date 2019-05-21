use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use cosmwasm_std::Coin;
  create_dir_all(&out_dir).unwrap();
  remove_schemas(&out_dir).unwrap();

  export_schema(&schema_for!(InstantiateMsg), &out_dir);
  export_schema(&schema_for!(ExecuteMsg), &out_dir);
  export_schema(&schema_for!(QueryMsg), &out_dir);
  export_schema(&schema_for!(ConfigResponse), &out_dir);
  export_schema(&schema_for!(CollectionRecord), &out_dir);
  export_schema(&schema_for!(CollectionInfo), &out_dir);
  export_schema(&schema_for!(CollectionListResponse), &out_dir);
}
