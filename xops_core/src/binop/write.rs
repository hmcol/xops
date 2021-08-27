use syn::parse_quote;

use super::{Buildable, ItemFn, TraitImpl};
use crate::utils::TypeConversion;

impl TraitImpl {
    pub fn try_deref_lhs(&self) -> Option<Self> {
        let deref_lhs_ty = self.lhs_ty.as_deref()?;

        let fn_ident = &self.item_fn.ident;
        let rhs_ty = &self.rhs_ty;
        let new_item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                (&self).#fn_ident(rhs)
            }
        };

        self.builder()
            .lhs_ty(deref_lhs_ty)
            .item_fn(new_item_fn)
            .build_option()
    }

    pub fn try_deref_rhs(&self) -> Option<Self> {
        let deref_rhs_ty = self.rhs_ty.as_deref()?;

        let fn_ident = &self.item_fn.ident;
        let new_item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #deref_rhs_ty) -> Self::Output {
                self.#fn_ident(&rhs)
            }
        };

        self.builder()
            .rhs_ty(deref_rhs_ty)
            .item_fn(new_item_fn)
            .build_option()
    }

    pub fn try_deref_both(&self) -> Option<Self> {
        let deref_lhs_ty = self.lhs_ty.as_deref()?;
        let deref_rhs_ty = self.rhs_ty.as_deref()?;

        let fn_ident = &self.item_fn.ident;
        let new_item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #deref_rhs_ty) -> Self::Output {
                (&self).#fn_ident(&rhs)
            }
        };

        self.builder()
            .lhs_ty(deref_lhs_ty)
            .rhs_ty(deref_rhs_ty)
            .item_fn(new_item_fn)
            .build_option()
    }

    pub fn ref_lhs_clone(&self) -> Self {
        let ref_lhs_ty = self.lhs_ty.as_ref();

        let fn_ident = &self.item_fn.ident;
        let rhs_ty = &self.rhs_ty;
        let new_item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #rhs_ty) -> Self::Output {
                self.clone().#fn_ident(rhs)
            }
        };

        self.builder()
            .lhs_ty(ref_lhs_ty)
            .item_fn(new_item_fn)
            .build_option()
            .unwrap()
    }

    pub fn ref_rhs_clone(&self) -> Self {
        let ref_rhs_ty = self.rhs_ty.as_ref();

        let fn_ident = &self.item_fn.ident;
        let new_item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #ref_rhs_ty) -> Self::Output {
                self.#fn_ident(rhs.clone())
            }
        };

        self.builder()
            .rhs_ty(ref_rhs_ty)
            .item_fn(new_item_fn)
            .build_option()
            .unwrap()
    }

    pub fn ref_both_clone(&self) -> Self {
        let ref_lhs_ty = self.lhs_ty.as_ref();
        let ref_rhs_ty = self.rhs_ty.as_ref();

        let fn_ident = &self.item_fn.ident;
        let new_item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #ref_rhs_ty) -> Self::Output {
                self.clone().#fn_ident(rhs.clone())
            }
        };

        self.builder()
            .lhs_ty(ref_lhs_ty)
            .rhs_ty(ref_rhs_ty)
            .item_fn(new_item_fn)
            .build_option()
            .unwrap()
    }

    pub fn commute(&self) -> Self {
        let lhs_ty = self.lhs_ty.clone();
        let rhs_ty = self.rhs_ty.clone();

        let fn_ident = &self.item_fn.ident;
        let new_item_fn = parse_quote! {
            fn #fn_ident(self, rhs: #lhs_ty) -> Self::Output {
                rhs.#fn_ident(self)
            }
        };

        self.builder()
            .lhs_ty(rhs_ty)
            .rhs_ty(lhs_ty)
            .item_fn(new_item_fn)
            .build_option()
            .unwrap()
    }
}

impl ItemFn {}
