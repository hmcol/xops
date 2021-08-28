#[cfg(test)]
use proc_macro::TokenStream;

/// prints `item` under the label `header`
///
/// primarily used for checking inputs of macros
/// 
#[cfg(test)]
pub fn print_ts(header: &str, item: &TokenStream) {
    println!("BEGIN {} \n{}\nEND\n", header, item.to_string());
}
