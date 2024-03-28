use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::spanned::Spanned;
use syn::{
    parse::{Parse, ParseBuffer},
    Expr, Ident, LitStr, Token,
};

use crate::KEYWORDS;

pub struct PeekKeywordInCondition {
    lit_var: Ident,
    stream_var: Ident,
    parse_fn: Option<Ident>,
    if_expr: Expr,
    other_expr: Option<Expr>,
}

impl Parse for PeekKeywordInCondition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![match]>()?;
        let stream_var = if input.peek(Token![self]) {
            Ident::new("self", input.parse::<Token![self]>()?.span())
        } else {
            input.parse()?
        };

        let inner: ParseBuffer;
        syn::braced!(inner in input);

        let case_inner: ParseBuffer;
        syn::parenthesized!(case_inner in inner);
        let lit_var = case_inner.parse()?;
        let parse_fn = if case_inner.peek(Token![,]) {
            case_inner.parse::<Token![,]>()?;
            Some(case_inner.parse()?)
        } else {
            None
        };

        inner.parse::<Token![=>]>()?;
        let if_expr = inner.parse()?;
        let other_expr = if !inner.is_empty() {
            if inner.peek(Token![;]) {
                inner.parse::<Token![;]>()?;
            }
            inner.parse::<Token![_]>()?;
            inner.parse::<Token![=>]>()?;
            Some(inner.parse()?)
        } else {
            None
        };
        Ok(Self {
            stream_var,
            lit_var,
            parse_fn,
            if_expr,
            other_expr,
        })
    }
}

impl ToTokens for PeekKeywordInCondition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut conditions = KEYWORDS
            .iter()
            .map(|kw| Self::extend_if_else(&self, kw, kw, true))
            .collect::<Vec<_>>();
        conditions.push(Self::extend_if_else(&self, "Self", "SelfType", true));
        conditions.push(Self::extend_if_else(
            &self,
            "self",
            "SelfValue",
            self.other_expr.is_some(),
        ));
        let else_expanded = self.other_expr.as_ref().map(|others| quote!({#others}));
        tokens.append_all(quote! {
            #(#conditions)*
            #else_expanded
        })
    }
}

impl PeekKeywordInCondition {
    fn extend_if_else(&self, kw: &'static str, ty: &'static str, has_else: bool) -> TokenStream {
        let Self {
            stream_var,
            lit_var,
            parse_fn,
            if_expr,
            ..
        } = self;
        let lit = LitStr::new(kw, lit_var.span());
        let ty = Ident::new(&ty.to_case(Case::UpperCamel), lit_var.span());
        let else_token = if has_else { Some(quote! {else}) } else { None };
        let parse = parse_fn.as_ref().map(|parse_fn| {
            quote! {
                let #parse_fn = || {
                    #stream_var.parse::<token::#ty>()
                };
            }
        });

        quote! {
            if #stream_var.peek(token::#ty) {
                let #lit_var = #lit;
                #parse
                #if_expr
            } #else_token
        }
    }
}
