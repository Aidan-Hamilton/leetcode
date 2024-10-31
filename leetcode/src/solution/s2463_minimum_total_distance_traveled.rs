/**
 * [2463] Minimum Total Distance Traveled
 *
 * There are some robots and factories on the X-axis. You are given an integer array robot where robot[i] is the position of the i^th robot. You are also given a 2D integer array factory where factory[j] = [positionj, limitj] indicates that positionj is the position of the j^th factory and that the j^th factory can repair at most limitj robots.
 * The positions of each robot are unique. The positions of each factory are also unique. Note that a robot can be in the same position as a factory initially.
 * All the robots are initially broken; they keep moving in one direction. The direction could be the negative or the positive direction of the X-axis. When a robot reaches a factory that did not reach its limit, the factory repairs the robot, and it stops moving.
 * At any moment, you can set the initial direction of moving for some robot. Your target is to minimize the total distance traveled by all the robots.
 * Return the minimum total distance traveled by all the robots. The test cases are generated such that all the robots can be repaired.
 * Note that
 *
 * 	All robots move at the same speed.
 * 	If two robots move in the same direction, they will never collide.
 * 	If two robots move in opposite directions and they meet at some point, they do not collide. They cross each other.
 * 	If a robot passes by a factory that reached its limits, it crosses it as if it does not exist.
 * 	If the robot moved from a position x to a position y, the distance it moved is |y - x|.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/09/15/example1.jpg" style="width: 500px; height: 320px;" />
 * Input: robot = [0,4,6], factory = [[2,2],[6,2]]
 * Output: 4
 * Explanation: As shown in the figure:
 * - The first robot at position 0 moves in the positive direction. It will be repaired at the first factory.
 * - The second robot at position 4 moves in the negative direction. It will be repaired at the first factory.
 * - The third robot at position 6 will be repaired at the second factory. It does not need to move.
 * The limit of the first factory is 2, and it fixed 2 robots.
 * The limit of the second factory is 2, and it fixed 1 robot.
 * The total distance is |2 - 0| + |2 - 4| + |6 - 6| = 4. It can be shown that we cannot achieve a better total distance than 4.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/09/15/example-2.jpg" style="width: 500px; height: 329px;" />
 * Input: robot = [1,-1], factory = [[-2,1],[2,1]]
 * Output: 2
 * Explanation: As shown in the figure:
 * - The first robot at position 1 moves in the positive direction. It will be repaired at the second factory.
 * - The second robot at position -1 moves in the negative direction. It will be repaired at the first factory.
 * The limit of the first factory is 1, and it fixed 1 robot.
 * The limit of the second factory is 1, and it fixed 1 robot.
 * The total distance is |2 - 1| + |(-2) - (-1)| = 2. It can be shown that we cannot achieve a better total distance than 2.
 *
 *  
 * Constraints:
 *
 * 	1 <= robot.length, factory.length <= 100
 * 	factory[j].length == 2
 * 	-10^9 <= robot[i], positionj <= 10^9
 * 	0 <= limitj <= robot.length
 * 	The input will be generated such that it is always possible to repair every robot.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-total-distance-traveled/
// discuss: https://leetcode.com/problems/minimum-total-distance-traveled/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        let mut robot = robot;
        let mut factory = factory;

        robot.sort();
        factory.sort();

        let mut flattened_factory = Vec::new();
        for f in &factory {
            for _ in 0..f[1] {
                flattened_factory.push(f[0]);
            }
        }

        let robot_count = robot.len();
        let factory_count = flattened_factory.len();

        let mut dp = vec![vec![0i64; factory_count + 1]; robot_count + 1];
        for i in 0..robot_count {
            dp[i][factory_count] = i64::MAX / 2; // 10e9
        }

        for i in (0..robot_count).rev() {
            for j in (0..factory_count).rev() {
                dp[i][j] = std::cmp::min(
                    (robot[i] - flattened_factory[j]).abs() as i64 + dp[i + 1][j + 1],
                    dp[i][j + 1],
                );
            }
        }

        dp[0][0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2463() {
        time_test!();

        assert_eq!(
            Solution::minimum_total_distance(vec![0, 4, 6], vec_vec![[2, 2], [6, 2]]),
            4
        );
        assert_eq!(
            Solution::minimum_total_distance(vec![1, -1], vec_vec![[-2, 1], [2, 1]]),
            2
        );
        assert_eq!(
            Solution::minimum_total_distance(vec![7], vec_vec![[0, 1]]),
            7
        );
        assert_eq!(
            Solution::minimum_total_distance(
                vec![-40, -14, -8, 1, 3, 5, 39],
                vec_vec![[-34, 5], [28, 2], [-12, 3]]
            ),
            92
        );
        assert_eq!(
            Solution::minimum_total_distance(
                vec![9, 11, 99, 101],
                vec_vec![[10, 1], [7, 1], [14, 1], [100, 1], [96, 1], [103, 1]]
            ),
            6
        );
        assert_eq!(
            Solution::minimum_total_distance(
                vec![-551, -510, -344, -264, -242, -50, 202, 185, 700, 947],
                vec_vec![[-79, 5], [-534, 5]]
            ),
            3172
        );
        assert_eq!(
            Solution::minimum_total_distance(
                vec![
                    403625544, 670355988, 886437985, 224430896, 126139936, -477101480, -868159607,
                    -293937930
                ],
                vec_vec![
                    [333473422, 7],
                    [912209329, 7],
                    [468372740, 7],
                    [-765827269, 4],
                    [155827122, 4],
                    [635462096, 2],
                    [-300275936, 2],
                    [-115627659, 0]
                ]
            ),
            509199280
        );
        assert_eq!(
            Solution::minimum_total_distance(
                vec![
                    9486709, 305615257, 214323605, 282129380, 763907021, -662831631, 628054452,
                    -132239126, 50488558, 381544523, -656107497, 810414334, 421675516, -304916551,
                    571202823, 478460906, -972398628, 325714139, -86452967, 660743346
                ],
                vec_vec![
                    [-755430217, 18],
                    [382914340, 2],
                    [977509386, 4],
                    [94299927, 9],
                    [32194684, 16],
                    [-371001457, 2],
                    [-426296769, 13],
                    [-284404215, 8],
                    [-421288725, 0],
                    [-893030428, 2],
                    [291827872, 17],
                    [-766616824, 8],
                    [-730172656, 17],
                    [-722387876, 1],
                    [510570520, 20],
                    [756326049, 7],
                    [-242350340, 14],
                    [6585224, 19],
                    [-173457765, 15]
                ]
            ),
            925405949
        );
    }
}
