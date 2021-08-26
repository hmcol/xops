/* use derive_builder;
use proc_macro2::TokenStream;

mod internal;
use internal::Args;
use syn::{Ident, Result, Type, parse::Parse, parse_macro_input, punctuated::Punctuated};
use quote::quote;


macro_rules! builder_extra {
    ($Base:ty : $Builder:ty : $( $f:ident ),* $(,)?) => {
        builder_extra! { @From $Base, $Builder, $($f),* }
        builder_extra! { @ParsedBuild $Base, $Builder }
    };
    (@From $Base:ty, $Builder:ty, $( $f:ident ),* $(,)?) => {
        impl From<$Base> for $Builder {
            fn from(base: $Base) -> Self {
                let mut bldr = <$Builder>::default();
                bldr $(.$f(base.$f))* ; //.to_owned()

                // return
                bldr
            }
        }
    };
    (@ParsedBuild $Base:ty, $Builder:ty) => {
        impl ParsedBuild for $Builder {
            type BaseStruct = $Base;

            fn parsed_build(&self) -> Result<Self::BaseStruct> {
                Ok(self.build().expect(
                    stringify!(failed to build $Base after parsing)
                ))
            }
        }
    };
}


pub fn impl_from(base: Type, builder: Type, fields: Vec<Ident>) -> TokenStream {
    quote! {
        impl From<#base> for #builder {
            fn from(base: #base) -> Self {
                let mut bldr = <#builder>::default();
                bldr #(.#fields(base.#fields))* ; //.to_owned()

                // return
                bldr
            }
        }
    }
}

pub fn impl_parsed_build(base: Type, builder: Type, fields: Vec<Ident>) -> TokenStream {
    quote! {
        impl ParsedBuild for #builder {
            type BaseStruct = #base;

            fn parsed_build(&self) -> Result<Self::BaseStruct> {
                Ok(self.build().expect(
                    stringify!(failed to build #base after parsing)
                ))
            }
        }
    }
}



/* 
pub fn builder_extra(item: TokenStream) -> TokenStream {
    //println!("BEGIN item \n{}\nEND\n", &item.to_string());

    let args = parse_macro_input!(item as Args);

    //dbg!(&args);

    let base = args.base;
    let builder = args.builder;
    let fields = args.fields.iter();

    let expanded = quote! {
        impl From<#base> for #builder {
            fn from(base: #base) -> Self {
                let mut bldr = <#builder>::default();
                bldr #(.#fields(base.#fields))* ; //.to_owned()

                // return
                bldr
            }
        }

        impl ParsedBuild for #builder {
            type BaseStruct = #base;

            fn parsed_build(&self) -> Result<Self::BaseStruct> {
                Ok(self.build().expect(
                    stringify!(failed to build #base after parsing)
                ))
            }
        }
    };

    // return
    TokenStream::from(expanded)
} */ */