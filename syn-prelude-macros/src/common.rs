use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Expr, ExprLit, ExprRange, Lit, RangeLimits,
};

pub(crate) struct TupleGen {
    pub(crate) span: Span,
    pub(crate) start_value: usize,
    pub(crate) end_value: usize,
}

impl Parse for TupleGen {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let range = input.parse::<ExprRange>()?;
        let start = if let Some(start) = &range.start {
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(n), ..
            }) = start.as_ref()
            {
                n.clone()
            } else {
                Err(syn::Error::new(start.span(), "unexpected expr"))?
            }
        } else {
            syn::LitInt::new("2", range.span())
        };
        let end = if let Some(end) = &range.end {
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(n), ..
            }) = end.as_ref()
            {
                n.clone()
            } else {
                Err(syn::Error::new(start.span(), "unexpected expr"))?
            }
        } else {
            Err(syn::Error::new(range.span(), "missing end"))?
        };

        let start_value: usize = start.base10_parse()?;
        if start_value <= 1 {
            Err(syn::Error::new(start.span(), "at least 2"))?;
        }
        let mut end_value: usize = end.base10_parse()?;
        if let RangeLimits::Closed(_) = range.limits {
            end_value += 1;
        }

        Ok(Self {
            span: range.span(),
            start_value,
            end_value,
        })
    }
}

impl TupleGen {
    pub(crate) fn iter(&self) -> impl Iterator<Item = usize> {
        (self.start_value..self.end_value).into_iter()
    }
}
