use super::indices::{FuncIdx, TableIdx};
use crate::{instructions::Expression, types::RefType};
use alloc::vec::Vec;
use wasm_core::values::Parse;
use wasm_derive::Parse;

#[derive(Parse, Debug)]
pub enum ElementKind {
    FuncRef,
}
#[derive(Parse, Debug)]
pub enum Elem {
    ActiveIndex(Expression, Vec<FuncIdx>),
    PassiveIndex(ElementKind, Vec<FuncIdx>),
    ActiveExplicitIndex(TableIdx, Expression, ElementKind, Vec<FuncIdx>),
    DeclarativeIndex(ElementKind, Vec<FuncIdx>),
    ActiveExpression(Expression, Vec<Expression>),
    PassiveExpression(RefType, Vec<Expression>),
    ActiveExplicitExpression(TableIdx, Expression, RefType, Vec<Expression>),
    DeclarativeExpression(RefType, Vec<Expression>),
}
