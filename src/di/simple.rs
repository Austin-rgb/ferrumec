use crate::di::{EnvContext, EnvError};
pub trait AsyncFrom<T, E>: Sized {
    async fn async_from(value: &T) -> Result<Self, E>;
}

pub trait ExtractFrom<T, E>
where
    Self: Sized,
{
    fn extract_from(value: &T) -> Result<Self, E>;
}

macro_rules! tuple_async_from {
    ($($T:ident),*) => {
        impl<$($T: AsyncFrom<C, E>,)* C,E > AsyncFrom<C,E> for ($($T,)*) {
            async fn async_from(value: &C) -> Result<Self,E> {
                Ok(
                ($($T::async_from(value).await?,)*))
            }
        }
    };
}

macro_rules! tuple_extract_from {
    ($($T:ident),*) => {
        impl<$($T: ExtractFrom<C, E>,)* C, E> ExtractFrom<C, E> for ($($T,)*) {
            fn extract_from(value: &C) -> Result<Self,E> {
                Ok(($($T::extract_from(value)?,)*))
            }
        }
    };
}

tuple_async_from!(T1, T2, T3, T4, T5);
tuple_async_from!(T1, T2, T3, T4);
tuple_async_from!(T1, T2, T3);
tuple_async_from!(T1, T2);
tuple_async_from!(T1);
tuple_extract_from!(T1, T2, T3, T4);
tuple_extract_from!(T1, T2, T3);
tuple_extract_from!(T1, T2);
tuple_extract_from!(T1);
