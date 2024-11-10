/**
 * [3102] Minimize Manhattan Distances
 *
 * You are given an array points representing integer coordinates of some points on a 2D plane, where points[i] = [xi, yi].
 * The distance between two points is defined as their <span data-keyword="manhattan-distance">Manhattan distance</span>.
 * Return the minimum possible value for maximum distance between any two points by removing exactly one point.
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">points = [[3,10],[5,15],[10,2],[4,4]]</span>
 * Output: <span class="example-io">12</span>
 * Explanation:
 * The maximum distance after removing each point is the following:
 *
 * 	After removing the 0^th point the maximum distance is between points (5, 15) and (10, 2), which is |5 - 10| + |15 - 2| = 18.
 * 	After removing the 1^st point the maximum distance is between points (3, 10) and (10, 2), which is |3 - 10| + |10 - 2| = 15.
 * 	After removing the 2^nd point the maximum distance is between points (5, 15) and (4, 4), which is |5 - 4| + |15 - 4| = 12.
 * 	After removing the 3^rd point the maximum distance is between points (5, 15) and (10, 2), which is |5 - 10| + |15 - 2| = 18.
 *
 * 12 is the minimum possible maximum distance between any two points after removing exactly one point.
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">points = [[1,1],[1,1],[1,1]]</span>
 * Output: <span class="example-io">0</span>
 * Explanation:
 * Removing any of the points results in the maximum distance between any two points of 0.
 * </div>
 *  
 * Constraints:
 *
 * 	3 <= points.length <= 10^5
 * 	points[i].length == 2
 * 	1 <= points[i][0], points[i][1] <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimize-manhattan-distances/
// discuss: https://leetcode.com/problems/minimize-manhattan-distances/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::cmp::{max, min};

impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        fn max_distance(points: &Vec<Vec<i32>>, excludeid: Option<usize>) -> (i32, usize, usize) {
            let mut min_sum = (usize::MAX, i32::MAX);
            let mut max_sum = (usize::MAX, i32::MIN);
            let mut min_diff = (usize::MAX, i32::MAX);
            let mut max_diff = (usize::MAX, i32::MIN);

            for (i, point) in points.iter().enumerate() {
                if Some(i) == excludeid {
                    continue;
                }
                let sum = point[0] + point[1];
                let diff = point[0] - point[1];

                if sum < min_sum.1 {
                    min_sum = (i, sum);
                }
                if sum > max_sum.1 {
                    max_sum = (i, sum);
                }
                if diff < min_diff.1 {
                    min_diff = (i, diff);
                }
                if diff > max_diff.1 {
                    max_diff = (i, diff);
                }
            }

            if (max_sum.1 - min_sum.1) > (max_diff.1 - min_diff.1) {
                (max_sum.1 - min_sum.1, min_sum.0, max_sum.0)
            } else {
                (max_diff.1 - min_diff.1, min_diff.0, max_diff.0)
            }
        }

        // find max index of points that give max distance
        let (dist, i, j) = max_distance(&points, None);
        // find points after removing max distances, removing other points would result in same max distance
        min(
            max_distance(&points, Some(i)).0,
            max_distance(&points, Some(j)).0,
        )
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3102() {
        time_test!();

        assert_eq!(
            Solution::minimum_distance(vec_vec![[3, 10], [5, 15], [10, 2], [4, 4]]),
            12
        );
        assert_eq!(
            Solution::minimum_distance(vec_vec![[1, 1], [1, 1], [1, 1]]),
            0
        );
        assert_eq!(
            Solution::minimum_distance(vec_vec![[3, 2], [3, 9], [7, 10], [4, 4], [8, 10], [2, 7]]),
            10
        );
    }
}
