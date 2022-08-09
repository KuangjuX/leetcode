impl Solution {
    pub fn dfs(matrix: &Vec<Vec<i32>>, res: &mut Vec<i32>, index: usize) {
        let x_start = index;
        let x_end = matrix.len() - index - 1;
        let y_start = index;
        let y_end = matrix[0].len() - index - 1;
        if x_start > x_end || x_end > 10{
            return
        }
        if y_start > y_end || y_end > 10 {
            return
        }
        if x_start == x_end && y_start == y_end {
            res.push(matrix[x_start][y_start]);
            return
        }
        for i in y_start..y_end {
            res.push(matrix[x_start][i]);
        }
        for i in x_start..=x_end {
            println!("i: {}", i);
            res.push(matrix[i][y_end]);
        }
        if x_start != x_end {
            for i in (y_start..y_end).rev() {
                res.push(matrix[x_end][i]);
            }
        }
        if y_start != y_end {
            for i in ((x_start + 1)..x_end).rev() {
                res.push(matrix[i][y_start]);
            }
        }
        Self::dfs(matrix, res, index + 1);
    }
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        Self::dfs(&matrix, &mut res, 0);
        res
    }
}
