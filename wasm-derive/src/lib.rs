extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{self, Ident, TokenTree};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{
    self, punctuated::Punctuated, token::Comma, Attribute, DataEnum, DataStruct, FieldsNamed,
};

#[proc_macro_derive(Parse, attributes(starting))]
pub fn wasm_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    match &ast.data {
        syn::Data::Struct(stru) => impl_struct(stru, name.clone()),
        syn::Data::Enum(enu) => impl_enum(enu, name.clone()),
        syn::Data::Union(_) => {
            panic!("no unions for now")
        }
    }
}
fn impl_enum(enu: &DataEnum, name: Ident) -> TokenStream {
    //let debug_name = name.to_string();
    let mut det_number = 0;
    let frags = enu.variants.iter().map(|variant| {
        let var_attr = &variant.attrs;
        if !var_attr.is_empty() {
            //dbg!(&var_attr);
            //dbg!(parse_from(var_attr.clone()));
            if let Some(det) = parse_from(var_attr.clone()) {
                det_number = det as u8;
            }
        }

        let det = det_number;
        det_number += 1;
        let var = variant.ident.clone();

        let variants = match variant.fields.clone() {
            syn::Fields::Named(_named) => {
                panic!("not named");
            }
            syn::Fields::Unnamed(fields) => {
                let mut idents: Punctuated<Ident, Comma> = Punctuated::new();
                let fields = fields.unnamed.iter().enumerate().map(|field| {
                    let ident = format_ident!("field{}", field.0);
                    let frag = quote! {
                        let (i,#ident) = Parse::parse(i)?;
                    };
                    idents.push(ident);
                    frag
                });
                let fields: VecStream = fields.collect::<Vec<_>>().into();
                let implementation = quote! {
                    #fields
                    let enu = Self::#var(#idents);
                    (i,enu)
                };
                //panic!(format!("{:#?}", implementation));
                quote! {
                    #det => {#implementation},
                }
            }
            syn::Fields::Unit => {
                quote! {
                    #det => (i,Self::#var),
                }
            }
        };
        variants
    });
    let frags: VecStream = frags.collect::<Vec<_>>().into();

    let implementation = quote! {
    impl Parse for #name {
        fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
        where
            E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + std::fmt::Debug,
        {
            let (i,determinant) = u8::parse(i)?;

            let enu = match determinant {
                #frags
                _ => {
                    return Err(nom::Err::Error(nom::error::make_error(i,nom::error::ErrorKind::Fail)));
                }
            };
            Ok(enu)
        }
    }};
    implementation.into()
}

fn impl_struct(stru: &DataStruct, name: Ident) -> TokenStream {
    //let debug_name = name.to_string();
    match &stru.fields {
        syn::Fields::Named(named) => {
            let (extracts_stream, content) = impl_named(named);
            let id = name;
            let implementation = quote! {
                impl Parse for #id {
                    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
                    where
                        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + std::fmt::Debug,
                    {
                        #extracts_stream
                        let structure = Self{
                            #content
                        };
                        Ok((i,structure))
                    }
                }
            };
            //panic!(format!("{:#?}", implementation));
            implementation.into()
        }
        syn::Fields::Unnamed(fields) => {
            let mut idents: Punctuated<Ident, Comma> = Punctuated::new();
            let fields = fields.unnamed.iter().enumerate().map(|field| {
                let ident = format_ident!("field{}", field.0);
                let frag = quote! {
                    let (i,#ident) = Parse::parse(i)?;
                };
                idents.push(ident);
                frag
            });
            let fields: VecStream = fields.collect::<Vec<_>>().into();
            let id = name;
            let implementation = quote! {
                impl Parse for #id {
                    fn parse<'a, E>(i: &'a [u8]) -> nom::IResult<&[u8], Self, E>
                    where
                        E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]> + std::fmt::Debug,
                    {
                        #fields
                        let structure = Self(
                            #idents
                        );
                        Ok((i,structure))
                    }
                }
            };
            implementation.into()
        }
        syn::Fields::Unit => {
            todo!()
        }
    }
}
fn impl_named(named: &FieldsNamed) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut identifiers = vec![];
    let extracts: Vec<proc_macro2::TokenStream> = named
        .named
        .iter()
        .map(|field| {
            let id = field.ident.clone().expect("no ident");
            let tokens = quote! {
                let (i,#id) = Parse::parse(i)?;
                //errors.push(format!("{:?}",&#id));
            };
            identifiers.push(quote! {
                #id,
            });
            tokens
        })
        .collect();
    let mut identifiers_iter = identifiers.into_iter();
    let mut identifiers_stream = identifiers_iter.next().expect("at least one");
    identifiers_stream.append_all(identifiers_iter);

    let mut extracts = extracts.into_iter();
    let mut extracts_stream = extracts.next().expect("at least one");
    extracts_stream.append_all(extracts);
    let content = {
        quote! {
            #identifiers_stream
        }
    };
    (extracts_stream, content)
}

struct VecStream(Vec<proc_macro2::TokenStream>);

impl ToTokens for VecStream {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let iter = self.0.iter();
        tokens.append_all(iter);
    }
}
impl From<Vec<proc_macro2::TokenStream>> for VecStream {
    fn from(vec: Vec<proc_macro2::TokenStream>) -> Self {
        Self(vec)
    }
}

fn parse_from(attrs: Vec<Attribute>) -> Option<u32> {
    let first = attrs.get(0)?;
    let path = first.path.segments.iter().next()?.clone().ident.to_string();
    if path != String::from("starting") {
        return None;
    }
    let tokens = first.tokens.clone().into_iter().take(2).collect::<Vec<_>>();
    let token1 = tokens.get(0)?;
    let token2 = tokens.get(1)?;
    if let TokenTree::Punct(punct) = token1 {
        if punct.to_string() != String::from("=") {
            return None;
        }
    } else {
        return None;
    }
    if let TokenTree::Literal(literal) = token2 {
        let text = literal.to_string();
        let text = text.trim_start_matches("0x");
        u32::from_str_radix(text, 16).ok()
    } else {
        None
    }
}
