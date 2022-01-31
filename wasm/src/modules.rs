use self::{
    code::Code,
    data::Data,
    element::Elem,
    export::Export,
    global::Global,
    import::Import,
    indices::{FuncIdx, TypeIdx},
    memory::Memory,
    table::Table,
};
use crate::types::FuncType;
use alloc::vec::Vec;
use nom::{
    bytes::complete::{tag, take},
    combinator::{consumed, map, opt, verify},
    multi::many0,
};
use wasm_core::values::{Name, Parse};
use wasm_derive::Parse;

mod code;
mod data;
mod element;
mod export;
mod global;
mod import;
pub mod indices;
mod memory;
mod table;

#[derive(Debug)]
pub enum Section {
    CustomSection(CustomSection),
    TypeSection(TypeSection),
    ImportSection(ImportSection),
    FunctionSection(FunctionSection),
    TableSection(TableSection),
    MemorySection(MemorySection),
    GlobalSection(GlobalSection),
    ExportSection(ExportSection),
    StartSection(StartSection),
    ElementSection(ElementSection),
    CodeSection(CodeSection),
    DataSection(DataSection),
    DataCountSection(DataCountSection),
}

#[derive(Debug)]
pub struct CustomSection {
    pub name: Name,
    pub data: Vec<u8>,
}
#[derive(Parse, Debug)]
pub struct TypeSection(Vec<FuncType>);
#[derive(Parse, Debug)]
pub struct ImportSection(Vec<Import>);
#[derive(Parse, Debug)]
pub struct FunctionSection(Vec<TypeIdx>);
#[derive(Parse, Debug)]
pub struct TableSection(Vec<Table>);
#[derive(Debug, Parse)]
pub struct MemorySection(Vec<Memory>);
#[derive(Parse, Debug)]
pub struct GlobalSection(Vec<Global>);
#[derive(Debug, Parse)]
pub struct ExportSection(Vec<Export>);
#[derive(Parse, Debug)]
pub struct StartSection(Option<FuncIdx>);
#[derive(Parse, Debug)]
pub struct ElementSection(Vec<Elem>);
#[derive(Parse, Debug)]
pub struct CodeSection(Vec<Code>);
#[derive(Parse, Debug)]
pub struct DataSection(Vec<Data>);
#[derive(Parse, Debug)]
pub struct DataCountSection(Option<u32>);

#[derive(Debug)]
pub struct Module {
    pub magic: Magic,
    pub version: Version,
    pub sections: Vec<Section>,
}
#[derive(Debug)]
pub struct Magic;
impl Parse for Magic {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + core::fmt::Debug,
    {
        let magic = [0x00, 0x61, 0x73, 0x6D];
        map(tag(magic), |_| Self)(i)
    }
}
#[derive(Debug)]
pub struct Version(pub [u8; 4]);

impl Parse for Version {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + core::fmt::Debug,
    {
        map(take(4_usize), |version: &[u8]| {
            Self([version[0], version[1], version[2], version[3]])
        })(i)
    }
}

impl Parse for Section {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + core::fmt::Debug,
    {
        let (i, id) = verify(u8::parse, |id| *id <= 12)(i)?;
        let (i, length) = u32::parse(i)?;
        let section = match id {
            //0 => map(CustomSection::parse, |sec| Self::CustomSection(sec))(i),
            0 => {
                let (i, (cons, name)) = consumed(Name::parse)(i)?;
                let (i, bytes) = take(length as usize - cons.len())(i)?;
                let sec = Self::CustomSection(CustomSection {
                    name,
                    data: bytes.to_vec(),
                });
                Ok((i, sec))
            }
            1 => map(TypeSection::parse_dbg, |sec| Self::TypeSection(sec))(i),
            2 => map(ImportSection::parse_dbg, |sec| Self::ImportSection(sec))(i),
            3 => map(FunctionSection::parse_dbg, |sec| Self::FunctionSection(sec))(i),
            4 => map(TableSection::parse_dbg, |sec| Self::TableSection(sec))(i),
            5 => map(MemorySection::parse_dbg, |sec| Self::MemorySection(sec))(i),
            6 => map(GlobalSection::parse_dbg, |sec| Self::GlobalSection(sec))(i),
            7 => map(ExportSection::parse_dbg, |sec| Self::ExportSection(sec))(i),
            8 => map(StartSection::parse_dbg, |sec| Self::StartSection(sec))(i),
            9 => map(ElementSection::parse_dbg, |sec| Self::ElementSection(sec))(i),
            10 => map(CodeSection::parse_dbg, |sec| Self::CodeSection(sec))(i),
            11 => map(DataSection::parse_dbg, |sec| Self::DataSection(sec))(i),
            12 => map(DataCountSection::parse_dbg, |sec| {
                Self::DataCountSection(sec)
            })(i),
            _ => unreachable!(),
        };
        section
    }
}

impl Parse for Module {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + core::fmt::Debug,
    {
        let (i, magic) = Magic::parse(i)?;
        let (i, version) = Version::parse(i)?;
        let (i, sections) = many0(Section::parse_dbg)(i)?;
        let (i, _) = opt(tag(b"\n"))(i)?;
        let module = Self {
            magic,
            version,
            sections,
        };
        Ok((i, module))
    }
}
