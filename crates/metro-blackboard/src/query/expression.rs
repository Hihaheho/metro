pub mod r#const;

macro_rules! define_expr {
    ($name:ident, $($f:ident: $t:ident),*) => {
        pub trait $name<$($t),*> {
            type Out;
            fn run(&self, $($f: $t),*) -> Self::Out;
        }

        impl <$($t,)* F, Out> $name<$($t),*> for F
        where
            F: Fn($($t),*) -> Out,
        {
            type Out = Out;
            fn run(&self, $($f: $t),*) -> Self::Out {
                self($($f),*)
            }
        }
    }
}

define_expr!(Expr0,);
define_expr!(Expr1, a: A);
define_expr!(Expr2, a: A, b: B);
define_expr!(Expr3, a: A, b: B, c: C);
define_expr!(Expr4, a: A, b: B, c: C, d: D);
