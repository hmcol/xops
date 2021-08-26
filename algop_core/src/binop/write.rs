use proc_macro2::TokenStream;

use super::{TraitImpl, TraitImplBuilder};

impl TraitImpl {
    pub fn try_deref(&self) -> TokenStream {
        let _own_ref_bldr: TraitImplBuilder = self.clone().into();

        // return
        TokenStream::default()
    }
}
