use proc_macro2::Span;
use syn::{punctuated::Punctuated, spanned::Spanned, Ident, PathArguments, PathSegment, Token};

pub trait PathHelpers {
    fn new() -> Self;
    fn from_ident<I: IntoIdent>(origin: I) -> Self;
    fn push_segment<S: IntoSegment>(&mut self, segment: S) -> &mut Self;
    fn modify_segment_at<F: FnOnce(&mut PathSegment)>(
        &mut self,
        index: usize,
        modify: F,
    ) -> &mut Self;
    fn push_arg(&mut self, index: usize, typ: syn::Type) -> &mut Self {
        self.modify_segment_at(index, |segment| {
            let span = typ.span();
            let arg = syn::GenericArgument::Type(typ);
            if let syn::PathArguments::AngleBracketed(args) = &mut segment.arguments {
                args.args.push(arg)
            } else {
                segment.arguments =
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Token![<](span),
                        args: {
                            let mut args = Punctuated::new();
                            args.push(arg);
                            args
                        },
                        gt_token: Token![>](span),
                    })
            }
        })
    }
    fn push_ident_arg(&mut self, index: usize, ident: Ident) -> &mut Self {
        self.push_arg(
            index,
            syn::Type::Path(syn::TypePath {
                qself: None,
                path: syn::Path::from_ident(ident),
            }),
        )
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

pub trait IntoIdent {
    fn into_ident(self) -> syn::Ident;
}

impl IntoIdent for Ident {
    fn into_ident(self) -> syn::Ident {
        self
    }
}

impl IntoIdent for &Ident {
    fn into_ident(self) -> syn::Ident {
        self.clone()
    }
}

impl IntoIdent for (&str, Span) {
    fn into_ident(self) -> syn::Ident {
        syn::Ident::new(self.0, self.1)
    }
}

impl PathHelpers for syn::Path {
    fn new() -> Self {
        Self {
            leading_colon: None,
            segments: Punctuated::new(),
        }
    }

    fn from_ident<I: IntoIdent>(origin: I) -> Self {
        let mut path = Self::new();
        path.push_segment(origin.into_ident());
        path
    }

    fn push_segment<S: IntoSegment>(&mut self, segment: S) -> &mut Self {
        self.segments.push(segment.into_segment());
        self
    }

    fn modify_segment_at<F: FnOnce(&mut PathSegment)>(
        &mut self,
        index: usize,
        modify: F,
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
