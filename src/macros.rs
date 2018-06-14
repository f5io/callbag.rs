#[macro_export]
macro_rules! pipe {
    ($a:expr, $b:expr) => ($b($a));
    ($a:expr, $b:expr, $($rest:expr),*) => {
        pipe!($b($a), $($rest),*)
    };
}
