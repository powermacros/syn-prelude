use syn::{
    parse::{discouraged::Speculative, ParseBuffer, ParseStream},
    token, Ident,
};
use syn_prelude_macros::{impl_try_parse_one_of_idents_for_tuple, match_keyword_cases};

pub trait TryParseOneOfIdents {
    fn try_parse_one_of_idents<Names: IdentNames>(&self, names: Names) -> Option<Ident>;
}

pub trait IdentNames {
    fn iter_name(&self) -> impl Iterator<Item = impl AsRef<str>>;
}

impl<'a> IdentNames for Vec<&'a str> {
    fn iter_name(&self) -> impl Iterator<Item = impl AsRef<str>> {
        self.iter()
    }
}

impl<'a> IdentNames for Vec<&'a String> {
    fn iter_name(&self) -> impl Iterator<Item = impl AsRef<str>> {
        self.iter()
    }
}

impl IdentNames for Vec<String> {
    fn iter_name(&self) -> impl Iterator<Item = impl AsRef<str>> {
        self.iter()
    }
}

macro_rules! ident_names_for_list {
    ($n:expr) => {
        impl<'a> IdentNames for [&'a str; $n] {
            fn iter_name(&self) -> impl Iterator<Item = impl AsRef<str>> {
                self.iter()
            }
        }
    };
}

ident_names_for_list!(1);
ident_names_for_list!(2);
ident_names_for_list!(3);
ident_names_for_list!(4);
ident_names_for_list!(5);
ident_names_for_list!(6);
ident_names_for_list!(7);
ident_names_for_list!(8);
ident_names_for_list!(9);
ident_names_for_list!(10);

impl_try_parse_one_of_idents_for_tuple!(20);

impl TryParseOneOfIdents for ParseStream<'_> {
    fn try_parse_one_of_idents<Names: IdentNames>(&self, names: Names) -> Option<Ident> {
        for name in names.iter_name() {
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
    fn try_parse_one_of_idents<Names: IdentNames>(&self, names: Names) -> Option<Ident> {
        for name in names.iter_name() {
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
