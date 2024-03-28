use proc_macro2::Span;

pub trait ToSpan {
    fn to_span(&self) -> Span;
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
                    first.to_span().resolved_at(last.to_span())
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
