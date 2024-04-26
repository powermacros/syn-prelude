use proc_macro2::Span;
use quote::quote;
use quote::{ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Expr, ExprLit, ExprRange, Lit, LitInt, RangeLimits,
};

pub struct SpanTuplesToSpan {
    span: Span,
    start_value: usize,
    end_value: usize,
}

impl Parse for SpanTuplesToSpan {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let range = input.parse::<ExprRange>()?;
        let start = if let Some(start) = &range.start {
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(n), ..
            }) = start.as_ref()
            {
                n.clone()
            } else {
                Err(syn::Error::new(start.span(), "unexpected expr"))?
            }
        } else {
            syn::LitInt::new("2", range.span())
        };
        let end = if let Some(end) = &range.end {
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(n), ..
            }) = end.as_ref()
            {
                n.clone()
            } else {
                Err(syn::Error::new(start.span(), "unexpected expr"))?
            }
        } else {
            Err(syn::Error::new(range.span(), "missing end"))?
        };

        let start_value: usize = start.base10_parse()?;
        if start_value <= 1 {
            Err(syn::Error::new(start.span(), "at least 2"))?;
        }
        let mut end_value: usize = end.base10_parse()?;
        if let RangeLimits::Closed(_) = range.limits {
            end_value += 1;
        }

        Ok(Self {
            span: range.span(),
            start_value,
            end_value,
        })
    }
}

impl ToTokens for SpanTuplesToSpan {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
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
    }
}
