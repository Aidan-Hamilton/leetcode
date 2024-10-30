/**
 * [1697] Checking Existence of Edge Length Limited Paths
 *
 * An undirected graph of n nodes is defined by edgeList, where edgeList[i] = [ui, vi, disi] denotes an edge between nodes ui and vi with distance disi. Note that there may be multiple edges between two nodes.
 * Given an array queries, where queries[j] = [pj, qj, limitj], your task is to determine for each queries[j] whether there is a path between pj and qj such that each edge on the path has a distance strictly less than limitj .
 * Return a boolean array answer, where answer.length == queries.length and the j^th value of answer is true if there is a path for queries[j] is true, and false otherwise.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/08/h.png" style="width: 267px; height: 262px;" />
 * Input: n = 3, edgeList = [[0,1,2],[1,2,4],[2,0,8],[1,0,16]], queries = [[0,1,2],[0,2,5]]
 * Output: [false,true]
 * Explanation: The above figure shows the given graph. Note that there are two overlapping edges between 0 and 1 with distances 2 and 16.
 * For the first query, between 0 and 1 there is no path where each distance is less than 2, thus we return false for this query.
 * For the second query, there is a path (0 -> 1 -> 2) of two edges with distances less than 5, thus we return true for this query.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/08/q.png" style="width: 390px; height: 358px;" />
 * Input: n = 5, edgeList = [[0,1,10],[1,2,5],[2,3,9],[3,4,13]], queries = [[0,4,14],[1,4,13]]
 * Output: [true,false]
 * Explanation: The above figure shows the given graph.
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 10^5
 * 	1 <= edgeList.length, queries.length <= 10^5
 * 	edgeList[i].length == 3
 * 	queries[j].length == 3
 * 	0 <= ui, vi, pj, qj <= n - 1
 * 	ui != vi
 * 	pj != qj
 * 	1 <= disi, limitj <= 10^9
 * 	There may be multiple edges between two nodes.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/checking-existence-of-edge-length-limited-paths/
// discuss: https://leetcode.com/problems/checking-existence-of-edge-length-limited-paths/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {
    /*
        Possible Solutions(Taken from discussion):
        Use a HashMap<Integer, HashSet> to represent nodes as key, and all nodes connected to it as the value.
        Use a HashMap<String, List> to represent connections from node A to node B, stylized as key "A_B" and list of distances of edges between those two nodes.
        DFS from each start node in query to try and reach end node, and if you do , return true.

        sort the query and work with least weight as possible
        we have to use union find concept here
        only work with edges that is less than given limit - so smart but did not see this.
    */
    pub fn distance_limited_paths_exist(
        n: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }
        // Union by rank
        fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
            let root_x = find(parent, x);
            let root_y = find(parent, y);
            if root_x != root_y {
                if rank[root_x] > rank[root_y] {
                    parent[root_y] = root_x;
                } else if rank[root_x] < rank[root_y] {
                    parent[root_x] = root_y;
                } else {
                    parent[root_y] = root_x;
                    rank[root_x] += 1;
                }
            }
        }
        // Sort queries and edge_list by weight
        let mut queries_with_index: Vec<(usize, &Vec<i32>)> = queries.iter().enumerate().collect();
        queries_with_index.sort_by_key(|&(_, query)| query[2]);

        let mut edge_list = edge_list;
        edge_list.sort_by_key(|edge| edge[2]);

        let mut parent: Vec<usize> = (0..n as usize).collect();
        let mut rank: Vec<usize> = vec![0; n as usize];
        let mut result = vec![false; queries.len()];
        let mut edge_index = 0;

        // Iterate through queries and check if there is a path between the two nodes
        for (i, query) in queries_with_index {
            while edge_index < edge_list.len() && edge_list[edge_index][2] < query[2] {
                union(
                    &mut parent,
                    &mut rank,
                    edge_list[edge_index][0] as usize,
                    edge_list[edge_index][1] as usize,
                );
                edge_index += 1;
            }
            // Check if the two nodes are connected
            if find(&mut parent, query[0] as usize) == find(&mut parent, query[1] as usize) {
                result[i] = true;
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
    fn test_1697() {
        time_test!();

        assert_eq!(
            Solution::distance_limited_paths_exist(
                3,
                vec_vec![[0, 1, 2], [1, 2, 4], [2, 0, 8], [1, 0, 16]],
                vec_vec![[0, 1, 2], [0, 2, 5]]
            ),
            vec![false, true]
        );
        assert_eq!(
            Solution::distance_limited_paths_exist(
                5,
                vec_vec![[0, 1, 10], [1, 2, 5], [2, 3, 9], [3, 4, 13]],
                vec_vec![[0, 4, 14], [1, 4, 13]]
            ),
            vec![true, false]
        );
    }
}
