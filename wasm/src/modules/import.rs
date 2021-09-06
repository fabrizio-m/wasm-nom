use wasm_core::values::{Name, Parse};

use crate::types::{GlobalType, MemType, TableType};

use super::indices;
use wasm_derive::Parse;

#[derive(Parse, Debug)]
pub struct Import {
    pub module: Name,
    pub name: Name,
    pub descriptor: ImportDescriptor,
}
#[derive(Parse, Debug)]
pub enum ImportDescriptor {
    Func(indices::TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}
