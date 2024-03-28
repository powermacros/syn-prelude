use syn::{
    parse::{discouraged::Speculative, ParseBuffer, ParseStream},
    token, Ident,
};
use syn_prelude_macros::match_keyword_cases;

pub trait TryParseAsIdent {
    fn try_parse_as_ident<S: AsRef<str>>(&self, name: S, ignore_case: bool) -> Option<Ident>;
    fn peek_as_ident<S: AsRef<str>>(&self, name: S, ignore_case: bool) -> bool;
}

impl TryParseAsIdent for ParseStream<'_> {
    fn try_parse_as_ident<S: AsRef<str>>(&self, name: S, ignore_case: bool) -> Option<Ident> {
        if ignore_case {
            let name = name.as_ref().to_lowercase();
            match_keyword_cases!(match name.as_str() {
                (lit, tk, parse: Self) => {
                    if self.peek(tk) {
                        let tk = parse(self).unwrap();
                        Some(Ident::new(lit, tk.span))
                    } else {
                        None
                    }
                }
                _ => {
                    let forked = self.fork();
                    if let Ok(ident) = forked.parse::<Ident>() {
                        if ident.to_string().to_lowercase().eq(&name) {
                            self.advance_to(&forked);
                            Some(ident)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            })
        } else {
            match_keyword_cases!(match name.as_ref() {
                (lit, tk, parse: Self) => {
                    if self.peek(tk) {
                        let tk = parse(self).unwrap();
                        Some(Ident::new(lit, tk.span))
                    } else {
                        None
                    }
                }
                _ => {
                    let forked = self.fork();
                    if let Ok(ident) = forked.parse::<Ident>() {
                        if ident.to_string().to_lowercase().eq(name.as_ref()) {
                            self.advance_to(&forked);
                            Some(ident)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            })
        }
    }

    fn peek_as_ident<S: AsRef<str>>(&self, name: S, ignore_case: bool) -> bool {
        if ignore_case {
            let name = name.as_ref().to_lowercase();
            match_keyword_cases!(match name.as_str() {
                (_lit, tk) => {
                    self.peek(tk)
                }
                _ => {
                    let forked = self.fork();
                    if let Ok(ident) = forked.parse::<Ident>() {
                        if ident.to_string().to_lowercase().eq(&name) {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            })
        } else {
            match_keyword_cases!(match name.as_ref() {
                (_lit, tk) => {
                    self.peek(tk)
                }
                _ => {
                    let forked = self.fork();
                    if let Ok(ident) = forked.parse::<Ident>() {
                        if ident.to_string().to_lowercase().eq(name.as_ref()) {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            })
        }
    }
}

impl TryParseAsIdent for ParseBuffer<'_> {
    fn try_parse_as_ident<S: AsRef<str>>(&self, name: S, ignore_case: bool) -> Option<Ident> {
        if ignore_case {
            let name = name.as_ref().to_lowercase();
            match_keyword_cases!(match name.as_str() {
                (lit, tk, parse: &Self) => {
                    if self.peek(tk) {
                        let tk = parse(self).unwrap();
                        Some(Ident::new(lit, tk.span))
                    } else {
                        None
                    }
                }
                _ => {
                    let forked = self.fork();
                    if let Ok(ident) = forked.parse::<Ident>() {
                        if ident.to_string().to_lowercase().eq(&name) {
                            self.advance_to(&forked);
                            Some(ident)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            })
        } else {
            match_keyword_cases!(match name.as_ref() {
                (lit, tk, parse: &Self) => {
                    if self.peek(tk) {
                        let tk = parse(self).unwrap();
                        Some(Ident::new(lit, tk.span))
                    } else {
                        None
                    }
                }
                _ => {
                    let forked = self.fork();
                    if let Ok(ident) = forked.parse::<Ident>() {
                        if ident.to_string().to_lowercase().eq(name.as_ref()) {
                            self.advance_to(&forked);
                            Some(ident)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            })
        }
    }

    fn peek_as_ident<S: AsRef<str>>(&self, name: S, ignore_case: bool) -> bool {
        if ignore_case {
            let name = name.as_ref().to_lowercase();
            match_keyword_cases!(match name.as_str() {
                (_lit, tk) => {
                    self.peek(tk)
                }
                _ => {
                    let forked = self.fork();
                    if let Ok(ident) = forked.parse::<Ident>() {
                        if ident.to_string().to_lowercase().eq(&name) {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            })
        } else {
            match_keyword_cases!(match name.as_ref() {
                (_lit, tk) => {
                    self.peek(tk)
                }
                _ => {
                    let forked = self.fork();
                    if let Ok(ident) = forked.parse::<Ident>() {
                        if ident.to_string().to_lowercase().eq(name.as_ref()) {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            })
        }
    }
}
