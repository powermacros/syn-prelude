use proc_macro2::TokenStream;
use quote::quote;
use quote::TokenStreamExt;
use syn::{Ident, LitInt};

use crate::common::TupleGen;

impl TupleGen {
    pub(crate) fn gen_tuples_for_impl_ident_names(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        for n in self.start_value..self.end_value {
            let iter_name = Ident::new(&format!("Tuple{n}Iter"), self.span);
            let tuple_t_list = (0..n).into_iter().map(|_| Ident::new("T", self.span));
            let iter_cases = (0..n).into_iter().map(|n| {
                let n = LitInt::new(&format!("{n}"), self.span);
                quote! {
                    #n => {
                        self.ptr += 1;
                        Some(&(self.tuple).#n)
                    }
                }
            });
            let str_ref_tuple_els = (0..n).into_iter().map(|_| quote!(&'a str));

            tokens.append_all(quote! {
                struct #iter_name<'a, T> {
                    tuple: &'a (#(#tuple_t_list),*),
                    ptr: usize,
                }

                impl<'a, T> Iterator for #iter_name<'a, T> {
                    type Item = &'a T;

                    fn next(&mut self) -> Option<Self::Item> {
                        match self.ptr {
                            #(#iter_cases)*
                            _ => None,
                        }
                    }
                }

                impl<'a> IdentNames for (#(#str_ref_tuple_els),*) {
                    fn iter_name(&self) -> impl Iterator<Item = impl AsRef<str>> {
                        #iter_name {
                            tuple: self,
                            ptr: 0,
                        }
                    }
                }
            })
        }
        tokens
    }
}
