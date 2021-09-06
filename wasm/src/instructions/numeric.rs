use nom::combinator::verify;
use wasm_core::values::Parse;
use wasm_derive::Parse;

#[derive(Parse, Debug)]
pub enum NumericInstruction {
    //const
    #[starting = 0x41]
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    //i32
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32NeU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    //i64
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64NeU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    //f32
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    //f64
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    //i32
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    //i64
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    //f32
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32CopySign,
    //f64
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64CopySign,
    //conversion
    //i32
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    //i64
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    //f32
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64u,
    F32DemoteF64,
    //f64
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64u,
    F64PromoteF32,
    //reinterpretation
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
    //extend
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
}

#[derive(Debug)]
pub enum SaturatingTruncationInstruction {
    I32TruncSatF32S,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,
}
impl Parse for SaturatingTruncationInstruction {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + std::fmt::Debug,
    {
        let (i, prefix) = verify(u32::parse, |prefix| *prefix <= 7)(i)?;
        let ins = match prefix {
            0 => Self::I32TruncSatF32S,
            1 => Self::I32TruncSatF32U,
            2 => Self::I32TruncSatF64S,
            3 => Self::I32TruncSatF64U,
            4 => Self::I64TruncSatF32S,
            5 => Self::I64TruncSatF32U,
            6 => Self::I64TruncSatF64S,
            7 => Self::I64TruncSatF64U,
            _ => unreachable!(),
        };
        Ok((i, ins))
    }
}
