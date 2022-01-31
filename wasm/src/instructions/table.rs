use crate::modules::indices::{ElemIdx, TableIdx};
use alloc::vec::Vec;
use nom::{
    character::complete::one_of,
    combinator::{map, verify},
    sequence::tuple,
};
use wasm_core::values::Parse;

#[derive(Debug)]
pub enum TableInstruction {
    TableGet(TableIdx),
    TableSet(TableIdx),
    TableInit(ElemIdx, TableIdx),
    TableDrop(ElemIdx),
    TableCopy(TableIdx, TableIdx),
    TableGrow(TableIdx),
    TableSize(TableIdx),
    TableFill(TableIdx),
}
impl Parse for TableInstruction {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + core::fmt::Debug,
    {
        let (i, op) = one_of([0x25, 0x26, 0xFC])(i)?;
        match op as u8 {
            op @ (0x25 | 0x26) => {
                let op = map(TableIdx::parse, |idx| match op {
                    0x25 => Self::TableGet(idx),
                    0x26 => Self::TableSet(idx),
                    _ => unreachable!(),
                })(i)?;
                Ok(op)
            }
            0xFC => {
                let mut prefix = verify(u32::parse, |prefix| *prefix >= 12 && *prefix <= 17);
                let (i, prefix) = prefix(i)?;

                let op = match prefix {
                    12 => {
                        let (i, content) = tuple((ElemIdx::parse, TableIdx::parse))(i)?;
                        (i, Self::TableInit(content.0, content.1))
                    }
                    13 => {
                        let (i, content) = ElemIdx::parse(i)?;
                        (i, Self::TableDrop(content))
                    }
                    14 => {
                        let (i, content) = tuple((TableIdx::parse, TableIdx::parse))(i)?;
                        (i, Self::TableCopy(content.0, content.1))
                    }
                    prefix @ (15 | 16 | 17) => {
                        let (i, idx) = TableIdx::parse(i)?;
                        let op = match prefix {
                            15 => Self::TableGrow(idx),
                            16 => Self::TableSize(idx),
                            17 => Self::TableFill(idx),
                            _ => unreachable!(),
                        };
                        (i, op)
                    }
                    _ => unreachable!(),
                };
                Ok(op)
            }
            _ => unreachable!(),
        }
    }
}
