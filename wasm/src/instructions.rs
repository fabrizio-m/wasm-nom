use self::{
    control::ControlInstruction,
    memory::MemoryInstruction,
    numeric::{NumericInstruction, SaturatingTruncationInstruction},
    parametric::ParametricInstruction,
    reference::ReferenceInstruction,
    table::TableInstruction,
    variable::VariableInstruction,
};
use crate::Suffix;
use alloc::vec::Vec;
use nom::{branch::alt, combinator::map, multi};
use wasm_core::values::Parse;

mod control;
mod memory;
mod numeric;
mod parametric;
mod reference;
mod table;
mod variable;

#[derive(Debug)]
pub struct Instructions(Vec<Instruction>);
impl<const END: u8> From<TerminatedInstructionSequence<END>> for Instructions {
    fn from(seq: TerminatedInstructionSequence<END>) -> Self {
        Self(seq.0)
    }
}
impl Instructions {
    fn parse_with_finalizer<'a, E, const END: u8>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + core::fmt::Debug,
    {
        let (i, seq) = <TerminatedInstructionSequence<END>>::parse(i)?;
        Ok((i, seq.into()))
    }
    fn empty() -> Self {
        Self(Vec::new())
    }
}
impl Parse for Instructions {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + core::fmt::Debug,
    {
        Self::parse_with_finalizer::<_, 0x0B>(i)
    }
}
impl From<Vec<Instruction>> for Instructions {
    fn from(instructions: Vec<Instruction>) -> Self {
        Self(instructions)
    }
}

#[derive(Debug)]
pub enum Instruction {
    Control(ControlInstruction),
    Reference(ReferenceInstruction),
    Parametric(ParametricInstruction),
    Variable(VariableInstruction),
    Table(TableInstruction),
    Memory(MemoryInstruction),
    Numeric(NumericInstruction),
    Saturating(SaturatingTruncationInstruction),
}
impl Parse for Instruction {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + core::fmt::Debug,
    {
        let parsers = (
            map(ControlInstruction::parse, |instruction| {
                Self::Control(instruction)
            }),
            map(ReferenceInstruction::parse, |instruction| {
                Self::Reference(instruction)
            }),
            map(ParametricInstruction::parse, |instruction| {
                Self::Parametric(instruction)
            }),
            map(VariableInstruction::parse, |instruction| {
                Self::Variable(instruction)
            }),
            map(TableInstruction::parse, |instruction| {
                Self::Table(instruction)
            }),
            map(MemoryInstruction::parse, |instruction| {
                Self::Memory(instruction)
            }),
            map(NumericInstruction::parse, |instruction| {
                Self::Numeric(instruction)
            }),
            map(SaturatingTruncationInstruction::parse, |instruction| {
                Self::Saturating(instruction)
            }),
        );
        alt(parsers)(i)
    }
}

#[derive(Debug)]
pub struct Expression(Vec<Instruction>);
impl Parse for Expression {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + core::fmt::Debug,
    {
        map(<TerminatedInstructionSequence<0x0B>>::parse, |seq| {
            Self(seq.0)
        })(i)
    }
}

struct TerminatedInstructionSequence<const END: u8>(Vec<Instruction>);

impl<const END: u8> Parse for TerminatedInstructionSequence<END> {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + core::fmt::Debug,
    {
        let (i, (instructions, _)) = multi::many_till(Instruction::parse, <Suffix<END>>::parse)(i)?;
        Ok((i, Self(instructions)))
    }
}
