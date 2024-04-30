use syn::{
    Expr, ExprLit, ExprPath, Lit, LitBool, LitByte, LitByteStr, LitChar, LitFloat, LitInt, LitStr,
    Path,
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
