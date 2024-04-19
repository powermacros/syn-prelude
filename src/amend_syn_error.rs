pub trait AmmendSynError {
    fn ammend_error(self, ammend: impl Fn(syn::Error) -> syn::Error) -> Self;
    fn add_error_prefix(self, prefix: impl AsRef<str>) -> Self
    where
        Self: Sized,
    {
        self.ammend_error(|err| {
            syn::Error::new(
                err.span(),
                format!("{} {}", prefix.as_ref(), err.to_string()),
            )
        })
    }
    fn add_error_suffix(self, suffix: impl AsRef<str>) -> Self
    where
        Self: Sized,
    {
        self.ammend_error(|err| {
            syn::Error::new(
                err.span(),
                format!("{} {}", err.to_string(), suffix.as_ref()),
            )
        })
    }
}

impl<T> AmmendSynError for syn::Result<T> {
    fn ammend_error(self, ammend: impl Fn(syn::Error) -> syn::Error) -> Self {
        self.map_err(|err| ammend(err))
    }
}
