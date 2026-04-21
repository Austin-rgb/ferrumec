#[macro_export]
macro_rules! run {
    ($func:expr) => {
        $crate::run($func)
    };
}

#[macro_export]
macro_rules! run_with {
    ($func:expr, $ctx:expr) => {
        $crate::run_with_ctx($func, $ctx)
    };
}
