use proc_macro2::Span;
use quote::quote;
use quote::{ToTokens, TokenStreamExt};
use syn::{parse::Parse, Ident, LitInt, Token};

pub struct RepeatTuplesForTryParseMultiIdents {
    span: Span,
    from: usize,
    to: usize,
}

impl Parse for RepeatTuplesForTryParseMultiIdents {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let from_lit = input.parse::<LitInt>()?;
        let from: usize = from_lit.base10_parse()?;
        if from < 2 {
            Err(syn::Error::new(from_lit.span(), "should begins with 2"))?;
        }
        if input.peek(Token![..=]) {
            input.parse::<Token![..=]>()?;
            let to_lit = input.parse::<LitInt>()?;
            let to: usize = to_lit.base10_parse()?;
            if from > to {
                Err(syn::Error::new(to_lit.span(), "illegal range"))?;
            }
            Ok(Self {
                from,
                to: to + 1,
                span,
            })
        } else if input.peek(Token![..]) {
            input.parse::<Token![..]>()?;
            let to_lit = input.parse::<LitInt>()?;
            let to: usize = to_lit.base10_parse()?;
            if from > to {
                Err(syn::Error::new(to_lit.span(), "illegal range"))?;
            }
            Ok(Self { from, to, span })
        } else {
            Ok(Self {
                from: 2,
                to: from + 1,
                span,
            })
        }
    }
}

impl ToTokens for RepeatTuplesForTryParseMultiIdents {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for n in self.from..self.to {
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
    }
}
