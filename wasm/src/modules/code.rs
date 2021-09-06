use crate::{instructions::Expression, types::ValueType};
use wasm_core::values::Parse;
use wasm_derive::Parse;

#[derive(Parse, Debug)]
pub struct Code {
    pub size: u32,
    pub code: Func,
}
#[derive(Parse, Debug)]
pub struct Func {
    pub locals: Vec<Local>,
    pub body: Expression,
}
#[derive(Parse, Debug)]
pub struct Local {
    pub count: u32,
    pub value_type: ValueType,
}
