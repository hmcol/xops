use algop_macros::*;
use std::ops::*;
use std::fmt;

struct Dog(i32);

impl fmt::Debug for Dog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dog({})", self.0)
    }
}

#[derive(Clone, Debug)]
struct Fish<T>
where
    T: Clone + fmt::Debug
{
    num: i32,
    data: T,
}

impl Default for Fish<String> {
    fn default() -> Self {
        Fish {
            num: 13,
            data: "glub".to_string(),
        }
    }
}

#[read_binop_impl]
impl<T> Mul<&Dog> for &Fish<T>
where
    T: Clone + fmt::Debug + std::iter::FromIterator<T>,
{
    type Output = Fish<T>;

    fn mul(self, rhs: &Dog) -> Self::Output {
        Fish {
            num: self.num * rhs.0,
            data: vec![self.data.clone(); rhs.0 as usize]
                    .iter()
                    .cloned()
                    .collect()
        }
    }
}


fn main() {
    let fish = |num: i32| Fish { num, data: "glub".to_string() };

    dbg!(&fish(7) * &Dog(3));
    // dbg!(&fish(7) *  Dog(3));
    // dbg!( fish(7) * &Dog(3));
    // dbg!( fish(7) *  Dog(3));
}


