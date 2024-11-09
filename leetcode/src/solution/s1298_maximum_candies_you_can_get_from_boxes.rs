/**
 * [1298] Maximum Candies You Can Get from Boxes
 *
 * You have n boxes labeled from 0 to n - 1. You are given four arrays: status, candies, keys, and containedBoxes where:
 *
 * 	status[i] is 1 if the i^th box is open and 0 if the i^th box is closed,
 * 	candies[i] is the number of candies in the i^th box,
 * 	keys[i] is a list of the labels of the boxes you can open after opening the i^th box.
 * 	containedBoxes[i] is a list of the boxes you found inside the i^th box.
 *
 * You are given an integer array initialBoxes that contains the labels of the boxes you initially have. You can take all the candies in any open box and you can use the keys in it to open new boxes and you also can use the boxes you find in it.
 * Return the maximum number of candies you can get following the rules above.
 *  
 * Example 1:
 *
 * Input: status = [1,0,1,0], candies = [7,5,4,100], keys = [[],[],[1],[]], containedBoxes = [[1,2],[3],[],[]], initialBoxes = [0]
 * Output: 16
 * Explanation: You will be initially given box 0. You will find 7 candies in it and boxes 1 and 2.
 * Box 1 is closed and you do not have a key for it so you will open box 2. You will find 4 candies and a key to box 1 in box 2.
 * In box 1, you will find 5 candies and box 3 but you will not find a key to box 3 so box 3 will remain closed.
 * Total number of candies collected = 7 + 4 + 5 = 16 candy.
 *
 * Example 2:
 *
 * Input: status = [1,0,0,0,0,0], candies = [1,1,1,1,1,1], keys = [[1,2,3,4,5],[],[],[],[],[]], containedBoxes = [[1,2,3,4,5],[],[],[],[],[]], initialBoxes = [0]
 * Output: 6
 * Explanation: You have initially box 0. Opening it you can find boxes 1,2,3,4 and 5 and their keys.
 * The total number of candies will be 6.
 *
 *  
 * Constraints:
 *
 * 	n == status.length == candies.length == keys.length == containedBoxes.length
 * 	1 <= n <= 1000
 * 	status[i] is either 0 or 1.
 * 	1 <= candies[i] <= 1000
 * 	0 <= keys[i].length <= n
 * 	0 <= keys[i][j] < n
 * 	All values of keys[i] are unique.
 * 	0 <= containedBoxes[i].length <= n
 * 	0 <= containedBoxes[i][j] < n
 * 	All values of containedBoxes[i] are unique.
 * 	Each box is contained in one box at most.
 * 	0 <= initialBoxes.length <= n
 * 	0 <= initialBoxes[i] < n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-candies-you-can-get-from-boxes/
// discuss: https://leetcode.com/problems/maximum-candies-you-can-get-from-boxes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_candies(
        mut status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut res = 0;
        let mut st = std::collections::HashSet::new();
        let mut q = std::collections::VecDeque::new();

        for &i in &initial_boxes {
            q.push_back(i);
        }

        while !q.is_empty() {
            let mut sz = q.len();
            let mut flag = false;

            while sz > 0 {
                sz -= 1;
                let tmp = q.pop_front().unwrap();

                if status[tmp as usize] == 1 || st.contains(&(tmp as usize)) {
                    res += candies[tmp as usize];
                    flag = true;

                    for &key in &keys[tmp as usize] {
                        st.insert(key as usize);
                    }

                    for &box_id in &contained_boxes[tmp as usize] {
                        q.push_back(box_id);
                    }
                } else if status[tmp as usize] == 0 {
                    q.push_back(tmp);
                }
            }
            if !flag {
                break;
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1298() {
        time_test!();

        assert_eq!(
            Solution::max_candies(
                vec![1, 0, 1, 0],
                vec![7, 5, 4, 100],
                vec_vec![[], [], [1], []],
                vec_vec![[1, 2], [3], [], []],
                vec![0]
            ),
            16
        );
        assert_eq!(
            Solution::max_candies(
                vec![1, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1],
                vec_vec![[1, 2, 3, 4, 5], [], [], [], [], []],
                vec_vec![[1, 2, 3, 4, 5], [], [], [], [], []],
                vec![0]
            ),
            6
        );
    }
}
