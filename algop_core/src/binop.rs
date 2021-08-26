use proc_macro2::TokenStream;
use quote::quote;

mod parse;
pub use parse::*;

mod write;
pub use write::*;

use crate::utils::print_tokens;

pub fn read_impl(impltn: TraitImpl) -> TokenStream {
    let expanded = quote! {
        #impltn
    };

    print_tokens("binop::read_impl expanded", &expanded);

    // return
    expanded
}
