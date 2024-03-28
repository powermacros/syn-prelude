use proc_macro2::Ident;
use syn::LitStr;

pub trait WithSuffix {
    fn with_suffix<S: AsRef<str>>(&self, suffix: S) -> Self;
}

impl WithSuffix for Ident {
    fn with_suffix<S: AsRef<str>>(&self, suffix: S) -> Self {
        Self::new(
            &format!("{}{}", self.to_string(), suffix.as_ref()),
            self.span(),
        )
    }
}

impl WithSuffix for LitStr {
    fn with_suffix<S: AsRef<str>>(&self, suffix: S) -> Self {
        Self::new(&format!("{}{}", self.value(), suffix.as_ref()), self.span())
    }
}
