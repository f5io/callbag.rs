#[macro_export]
macro_rules! pipe {
    ($a:expr, $b:expr) => ($b($a));
    ($a:expr, $b:expr, $($rest:expr),*) => {
        pipe!($b($a), $($rest),*)
    };
}

#[macro_export]
macro_rules! combine {
    ($a:expr, $b:expr) => (combine($a, $b));
    ($a:expr, $b:expr, $($rest:expr),*) => {
        combine!(combine($a, $b), $($rest),*)
    };
}
