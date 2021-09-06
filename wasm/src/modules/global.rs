use crate::{instructions::Expression, types::GlobalType};
use wasm_core::values::Parse;
use wasm_derive::Parse;

#[derive(Parse, Debug)]
pub struct Global {
    pub global_type: GlobalType,
    pub expression: Expression,
}
