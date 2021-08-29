use proc_macro2::TokenStream;
use quote::quote;

mod parse;
pub use parse::*;

mod write;
pub use write::*;

use crate::utils::print_tokens;

pub fn expand(args: BinOpArgs, impl_: TraitImpl) -> TokenStream {
    if args.dev_print {
        dbg!(args);
        print_tokens("binop impltn", &impl_);
    }

    if args.commute {
        expand_commute(args, impl_)
    } else if args.refs_clone {
        expand_refs_clone(args, impl_)
    } else if args.derefs {
        expand_derefs(args, impl_)
    } else {
        quote!(#impl_)
    }
}

pub fn expand_option(args: BinOpArgs, option_impl: Option<TraitImpl>) -> TokenStream {
    match option_impl {
        Some(impl_) => expand(args, impl_),
        None => TokenStream::default(),
    }
}

fn expand_commute(args: BinOpArgs, impl_: TraitImpl) -> TokenStream {
    let new_args = BinOpArgs {
        commute: false,
        ..args
    };

    let expanded = expand(new_args, impl_.clone());
    let expanded_comm = expand(new_args, impl_.commute());

    quote! {
        #expanded
        #expanded_comm
    }
}

fn expand_refs_clone(args: BinOpArgs, impl_: TraitImpl) -> TokenStream {
    let new_args = BinOpArgs {
        refs_clone: false,
        ..args
    };

    let expanded = expand(new_args, impl_.clone());
    let expanded_ref_own = expand(new_args, impl_.ref_lhs_clone());
    let expanded_own_ref = expand(new_args, impl_.ref_rhs_clone());
    let expanded_ref_ref = expand(new_args, impl_.ref_both_clone());

    quote! {
        #expanded
        #expanded_ref_own
        #expanded_own_ref
        #expanded_ref_ref
    }
}

fn expand_derefs(args: BinOpArgs, impl_: TraitImpl) -> TokenStream {
    let new_args = BinOpArgs {
        derefs: false,
        ..args
    };

    let expanded = expand(new_args, impl_.clone());
    let expanded_deref_ref = expand_option(new_args, impl_.try_deref_lhs());
    let expanded_ref_deref = expand_option(new_args, impl_.try_deref_rhs());
    let expanded_deref_deref = expand_option(new_args, impl_.try_deref_both());

    quote! {
        #expanded
        #expanded_deref_ref
        #expanded_ref_deref
        #expanded_deref_deref
    }
}

/* pub fn apply(args: &[MetaArg], impltn: TraitImpl) -> TokenStream {
    if !args.is_empty() {
        let arg1 = &args[0];
        let other_args = &args[1..];

        let impltn_with_args = parse_quote! {
            #[binop( #(#other_args),* )]
            #impltn
        };

        match arg1 {
            MetaArg::Commute => with_commute(impltn_with_args),
            MetaArg::RefsClone => with_refs_clone(impltn_with_args),
            MetaArg::Derefs => with_derefs(impltn_with_args),
        }
    } else {
        quote!(#impltn)
    }
} */

pub fn read_impl(impltn: TraitImpl) -> TokenStream {
    let expanded = quote! {
        #impltn
    };

    #[cfg(test)]
    print_tokens("binop::read_impl expanded", &expanded);

    // return
    expanded
}

pub fn with_commute(impltn: TraitImpl) -> TokenStream {
    let commute = impltn.commute();

    let expanded = quote! {
        #impltn

        #commute
    };

    #[cfg(test)]
    print_tokens("binop::with_commute expanded", &expanded);

    // return
    expanded
}

pub fn with_refs_clone(impltn: TraitImpl) -> TokenStream {
    let ref_own = impltn.ref_lhs_clone();
    let own_ref = impltn.ref_rhs_clone();
    let ref_ref = impltn.ref_both_clone();

    let expanded = quote! {
        #impltn

        #ref_own
        #own_ref
        #ref_ref
    };

    #[cfg(test)]
    print_tokens("binop::with_refs expanded", &expanded);

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

    #[cfg(test)]
    print_tokens("binop::with_derefs expanded", &expanded);

    // return
    expanded
}
