use crate::types::TableType;
use wasm_core::values::Parse;
use wasm_derive::Parse;

#[derive(Parse, Debug)]
pub struct Table {
    table_type: TableType,
}
