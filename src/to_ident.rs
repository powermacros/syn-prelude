use convert_case::{Case, Casing};
use proc_macro2::Span;
use syn::{token, Ident, LitStr};
use syn_prelude_macros::repeat_keyword_lit_and_types;

pub trait ToIdent {
    fn to_ident(&self) -> Ident;
}

impl ToIdent for LitStr {
    fn to_ident(&self) -> Ident {
        Ident::new(&self.value(), self.span())
    }
}

macro_rules! impl_to_ident_for_tokens {
    ($token:literal, $name:ident) => {
        impl ToIdent for token::$name {
            fn to_ident(&self) -> Ident {
                Ident::new($token, self.span)
            }
        }
    };
}

repeat_keyword_lit_and_types!(impl_to_ident_for_tokens);

impl ToIdent for (&str, Span) {
    fn to_ident(&self) -> Ident {
        Ident::new(self.0, self.1)
    }
}

impl ToIdent for &str {
    fn to_ident(&self) -> Ident {
        Ident::new(self, Span::call_site())
    }
}

impl ToIdent for (String, Span) {
    fn to_ident(&self) -> Ident {
        Ident::new(self.0.as_str(), self.1)
    }
}

impl ToIdent for (&String, Span) {
    fn to_ident(&self) -> Ident {
        Ident::new(self.0, self.1)
    }
}

pub trait ToIdentWithCase {
    fn to_ident_with_case(&self, case: Case) -> Ident;
}

impl ToIdentWithCase for Ident {
    fn to_ident_with_case(&self, case: Case) -> Ident {
        Ident::new(&self.to_string().to_case(case), self.span())
    }
}

impl ToIdentWithCase for LitStr {
    fn to_ident_with_case(&self, case: Case) -> Ident {
        Ident::new(&self.value().to_case(case), self.span())
    }
}

macro_rules! impl_to_ident_with_case_for_tokens {
    ($token:literal, $name:ident) => {
        impl ToIdentWithCase for token::$name {
            fn to_ident_with_case(&self, case: Case) -> Ident {
                Ident::new(&$token.to_case(case), self.span)
            }
        }
    };
}

repeat_keyword_lit_and_types!(impl_to_ident_with_case_for_tokens);

impl ToIdentWithCase for (&str, Span) {
    fn to_ident_with_case(&self, case: Case) -> Ident {
        Ident::new(&self.0.to_case(case), self.1)
    }
}

impl ToIdentWithCase for (String, Span) {
    fn to_ident_with_case(&self, case: Case) -> Ident {
        Ident::new(&self.0.to_case(case), self.1)
    }
}

impl ToIdentWithCase for (&String, Span) {
    fn to_ident_with_case(&self, case: Case) -> Ident {
        Ident::new(&self.0.to_case(case), self.1)
    }
}

pub trait ToIdentWithPrefix {
    fn to_ident_with_prefix<S: AsRef<str>>(&self, prefix: S) -> Ident;
}

impl ToIdentWithPrefix for Ident {
    fn to_ident_with_prefix<S: AsRef<str>>(&self, prefix: S) -> Ident {
        Ident::new(
            &format!("{}{}", prefix.as_ref(), self.to_string()),
            self.span(),
        )
    }
}

impl ToIdentWithPrefix for LitStr {
    fn to_ident_with_prefix<S: AsRef<str>>(&self, prefix: S) -> Ident {
        Ident::new(&format!("{}{}", prefix.as_ref(), self.value()), self.span())
    }
}

impl ToIdentWithPrefix for (&str, Span) {
    fn to_ident_with_prefix<S: AsRef<str>>(&self, prefix: S) -> Ident {
        Ident::new(&format!("{}{}", prefix.as_ref(), self.0), self.1)
    }
}

impl ToIdentWithPrefix for (String, Span) {
    fn to_ident_with_prefix<S: AsRef<str>>(&self, prefix: S) -> Ident {
        Ident::new(&format!("{}{}", prefix.as_ref(), &self.0), self.1)
    }
}

impl ToIdentWithPrefix for (&String, Span) {
    fn to_ident_with_prefix<S: AsRef<str>>(&self, prefix: S) -> Ident {
        Ident::new(&format!("{}{}", prefix.as_ref(), self.0), self.1)
    }
}

pub trait ToIdentWithSuffix {
    fn to_ident_with_suffix<S: AsRef<str>>(&self, suffix: S) -> Ident;
}

impl ToIdentWithSuffix for Ident {
    fn to_ident_with_suffix<S: AsRef<str>>(&self, suffix: S) -> Ident {
        Ident::new(
            &format!("{}{}", self.to_string(), suffix.as_ref()),
            self.span(),
        )
    }
}

impl ToIdentWithSuffix for LitStr {
    fn to_ident_with_suffix<S: AsRef<str>>(&self, suffix: S) -> Ident {
        Ident::new(&format!("{}{}", self.value(), suffix.as_ref()), self.span())
    }
}

impl ToIdentWithSuffix for (&str, Span) {
    fn to_ident_with_suffix<S: AsRef<str>>(&self, suffix: S) -> Ident {
        Ident::new(&format!("{}{}", self.0, suffix.as_ref()), self.1)
    }
}

impl ToIdentWithSuffix for (String, Span) {
    fn to_ident_with_suffix<S: AsRef<str>>(&self, suffix: S) -> Ident {
        Ident::new(&format!("{}{}", &self.0, suffix.as_ref()), self.1)
    }
}

impl ToIdentWithSuffix for (&String, Span) {
    fn to_ident_with_suffix<S: AsRef<str>>(&self, suffix: S) -> Ident {
        Ident::new(&format!("{}{}", self.0, suffix.as_ref()), self.1)
    }
}
