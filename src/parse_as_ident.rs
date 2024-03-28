use convert_case::Case;
use syn::{
    parse::{discouraged::Speculative, ParseBuffer, ParseStream},
    token, Error, Ident, LitStr, Result,
};
use syn_prelude_macros::{match_keyword_cases, peek_keyword_in_condition};

use crate::{ToErr, ToIdentWithCase, ToSynError};

pub trait ParseAsIdent {
    fn parse_as_ident(&self) -> Result<Ident>;
    fn parse_as_named_ident<S: AsRef<str>>(&self, name: S, ignore_case: bool) -> Result<Ident>;
}

impl ParseAsIdent for ParseStream<'_> {
    fn parse_as_named_ident<S: AsRef<str>>(&self, name: S, ignore_case: bool) -> Result<Ident> {
        if ignore_case {
            let name = name.as_ref().to_lowercase();
            match_keyword_cases!(match name.as_str() {
                (lit, tk, parse: Self) => {
                    if self.peek(tk) {
                        Ok(Ident::new(name.as_ref(), parse(self)?.span))
                    } else {
                        Err(Error::new(
                            self.span(),
                            format!("expect token `{lit}`"),
                        ))
                    }
                }
                _ => {
                let forked = self.fork();
                    if let Ok(ident) = forked.parse::<Ident>() {
                        if ident.to_string().to_lowercase().eq(&name) {
                            self.advance_to(&forked);
                            Ok(ident)
                        } else {
                            Err(Error::new(
                                ident.span(),
                                format!("expect ident `{}` but `{}`", name, ident.to_string()),
                            ))
                        }
                    } else {
                        Err(Error::new(
                            forked.span(),
                            format!("expect ident `{}`", name),
                        ))
                    }
                }
            })
        } else {
            match_keyword_cases!(match name.as_ref() {
                (lit, tk, parse: Self) => {
                    if self.peek(tk) {
                        Ok(Ident::new(name.as_ref(), parse(self)?.span))
                    } else {
                        Err(Error::new(
                            self.span(),
                            format!("expect token `{lit}`"),
                        ))
                    }
                }
                _ => {
                let forked = self.fork();
                    if let Ok(ident) = forked.parse::<Ident>() {
                        if ident.to_string().to_lowercase().eq(name.as_ref()) {
                            self.advance_to(&forked);
                            Ok(ident)
                        } else {
                            Err(Error::new(
                                ident.span(),
                                format!("expect ident `{}` but `{}`", name.as_ref(), ident.to_string()),
                            ))
                        }
                    } else {
                        Err(Error::new(
                            forked.span(),
                            format!("expect ident `{}`", name.as_ref()),
                        ))
                    }
                }
            })
        }
    }

    fn parse_as_ident(&self) -> Result<Ident> {
        if self.peek(Ident) {
            self.parse()
        } else if self.peek(LitStr) {
            let lit = self.parse::<LitStr>()?;
            Ok(lit.to_ident_with_case(Case::Camel))
        } else {
            peek_keyword_in_condition!(match self {
                (lit, parse) => {
                    Ok(Ident::new(lit, parse()?.span))
                }
                _ => {
                    self.span()
                        .to_syn_error("expect ident or alternatives")
                        .to_err()
                }
            })
        }
    }
}

impl ParseAsIdent for ParseBuffer<'_> {
    fn parse_as_named_ident<S: AsRef<str>>(&self, name: S, ignore_case: bool) -> Result<Ident> {
        if ignore_case {
            let name = name.as_ref().to_lowercase();
            match_keyword_cases!(match name.as_str() {
                (lit, tk, parse: &Self) => {
                    if self.peek(tk) {
                        Ok(Ident::new(name.as_ref(), parse(self)?.span))
                    } else {
                        Err(Error::new(
                            self.span(),
                            format!("expect token `{lit}`"),
                        ))
                    }
                }
                _ => {
                let forked = self.fork();
                    if let Ok(ident) = forked.parse::<Ident>() {
                        if ident.to_string().to_lowercase().eq(&name) {
                            self.advance_to(&forked);
                            Ok(ident)
                        } else {
                            Err(Error::new(
                                ident.span(),
                                format!("expect ident `{}` but `{}`", name, ident.to_string()),
                            ))
                        }
                    } else {
                        Err(Error::new(
                            forked.span(),
                            format!("expect ident `{}`", name),
                        ))
                    }
                }
            })
        } else {
            match_keyword_cases!(match name.as_ref() {
                (lit, tk, parse: &Self) => {
                    if self.peek(tk) {
                        Ok(Ident::new(name.as_ref(), parse(self)?.span))
                    } else {
                        Err(Error::new(
                            self.span(),
                            format!("expect token `{lit}`"),
                        ))
                    }
                }
                _ => {
                let forked = self.fork();
                    if let Ok(ident) = forked.parse::<Ident>() {
                        if ident.to_string().to_lowercase().eq(name.as_ref()) {
                            self.advance_to(&forked);
                            Ok(ident)
                        } else {
                            Err(Error::new(
                                ident.span(),
                                format!("expect ident `{}` but `{}`", name.as_ref(), ident.to_string()),
                            ))
                        }
                    } else {
                        Err(Error::new(
                            forked.span(),
                            format!("expect ident `{}`", name.as_ref()),
                        ))
                    }
                }
            })
        }
    }

    fn parse_as_ident(&self) -> Result<Ident> {
        if self.peek(Ident) {
            self.parse()
        } else if self.peek(LitStr) {
            let lit = self.parse::<LitStr>()?;
            Ok(lit.to_ident_with_case(Case::Camel))
        } else {
            peek_keyword_in_condition!(match self {
                (lit, parse) => {
                    Ok(Ident::new(lit, parse()?.span))
                }
                _ => {
                    self.span()
                        .to_syn_error("expect ident or alternatives")
                        .to_err()
                }
            })
        }
    }
}
