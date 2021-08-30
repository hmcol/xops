use darling::FromMeta;
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    token, Attribute, Block, FnArg, Generics, Ident, Path, Receiver, Token, Type,
};

// structs -----------------------------------------------------------------------------------------

/// Arguments for the macro `#[binop(...)]`
#[derive(Clone, Copy, Default, FromMeta, Debug)]
#[darling(default)]
pub struct BinOpArgs {
    pub dev_print: bool,
    pub commute: bool,
    pub refs_clone: bool,
    pub derefs: bool,
}

/// Type definition for the output of a binary operation: `type Output = C;`
#[derive(Clone, Debug)]
pub struct BinOpOutput {
    pub type_token: Token![type],
    pub ident: Ident,
    pub eq_token: Token![=],
    pub ty: Type,
    pub semi_token: Token![;],
}

/// Method implementation for a binary operation:
/// `fn op(self, rhs: B) -> C { .. }`
#[derive(Clone, Debug)]
pub struct BinOpFn {
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
/// impl Op<B> for A {
///     type Output = C;
///     
///     fn op(self, rhs: B) -> Self::Output {
///         ...
///     }
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
#[derive(Clone, Debug)]
pub struct BinOpImpl {
    pub attrs: Vec<Attribute>,
    pub impl_token: Token![impl],
    pub generics: Generics,
    pub trait_: Path, // likely not the most general, should be its own thing
    pub lt_token: Option<Token![<]>,
    pub rhs_ty: Type,
    pub for_token: Token![for],
    pub lhs_ty: Type,
    pub brace_token: token::Brace,
    pub item_out: BinOpOutput,
    pub item_fn: BinOpFn,
}

// impl Parse --------------------------------------------------------------------------------------

impl Parse for BinOpOutput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(BinOpOutput {
            type_token: input.parse()?,
            ident: input.parse()?,
            eq_token: input.parse()?,
            ty: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl Parse for BinOpFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(BinOpFn {
            attrs: input.call(Attribute::parse_outer)?,
            fn_token: input.parse()?,
            ident: input.parse()?,
            paren_token: parenthesized!(content in input),
            lhs_arg: content.parse()?,
            comma_token: content.parse()?,
            rhs_arg: content.parse()?,
            arrow_token: input.parse()?,
            out_ty: input.parse()?,
            block: input.parse()?,
        })
    }
}

impl Parse for BinOpImpl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let impl_token = input.parse()?;

        let mut generics: Generics = input.parse()?;

        let trait_ = input.call(Path::parse_mod_style)?;

        let mut lt_token = None;
        if input.peek(Token![<]) {
            if input.peek2(Token![>]) {
                let _: Token![<] = input.parse()?;
                let _: Token![>] = input.parse()?;
            } else {
                lt_token = Some(input.parse()?);
            }
        }

        let rhs_ty;
        let for_token;
        let lhs_ty: Type;
        if lt_token.is_some() {
            rhs_ty = input.parse()?;
            let _: Token![>] = input.parse()?;
            for_token = input.parse()?;
            lhs_ty = input.parse()?;
        } else {
            for_token = input.parse()?;
            lhs_ty = input.parse()?;
            rhs_ty = lhs_ty.clone();
        }

        generics.where_clause = input.parse()?;

        let content;
        let brace_token = braced!(content in input);
        let item_out = content.parse()?;
        let item_fn = content.parse()?;

        Ok(BinOpImpl {
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
        })
    }
}
