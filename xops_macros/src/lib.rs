use darling::FromMeta;
use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs};

use xops_core::*;

/// For deriving extra implementations of a binary operation.
///
/// The `binop` attribute can be applied to any binary operation trait implementation (see [`xops_core::BinOpImpl`].
/// 
/// It has three possible arguments:
/// 
/// - **`commute`**
///     - from `A op B`, derive `B op A`
///     - derives an implementation with the argument types swapped.
/// 
/// - **`refs_clone`** 
///     - from `A op B`, derive `&A op B`, `A op &B`, `&A op &B`
///     - derives implementations for reference types using cloning.
/// 
/// - **`derefs`**
///     - from `&A op &B`, derive `A op &B`, `&A op B`, `A op B`
///     - derives implementations for owned types.
///
/// ## Caution!
///
/// Avoid using `Self` in the output type as this often causes `binop` to fail. Sometimes it won't fail, but if the type-checker is giving you errors, try using more explicit types.
/// 
/// 
/// # Example 1
/// 
/// The following example shows `binop` deriving both the commutation and all the reference implementations from the owned implementation using `Clone`able types.
/// ```
/// use std::ops::Mul;
/// use xops_macros::binop;
/// 
/// #[derive(Clone, Debug)]
/// struct Dog(i32);
/// 
/// #[derive(Clone, Debug)]
/// struct Cat(i32);
/// 
/// #[derive(PartialEq, Eq, Debug)]
/// struct Fish(i32);
/// 
/// #[binop(commute, refs_clone)]
/// impl Mul<Cat> for Dog {
///     type Output = Fish;
/// 
///     fn mul(self, rhs: Cat) -> Self::Output {
///         Fish(self.0 * rhs.0)
///     }
/// }
/// 
/// fn main() {
///     assert_eq!( Dog(3) *  Cat(5), Fish(15));
///     assert_eq!(&Dog(3) *  Cat(5), Fish(15));
///     assert_eq!( Dog(3) * &Cat(5), Fish(15));
///     assert_eq!(&Dog(3) * &Cat(5), Fish(15));
/// 
///     assert_eq!( Cat(3) *  Dog(5), Fish(15));
///     assert_eq!(&Cat(3) *  Dog(5), Fish(15));
///     assert_eq!( Cat(3) * &Dog(5), Fish(15));
///     assert_eq!(&Cat(3) * &Dog(5), Fish(15));
/// }
/// 
/// ```
/// The attribute `binop(commute, refs_clone)` above is equivalent to the following:
/// ```ignore
/// impl Mul<Cat> for &Dog {
///     type Output = Fish;
/// 
///     fn mul(self, rhs: Cat) -> Self::Output {
///         self.clone() * rhs
///     }
/// }
/// impl Mul<&Cat> for Dog {
///     type Output = Fish;
/// 
///     fn mul(self, rhs: &Cat) -> Self::Output {
///         self * rhs.clone()
///     }
/// }
/// impl Mul<&Cat> for &Dog {
///     type Output = Fish;
/// 
///     fn mul(self, rhs: &Cat) -> Self::Output {
///         self.clone() * rhs.clone()
///     }
/// }
/// impl Mul<Dog> for Cat {
///     type Output = Fish;
/// 
///     fn mul(self, rhs: Dog) -> Self::Output {
///         rhs * self
///     }
/// }
/// impl Mul<Dog> for &Cat {
///     type Output = Fish;
/// 
///     fn mul(self, rhs: Dog) -> Self::Output {
///         self.clone() * rhs
///     }
/// }
/// impl Mul<&Dog> for Cat {
///     type Output = Fish;
/// 
///     fn mul(self, rhs: &Dog) -> Self::Output {
///         self * rhs.clone()
///     }
/// }
/// impl Mul<&Dog> for &Dog {
///     type Output = Cat;
/// 
///     fn mul(self, rhs: &Dog) -> Self::Output {
///         self.clone() * rhs.clone()
///     }
/// }
/// ```
/// 
/// # Example 2
/// 
/// The following example shows `binop` deriving owned operations from the referenced implementation.
/// ```
/// use std::ops::Add;
/// use xops_macros::binop;
/// 
/// #[derive(PartialEq, Eq, Debug)]
/// struct WrappedVec<T> {
///     inner: Vec<T>,
/// }
/// 
/// #[binop(derefs)]
/// impl<T> Add for &WrappedVec<T>
/// where
///     T: Copy + Add<Output = T>,
/// {
///     type Output = WrappedVec<T>;
///     
///     fn add(self, rhs: &WrappedVec<T>) -> Self::Output {
///         let inner = self
///             .inner
///             .iter()
///             .zip(rhs.inner.iter())
///             .map(|(&left, &right)| left + right)
///             .collect();
/// 
///         WrappedVec { inner }
///     }
/// }
/// 
/// fn main() {
///     let a = WrappedVec::<i32> {
///         inner: (0..5).collect(),
///     };
///     
///     let a2 = &a + &a; // ref + ref
///     assert_eq!(a2, WrappedVec { inner: vec![0, 2, 4, 6, 8] });
/// 
///     let a3 = a2 + &a; // owned + ref
///     assert_eq!(a3, WrappedVec { inner: vec![0, 3, 6, 9, 12] });
/// 
///     let a4 = &a + a3; // ref + owned
///     assert_eq!(a4, WrappedVec { inner: vec![0, 4, 8, 12, 16] });
///     
///     let a5 = a + a4; // owned + owned
///     assert_eq!(a5, WrappedVec { inner: vec![0, 5, 10, 15, 20] });
/// }
/// ```
#[proc_macro_attribute]
pub fn binop(args: TokenStream, item: TokenStream) -> TokenStream {
    // print_ts("binop args", &args);
    // print_ts("binop item", &item);

    let attr_args = parse_macro_input!(args as AttributeArgs);
    let binop_impl = parse_macro_input!(item as BinOpImpl);

    let binop_args = match BinOpArgs::from_list(&attr_args) {
        Ok(args) => args,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let expanded = binop_impl.expand(binop_args);

    TokenStream::from(expanded)
}

// testing -----------------------------------------------------------------------------------------



fn print_ts(header: &str, item: &TokenStream) {
    println!("BEGIN {} \n{}\nEND\n", header, item.to_string());
}

#[proc_macro_attribute]
pub fn read_binop_impl(_args: TokenStream, item: TokenStream) -> TokenStream {
    print_ts("read_binop_impl item", &item);

    let expanded = binop_read(parse_macro_input!(item as BinOpImpl));
    TokenStream::from(expanded)
}
