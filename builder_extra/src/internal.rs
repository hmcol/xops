use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{Ident, Result, Token, Type, parse::{Parse, ParseStream}, punctuated::Punctuated};
use derive_builder::*;

#[derive(Clone, Builder, Debug)]
#[builder(derive(Debug))]
pub struct Args {
    pub base: Type,
    pub comma_token1: Token![,],
    pub builder: Type,
    pub comma_token2: Token![,],
    pub fields: Punctuated<Ident, Token![,]>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut bldr = ArgsBuilder::default();

        bldr.base(input.parse()?)
            .comma_token1(input.parse()?)
            .builder(input.parse()?)
            .comma_token2(input.parse()?)
            .fields(input.call(Punctuated::parse_terminated)?);

        Ok(bldr.build().expect("failed to build `Args` after parse"))
    }
}

impl ToTokens for Args {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.base.to_tokens(tokens);
        self.comma_token1.to_tokens(tokens);
        self.builder.to_tokens(tokens);
        self.comma_token2.to_tokens(tokens);
        self.fields.to_tokens(tokens);
    }
}












