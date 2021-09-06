use wasm_core::values::Parse;
use wasm_derive::Parse;

use crate::modules::indices::{GlobalIdx, LocalIdx};

#[derive(Parse, Debug)]
pub enum VariableInstruction {
    #[starting = 0x20]
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),
}
