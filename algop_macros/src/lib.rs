use proc_macro::TokenStream;
use syn::parse_macro_input;

/// this workspace
use algop_core::*;

// this crate
mod utils;
use utils::print_ts;


// helpers -----------------------------------------------------------------------------------------

/// prints `item` under the label `header`
/// 
/// primarily used for checking inputs of macros


// main macros -------------------------------------------------------------------------------------

#[proc_macro_attribute]
pub fn read_binop_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    print_ts("read_binop_impl item", &item);

    let expanded = algop_core::binop::read_impl(parse_macro_input!(item as binop::TraitImpl));
    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
