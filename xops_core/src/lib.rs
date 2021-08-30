/// Handling Binary Operations
mod binop;
pub use crate::binop::{
    read_impl as binop_read, BinOpArgs, BinOpFn, BinOpImpl, BinOpOutput,
};

mod utils;
