use syn::{
    parse::{ParseBuffer, ParseStream},
    spanned::Spanned,
    token, Error, Ident, LitStr,
};
use syn_prelude_macros::peek_keyword_in_condition;

use crate::ToLitStr;

pub trait ParseAsLitStr {
    fn parse_as_lit_str(&self) -> syn::Result<LitStr>;
}

impl ParseAsLitStr for ParseStream<'_> {
    fn parse_as_lit_str(&self) -> syn::Result<LitStr> {
        if self.peek(LitStr) {
            self.parse()
        } else if self.peek(Ident) {
            let ident = self.parse::<Ident>()?;
            Ok(ident.to_lit_str())
        } else {
            peek_keyword_in_condition!(match self {
                (lit, parse) => {
                    Ok(LitStr::new(lit, parse()?.span()))
                }
                _ => {
                    Err(Error::new(self.span(), "expect literal string or ident"))
                }
            })
        }
    }
}

impl ParseAsLitStr for ParseBuffer<'_> {
    fn parse_as_lit_str(&self) -> syn::Result<LitStr> {
        if self.peek(LitStr) {
            self.parse()
        } else if self.peek(Ident) {
            let ident = self.parse::<Ident>()?;
            Ok(ident.to_lit_str())
        } else {
            peek_keyword_in_condition!(match self {
                (lit, parse) => {
                    Ok(LitStr::new(lit, parse()?.span()))
                }
                _ => {
                    Err(Error::new(self.span(), "expect literal string or ident"))
                }
            })
        }
    }
}
