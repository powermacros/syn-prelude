use match_keyword_cases::MatchKeywordCases;
use peek_keyword_in_condition::PeekKeywordInCondition;
use quote::ToTokens;
use repeat_keyword_lit_and_types::RepleatKeywordLitAndTypes;

mod common;
mod match_keyword_cases;
mod path_helpers;
mod peek_keyword_in_condition;
mod repeat_keyword_lit_and_types;
mod to_span;
mod try_parse_one_of_idents;

pub(crate) const KEYWORDS: [&'static str; 50] = [
    "abstract", "as", "async", "auto", "await", "become", "box", "break", "const", "continue",
    "crate", "default", "do", "dyn", "else", "enum", "extern", "final", "fn", "for", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "static", "struct", "super", "trait", "try", "type", "typeof", "union", "unsafe",
    "unsized", "use", "virtual", "where", "while", "yield",
];

///
/// repeat macro! as passing (keyword, token) pairs
///
/// exmpales:
/// ```rust
/// macro_rules! impl_to_ident_for_tokens {
///     ($token:literal, $name:ident) => {
///         impl ToIdent for token::$name {
///             fn to_ident(&self) -> Ident {
///                 Ident::new($token, self.span)
///             }
///         }
///     };
/// }
///
/// repeat_keyword_lit_and_types!(impl_to_ident_for_tokens);
/// ```
///
#[proc_macro]
pub fn repeat_keyword_lit_and_types(s: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<RepleatKeywordLitAndTypes>(s) {
        Ok(s) => s.to_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}

///
/// example:
///
/// ```rust
/// match_key_cases!(match match_expr {
///   (lit, token_type, parse: StreamType) => {
///      if input.peek(token_type) {
///        let tk = parse()?;
///        // do things with lit and tk
///      }
///   }
///   _ => {
///     // do others
///   }
/// })
/// ```
#[proc_macro]
pub fn match_keyword_cases(s: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<MatchKeywordCases>(s) {
        Ok(s) => s.to_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn peek_keyword_in_condition(s: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<PeekKeywordInCondition>(s) {
        Ok(s) => s.to_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn gen_tuples_for_impl_ident_names(s: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<common::TupleGen>(s) {
        Ok(s) => s.gen_tuples_for_impl_ident_names().into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn gen_tuples_for_impl_to_span(s: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<common::TupleGen>(s) {
        Ok(s) => s.gen_tuples_for_impl_to_span().into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn gen_tuples_for_impl_into_idents(s: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<common::TupleGen>(s) {
        Ok(s) => s.gen_tuples_for_impl_into_idents().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
