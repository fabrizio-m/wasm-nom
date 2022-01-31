use alloc::vec::Vec;
use wasm_core::values::Parse;
use wasm_derive::Parse;

use crate::types::ValueType;

#[derive(Parse, Debug)]
pub enum ParametricInstruction {
    #[starting = 0x1A]
    Drop,
    Select,
    SelectTyped(Vec<ValueType>),
}
