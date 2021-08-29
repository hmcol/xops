use std::convert::TryFrom;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    braced,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    parse_quote, token, Attribute, Block, FnArg, Generics, Ident, Path, Receiver, Token, Type,
};

use derive_builder::*;

// structs -----------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum MetaArg {
    Commute,
    RefsClone,
    Derefs,
}

impl TryFrom<&str> for MetaArg {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "commute" => Ok(MetaArg::Commute),
            "refs_clone" => Ok(MetaArg::RefsClone),
            "derefs" => Ok(MetaArg::Derefs),
            _ => Err("unrecognized binop argument; expected `commute`, `refs_clone`, or `derefs`"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImplArgs {
    pub args: Vec<MetaArg>,
}

/// Type definition for the output of a binary operation: `type Output = C;`
#[derive(Clone, Builder, Debug)]
#[builder(derive(Debug))]
//#[builder(pattern = "immutable")]
pub struct ItemOutput {
    pub type_token: Token![type],
    pub ident: Ident,
    pub eq_token: Token![=],
    pub ty: Type,
    pub semi_token: Token![;],
}

impl ItemOutput {
    pub fn with_self() -> Self {
        Self::_from_ty(parse_quote!(Self))
    }

    pub fn _from_ty(ty: Type) -> Self {
        ItemOutputBuilder::default()
            .ty(ty.clone())
            .build()
            .unwrap_or_else(|_| panic!("failed to build `ItemOutput` with `{:?}`", ty))
    }
}

/// Method implementation for a binary operation:
/// `fn op(self, rhs: B) -> C { .. }`
#[derive(Clone, Builder, Debug)]
#[builder(derive(Debug))]
pub struct ItemFn {
    pub attrs: Vec<Attribute>,
    pub fn_token: Token![fn],
    pub ident: Ident,
    pub paren_token: token::Paren,
    pub lhs_arg: Receiver,
    pub comma_token: Token![,],
    pub rhs_arg: FnArg,
    pub arrow_token: Token![->],
    pub out_ty: Type,
    pub block: Block,
}

/// An impl block for a binary operation.
///
/// This represents a rather small subset of `syn::ItemImpl`, and is therefore
/// very picky about what it will accept.
///
/// Example:
/// ```
/// impl BinOpTrait<B> for A {
///     type Output = C;
///     
///     fn op(self, rhs: B) -> Self::Output { ... }
/// }
/// ```
/// where `BinOpTrait` is any of the binary operations in `std::ops` (e.g.,
/// `Add`, `Mul`, `Rem`, `Shl`, etc.).
///
/// It should work for any typical implementation of the `std::ops` binary
/// operations (including e.g., generics, default `Rhs = Self`, mutable method
/// arguments, etc.), and any custom binary operation, provided that its trait
/// implementation is sufficiently similar.
///
/// In theory, generics can be freely used, or at least up to the same freedom
/// as `syn::Generics` allows
///
/// A feature of `syn::ItemImpl` which is lacking here is support for optional
/// default and visibility keywords; these may be added later.
#[derive(Clone, Builder, Debug)]
#[builder(derive(Debug))]
pub struct TraitImpl {
    pub attrs: Vec<Attribute>,
    pub impl_token: Token![impl],
    pub generics: Generics,
    pub trait_: Path, // likely not the most general, should be its own thing
    #[builder(default)]
    pub lt_token: Token![<],
    pub rhs_ty: Type,
    pub for_token: Token![for],
    pub lhs_ty: Type,
    pub brace_token: token::Brace,
    pub item_out: ItemOutput,
    pub item_fn: ItemFn,
}

// builder helper stuff ----------------------------------------------------------------------------

pub trait Buildable: Clone {
    type BuilderStruct: From<Self>;
    fn builder(&self) -> Self::BuilderStruct {
        self.clone().into()
    }
}

/// A helper trait for builders (derived from `derive_builder`) of structs
/// implementing `Parse`.
trait ParsedBuild: Clone {
    /// Original struct from which the builder was derived
    type BaseStruct: Parse;

    /// Builds the struct (with the usual `build` method), converting the
    /// result into a `syn::Result`.
    ///
    /// This is intended to be used after parsing and, at which point, building
    /// the struct should be infallible. If this method returns an error, then
    /// it is most likely that some field was not initialized in the builder
    /// during parsing
    ///
    /// If there were a proper trait for builder, then this could be given a
    /// default implementation
    fn parsed_build(&self) -> syn::Result<Self::BaseStruct>;
}

/// Implements some helpful builder features
///
/// at this point, prolly just gonna make my own darn builder deriver
macro_rules! build_extra {
    (@impl_from $Base:ty, $Builder:ty: $( $field:ident ),* $(,)?) => {
        impl From<$Base> for $Builder {
            fn from(base: $Base) -> Self {
                let mut bldr = <$Builder>::default();
                bldr $( .$field(base.$field) )* ;

                // return
                bldr
            }
        }
        impl From<&$Base> for $Builder {
            fn from(base: &$Base) -> Self {
                base.clone().into()
            }
        }
    };
    (@impl_buildable $Base:ty, $Builder:ty) => {
        impl Buildable for $Base {
            type BuilderStruct = $Builder;
        }
    };
    (@err_str $Base:ty) => {
        format!(
            "some struct `{0}Builder` failed to build during a call to method `parsed_build` \
            \n\nnote: Perhaps the implementation of `Parse` for `{0}` failed to initialize \
            \n      some field(s) in its builder before returning \n\n",
            stringify!($Base)
        ).as_str()
    };
    (@impl_parsed_build $Base:ty, $Builder:ty) => {
        impl ParsedBuild for $Builder {
            type BaseStruct = $Base;

            fn parsed_build(&self) -> syn::Result<Self::BaseStruct> {
                self.build()
                    .map(Ok)
                    .expect(build_extra!(@err_str $Base))
            }
        }
    };
    (@impl_build_ok $Base:ty, $Builder:ty) => {
        impl $Builder {
            pub fn build_option(&self) -> Option<$Base> {
                Some(self.build().expect("if this fails ur dumb"))
            }
        }
    };
    ($Base:ty, $Builder:ty : $( $field:ident ),* $(,)?) => {
        build_extra! { @impl_from $Base, $Builder: $($field),* }
        build_extra! { @impl_buildable $Base, $Builder }
        build_extra! { @impl_parsed_build $Base, $Builder }
        build_extra! {@impl_build_ok $Base, $Builder }
    };
}

build_extra! { ItemOutput, ItemOutputBuilder:
    type_token,
    ident,
    eq_token,
    ty,
    semi_token,
}

build_extra! { ItemFn, ItemFnBuilder:
    attrs,
    fn_token,
    ident,
    paren_token,
    lhs_arg,
    comma_token,
    rhs_arg,
    arrow_token,
    out_ty,
    block,
}

build_extra! { TraitImpl, TraitImplBuilder:
    attrs,
    impl_token,
    generics,
    trait_,
    lt_token,
    rhs_ty,
    for_token,
    lhs_ty,
    brace_token,
    item_out,
    item_fn,
}

// impl Parse --------------------------------------------------------------------------------------

impl Parse for MetaArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        MetaArg::try_from(ident.to_string().as_str())
            .map_err(|msg| syn::Error::new(ident.span(), msg))
    }
}

