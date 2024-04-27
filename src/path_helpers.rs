use proc_macro2::Span;
use syn::{punctuated::Punctuated, Ident, PathArguments, PathSegment, Token};

pub trait PathHelpers {
    fn new() -> Self;
    fn from_ident(origin: &Ident) -> Self;
    fn push_segment<S: IntoSegment>(&mut self, segment: S) -> &mut Self;
    fn modify_segment_at(
        &mut self,
        index: usize,
        modify: impl FnMut(&mut PathSegment),
    ) -> &mut Self;
    fn push_ident_arg(&mut self, index: usize, ident: &Ident) -> &mut Self {
        self.modify_segment_at(index, |segment| {
            let arg = syn::GenericArgument::Type(syn::Type::Path(syn::TypePath {
                qself: None,
                path: syn::Path::from_ident(ident),
            }));
            if let syn::PathArguments::AngleBracketed(args) = &mut segment.arguments {
                args.args.push(arg)
            } else {
                segment.arguments =
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Token![<](ident.span()),
                        args: {
                            let mut args = Punctuated::new();
                            args.push(arg);
                            args
                        },
                        gt_token: Token![>](ident.span()),
                    })
            }
        })
    }
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

    fn modify_segment_at(
        &mut self,
        index: usize,
        mut modify: impl FnMut(&mut PathSegment),
    ) -> &mut Self {
        if let Some((_, segment)) = self
            .segments
            .iter_mut()
            .enumerate()
            .find(|(i, _)| *i == index)
        {
            modify(segment);
        }
        self
    }
}
