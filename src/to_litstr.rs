use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span};
use syn::{token, LitStr};
use syn_prelude_macros::repeat_keyword_lit_and_types;

pub trait ToLitStr {
    fn to_lit_str(&self) -> LitStr;
}

impl ToLitStr for Ident {
    fn to_lit_str(&self) -> LitStr {
        LitStr::new(&self.to_string(), self.span())
    }
}

macro_rules! impl_to_lit_str_for_tokens {
    ($token:literal, $name:ident) => {
        impl ToLitStr for token::$name {
            fn to_lit_str(&self) -> LitStr {
                LitStr::new($token, self.span)
            }
        }
    };
}

repeat_keyword_lit_and_types!(impl_to_lit_str_for_tokens);

impl ToLitStr for (&str, Span) {
    fn to_lit_str(&self) -> LitStr {
        LitStr::new(self.0, self.1)
    }
}

impl ToLitStr for (String, Span) {
    fn to_lit_str(&self) -> LitStr {
        LitStr::new(&self.0, self.1)
    }
}

impl ToLitStr for (&String, Span) {
    fn to_lit_str(&self) -> LitStr {
        LitStr::new(self.0, self.1)
    }
}

pub trait ToLitStrWithCase {
    fn to_lit_str_with_case(&self, case: Case) -> LitStr;
}

impl ToLitStrWithCase for Ident {
    fn to_lit_str_with_case(&self, case: Case) -> LitStr {
        LitStr::new(&self.to_string().to_case(case), self.span())
    }
}

macro_rules! impl_to_lit_str_with_case_for_tokens {
    ($token:literal, $name:ident) => {
        impl ToLitStrWithCase for token::$name {
            fn to_lit_str_with_case(&self, case: Case) -> LitStr {
                LitStr::new(&$token.to_case(case), self.span)
            }
        }
    };
}

repeat_keyword_lit_and_types!(impl_to_lit_str_with_case_for_tokens);

impl ToLitStrWithCase for (&str, Span) {
    fn to_lit_str_with_case(&self, case: Case) -> LitStr {
        LitStr::new(&self.0.to_case(case), self.1)
    }
}

impl ToLitStrWithCase for (String, Span) {
    fn to_lit_str_with_case(&self, case: Case) -> LitStr {
        LitStr::new(&self.0.to_case(case), self.1)
    }
}

impl ToLitStrWithCase for (&String, Span) {
    fn to_lit_str_with_case(&self, case: Case) -> LitStr {
        LitStr::new(&self.0.to_case(case), self.1)
    }
}