impl MetaArg {
    pub fn parse_list(input: ParseStream) -> syn::Result<Vec<Self>> {
        let mut args = Vec::new();

        while input.peek(Ident::peek_any) {
            args.push(input.parse()?);
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
        }

        Ok(args)
    }
}

impl Parse for ImplArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Vec::new();

        while input.peek(Ident::peek_any) {
            args.push(input.parse()?);
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
        }

        // args.push(input.parse()?);

        Ok(ImplArgs { args })
    }
}

impl Parse for ItemOutput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut bldr = ItemOutputBuilder::default();

        bldr.type_token(input.parse()?)
            .ident(input.parse()?)
            .eq_token(input.parse()?)
            .ty(input.parse()?)
            .semi_token(input.parse()?);

        bldr.parsed_build()
    }
}

impl Parse for ItemFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut bldr = ItemFnBuilder::default();

        bldr.attrs(input.call(Attribute::parse_outer)?)
            .fn_token(input.parse()?)
            .ident(input.parse()?);

        let content;
        bldr.paren_token(parenthesized!(content in input));
        // maybe unnecessary, but should guarantee proper evaluation order
        bldr.lhs_arg(content.parse()?)
            .comma_token(content.parse()?)
            .rhs_arg(content.parse()?)
            .arrow_token(input.parse()?)
            .out_ty(input.parse()?)
            .block(input.parse()?);

        bldr.parsed_build()
    }
}

