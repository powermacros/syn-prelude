use std::fmt::Display;

use proc_macro2::Span;
use syn::{punctuated::Punctuated, spanned::Spanned, Error, Ident, LitStr};

use crate::ToSpan;

pub trait ToErr {
    fn to_err<T>(self) -> syn::Result<T>;
}

impl ToErr for syn::Error {
    fn to_err<T>(self) -> syn::Result<T> {
        Err(self)
    }
}

pub trait ToSynError {
    fn to_syn_error<S: Display>(&self, message: S) -> Error;
}

impl ToSynError for Span {
    fn to_syn_error<S: Display>(&self, message: S) -> Error {
        Error::new(self.span(), message)
    }
}

impl ToSynError for Ident {
    fn to_syn_error<S: Display>(&self, message: S) -> Error {
        Error::new(self.span(), message)
    }
}

impl ToSynError for LitStr {
    fn to_syn_error<S: Display>(&self, message: S) -> Error {
        Error::new(self.span(), message)
    }
}

pub trait ToOptionalSynError {
    fn to_optional_syn_error<S: Display>(&self, message: S) -> Option<Error>;
}

impl ToOptionalSynError for Vec<Span> {
    fn to_optional_syn_error<S: Display>(&self, message: S) -> Option<Error> {
        let mut span: Option<Span> = None;
        for s in self {
            if let Some(x) = &mut span {
                span = x.join(*s)
            } else {
                span = Some(*s)
            }
        }
        span.map(|s| s.to_syn_error(message))
    }
}

impl<T: ToSpan> ToOptionalSynError for Vec<T> {
    fn to_optional_syn_error<S: Display>(&self, message: S) -> Option<Error> {
        if let Some(first) = self.first() {
            let span = first.to_span();
            Some(if let Some(last) = self.last() {
                Error::new(span.join(last.to_span()).unwrap_or(span), message)
            } else {
                Error::new(span, message)
            })
        } else {
            None
        }
    }
}

impl<T: ToSpan + Clone> ToOptionalSynError for Vec<Option<T>> {
    fn to_optional_syn_error<S: Display>(&self, message: S) -> Option<Error> {
        if let Some(spans) = self
            .iter()
            .map(|span| span.clone())
            .collect::<Option<Vec<_>>>()
        {
            spans.to_optional_syn_error(message)
        } else {
            None
        }
    }
}

impl<T: ToSpan, P> ToOptionalSynError for Punctuated<T, P> {
    fn to_optional_syn_error<S: Display>(&self, message: S) -> Option<Error> {
        if let Some(first) = self.first() {
            let span = first.to_span();
            Some(if let Some(last) = self.last() {
                Error::new(span.join(last.to_span()).unwrap_or(span), message)
            } else {
                Error::new(span, message)
            })
        } else {
            None
        }
    }
}

impl ToOptionalSynError for Vec<Option<Span>> {
    fn to_optional_syn_error<S: Display>(&self, message: S) -> Option<Error> {
        if let Some(spans) = self
            .iter()
            .map(|span| span.clone())
            .collect::<Option<Vec<_>>>()
        {
            spans.to_optional_syn_error(message)
        } else {
            None
        }
    }
}
