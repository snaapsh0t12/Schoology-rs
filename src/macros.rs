

// #[cfg()]
macro_rules! api {
    ($e:expr) => {
        concat!("https://api.schoology.com/v1/", $e)
    };
    ($e:expr, $($rest:tt)*) => {
        format!(api!($e), $($rest)*)
    };
}