impl Parse for TraitImpl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut bldr = TraitImplBuilder::default();

        bldr.attrs(input.call(Attribute::parse_outer)?)
            .impl_token(input.parse()?);

        let mut generics: Generics = input.parse()?;

        bldr.trait_(input.call(Path::parse_mod_style)?);

        if input.peek(Token![<]) {
            bldr.lt_token(input.parse()?);
            if !input.peek(Token![>]) {
                bldr.rhs_ty(input.parse()?);
            }
            let _: Token![>] = input.parse()?;
        }

        bldr.for_token(input.parse()?).lhs_ty(input.parse()?);

        if bldr.rhs_ty.is_none() {
            bldr.rhs_ty = bldr.lhs_ty.clone();
        }

        /* bldr.lt_token(if input.peek(Token![<]) {
            if input.peek2(Token![>]) {
                let _: Token![<] = input.parse()?;
                let _: Token![>] = input.parse()?;
                None
            } else {
                Some(input.parse()?)
            }
        } else {
            None
        });

        if bldr.lt_token.is_none() {
            bldr.for_token(input.parse()?).lhs_ty(input.parse()?);
            let lhs_ty = bldr.lhs_ty.clone().unwrap();
            bldr.rhs_ty(lhs_ty);
        } else {
            bldr.rhs_ty(input.parse()?);
            let _: Token![>] = input.parse()?;
            bldr.for_token(input.parse()?).lhs_ty(input.parse()?);
        } */

        generics.where_clause = input.parse()?;
        bldr.generics(generics);

        let content;
        bldr.brace_token(braced!(content in input));
        // maybe unnecessary, but clippy whines if these are strung together.
        // guarantees proper evaluation order, if that is indeed a concern.
        bldr.item_out(content.parse()?).item_fn(content.parse()?);

        bldr.parsed_build()
    }
}

// impl ToTokens -----------------------------------------------------------------------------------

impl ToTokens for MetaArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            MetaArg::Commute => quote!(commute),
            MetaArg::RefsClone => quote!(refs_clone),
            MetaArg::Derefs => quote!(derefs),
        })
    }
}



impl ToTokens for ImplArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = &self.args;
        tokens.append_all(quote! {
            binop( #(#args,)* )
        })
    }
}

impl ToTokens for ItemOutput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.type_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for ItemFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.attrs);
        self.fn_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.lhs_arg.to_tokens(tokens);
            self.comma_token.to_tokens(tokens);
            self.rhs_arg.to_tokens(tokens);
        });
        self.arrow_token.to_tokens(tokens);
        // tokens.append_all(quote!(Self::Output));
        self.out_ty.to_tokens(tokens);
        self.block.to_tokens(tokens);
    }
}

macro_rules! lr_angled {
    ($item:expr) => {
        {
            let rhs_ty = $item;
            quote!(<#rhs_ty>)
        }
    };
}

impl ToTokens for TraitImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.attrs);
        self.impl_token.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.trait_.to_tokens(tokens);
        tokens.append_all(lr_angled!(&self.rhs_ty));
        self.for_token.to_tokens(tokens);
        self.lhs_ty.to_tokens(tokens);
        self.generics.where_clause.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            self.item_out.to_tokens(tokens);
            self.item_fn.to_tokens(tokens);
        });
    }
}
