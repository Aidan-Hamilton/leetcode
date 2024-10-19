/**
 * [1405] Longest Happy String
 *
 * A string s is called happy if it satisfies the following conditions:
 *
 * 	s only contains the letters 'a', 'b', and 'c'.
 * 	s does not contain any of "aaa", "bbb", or "ccc" as a substring.
 * 	s contains at most a occurrences of the letter 'a'.
 * 	s contains at most b occurrences of the letter 'b'.
 * 	s contains at most c occurrences of the letter 'c'.
 *
 * Given three integers a, b, and c, return the longest possible happy string. If there are multiple longest happy strings, return any of them. If there is no such string, return the empty string "".
 * A substring is a contiguous sequence of characters within a string.
 *  
 * Example 1:
 *
 * Input: a = 1, b = 1, c = 7
 * Output: "ccaccbcc"
 * Explanation: "ccbccacc" would also be a correct answer.
 *
 * Example 2:
 *
 * Input: a = 7, b = 1, c = 0
 * Output: "aabaa"
 * Explanation: It is the only correct answer in this case.
 *
 *  
 * Constraints:
 *
 * 	0 <= a, b, c <= 100
 * 	a + b + c > 0
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-happy-string/
// discuss: https://leetcode.com/problems/longest-happy-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut heap = BinaryHeap::new();
        if a > 0 {
            heap.push((a, 'a'));
        }
        if b > 0 {
            heap.push((b, 'b'));
        }
        if c > 0 {
            heap.push((c, 'c'));
        }

        let mut result = String::new();

        while let Some((mut count1, char1)) = heap.pop() {
            if result.len() >= 2 && result.chars().rev().take(2).all(|c| c == char1) {
                if let Some((mut count2, char2)) = heap.pop() {
                    result.push(char2);
                    count2 -= 1;
                    if count2 > 0 {
                        heap.push((count2, char2));
                    }
                    heap.push((count1, char1));
                } else {
                    break;
                }
            } else {
                result.push(char1);
                count1 -= 1;
                if count1 > 0 {
                    heap.push((count1, char1));
                }
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
    fn test_1405() {
        let start = std::time::Instant::now();

        let result = Solution::longest_diverse_string(1, 1, 7);
        assert!(result == "ccbccacc" || result == "ccaccbcc");
        assert_eq!(Solution::longest_diverse_string(7, 1, 0), "aabaa");

        println!("Time elapsed in {} is: {:?}", file!(), start.elapsed());
    }
}
