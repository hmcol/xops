use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse_quote;

use crate::{utils::TypeConversion, BinOpFn, BinOpImpl, BinOpOutput};

impl BinOpImpl {
    /// If `lhs_ty = &A`, this returns an implementation of `A op B` utilizing `&A op B`.
    /// 
    /// If `lhs_ty` is not a reference type, this returns `None`.
    /// 
    /// In other words, if `self` is of the form
    /// ```
    /// impl Op<B> for &A {
    ///     ...
    /// }
    /// ```
    /// then this produces the implementation
    /// ```
    /// impl Op<B> for A {
    ///     ...
    ///     fn op(self, rhs: B) -> Self::Output {
    ///         (&self).op(rhs)
    ///     }
    /// }
    /// ```
    pub fn try_deref_lhs(&self) -> Option<Self> {
        let lhs_ty = self.lhs_ty.as_deref()?;
        let rhs_ty = &self.rhs_ty;
        let fn_ident = &self.item_fn.ident;
        let item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                (&self).#fn_ident(rhs)
            }
        };

        Some(BinOpImpl {
            lhs_ty,
            item_fn,
            ..self.clone()
        })
    }

    /// If `rhs_ty = &B`, this returns an implementation of `A op B` utilizing `A op &B`.
    /// 
    /// If `rhs_ty` is not a reference type, this returns `None`.
    /// 
    /// In other words, if `self` is of the form
    /// ```
    /// impl Op<&B> for A {
    ///     ...
    /// }
    /// ```
    /// then this produces the implementation
    /// ```
    /// impl Op<B> for A {
    ///     ...
    ///     fn op(self, rhs: B) -> Self::Output {
    ///         self.op(&rhs)
    ///     }
    /// }
    /// ```
    pub fn try_deref_rhs(&self) -> Option<Self> {
        let rhs_ty = self.rhs_ty.as_deref()?;
        let fn_ident = &self.item_fn.ident;
        let item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                self.#fn_ident(&rhs)
            }
        };

        Some(BinOpImpl {
            rhs_ty,
            item_fn,
            ..self.clone()
        })
    }

    /// If `lhs_ty = &A` and `rhs_ty = &B`, this returns an implementation of `A op B` utilizing `&A op &B`.
    /// 
    /// If `lhs_ty` and `rhs_ty` not a references type, this returns `None`.
    /// 
    /// In other words, if `self` is of the form
    /// ```
    /// impl Op<&B> for &A {
    ///     ...
    /// }
    /// ```
    /// then this produces the implementation
    /// ```
    /// impl Op<B> for A {
    ///     ...
    ///     fn op(self, rhs: B) -> Self::Output {
    ///         (&self).op(&rhs)
    ///     }
    /// }
    /// ```
    pub fn try_deref_both(&self) -> Option<Self> {
        let lhs_ty = self.lhs_ty.as_deref()?;
        let rhs_ty = self.rhs_ty.as_deref()?;
        let fn_ident = &self.item_fn.ident;
        let item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                (&self).#fn_ident(&rhs)
            }
        };

        Some(BinOpImpl {
            lhs_ty,
            rhs_ty,
            item_fn,
            ..self.clone()
        })
    }

    /// Returns an implementation of `&A op B` utilizing `A op B`.
    /// 
    /// The macro user must enure that `A: Clone`.
    /// 
    /// In other words, if `self` is of the form
    /// ```
    /// impl Op<B> for A {
    ///     ...
    /// }
    /// ```
    /// then this produces the implementation
    /// ```
    /// impl Op<B> for &A {
    ///     ...
    ///     fn op(self, rhs: B) -> Self::Output {
    ///         self.clone().op(rhs)
    ///     }
    /// }
    /// ```
    pub fn ref_lhs_clone(&self) -> Self {
        let fn_ident = &self.item_fn.ident;
        let lhs_ty = self.lhs_ty.as_ref();
        let rhs_ty = &self.rhs_ty;

        let item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                self.clone().#fn_ident(rhs)
            }
        };

        BinOpImpl {
            lhs_ty,
            item_fn,
            ..self.clone()
        }
    }

    /// Returns an implementation of `A op &B` utilizing `A op B`.
    /// 
    /// The macro user must enure that `B: Clone`.
    /// 
    /// In other words, if `self` is of the form
    /// ```
    /// impl Op<B> for A {
    ///     ...
    /// }
    /// ```
    /// then this produces the implementation
    /// ```
    /// impl Op<&B> for A {
    ///     ...
    ///     fn op(self, rhs: B) -> Self::Output {
    ///         self.op(rhs.clone())
    ///     }
    /// }
    /// ```
    pub fn ref_rhs_clone(&self) -> Self {
        let fn_ident = &self.item_fn.ident;
        let rhs_ty = self.rhs_ty.as_ref();

        let item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                self.#fn_ident(rhs.clone())
            }
        };

        BinOpImpl {
            rhs_ty,
            item_fn,
            ..self.clone()
        }
    }

    /// Returns an implementation of `&A op &B` utilizing `A op B`.
    /// 
    /// The macro user must enure that `A: Clone` and `B: Clone`.
    /// 
    /// In other words, if `self` is of the form
    /// ```
    /// impl Op<B> for A {
    ///     ...
    /// }
    /// ```
    /// then this produces the implementation
    /// ```
    /// impl Op<&B> for &A {
    ///     ...
    ///     fn op(self, rhs: B) -> Self::Output {
    ///         self.clone().op(rhs.clone())
    ///     }
    /// }
    /// ```
    pub fn ref_both_clone(&self) -> Self {
        let fn_ident = &self.item_fn.ident;
        let lhs_ty = self.lhs_ty.as_ref();
        let rhs_ty = self.rhs_ty.as_ref();

        let item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                self.clone().#fn_ident(rhs.clone())
            }
        };

        BinOpImpl {
            lhs_ty,
            rhs_ty,
            item_fn,
            ..self.clone()
        }
    }

    /// Returns an implementation of `B op A` utilizing `A op B`.
    /// 
    /// The macro user must enure that some `impl Op<A> for B` does not exist elsewhere.
    /// 
    /// In other words, if `self` is of the form
    /// ```
    /// impl Op<B> for A {
    ///     ...
    /// }
    /// ```
    /// then this produces the implementation
    /// ```
    /// impl Op<A> for B {
    ///     ...
    ///     fn op(self, rhs: A) -> Self::Output {
    ///         rhs.op(self)
    ///     }
    /// }
    /// ```
    pub fn commute(&self) -> Self {
        let lhs_ty = self.rhs_ty.clone();
        let rhs_ty = self.lhs_ty.clone();

        let fn_ident = &self.item_fn.ident;
        let item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                rhs.#fn_ident(self)
            }
        };

        BinOpImpl {
            lhs_ty,
            rhs_ty,
            item_fn,
            ..self.clone()
        }
    }
}

// impl ToTokens -----------------------------------------------------------------------------------

impl ToTokens for BinOpOutput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.type_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}

impl ToTokens for BinOpFn {
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
        self.out_ty.to_tokens(tokens);
        self.block.to_tokens(tokens);
    }
}

macro_rules! lr_angled {
    ($item:expr) => {{
        let rhs_ty = $item;
        quote!(<#rhs_ty>)
    }};
}

impl ToTokens for BinOpImpl {
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
