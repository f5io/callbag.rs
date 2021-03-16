#[macro_export]
macro_rules! pipe {
    ($a:expr, $b:expr) => ($b($a));
    ($a:expr, $b:expr, $($rest:expr),*) => {
        $crate::pipe!($b($a), $($rest),*)
    };
}

#[macro_export]
macro_rules! merge {
    ($a:expr, $b:expr) => ($crate::operators::merge($a, $b));
    ($a:expr, $b:expr, $($rest:expr),*) => {
        $crate::merge!($crate::operators::merge($a, $b), $($rest),*)
    };
}
