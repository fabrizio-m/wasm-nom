use super::Instructions;
use crate::{
    instructions::Instruction,
    modules::indices::{FuncIdx, LabelIdx, TableIdx, TypeIdx},
    types::ValueType,
};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::one_of, multi::many_till,
    sequence::tuple, Parser,
};
use wasm_core::values::Parse;

#[derive(Debug)]
pub enum BlockType {
    Empty,
    ValType(ValueType),
    TypeIdx(TypeIdx),
}

impl Parse for BlockType {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + std::fmt::Debug,
    {
        let empty = tag([0x40]).map(|_| Self::Empty);
        let val_type = ValueType::parse.map(|val_type| Self::ValType(val_type));
        let type_index = TypeIdx::parse.map(|type_index| Self::TypeIdx(type_index));
        alt((empty, val_type, type_index))(i)
    }
}

#[derive(Debug)]
pub enum ControlInstruction {
    Unreachable,
    Nop,
    Block(BlockType, Instructions),
    Loop(BlockType, Instructions),
    IfElse(BlockType, Instructions, Instructions),
    Br(LabelIdx),
    BrIf(LabelIdx),
    BrTable {
        table: Vec<LabelIdx>,
        default: LabelIdx,
    },
    Return,
    Call(FuncIdx),
    CallIndirect(FuncIdx, TableIdx),
}
const END: u8 = 0x0B;
impl Parse for ControlInstruction {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + std::fmt::Debug,
    {
        let (i, opcode) = one_of([
            0x00, 0x01, 0x02, 0x03, 0x04, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
        ])(i)?;
        let instruction = match opcode as u8 {
            0x00 => (i, Self::Unreachable),
            0x01 => (i, Self::Nop),
            op @ (0x02 | 0x03) => {
                let (i, (block, ins)) = tuple((BlockType::parse, Instructions::parse))(i)?;

                let op = match op {
                    0x02 => Self::Block(block, ins),
                    0x03 => Self::Loop(block, ins),
                    _ => unreachable!(),
                };
                (i, op)
            }
            0x04 => {
                let (i, block) = BlockType::parse(i)?;
                let (i, (if_branch, finalizer)) =
                    many_till(Instruction::parse, one_of([END, 0x05]))(i)?;
                let ins = match finalizer as u8 {
                    END => (
                        i,
                        Self::IfElse(block, if_branch.into(), Instructions::empty()),
                    ),
                    0x05 => {
                        let (i, else_branch) = Instructions::parse(i)?;
                        (i, Self::IfElse(block, if_branch.into(), else_branch))
                    }
                    _ => unreachable!(),
                };
                ins
            }
            0x0C => {
                let (i, label) = LabelIdx::parse(i)?;
                (i, Self::Br(label))
            }
            0x0D => {
                let (i, label) = LabelIdx::parse(i)?;
                (i, Self::BrIf(label))
            }
            0x0E => {
                let labels = <Vec<LabelIdx>>::parse;
                let default = LabelIdx::parse;
                let (i, (table, default)) = tuple((labels, default))(i)?;
                (i, Self::BrTable { table, default })
            }
            0x0F => (i, Self::Return),
            0x10 => {
                let (i, function) = FuncIdx::parse(i)?;
                (i, Self::Call(function))
            }
            0x11 => {
                let (i, call) = tuple((TypeIdx::parse, TableIdx::parse))(i)?;
                (i, Self::CallIndirect(call.0, call.1))
            }
            _ => unreachable!(),
        };
        Ok(instruction)
    }
}
