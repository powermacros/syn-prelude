use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse::Parse, Ident, LitStr};

use crate::KEYWORDS;

pub(crate) struct RepleatKeywordLitAndTypes(Ident);

impl Parse for RepleatKeywordLitAndTypes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl ToTokens for RepleatKeywordLitAndTypes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let macro_ident = &self.0;
        let macros = KEYWORDS
            .iter()
            .map(|kw| {
                let lit = LitStr::new(kw, macro_ident.span());
                let ident = Ident::new(&kw.to_case(Case::UpperCamel), macro_ident.span());
                quote!(#macro_ident!(#lit, #ident))
            })
            .collect::<Vec<_>>();
        tokens.append_all(quote! {
            #(#macros;)*
            #macro_ident!("Self", SelfType);
            #macro_ident!("self", SelfValue);
        });
    }
}
