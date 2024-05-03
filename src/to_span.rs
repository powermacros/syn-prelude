use proc_macro2::Span;
use syn_prelude_macros::gen_tuples_for_impl_to_span;

pub trait ToSpan {
    fn to_span(&self) -> Span;
}

impl ToSpan for Span {
    fn to_span(&self) -> Span {
        *self
    }
}

impl ToSpan for &syn::Ident {
    fn to_span(&self) -> Span {
        self.span()
    }
}

impl<T: ToSpan> ToSpan for Vec<T> {
    fn to_span(&self) -> Span {
        if let Some(first) = self.first() {
            if self.len() > 1 {
                if let Some(last) = self.last() {
                    first
                        .to_span()
                        .join(last.to_span())
                        .unwrap_or(first.to_span())
                } else {
                    first.to_span()
                }
            } else {
                first.to_span()
            }
        } else {
            Span::call_site()
        }
    }
}

gen_tuples_for_impl_to_span!(2..10);
