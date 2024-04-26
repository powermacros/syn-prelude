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

impl<T: ToSpan + Clone> ToOptionalSynError for Vec<Option<T>> {
    fn to_optional_syn_error<S: Display>(&self, message: S) -> Option<Error> {
        self.iter()
            .map(|s| s.as_ref().map(|s| s.to_span()))
            .reduce(|a, b| match (a, b) {
                (None, None) => None,
                (None, Some(b)) => Some(b.to_span()),
                (Some(a), None) => Some(a.to_span()),
                (Some(a), Some(b)) => a.to_span().join(b.to_span()),
            })
            .flatten()
            .map(|span| span.to_syn_error(message))
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

pub trait CombineSynErrors {
    fn combine_errors(self) -> Option<syn::Error>;
}

impl CombineSynErrors for Vec<syn::Error> {
    fn combine_errors(self) -> Option<syn::Error> {
        let mut i = self.into_iter();
        if let Some(mut err) = i.next() {
            while let Some(e) = i.next() {
                err.combine(e);
            }
            Some(err)
        } else {
            None
        }
    }
}

pub trait JoinSynErrors {
    fn join_errors(self) -> syn::Result<()>;
}

impl<T> JoinSynErrors for Vec<syn::Result<T>> {
    fn join_errors(self) -> syn::Result<()> {
        let mut final_err: Option<syn::Error> = None;
        let mut i = self.into_iter();
        while let Some(res) = i.next() {
            if let Err(err) = res {
                if let Some(final_err) = &mut final_err {
                    final_err.combine(err);
                } else {
                    final_err = Some(err);
                }
            }
        }
        if let Some(err) = final_err {
            Err(err)
        } else {
            Ok(())
        }
    }
}

impl<T1, T2> JoinSynErrors for (syn::Result<T1>, syn::Result<T2>) {
    fn join_errors(self) -> syn::Result<()> {
        match self {
            (Ok(_), Ok(_)) => Ok(()),
            (Ok(_), Err(err2)) => Err(err2),
            (Err(err1), Ok(_)) => Err(err1),
            (Err(mut err1), Err(err2)) => {
                err1.combine(err2);
                Err(err1)
            }
        }
    }
}

impl<T1, T2, T3> JoinSynErrors for (syn::Result<T1>, syn::Result<T2>, syn::Result<T3>) {
    fn join_errors(self) -> syn::Result<()> {
        let mut err = self.0.err();
        if let Err(err2) = self.1 {
            if let Some(err) = &mut err {
                err.combine(err2);
            } else {
                err = Some(err2);
            }
        }
        if let Err(err3) = self.2 {
            if let Some(err) = &mut err {
                err.combine(err3);
            } else {
                err = Some(err3);
            }
        }
        if let Some(err) = err {
            Err(err)
        } else {
            Ok(())
        }
    }
}
