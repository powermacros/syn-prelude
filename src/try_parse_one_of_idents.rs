use syn::{
    parse::{discouraged::Speculative, ParseBuffer, ParseStream},
    token, Ident,
};
use syn_prelude_macros::match_keyword_cases;

pub trait TryParseOneOfIdents {
    fn try_parse_one_of_idents<S: AsRef<str>>(&self, names: Vec<S>) -> Option<Ident>;
}

impl TryParseOneOfIdents for ParseStream<'_> {
    fn try_parse_one_of_idents<S: AsRef<str>>(&self, names: Vec<S>) -> Option<Ident> {
        for name in names {
            match_keyword_cases!(match name.as_ref() {
                (lit, tk, parse: Self) => {
                    return if self.peek(tk) {
                        Some(Ident::new(lit, parse(self).unwrap().span))
                    } else {
                        None
                    };
                }
                _ => {
                    let forked = self.fork();
                    if let Ok(ident) = forked.parse::<Ident>() {
                        if ident.eq(name.as_ref()) {
                            self.advance_to(&forked);
                            return Some(ident);
                        }
                    }
                }
            })
        }
        None
    }
}

impl TryParseOneOfIdents for ParseBuffer<'_> {
    fn try_parse_one_of_idents<S: AsRef<str>>(&self, names: Vec<S>) -> Option<Ident> {
        for name in names {
            match_keyword_cases!(match name.as_ref() {
                (lit, tk, parse: &ParseBuffer) => {
                    if self.peek(tk) {
                        return Some(Ident::new(lit, parse(self).unwrap().span));
                    }
                }
                _ => {
                    let forked = self.fork();
                    if let Ok(ident) = forked.parse::<Ident>() {
                        if ident.eq(name.as_ref()) {
                            self.advance_to(&forked);
                            return Some(ident);
                        }
                    }
                }
            })
        }
        None
    }
}
