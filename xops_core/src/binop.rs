use proc_macro2::TokenStream;
use quote::quote;

mod parse;
pub use parse::*;

mod write;
use syn::parse_quote;
pub use write::*;

use crate::utils::print_tokens;

pub fn apply(args: &[MetaArg], impltn: TraitImpl) -> TokenStream {
    if !args.is_empty() {
        let arg1 = &args[0];
        let other_args = &args[1..];

        let impltn_with_args = parse_quote! {
            #[binop( #(#other_args),* )]
            #impltn
        };

        match arg1 {
            MetaArg::Commute => with_commute(impltn_with_args),
            MetaArg::RefsClone =>with_refs(impltn_with_args),
            MetaArg::Derefs => with_derefs(impltn_with_args),
        }
    } else {
        quote!(#impltn)
    }
}


pub fn read_impl(impltn: TraitImpl) -> TokenStream {
    let expanded = quote! {
        #impltn
    };

    print_tokens("binop::read_impl expanded", &expanded);

    // return
    expanded
}

pub fn with_derefs(impltn: TraitImpl) -> TokenStream {
    let deref_ref = impltn.try_deref_lhs();
    let ref_deref = impltn.try_deref_rhs();
    let deref_deref = impltn.try_deref_both();

    let expanded = quote! {
        #impltn
        
        #deref_ref
        #ref_deref
        #deref_deref
    };

    print_tokens("binop::with_derefs expanded", &expanded);

    // return
    expanded
}

pub fn with_refs(impltn: TraitImpl) -> TokenStream {
    let ref_own = impltn.ref_lhs_clone();
    let own_ref = impltn.ref_rhs_clone();
    let ref_ref = impltn.ref_both_clone();

    let expanded = quote! {
        #impltn

        #ref_own
        #own_ref
        #ref_ref
    };

    print_tokens("binop::with_refs expanded", &expanded);

    // return
    expanded
}

pub fn with_commute(impltn: TraitImpl) -> TokenStream {
    let commute = impltn.commute();

    let expanded = quote! {
        #impltn

        #commute
    };

    print_tokens("binop::with_commute expanded", &expanded);

    // return
    expanded
}