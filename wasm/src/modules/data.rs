use super::indices::MemIdx;
use crate::instructions::Expression;
use alloc::vec::Vec;
use wasm_core::values::Parse;
use wasm_derive::Parse;

#[derive(Parse, Debug)]
pub enum Data {
    Active(Expression, Vec<u8>),
    Passive(Vec<u8>),
    ActiveExplicit(MemIdx, Expression, Vec<u8>),
}
