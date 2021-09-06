use crate::types::MemType;
use wasm_core::values::Parse;
use wasm_derive::Parse;

#[derive(Parse, Debug)]
pub struct Memory {
    memory_type: MemType,
}
