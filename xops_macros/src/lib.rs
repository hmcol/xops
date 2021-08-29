use darling::FromMeta;
use proc_macro::TokenStream;
use syn::{AttributeArgs, parse_macro_input};

use xops_core::binop;


/// for deriving extra implementations of the operation
/// 
/// note that the type `Output` must be given with respect to outer scope, i.e.,
/// it should not in any way depend on the actual type to which `Self` refers.
/// 
/// For example, consider the following implementation with types `A != B`:
/// ```
/// #[binop(commute)]
/// impl Add<B> for A /* Self */ {
///     type Output = Self /* A */;
///     
///     fn add(self: A, rhs: B) -> Self::Output /* A */ {
///         ...
///     }
/// }
/// ```
/// The `#[binop(commute)]` will naively interpret the `Output` type definition,
/// which produces the following commutation:
/// ```
/// impl Add<A> for B /* Self */ {
///     type Output = Self /* B */;
///     
///     fn add(self: B, rhs: A) -> Self::Output /* B */ {
///         rhs.add(self) /* : A */
///     }
/// }
/// ```
/// The output is `Self` in both cases, but this is `A` in the first and `B` in
/// the second. Since the second implementation attempts to directly utilize
/// the first, the types will conflict and compilation will fail.
/// 
/// This can theoretically be fixed by first expanding all types present in the
/// implementation into paths which resolve in the outer scope.
#[proc_macro_attribute]
pub fn binop(args: TokenStream, item: TokenStream) -> TokenStream {
    // print_ts("binop args", &args);
    // print_ts("binop item", &item);

    let attr_args = parse_macro_input!(args as AttributeArgs);
    let binop_impl = parse_macro_input!(item as binop::TraitImpl);

    let binop_args = match binop::BinOpArgs::from_list(&attr_args) {
        Ok(args) => args,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };

    let expanded = binop::expand(binop_args, binop_impl);

    TokenStream::from(expanded)
}

// testing -----------------------------------------------------------------------------------------

fn print_ts(header: &str, item: &TokenStream) {
    println!("BEGIN {} \n{}\nEND\n", header, item.to_string());
}


#[proc_macro_attribute]
pub fn read_binop_impl(_args: TokenStream, item: TokenStream) -> TokenStream {
    print_ts("read_binop_impl item", &item);

    let expanded = binop::read_impl(parse_macro_input!(item as binop::TraitImpl));
    TokenStream::from(expanded)
}