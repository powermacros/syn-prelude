use quote::quote_spanned;
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Bracket, Expr, ExprLit, ExprMacro, ExprPath,
    Ident, Lit, LitBool, LitByte, LitByteStr, LitChar, LitFloat, LitInt, LitStr, Macro, Path,
    PathSegment, Token,
};

pub trait ToExpr {
    fn to_expr(self) -> Expr;
}

impl ToExpr for LitStr {
    fn to_expr(self) -> Expr {
        Expr::Lit(ExprLit {
            lit: Lit::Str(self),
            attrs: vec![],
        })
    }
}

impl ToExpr for &LitStr {
    fn to_expr(self) -> Expr {
        Expr::Lit(ExprLit {
            lit: Lit::Str(self.clone()),
            attrs: vec![],
        })
    }
}

impl ToExpr for LitInt {
    fn to_expr(self) -> Expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(self),
            attrs: vec![],
        })
    }
}

impl ToExpr for &LitInt {
    fn to_expr(self) -> Expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(self.clone()),
            attrs: vec![],
        })
    }
}

impl ToExpr for LitFloat {
    fn to_expr(self) -> Expr {
        Expr::Lit(ExprLit {
            lit: Lit::Float(self),
            attrs: vec![],
        })
    }
}

impl ToExpr for &LitFloat {
    fn to_expr(self) -> Expr {
        Expr::Lit(ExprLit {
            lit: Lit::Float(self.clone()),
            attrs: vec![],
        })
    }
}

impl ToExpr for LitBool {
    fn to_expr(self) -> Expr {
        Expr::Lit(ExprLit {
            lit: Lit::Bool(self),
            attrs: vec![],
        })
    }
}

impl ToExpr for &LitBool {
    fn to_expr(self) -> Expr {
        Expr::Lit(ExprLit {
            lit: Lit::Bool(self.clone()),
            attrs: vec![],
        })
    }
}

impl ToExpr for LitByte {
    fn to_expr(self) -> Expr {
        Expr::Lit(ExprLit {
            lit: Lit::Byte(self),
            attrs: vec![],
        })
    }
}

impl ToExpr for &LitByte {
    fn to_expr(self) -> Expr {
        Expr::Lit(ExprLit {
            lit: Lit::Byte(self.clone()),
            attrs: vec![],
        })
    }
}

impl ToExpr for LitByteStr {
    fn to_expr(self) -> Expr {
        Expr::Lit(ExprLit {
            lit: Lit::ByteStr(self),
            attrs: vec![],
        })
    }
}

impl ToExpr for &LitByteStr {
    fn to_expr(self) -> Expr {
        Expr::Lit(ExprLit {
            lit: Lit::ByteStr(self.clone()),
            attrs: vec![],
        })
    }
}

impl ToExpr for LitChar {
    fn to_expr(self) -> Expr {
        Expr::Lit(ExprLit {
            lit: Lit::Char(self),
            attrs: vec![],
        })
    }
}

impl ToExpr for &LitChar {
    fn to_expr(self) -> Expr {
        Expr::Lit(ExprLit {
            lit: Lit::Char(self.clone()),
            attrs: vec![],
        })
    }
}

impl ToExpr for Path {
    fn to_expr(self) -> Expr {
        Expr::Path(ExprPath {
            attrs: vec![],
            qself: None,
            path: self,
        })
    }
}

impl ToExpr for &Path {
    fn to_expr(self) -> Expr {
        Expr::Path(ExprPath {
            attrs: vec![],
            qself: None,
            path: self.clone(),
        })
    }
}

impl<T: ToExpr> ToExpr for Vec<T> {
    fn to_expr(self) -> Expr {
        let exprs = self.into_iter().map(|x| x.to_expr()).collect::<Vec<_>>();
        let span = exprs.first().map(|x| x.span()).unwrap();

        Expr::Macro(ExprMacro {
            attrs: vec![],
            mac: Macro {
                path: Path {
                    leading_colon: None,
                    segments: {
                        let mut segments = Punctuated::new();
                        segments.push(PathSegment {
                            ident: Ident::new("vec", span),
                            arguments: syn::PathArguments::None,
                        });
                        segments
                    },
                },
                bang_token: Token![!](span),
                delimiter: syn::MacroDelimiter::Bracket(Bracket(span)),
                tokens: quote_spanned!(span => #(#exprs),*).into(),
            },
        })
    }
}
