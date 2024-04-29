use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

use crate::common::TupleGen;

impl TupleGen {
    pub(crate) fn gen_tuples_for_impl_into_idents(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        for index in self.iter() {
            tokens.append_all(self.ident_tuple(index));
            tokens.append_all(self.str_tuple(index));
        }
        tokens
    }

    fn ident_tuple(&self, index: usize) -> TokenStream {
        let name = syn::Ident::new(&format!("IdentTuple{index}IntoIdentIter"), self.span);
        let lifetimes = (0..index)
            .into_iter()
            .map(|i| syn::Lifetime::new(&format!("'l{i}"), self.span))
            .collect::<Vec<_>>();
        let types = lifetimes
            .iter()
            .map(|i| quote!(&#i Ident))
            .collect::<Vec<_>>();
        let iter_cases = (0..index).into_iter().map(|i| {
            let index = syn::LitInt::new(&format!("{i}"), self.span);
            quote! {
                #index => {
                    self.cursor += 1;
                    Some(self.tuple.#index.clone())
                },
            }
        });

        quote! {
            struct #name<#(#lifetimes),*> {
                tuple: (#(#types),*),
                cursor: usize,
            }

            impl<#(#lifetimes),*> Iterator for #name<#(#lifetimes),*> {
                type Item = Ident;

                fn next(&mut self) -> Option<Self::Item> {
                    match self.cursor {
                        #(#iter_cases)*
                        _ => None,
                    }
                }
            }

            impl<#(#lifetimes),*> IntoIdents for (#(#types),*) {
                fn into_idents(self) -> impl Iterator<Item = Ident> {
                    #name {
                        tuple: self,
                        cursor: 0,
                    }
                }
            }
        }
    }
    fn str_tuple(&self, index: usize) -> TokenStream {
        let name = syn::Ident::new(&format!("StrTuple{index}IntoIdentIter"), self.span);
        let types = (0..index)
            .into_iter()
            .map(|i| syn::Ident::new(&format!("S{}", i + 1), self.span))
            .collect::<Vec<_>>();
        let type_decls = (0..index)
            .into_iter()
            .map(|i| {
                let t = syn::Ident::new(&format!("S{}", i + 1), self.span);
                quote!(#t: AsRef<str>)
            })
            .collect::<Vec<_>>();
        let indexes = (0..index)
            .into_iter()
            .map(|i| syn::LitInt::new(&format!("{i}"), self.span))
            .collect::<Vec<_>>();
        let iter_cases = indexes.iter().map(|index| {
            quote! {
                #index => {
                    self.cursor += 1;
                    Some(syn::Ident::new(self.tuple.#index.as_ref(), self.span))
                },
            }
        });

        let init_tuples = indexes.iter().map(|i| quote!(self.#i));
        let init_span = syn::LitInt::new(&format!("{index}"), self.span);

        quote! {
            struct #name<#(#types),*> {
                tuple: (#(#types),*),
                span: proc_macro2::Span,
                cursor: usize,
            }

            impl<#(#type_decls),*> Iterator for #name<#(#types),*> {
                type Item = Ident;

                fn next(&mut self) -> Option<Self::Item> {
                    match self.cursor {
                        #(#iter_cases)*
                        _ => None,
                    }
                }
            }

            impl<#(#type_decls),*> IntoIdents for (#(#types,)* proc_macro2::Span) {
                fn into_idents(self) -> impl Iterator<Item = Ident> {
                    #name {
                        tuple: (#(#init_tuples),*),
                        span: self.#init_span,
                        cursor: 0,
                    }
                }
            }
        }
    }
}
