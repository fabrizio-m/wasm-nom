use nom::{branch::alt, bytes::complete::tag, sequence::tuple, Parser};
use std::fmt::Debug;
use wasm_core::values::Parse;
use wasm_derive::Parse;

#[derive(Parse, Debug)]
pub enum NumType {
    #[starting = 0x7F]
    I32,
    I64,
    F32,
    F64,
}

#[derive(Parse, Debug)]
pub enum RefType {
    #[starting = 0x70]
    FuncRef,
    ExternRef,
}
#[derive(Debug)]
pub enum ValueType {
    NumType(NumType),
    RefType(RefType),
}
impl Parse for ValueType {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + std::fmt::Debug,
    {
        let num_type = NumType::parse.map(|num_type| ValueType::NumType(num_type));
        let ref_type = RefType::parse.map(|ref_type| ValueType::RefType(ref_type));
        alt((num_type, ref_type))(i)
    }
}
type ResultType = Vec<ValueType>;

#[derive(Debug)]
pub struct FuncType {
    pub rt1: ResultType,
    pub rt2: ResultType,
}

impl Parse for FuncType {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + std::fmt::Debug,
    {
        let (i, _) = tag([0x60])(i)?;
        let (i, rt1) = Parse::parse(i)?;
        let (i, rt2) = Parse::parse(i)?;
        Ok((i, FuncType { rt1, rt2 }))
    }
}
#[derive(Debug)]
pub struct Limit {
    pub min: u32,
    pub max: Option<u32>,
}

impl Parse for Limit {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + std::fmt::Debug,
    {
        let without_max = tuple((tag([0x00]), u32::parse)).map(|(_, min)| Limit { min, max: None });
        let with_max = tuple((tag([0x01]), u32::parse, u32::parse)).map(|(_, min, max)| Limit {
            min,
            max: max.into(),
        });
        alt((without_max, with_max))(i)
    }
}
#[derive(Parse, Debug)]
pub struct MemType {
    pub lim: Limit,
}
#[derive(Parse, Debug)]
pub struct TableType {
    pub et: RefType,
    pub lim: Limit,
}

#[derive(Parse, Debug)]
pub struct GlobalType {
    pub t: ValueType,
    pub m: Mutability,
}

#[derive(Parse, Debug)]
pub enum Mutability {
    Const,
    Var,
}
