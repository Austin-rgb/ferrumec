pub trait AsyncFrom<T, E>: Sized {
    fn async_from(v: &T) -> impl std::future::Future<Output = Result<Self, E>>;
}

mod private {
    pub trait Sealed {}
    impl<T> Sealed for T {}
}

pub trait Handler<Args>: private::Sealed {
    type Output;
    fn call(self, args: Args) -> Self::Output;
}

macro_rules! taf {
    ($($T:ident),*) => {
        impl<$($T: AsyncFrom<C, E>,)* C,E > AsyncFrom<C,E> for ($($T,)*) {
            async fn async_from(v: &C) -> Result<Self,E> {
                Ok(
                ($($T::async_from(v).await?,)*))
            }
        }
    };
}

macro_rules! impl_handler {
    () => {
        impl<F, R> Handler<()> for F
        where
            F: FnOnce() -> R,
        {
            type Output = R;

            fn call(self, _: ()) -> R {
                self()
            }
        }
    };

    ($($T:ident),+ $(,)?) => {
        impl<F, $($T,)+ R> Handler<($($T,)+)> for F
        where
            F: FnOnce($($T),+) -> R,
        {
            type Output = R;

            #[allow(non_snake_case)]
            fn call(self, args: ($($T,)+)) -> R {
                let ($($T,)+) = args;
                self($($T),+)
            }
        }
    };
}

impl_handler!();
impl_handler!(A);
impl_handler!(A, B);
impl_handler!(A, B, C);
impl_handler!(A, B, C, D);
impl_handler!(A, B, C, D, E);

// Add more as needed

taf!(T1, T2, T3, T4, T5);
taf!(T1, T2, T3, T4);
taf!(T1, T2, T3);
taf!(T1, T2);
taf!(T1);

pub async fn inject<F, Args, T, E>(f: F, ctx: &T) -> Result<F::Output, E>
where
    F: Handler<Args>,
    Args: AsyncFrom<T, E>,
{
    let args = Args::async_from(ctx).await?;
    Ok(f.call(args))
}

pub trait Inject {
    type Error;
    fn inject<F, Args>(&self, f: F) -> impl Future<Output = Result<F::Output, Self::Error>>
    where
        F: Handler<Args>,
        Args: AsyncFrom<Self, Self::Error>,
        Self: Sized,
    {
        async { inject(f, self).await }
    }
}
