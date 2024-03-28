use proc_macro2::Ident;
use syn::LitStr;

pub trait WithPrefix {
    fn with_prefix<S: AsRef<str>>(&self, prefix: S) -> Self;
}

impl WithPrefix for Ident {
    fn with_prefix<S: AsRef<str>>(&self, prefix: S) -> Self {
        Self::new(
            &format!("{}{}", prefix.as_ref(), self.to_string()),
            self.span(),
        )
    }
}

impl WithPrefix for LitStr {
    fn with_prefix<S: AsRef<str>>(&self, prefix: S) -> Self {
        Self::new(&format!("{}{}", prefix.as_ref(), self.value()), self.span())
    }
}
