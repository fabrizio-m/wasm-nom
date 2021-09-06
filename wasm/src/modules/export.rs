use super::indices::{FuncIdx, GlobalIdx, MemIdx, TableIdx};
use wasm_core::values::{Name, Parse};
use wasm_derive::Parse;

#[derive(Parse, Debug)]
pub struct Export {
    pub name: Name,
    pub descriptor: ExportDescriptor,
}
#[derive(Parse, Debug)]
pub enum ExportDescriptor {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}
