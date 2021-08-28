use proc_macro::TokenStream;
use syn::parse_macro_input;

/// this workspace
use xops_core::binop;

// this crate
mod utils;
use utils::print_ts;


// helpers -----------------------------------------------------------------------------------------

/// prints `item` under the label `header`
/// 
/// primarily used for checking inputs of macros


// main macros -------------------------------------------------------------------------------------

#[proc_macro_attribute]
pub fn binop(args: TokenStream, item: TokenStream) -> TokenStream {
    print_ts("binop args", &args);
    print_ts("binop item", &item);

    let args = parse_macro_input!(args with binop::MetaArg::parse_list);
    let item = parse_macro_input!(item as binop::TraitImpl);

    // print_ts("binop args", &args);
    dbg!(&args);
    // dbg!(&item);

    let expanded = binop::apply(&args, item);
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn read_binop_impl(_args: TokenStream, item: TokenStream) -> TokenStream {
    print_ts("read_binop_impl item", &item);

    let expanded = binop::read_impl(parse_macro_input!(item as binop::TraitImpl));
    TokenStream::from(expanded)
}
/* 
#[proc_macro_attribute]
pub fn binop_with_derefs(_args: TokenStream, item: TokenStream) -> TokenStream {
    print_ts("binop_with_derefs item", &item);

    let expanded = binop::with_derefs(parse_macro_input!(item as binop::TraitImpl));
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn binop_with_refs(_args: TokenStream, item: TokenStream) -> TokenStream {
    print_ts("binop_with_refs item", &item);

    let expanded = binop::with_refs(parse_macro_input!(item as binop::TraitImpl));
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn binop_with_commute(_args: TokenStream, item: TokenStream) -> TokenStream {
    print_ts("binop_commute item", &item);

    let expanded = binop::with_commute(parse_macro_input!(item as binop::TraitImpl));
    TokenStream::from(expanded)
}
 */