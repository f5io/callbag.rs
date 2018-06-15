#[macro_export]
macro_rules! pipe {
    ($a:expr, $b:expr) => ($b($a));
    ($a:expr, $b:expr, $($rest:expr),*) => {
        pipe!($b($a), $($rest),*)
    };
}

#[macro_export]
macro_rules! merge {
    ($a:expr, $b:expr) => (merge($a, $b));
    ($a:expr, $b:expr, $($rest:expr),*) => {
        merge!(merge($a, $b), $($rest),*)
    };
}
