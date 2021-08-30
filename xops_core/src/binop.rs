use proc_macro2::TokenStream;
use quote::quote;

mod parse;
pub use parse::*;

mod write;
pub use write::*;

use crate::utils::print_tokens;

impl BinOpImpl {
    pub fn expand(&self, args: BinOpArgs) -> TokenStream {
        expand(args, self.clone())
    }
}

fn expand(args: BinOpArgs, impl_: BinOpImpl) -> TokenStream {
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

pub fn expand_option(args: BinOpArgs, option_impl: Option<BinOpImpl>) -> TokenStream {
    match option_impl {
        Some(impl_) => expand(args, impl_),
        None => TokenStream::default(),
    }
}

fn expand_commute(args: BinOpArgs, impl_: BinOpImpl) -> TokenStream {
    let new_args = BinOpArgs {
        commute: false,
        ..args
    };

    let expanded = impl_.expand(new_args); // expand(new_args, impl_.clone());
    let expanded_comm = impl_.commute().expand(new_args); // expand(new_args, impl_.commute());

    quote! {
        #expanded
        #expanded_comm
    }
}

fn expand_refs_clone(args: BinOpArgs, impl_: BinOpImpl) -> TokenStream {
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

fn expand_derefs(args: BinOpArgs, impl_: BinOpImpl) -> TokenStream {
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

pub fn read_impl(impltn: BinOpImpl) -> TokenStream {
    let expanded = quote! {
        #impltn
    };

    #[cfg(test)]
    print_tokens("binop::read_impl expanded", &expanded);

    // return
    expanded
}