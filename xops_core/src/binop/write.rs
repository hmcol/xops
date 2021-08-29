use syn::parse_quote;

use super::TraitImpl;
use crate::utils::TypeConversion;

impl TraitImpl {
    pub fn try_deref_lhs(&self) -> Option<Self> {
        let lhs_ty = self.lhs_ty.as_deref()?;
        let rhs_ty = &self.rhs_ty;
        let fn_ident = &self.item_fn.ident;
        let item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                (&self).#fn_ident(rhs)
            }
        };

        Some(TraitImpl {
            lhs_ty,
            item_fn,
            ..self.clone()
        })
    }

    pub fn try_deref_rhs(&self) -> Option<Self> {
        let rhs_ty = self.rhs_ty.as_deref()?;
        let fn_ident = &self.item_fn.ident;
        let item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                self.#fn_ident(&rhs)
            }
        };

        Some(TraitImpl {
            rhs_ty,
            item_fn,
            ..self.clone()
        })
    }

    pub fn try_deref_both(&self) -> Option<Self> {
        let lhs_ty = self.lhs_ty.as_deref()?;
        let rhs_ty = self.rhs_ty.as_deref()?;
        let fn_ident = &self.item_fn.ident;
        let item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                (&self).#fn_ident(&rhs)
            }
        };

        Some(TraitImpl {
            lhs_ty,
            rhs_ty,
            item_fn,
            ..self.clone()
        })
    }

    pub fn ref_lhs_clone(&self) -> Self {
        let fn_ident = &self.item_fn.ident;
        let lhs_ty = self.lhs_ty.as_ref();
        let rhs_ty = &self.rhs_ty;

        let item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                self.clone().#fn_ident(rhs)
            }
        };

        TraitImpl {
            lhs_ty,
            item_fn,
            ..self.clone()
        }
    }

    pub fn ref_rhs_clone(&self) -> Self {
        let fn_ident = &self.item_fn.ident;
        // let lhs_ty = &self.lhs_ty;
        let rhs_ty = self.rhs_ty.as_ref();

        let item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                self.#fn_ident(rhs.clone())
            }
        };

        TraitImpl {
            rhs_ty,
            item_fn,
            ..self.clone()
        }
    }

    pub fn ref_both_clone(&self) -> Self {
        let fn_ident = &self.item_fn.ident;
        let lhs_ty = self.lhs_ty.as_ref();
        let rhs_ty = self.rhs_ty.as_ref();

        let item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                self.clone().#fn_ident(rhs.clone())
            }
        };

        TraitImpl {
            lhs_ty,
            rhs_ty,
            item_fn,
            ..self.clone()
        }
    }

    pub fn commute(&self) -> Self {
        let lhs_ty = self.rhs_ty.clone();
        let rhs_ty = self.lhs_ty.clone();

        let fn_ident = &self.item_fn.ident;
        let item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                rhs.#fn_ident(self)
            }
        };

        TraitImpl {
            lhs_ty,
            rhs_ty,
            item_fn,
            ..self.clone()
        }
    }
}
