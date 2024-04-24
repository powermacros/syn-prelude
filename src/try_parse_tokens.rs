use syn::{parse::ParseBuffer, Token};

pub trait TryParseTokens {
    fn try_parse_comma(&self) -> Option<Token![,]>;
    fn try_parse_colon(&self) -> Option<Token![:]>;
    fn try_parse_semi(&self) -> Option<Token![;]>;
    fn try_parse_dot(&self) -> Option<Token![.]>;
    fn try_parse_eq(&self) -> Option<Token![=]>;
}

impl TryParseTokens for ParseBuffer<'_> {
    fn try_parse_comma(&self) -> Option<Token![,]> {
        if self.peek(Token![,]) {
            self.parse::<Token![,]>().ok()
        } else {
            None
        }
    }

    fn try_parse_colon(&self) -> Option<Token![:]> {
        if self.peek(Token![:]) {
            self.parse::<Token![:]>().ok()
        } else {
            None
        }
    }

    fn try_parse_semi(&self) -> Option<Token![;]> {
        if self.peek(Token![;]) {
            self.parse::<Token![;]>().ok()
        } else {
            None
        }
    }

    fn try_parse_dot(&self) -> Option<Token![.]> {
        if self.peek(Token![.]) {
            self.parse::<Token![.]>().ok()
        } else {
            None
        }
    }

    fn try_parse_eq(&self) -> Option<Token![=]> {
        if self.peek(Token![=]) {
            self.parse::<Token![=]>().ok()
        } else {
            None
        }
    }
}
