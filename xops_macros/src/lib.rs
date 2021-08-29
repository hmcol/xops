use proc_macro::TokenStream;
use syn::parse_macro_input;

/// this workspace
use xops_core::binop;

// this crate

mod utils;
#[cfg(test)]
use utils::print_ts;


// helpers -----------------------------------------------------------------------------------------



// main macros -------------------------------------------------------------------------------------


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
    #[cfg(test)]
    print_ts("binop args", &args);
    #[cfg(test)]
    print_ts("binop item", &item);

    let args = parse_macro_input!(args with binop::MetaArg::parse_list);
    let item = parse_macro_input!(item as binop::TraitImpl);

    // print_ts("binop args", &args);
    #[cfg(test)]
    dbg!(&args);
    // dbg!(&item);

    let expanded = binop::apply(&args, item);
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn read_binop_impl(_args: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(test)]
    print_ts("read_binop_impl item", &item);

    let expanded = binop::read_impl(parse_macro_input!(item as binop::TraitImpl));
    TokenStream::from(expanded)
}