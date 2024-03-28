use syn::{
    parse::{Parse, ParseStream, Peek},
    punctuated::Punctuated,
};

pub trait ParseTerminatedUntil {
    fn parse_terminated_until<T, P>(
        &self,
        parser: fn(ParseStream) -> syn::Result<T>,
        separator: P,
    ) -> syn::Result<Punctuated<T, P::Token>>
    where
        P: Peek,
        P::Token: Parse;
}

impl ParseTerminatedUntil for ParseStream<'_> {
    fn parse_terminated_until<T, P>(
        &self,
        parser: fn(ParseStream) -> syn::Result<T>,
        separator: P,
    ) -> syn::Result<Punctuated<T, P::Token>>
    where
        P: Peek,
        P::Token: Parse,
    {
        let _ = separator;
        let mut punctuated = Punctuated::new();

        loop {
            if self.is_empty() {
                break;
            }
            punctuated.push_value(parser(self)?);
            if self.is_empty() {
                break;
            }
            if self.peek(separator) {
                punctuated.push_punct(self.parse()?);
            } else {
                break;
            }
        }

        Ok(punctuated)
    }
}
