use nom::{
    branch::alt,
    combinator::{map, verify},
    sequence::tuple,
};
use wasm_core::values::Parse;
use wasm_derive::Parse;

use crate::{modules::indices::DataIdx, Prefix, Suffix};

#[derive(Parse, Debug)]
pub struct MemArg {
    pub align: u32,
    pub offset: u32,
}

type Zero = Suffix<0x00>;
#[derive(Debug)]
pub enum MemoryInstruction {
    I32Load(MemArg),
    I64Load(MemArg),
    F32Load(MemArg),
    F64Load(MemArg),
    I32Load8S(MemArg),
    I32Load8U(MemArg),
    I32Load16S(MemArg),
    I32Load16U(MemArg),
    I64Load8S(MemArg),
    I64Load8U(MemArg),
    I64Load16S(MemArg),
    I64Load16U(MemArg),
    I64Load32S(MemArg),
    I64Load32U(MemArg),
    I32Store(MemArg),
    I64Store(MemArg),
    F32Store(MemArg),
    F64Store(MemArg),
    I32Store8(MemArg),
    I32Store16(MemArg),
    I64Store8(MemArg),
    I64Store16(MemArg),
    I64Store32(MemArg),
    Size(Zero),
    Grow(Zero),
    Init(Prefix<8>, DataIdx, Zero),
    Drop(Prefix<9>, DataIdx),
    Copy(Prefix<10>, Zero, Zero),
    Fill(Prefix<11>, Zero),
}

impl Parse for MemoryInstruction {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + std::fmt::Debug,
    {
        let (i, op) = verify(u8::parse, |op| (*op >= 0x28 && *op <= 0x40) || *op == 0xFC)(i)?;
        let op = match op {
            0x28..=0x3E => {
                let (i, mem_arg) = MemArg::parse(i)?;
                (i, simple(op, mem_arg))
            }
            0x3F => {
                let (i, prefix) = Zero::parse(i)?;
                (i, Self::Size(prefix))
            }
            0x40 => {
                let (i, prefix) = Zero::parse(i)?;
                (i, Self::Grow(prefix))
            }
            0xFC => prefixed(i)?,
            _ => unreachable!(),
        };
        Ok(op)
    }
}
fn prefixed<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], MemoryInstruction, E>
where
    E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + std::fmt::Debug,
{
    let init = tuple((<Prefix<8>>::parse, DataIdx::parse, Zero::parse));
    let init = map(init, |data| MemoryInstruction::Init(data.0, data.1, data.2));
    let drop = tuple((<Prefix<9>>::parse, DataIdx::parse));
    let drop = map(drop, |data| MemoryInstruction::Drop(data.0, data.1));
    let copy = tuple((<Prefix<10>>::parse, Zero::parse, Zero::parse));
    let copy = map(copy, |data| MemoryInstruction::Copy(data.0, data.1, data.2));
    let fill = tuple((<Prefix<11>>::parse, Zero::parse));
    let fill = map(fill, |data| MemoryInstruction::Fill(data.0, data.1));
    alt((init, drop, copy, fill))(i)
}
fn simple(op: u8, mem_arg: MemArg) -> MemoryInstruction {
    match op {
        0x28 => MemoryInstruction::I32Load(mem_arg),
        0x29 => MemoryInstruction::I64Load(mem_arg),
        0x2A => MemoryInstruction::F32Load(mem_arg),
        0x2B => MemoryInstruction::F64Load(mem_arg),
        0x2C => MemoryInstruction::I32Load8S(mem_arg),
        0x2D => MemoryInstruction::I32Load8U(mem_arg),
        0x2E => MemoryInstruction::I32Load16S(mem_arg),
        0x2F => MemoryInstruction::I32Load16U(mem_arg),
        0x30 => MemoryInstruction::I64Load8S(mem_arg),
        0x31 => MemoryInstruction::I64Load8U(mem_arg),
        0x32 => MemoryInstruction::I64Load16S(mem_arg),
        0x33 => MemoryInstruction::I64Load16U(mem_arg),
        0x34 => MemoryInstruction::I64Load32S(mem_arg),
        0x35 => MemoryInstruction::I64Load32U(mem_arg),
        0x36 => MemoryInstruction::I32Store(mem_arg),
        0x37 => MemoryInstruction::I64Store(mem_arg),
        0x38 => MemoryInstruction::F32Store(mem_arg),
        0x39 => MemoryInstruction::F64Store(mem_arg),
        0x3A => MemoryInstruction::I32Store8(mem_arg),
        0x3B => MemoryInstruction::I32Store16(mem_arg),
        0x3C => MemoryInstruction::I64Store8(mem_arg),
        0x3D => MemoryInstruction::I64Store16(mem_arg),
        0x3E => MemoryInstruction::I64Store32(mem_arg),
        _ => unreachable!(),
    }
}
