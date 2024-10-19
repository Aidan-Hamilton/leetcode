/**
 * [__PROBLEM_ID__] __PROBLEM_TITLE__
 *
 * __PROBLEM_DESC__
 */
pub struct Solution {}__EXTRA_USE__

// problem: __PROBLEM_LINK__
// discuss: __DISCUSS_LINK__

// submission codes start here

__PROBLEM_DEFAULT_CODE__

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test___PROBLEM_ID__() {
        let start = std::time::Instant::now();

        __TEST_CASES__

        println!(
            "Time elapsed in {} is: {:?}",
            file!(),
            start.elapsed()
        );
    }
}
