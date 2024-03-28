use syn::parse::{discouraged::Speculative, Parse, ParseBuffer, ParseStream};

pub trait ForkWithParseFn {
    fn fork_with_parse_fn<T, F: Fn(ParseStream) -> syn::Result<T>>(
        &self,
        parse: F,
    ) -> syn::Result<T>;
}

impl ForkWithParseFn for ParseStream<'_> {
    fn fork_with_parse_fn<T, F: Fn(ParseStream) -> syn::Result<T>>(
        &self,
        parse: F,
    ) -> syn::Result<T> {
        let fork = self.fork();
        let res = parse(&fork);
        if res.is_ok() {
            self.advance_to(&fork);
        }
        res
    }
}

impl ForkWithParseFn for ParseBuffer<'_> {
    fn fork_with_parse_fn<T, F: Fn(ParseStream) -> syn::Result<T>>(
        &self,
        parse: F,
    ) -> syn::Result<T> {
        let fork = self.fork();
        let res = parse(&fork);
        if res.is_ok() {
            self.advance_to(&fork);
        }
        res
    }
}

pub trait ForkWithParsible {
    fn fork_with_parsible<T: Parse>(&self) -> syn::Result<T>;
}

impl ForkWithParsible for ParseStream<'_> {
    fn fork_with_parsible<T: Parse>(&self) -> syn::Result<T> {
        let fork = self.fork();
        let res = T::parse(&fork);
        if res.is_ok() {
            self.advance_to(&fork);
        }
        res
    }
}

impl ForkWithParsible for ParseBuffer<'_> {
    fn fork_with_parsible<T: Parse>(&self) -> syn::Result<T> {
        let fork = self.fork();
        let res = T::parse(&fork);
        if res.is_ok() {
            self.advance_to(&fork);
        }
        res
    }
}
