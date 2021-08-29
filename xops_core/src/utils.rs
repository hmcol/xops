use quote::{quote};
use quote::ToTokens;
use syn::{Type, parse_quote};

/// quotes and prints `item` under the label `header`
///
/// primarily used for checking implementations of Parse and/or ToTokens
pub fn print_tokens<T: ToTokens>(header: &str, item: T) {
    println!("BEGIN {} \n{}\nEND\n", header, quote!(#item));
}

pub trait TypeConversion: Sized {
    fn as_ref(&self) -> Self;
    fn as_verbatim(&self) -> Self;
    fn as_deref(&self) -> Option<Self>;
    fn try_deref(&self) -> Result<Self, String>;
}

impl TypeConversion for Type {
    fn as_ref(&self) -> Self {
        Type::Reference(parse_quote!(&#self))
    }

    fn as_deref(&self) -> Option<Self> {
        if let Type::Reference(ref_ty) = self {
            let ty = &ref_ty.elem;
            Some(parse_quote!(#ty))
        } else {
            None
        }
    }

    fn try_deref(&self) -> Result<Self, String> {
        if let Type::Reference(ref_ty) = self {
            let ty = &ref_ty.elem;
            Ok(parse_quote!(#ty))
        } else {
            Err(format!("could not dereference type `{}`", quote!(self)))
        }
    }

    fn as_verbatim(&self) -> Self {
        Type::Verbatim(quote!(#self))
    }
}


