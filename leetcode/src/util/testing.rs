use std::io::{self, Write};

#[macro_export]
macro_rules! assert_eq_sorted {
    ($left:expr, $right:expr) => ({
        let (mut v1, mut v2) = ($left, $right);
        v1.sort();
        v2.sort();
        assert_eq!(v1, v2)
    });
    ($left:expr, $right:expr,) => ({
        assert_eq_sorted!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        let (mut v1, mut v2) = ($left, $right);
        v1.sort();
        v2.sort();
        assert_eq!(v1, v2, $($arg)+)
    });
}

// Based on https://docs.rs/time-test/latest/time_test
// TestTimer allows to easily time tests. It's recommended to use the time_test!() macro instead of invoking this type directly.
pub struct TestTimer(std::time::Instant, String);

impl TestTimer {
    pub fn new(desc: String) -> TestTimer {
        TestTimer(std::time::Instant::now(), desc)
    }
}

impl Drop for TestTimer {
    fn drop(&mut self) {
        let dur = self.0.elapsed();
        if self.1.is_empty() {
            write!(io::stderr(), "(Time elapsed {:?}) ", dur).unwrap_or(());
        } else {
            write!(io::stderr(), "(Time elapsed in {} is: {:?}) ", self.1, dur).unwrap_or(());
        }
    }
}

#[macro_export]
macro_rules! time_test {
    () => {
        let _tt = crate::util::testing::TestTimer::new(String::new());
    };
    ($desc:expr) => {
        let _tt = crate::util::testing::TestTimer::new($desc.to_string());
    };
}
