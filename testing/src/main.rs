use xops_macros::*;
use std::ops::*;
use std::fmt;

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

/* #[binop_with_commute]
#[binop_with_derefs]
impl<T> std::ops::Mul<&Dog> for &Fish<T>
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
                    .collect()
        }
    }
} */

/* #[binop_with_commute]
#[binop_with_refs]
impl Mul<Fish<String>> for Dog {
    type Output = Dog;

    fn mul(self, rhs: Fish<String>) -> Dog {
        Dog(self.0 * rhs.num * (rhs.data.len() as i32))
    }
} */

#[binop(commute, refs_clone)]
impl Mul<Fish<String>> for Dog {
    type Output = Dog;

    fn mul(self, rhs: Fish<String>) -> Dog {
        Dog(self.0 * rhs.num * (rhs.data.len() as i32))
    }
}



fn main() {
    let fish = |num: i32| Fish { num, data: "glub".to_string() };

    dbg!(&fish(7) * &Dog(3));
    dbg!( fish(7) * &Dog(3));
    dbg!(&fish(7) *  Dog(3));
    dbg!( fish(7) *  Dog(3));

    println!();

    dbg!(&Dog(3) * &fish(7));
    dbg!( Dog(3) * &fish(7));
    dbg!(&Dog(3) *  fish(7));
    dbg!( Dog(3) *  fish(7));
    
}


