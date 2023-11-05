use super::Expr0;

#[derive(Debug)]
pub struct Const<T>(T);

impl<T> From<T> for Const<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

trait IntoConst: Sized {
    fn into_const(self) -> Const<Self>;
}

impl<T> IntoConst for T {
    fn into_const(self) -> Const<Self> {
        Const(self)
    }
}

impl<T: Clone> Expr0 for Const<T> {
    type Out = T;
    fn run(&self) -> Self::Out {
        self.0.clone()
    }
}
