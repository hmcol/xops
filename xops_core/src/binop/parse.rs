use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Attribute, Block, FnArg, Generics, Ident, Path, Receiver, Token, Type, braced, parenthesized, parse::{Parse, ParseStream}, parse_quote, token};
use darling::FromMeta;

// structs -----------------------------------------------------------------------------------------

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
pub struct ItemOutput {
    pub type_token: Token![type],
    pub ident: Ident,
    pub eq_token: Token![=],
    pub ty: Type,
    pub semi_token: Token![;],
}

impl ItemOutput {
    pub fn with_self() -> Self {
        Self::from_ty(parse_quote!(Self))
    }

    pub fn from_ty(ty: Type) -> Self {
        ItemOutput {
            type_token: <Token![type]>::default(),
            ident: parse_quote!(Output),
            eq_token: <Token![=]>::default(),
            ty,
            semi_token: <Token![;]>::default(),
        }
    }
}

/// Method implementation for a binary operation:
/// `fn op(self, rhs: B) -> C { .. }`
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct TraitImpl {
    pub attrs: Vec<Attribute>,
    pub impl_token: Token![impl],
    pub generics: Generics,
    pub trait_: Path, // likely not the most general, should be its own thing
    pub lt_token: Option<Token![<]>,
    pub rhs_ty: Type,
    pub for_token: Token![for],
    pub lhs_ty: Type,
    pub brace_token: token::Brace,
    pub item_out: ItemOutput,
    pub item_fn: ItemFn,
}

// impl Parse --------------------------------------------------------------------------------------

impl Parse for ItemOutput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ItemOutput {
            type_token: input.parse()?,
            ident: input.parse()?,
            eq_token: input.parse()?,
            ty: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl Parse for ItemFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(ItemFn {
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

impl Parse for TraitImpl {
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

        Ok(TraitImpl {
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

// impl ToTokens -----------------------------------------------------------------------------------

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
