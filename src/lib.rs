//! Procedural macros to help with overloading operators.
//! 
//! # About
//! 
//! xops = e**X**(tra/tended/cellent) + **OP**erat(or/ion) + **S**
//! 
//! This crate provides macros to alleviate repetition when implementing related 'families' of operations. 
//! 
//! # Usage/Examples
//! 
//! See [`binop`]
//! 
//! # Operator Overloading Basics
//! 
//! All the traits for overloading operators in [`std::ops`] follow a common pattern. Take, for example, the `Add` trait for overloading the `+` operator; its trait definition looks like this:
//! ```
//! trait Add<Rhs = Self> {
//!     type Output;
//!     fn add(self, rhs: Rhs) -> Self::Output;
//! }
//! ```
//! The receiving type `Self` is the left-hand side of the `+` operator and the generic type argument `Rhs` is right-hand side. Implementing `Add<B>` for `A` will then make the expression `a + b` equivalent to `<A as Add<B>>::add(a, b)` , for any `a: A` and `b: B`.
//! 
//! For examples of implementations of these traits, see [`std::ops`].
//! 
//! # Discussion
//! 
//! The functionality of xops is very much like other 'derive' crates, automatically deriving trait implementations. However, instead of the attributes being placed on a struct, enum, or union (as with the `derive` attribute), the attributes in xops are placed on *trait implementations*, i.e., items of the form `impl Trait for Type { ... }`. 
//! 
//! The reasoning behind putting the attributes on trait implementations has both pragmatic and semantic components:
//! 
//! - From a trait implementation, xops is able to directly parse all of the information it needs to do its job, namely type, trait, and method identifiers. If xops used derive macros, all this information would either need to be given by the user or xops would need some sort of catalogue about all the standard library operations. With the current approach, however, you are not even limited to just the standard library operations, xops will work on any traits with the same sort of layout. (Although, Rust does not support custom operation overloading, so there probably isn't much use outside the standard library operations).
//! 
//! - In Rust, when we implement `Add<B>` for `A`, we are essentially saying *`A` is capable of addition with `B`*, and this is subtlety distinct from *`B` is capable of addition with `A`*, which we would achieve by implementing `Add<A>` for `B`. In mathematics, on the other hand, it would be more common to say something like *addition is defined between `A` and `B`*; the difference being that the operation itself is treated as more of a first-class citizen. The approach of xops is in agreement with the latter interpretation, to the extent that Rust permits.
//! 
//! 



pub use xops_macros::binop;

#[cfg(test)]
mod tests {
    use std::fmt;
    use std::ops::Mul;
    use xops_macros::*;

    #[derive(Clone)]
    struct Dog(i32);

    impl fmt::Debug for Dog {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Dog({})", self.0)
        }
    }

    #[derive(Clone)]
    struct Fish<T: Clone> {
        num: i32,
        data: T,
    }

    impl<T: Clone + fmt::Debug> fmt::Debug for Fish<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Fish({}, {:?})", self.num, self.data)
        }
    }

    impl Default for Fish<String> {
        fn default() -> Self {
            Fish {
                num: 13,
                data: "glub".to_string(),
            }
        }
    }

    #[binop(commute, derefs)]
    impl<T> Mul<&Dog> for &Fish<T>
    where
        T: Clone + fmt::Debug + std::iter::FromIterator<T>,
    {
        type Output = Fish<T>;

        fn mul(self, rhs: &Dog) -> Fish<T> {
            Fish {
                num: self.num * rhs.0,
                data: vec![self.data.clone(); rhs.0 as usize]
                    .iter()
                    .cloned()
                    .collect(),
            }
        }
    }
    
    /* // #[read_binop_impl]
    #[binop(commute, refs_clone)]
    impl Mul<Fish<String>> for Dog {
        type Output = Dog;

        fn mul(self, rhs: Fish<String>) -> Dog {
            Dog(self.0 * rhs.num * (rhs.data.len() as i32))
        }
    } */

    #[test]
    fn derived_ops_test() {
        let fish = |num: i32| Fish {
            num,
            data: "glub".to_string(),
        };


        dbg!(&fish(7) * &Dog(3));
        dbg!( fish(7) * &Dog(3));
        // dbg!(&fish(7) *  Dog(3));
        dbg!( fish(7) *  Dog(3));

        println!();

        dbg!(&Dog(3) * &fish(7));
        dbg!( Dog(3) * &fish(7));
        dbg!(&Dog(3) *  fish(7));
        dbg!( Dog(3) *  fish(7));

        let x = std::any::type_name::<Fish<String>>();
        dbg!(x);
    }
}
