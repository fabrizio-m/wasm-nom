use wasm_core::values::Parse;
use wasm_derive::Parse;

use crate::{modules::indices::FuncIdx, types::RefType};

#[derive(Parse, Debug)]
pub enum ReferenceInstruction {
    #[starting = 0xD0]
    Null(RefType),
    IsNull,
    Func(FuncIdx),
}
