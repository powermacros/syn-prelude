use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::LitInt;

use crate::common::TupleGen;

impl TupleGen {
    pub(crate) fn gen_tuples_for_impl_to_span(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        for n in self.start_value..self.end_value {
            let t_types = (0..n).into_iter().map(|n| {
                let ident = syn::Ident::new(&format!("T{n}"), self.span);
                quote!(#ident: ToSpan)
            });
            let t_tuples = (0..n).into_iter().map(|n| {
                let ident = syn::Ident::new(&format!("T{n}"), self.span);
                quote!(#ident)
            });
            let joins = (1..n).into_iter().map(|n| {
                let n = LitInt::new(&format!("{n}"), self.span);
                quote! {
                    if let Some(new) = span.join(self.#n.to_span()) {
                        span = new;
                    }
                }
            });

            tokens.append_all(quote! {
                impl<#(#t_types),*> ToSpan for (#(#t_tuples),*) {
                    fn to_span(&self) -> Span {
                        let mut span = self.0.to_span();
                        #(#joins)*
                        span
                    }
                }
            })
        }
        tokens
    }
}
