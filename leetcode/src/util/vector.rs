#[macro_export]
macro_rules! vec_string {
    ($($e:expr),*) => {vec![$($e.to_owned()), *]};
    ($($e:expr,)*) => {vec![$($e.to_owned()), *]};
}

#[macro_export]
macro_rules! vec_vec {
    [$($token_tree:tt),* $(,)?] => {
        vec![$(vec!$token_tree),*]
    };
}
