pub trait AsyncFrom<T, E>: Sized {
    async fn async_from(v: &T) -> Result<Self, E>;
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
            fn call(self, args: ()) -> R {
                self()
            }
        }
    };
    ($A:ident $(, $As:ident)*) => {
        impl_handler!($($As),*);
        impl<F, $A, $($As,)* R> Handler<($A, $($As,)*)> for F
        where
            F: FnOnce($A, $($As),*) -> R,
        {
            type Output = R;
            fn call(self, args: ($A, $($As,)*)) -> R {
                let ($A, $($As),*) = args;
                self($A, $($As),*)
            }
        }
    };
}

impl_handler!(A, B, C, D, E, G, H, I, J);

// Add more as needed

taf!(T1, T2, T3, T4, T5);
taf!(T1, T2, T3, T4);
taf!(T1, T2, T3);
taf!(T1, T2);
taf!(T1);

pub async fn inject<F, Args, T, E>(f: F, ctx: T) -> Result<F::Output, E>
where
    F: Handler<Args>,
    Args: AsyncFrom<T, E>,
{
    let args = Args::async_from(&ctx).await?;
    Ok(f.call(args))
}

