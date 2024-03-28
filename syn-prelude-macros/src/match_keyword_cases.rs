use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseBuffer},
    spanned::Spanned,
    Expr, Ident, LitStr, Token, Type,
};

use crate::KEYWORDS;

pub(crate) struct MatchKeywordCases {
    expr: Expr,
    lit_var: Ident,
    token_var: Ident,
    parse_fn: Option<Ident>,
    parse_type: Option<Type>,
    token_case: Expr,
    other_case: Expr,
}

impl Parse for MatchKeywordCases {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Token![match]>()?;
        let expr = input.parse()?;
        let inner: ParseBuffer;
        syn::braced!(inner in input);

        let case_inner: ParseBuffer;
        syn::parenthesized!(case_inner in inner);
        let lit_var = case_inner.parse()?;
        case_inner.parse::<Token![,]>()?;
        let token_var = case_inner.parse()?;
        let (parse_fn, parse_type) = if case_inner.peek(Token![,]) {
            case_inner.parse::<Token![,]>()?;
            let parse_fn = case_inner.parse()?;

            case_inner.parse::<Token![:]>()?;
            let parse_type = case_inner.parse()?;
            (Some(parse_fn), Some(parse_type))
        } else {
            (None, None)
        };

        inner.parse::<Token![=>]>()?;
        let token_case = inner.parse()?;

        if inner.peek(Token![,]) {
            inner.parse::<Token![,]>()?;
        }
        inner.parse::<Token![_]>()?;
        inner.parse::<Token![=>]>()?;
        let other_case = inner.parse()?;
        Ok(Self {
            expr,
            lit_var,
            token_var,
            parse_fn,
            parse_type,
            token_case,
            other_case,
        })
    }
}

impl ToTokens for MatchKeywordCases {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            expr, other_case, ..
        } = self;
        let mut cases = KEYWORDS
            .iter()
            .map(|kw| self.expand_case(kw, kw, expr.span()))
            .collect::<Vec<_>>();
        cases.push(self.expand_case("Self", "SelfType", expr.span()));
        cases.push(self.expand_case("self", "SelfValue", expr.span()));
        cases.push(quote!(_ => #other_case));
        tokens.append_all(quote! {
            match #expr {
                #(#cases,)*
            }
        })
    }
}

impl MatchKeywordCases {
    fn expand_case(&self, kw: &'static str, ty: &'static str, span: Span) -> TokenStream {
        let Self {
            lit_var,
            token_case,
            token_var,
            parse_fn,
            parse_type,
            ..
        } = self;
        let lit = LitStr::new(kw, span);
        let ident = Ident::new(&ty.to_case(Case::UpperCamel), span);
        let parse = parse_fn.as_ref().map(|parse_fn| {
            quote!(let #parse_fn = |stream: #parse_type| {stream.parse::<token::#ident>()};)
        });

        quote!(#lit => {
            let #lit_var = #lit;
            let #token_var = token::#ident;
            #parse

            #token_case
        })
    }
}
