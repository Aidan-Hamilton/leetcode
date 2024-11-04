/**
 * [3163] String Compression III
 *
 * Given a string word, compress it using the following algorithm:
 *
 * 	Begin with an empty string comp. While word is not empty, use the following operation:
 *
 * 		Remove a maximum length prefix of word made of a single character c repeating at most 9 times.
 * 		Append the length of the prefix followed by c to comp.
 *
 *
 *
 * Return the string comp.
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">word = "abcde"</span>
 * Output: <span class="example-io">"1a1b1c1d1e"</span>
 * Explanation:
 * Initially, comp = "". Apply the operation 5 times, choosing "a", "b", "c", "d", and "e" as the prefix in each operation.
 * For each prefix, append "1" followed by the character to comp.
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">word = "aaaaaaaaaaaaaabb"</span>
 * Output: <span class="example-io">"9a5a2b"</span>
 * Explanation:
 * Initially, comp = "". Apply the operation 3 times, choosing "aaaaaaaaa", "aaaaa", and "bb" as the prefix in each operation.
 *
 * 	For prefix "aaaaaaaaa", append "9" followed by "a" to comp.
 * 	For prefix "aaaaa", append "5" followed by "a" to comp.
 * 	For prefix "bb", append "2" followed by "b" to comp.
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= word.length <= 2 * 10^5
 * 	word consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/string-compression-iii/
// discuss: https://leetcode.com/problems/string-compression-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut comp = String::new();
        let mut chars = word.chars().peekable();

        while let Some(&c) = chars.peek() {
            let mut count = 0;
            while count < 9 && chars.peek() == Some(&c) {
                chars.next();
                count += 1;
            }
            comp.push_str(&format!("{}{}", count, c));
        }

        comp
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3163() {
        time_test!();

        assert_eq!(
            Solution::compressed_string("abcde".to_owned()),
            "1a1b1c1d1e".to_owned()
        );
        assert_eq!(
            Solution::compressed_string("aaaaaaaaaaaaaabb".to_owned()),
            "9a5a2b".to_owned()
        );
        assert_eq!(Solution::compressed_string("o".to_owned()), "1o".to_owned());
        assert_eq!(
            Solution::compressed_string("vvv".to_owned()),
            "3v".to_owned()
        );
        assert_eq!(
            Solution::compressed_string("xxlaa".to_owned()),
            "2x1l2a".to_owned()
        );
        assert_eq!(
            Solution::compressed_string("xbbbbjj".to_owned()),
            "1x4b2j".to_owned()
        );
        assert_eq!(
            Solution::compressed_string(
                "yyyyyyyyfffccccqqwwffffffffrrrrrrrrraaaaayyyyyyyyy".to_owned()
            ),
            "8y3f4c2q2w8f9r5a9y".to_owned()
        );
        assert_eq!(
            Solution::compressed_string(
                "uuuuuuuuuuccccccvvvvvvvyyyyyyyylyyyqqqqqqqqqoaqqqq".to_owned()
            ),
            "9u1u6c7v8y1l3y9q1o1a4q".to_owned()
        );
    }
}
