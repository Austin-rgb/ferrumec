mod private {
    pub trait Sealed {}
    impl<T> Sealed for T {}
}

pub trait Handler<Args>: private::Sealed {
    type Output;
    fn call(self, args: Args) -> Self::Output;
}

impl<F, R> Handler<()> for F
where
    F: FnOnce() -> R,
{
    type Output = R;
    fn call(self, _: ()) -> R {
        self()
    }
}

impl<F, A, R> Handler<(A,)> for F
where
    F: FnOnce(A) -> R,
{
    type Output = R;
    fn call(self, args: (A,)) -> R {
        self(args.0)
    }
}

impl<F, A, B, R> Handler<(A, B)> for F
where
    F: FnOnce(A, B) -> R,
{
    type Output = R;
    fn call(self, args: (A, B)) -> R {
        self(args.0, args.1)
    }
}

impl<F, A, B, C, R> Handler<(A, B, C)> for F
where
    F: FnOnce(A, B, C) -> R,
{
    type Output = R;
    fn call(self, args: (A, B, C)) -> R {
        self(args.0, args.1, args.2)
    }
}

impl<F, A, B, C, D, R> Handler<(A, B, C, D)> for F
where
    F: FnOnce(A, B, C, D) -> R,
{
    type Output = R;
    fn call(self, args: (A, B, C, D)) -> R {
        self(args.0, args.1, args.2, args.3)
    }
}
