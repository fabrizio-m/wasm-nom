use alloc::vec::Vec;
use core::{any::type_name, fmt::Debug};

use nom::{
    combinator::opt,
    error::{dbg_dmp, ContextError, ParseError},
    multi::count,
    IResult,
};
use nom_leb128::{leb128_i32, leb128_i64, leb128_u32};

impl Parse for u32 {
    fn parse<'a, E>(i: &'a [u8]) -> IResult<&[u8], Self, E>
    where
        E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + core::fmt::Debug,
    {
        leb128_u32(i)
    }
}
impl Parse for u8 {
    fn parse<'a, E>(i: &'a [u8]) -> IResult<&[u8], Self, E>
    where
        E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + core::fmt::Debug,
    {
        nom::number::complete::u8(i)
    }
}
impl Parse for Name {
    fn parse<'a, E>(i: &'a [u8]) -> IResult<&[u8], Self, E>
    where
        E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + core::fmt::Debug,
    {
        let (i, name) = <Vec<u8>>::parse(i)?;

        Ok((i, Name(name)))
    }
}
#[derive(Debug)]
pub struct Name(Vec<u8>);

pub trait Parse
where
    Self: Sized,
{
    fn parse<'a, E>(i: &'a [u8]) -> IResult<&[u8], Self, E>
    where
        E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + core::fmt::Debug;
    fn parse_dbg<'a, E>(i: &'a [u8]) -> IResult<&[u8], Self, E>
    where
        E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + core::fmt::Debug,
    {
        dbg_dmp(Self::parse, type_name::<Self>())(i)
    }
    ///parses with minimal error information
    fn parse_simple<'a>(i: &'a [u8]) -> IResult<&[u8], Self, nom::error::Error<&'a [u8]>> {
        Self::parse(i)
    }
}
#[derive(Debug)]
pub struct DebugWrapper<T: Parse + Debug>(T);
impl<T: Parse + Debug> Parse for DebugWrapper<T> {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + core::fmt::Debug,
    {
        //println!("{} --> :{}", type_name::<T>(), i.len());
        let (i, val) = dbg_dmp(T::parse, type_name::<T>())(i)?;
        //println!("{} <-- :{}", type_name::<T>(), i.len());
        //println!("OK: {:?}", val);

        Ok((i, Self(val)))
    }
}

impl<T> Parse for Vec<T>
where
    T: Parse,
{
    fn parse<'a, E>(i: &'a [u8]) -> IResult<&[u8], Self, E>
    where
        E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + core::fmt::Debug,
    {
        let (i, length) = u32::parse(i)?;
        count(T::parse, length as usize)(i)
    }
}

impl Parse for f32 {
    fn parse<'a, E>(i: &'a [u8]) -> IResult<&[u8], Self, E>
    where
        E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + core::fmt::Debug,
    {
        nom::number::complete::le_f32(i)
    }
}
impl Parse for f64 {
    fn parse<'a, E>(i: &'a [u8]) -> IResult<&[u8], Self, E>
    where
        E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + core::fmt::Debug,
    {
        nom::number::complete::le_f64(i)
    }
}
impl Parse for i32 {
    fn parse<'a, E>(i: &'a [u8]) -> IResult<&[u8], Self, E>
    where
        E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + core::fmt::Debug,
    {
        leb128_i32(i)
    }
}
impl Parse for i64 {
    fn parse<'a, E>(i: &'a [u8]) -> IResult<&[u8], Self, E>
    where
        E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + core::fmt::Debug,
    {
        leb128_i64(i)
    }
}
impl<T> Parse for Option<T>
where
    T: Parse,
{
    fn parse<'a, E>(i: &'a [u8]) -> IResult<&[u8], Self, E>
    where
        E: ParseError<&'a [u8]> + ContextError<&'a [u8]> + core::fmt::Debug,
    {
        opt(T::parse)(i)
    }
}
