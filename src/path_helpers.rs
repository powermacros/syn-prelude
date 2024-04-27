use proc_macro2::Span;
use syn::{punctuated::Punctuated, Ident, PathArguments, PathSegment};

pub trait PathHelpers {
    fn new() -> Self;
    fn from_ident(origin: &Ident) -> Self;
    fn push_segment<S: IntoSegment>(&mut self, segment: S) -> &mut Self;
}

pub trait IntoSegment {
    fn into_segment(self) -> PathSegment;
}

impl IntoSegment for PathSegment {
    fn into_segment(self) -> PathSegment {
        self
    }
}

impl IntoSegment for &PathSegment {
    fn into_segment(self) -> PathSegment {
        self.clone()
    }
}

impl IntoSegment for &Ident {
    fn into_segment(self) -> PathSegment {
        PathSegment {
            ident: self.clone(),
            arguments: PathArguments::None,
        }
    }
}

impl IntoSegment for Ident {
    fn into_segment(self) -> PathSegment {
        PathSegment {
            ident: self,
            arguments: PathArguments::None,
        }
    }
}

impl IntoSegment for (&str, Span) {
    fn into_segment(self) -> PathSegment {
        PathSegment {
            ident: Ident::new(self.0, self.1),
            arguments: PathArguments::None,
        }
    }
}

impl PathHelpers for syn::Path {
    fn new() -> Self {
        Self {
            leading_colon: None,
            segments: Punctuated::new(),
        }
    }

    fn from_ident(origin: &Ident) -> Self {
        let mut path = Self::new();
        path.push_segment(origin);
        path
    }

    fn push_segment<S: IntoSegment>(&mut self, segment: S) -> &mut Self {
        self.segments.push(segment.into_segment());
        self
    }
}
