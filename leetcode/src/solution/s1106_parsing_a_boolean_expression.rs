/**
 * [1106] Parsing A Boolean Expression
 *
 * A boolean expression is an expression that evaluates to either true or false. It can be in one of the following shapes:
 *
 * 	't' that evaluates to true.
 * 	'f' that evaluates to false.
 * 	'!(subExpr)' that evaluates to the logical NOT of the inner expression subExpr.
 * 	'&amp;(subExpr1, subExpr2, ..., subExprn)' that evaluates to the logical AND of the inner expressions subExpr1, subExpr2, ..., subExprn where n >= 1.
 * 	'|(subExpr1, subExpr2, ..., subExprn)' that evaluates to the logical OR of the inner expressions subExpr1, subExpr2, ..., subExprn where n >= 1.
 *
 * Given a string expression that represents a boolean expression, return the evaluation of that expression.
 * It is guaranteed that the given expression is valid and follows the given rules.
 *  
 * Example 1:
 *
 * Input: expression = "&amp;(|(f))"
 * Output: false
 * Explanation:
 * First, evaluate |(f) --> f. The expression is now "&amp;(f)".
 * Then, evaluate &amp;(f) --> f. The expression is now "f".
 * Finally, return false.
 *
 * Example 2:
 *
 * Input: expression = "|(f,f,f,t)"
 * Output: true
 * Explanation: The evaluation of (false OR false OR false OR true) is true.
 *
 * Example 3:
 *
 * Input: expression = "!(&amp;(f,t))"
 * Output: true
 * Explanation:
 * First, evaluate &amp;(f,t) --> (false AND true) --> false --> f. The expression is now "!(f)".
 * Then, evaluate !(f) --> NOT false --> true. We return true.
 *
 *  
 * Constraints:
 *
 * 	1 <= expression.length <= 2 * 10^4
 * 	expression[i] is one following characters: '(', ')', '&amp;', '|', '!', 't', 'f', and ','.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/parsing-a-boolean-expression/
// discuss: https://leetcode.com/problems/parsing-a-boolean-expression/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        // Tokenize each expression for evaluation
        fn evaluate_expression(expression: &[u8], start: usize, end: usize) -> bool {
            match expression[start] {
                b't' => true,
                b'f' => false,
                b'!' => !evaluate_expression(expression, start + 2, end - 1),
                b'&' => {
                    let mut result = true;
                    let mut i = start + 2;
                    while i < end - 1 {
                        let j = find_next(expression, i, end - 1);
                        result &= evaluate_expression(expression, i, j);
                        i = j + 1;
                    }
                    result
                }
                b'|' => {
                    let mut result = false;
                    let mut i = start + 2;
                    while i < end - 1 {
                        let j = find_next(expression, i, end - 1);
                        result |= evaluate_expression(expression, i, j);
                        i = j + 1;
                    }
                    result
                }
                _ => unreachable!(), // Nothing else to tokenize
            }
        }
        // walk the string till we find the next expression
        fn find_next(expression: &[u8], start: usize, end: usize) -> usize {
            let mut balance = 0;
            for i in start..end {
                match expression[i] {
                    b'(' => balance += 1,
                    b')' => balance -= 1,
                    b',' if balance == 0 => return i,
                    _ => {}
                }
            }
            end
        }

        evaluate_expression(expression.as_bytes(), 0, expression.len())
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1106() {
        let start = std::time::Instant::now();

        assert_eq!(Solution::parse_bool_expr("&(|(f))".to_owned()), false);
        assert_eq!(Solution::parse_bool_expr("|(f,f,f,t)".to_owned()), true);
        assert_eq!(Solution::parse_bool_expr("!(&(f,t))".to_owned()), true);

        println!("Time elapsed in {} is: {:?}", file!(), start.elapsed());
    }
}
