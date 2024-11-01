/**
 * [1957] Delete Characters to Make Fancy String
 *
 * A fancy string is a string where no three consecutive characters are equal.
 * Given a string s, delete the minimum possible number of characters from s to make it fancy.
 * Return the final string after the deletion. It can be shown that the answer will always be unique.
 *  
 * Example 1:
 *
 * Input: s = "le<u>e</u>etcode"
 * Output: "leetcode"
 * Explanation:
 * Remove an 'e' from the first group of 'e's to create "leetcode".
 * No three consecutive characters are equal, so return "leetcode".
 *
 * Example 2:
 *
 * Input: s = "<u>a</u>aab<u>aa</u>aa"
 * Output: "aabaa"
 * Explanation:
 * Remove an 'a' from the first group of 'a's to create "aabaaaa".
 * Remove two 'a's from the second group of 'a's to create "aabaa".
 * No three consecutive characters are equal, so return "aabaa".
 *
 * Example 3:
 *
 * Input: s = "aab"
 * Output: "aab"
 * Explanation: No three consecutive characters are equal, so return "aab".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists only of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/delete-characters-to-make-fancy-string/
// discuss: https://leetcode.com/problems/delete-characters-to-make-fancy-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut result = String::new();
        let mut count = 0;
        let mut prev_char = '\0';

        for c in s.chars() {
            if c == prev_char {
                count += 1;
            } else {
                count = 1;
                prev_char = c;
            }

            if count < 3 {
                result.push(c);
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1957() {
        time_test!();

        assert_eq!(
            Solution::make_fancy_string("leeetcode".to_owned()),
            "leetcode".to_owned()
        );
        assert_eq!(
            Solution::make_fancy_string("aaabaaaa".to_owned()),
            "aabaa".to_owned()
        );
        assert_eq!(
            Solution::make_fancy_string("aab".to_owned()),
            "aab".to_owned()
        );
    }
}
