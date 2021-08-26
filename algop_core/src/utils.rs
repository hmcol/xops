use quote::{quote, ToTokens};

/// quotes and prints `item` under the label `header`
///
/// primarily used for checking implementations of Parse and/or ToTokens
pub fn print_tokens<T: ToTokens>(header: &str, item: T) {
    println!("BEGIN {} \n{}\nEND\n", header, quote!(#item));
}
