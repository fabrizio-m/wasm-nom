use nom::combinator::{map, verify};
use wasm_core::values::Parse;
pub mod instructions;
pub mod modules;
pub mod types;

#[derive(Debug)]
pub struct Prefix<const P: u32>;
impl<const P: u32> Parse for Prefix<P> {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + std::fmt::Debug,
    {
        map(verify(u32::parse, |prefix| *prefix == P), |_| Self)(i)
    }
}
#[derive(Debug)]
pub struct Suffix<const P: u8>;

impl<const P: u8> Parse for Suffix<P> {
    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
    where
        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + std::fmt::Debug,
    {
        map(verify(u8::parse, |prefix| *prefix == P), |_| Self)(i)
    }
}

#[test]
fn test1() {
    use modules::Module;
    let file = include_bytes!("loop.wasm");
    let module = Module::parse::<nom_supreme::error::ErrorTree<_>>(file);
    println!("{:?}", file);
    println!("{:#?}", module);
}
