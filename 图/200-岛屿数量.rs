const X: [i32; 4] = [1, -1, 0, 0];
const Y: [i32; 4] = [0, 0, 1, -1];

impl Solution {
    pub fn dfs(x: i32, y: i32, grid: &Vec<Vec<char>>, visted: &mut Vec<Vec<bool>>) {
        let row = grid.len();
        let col = grid[0].len();
        for i in 0..4 {
            let dx = x + X[i];
            let dy = y + Y[i];
            if dx < 0 || dy < 0 || dx >= row as i32 || dy >= col as i32 {
                continue;
            }else if grid[dx as usize][dy as usize] == '0' || visted[dx as usize][dy as usize] == true{
                continue;
            }else {
                visted[dx as usize][dy as usize] = true;
                Self::dfs(dx, dy, grid, visted);
            }
        }
        return
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut res = 0;
        let row = grid.len();
        let col = grid[0].len();
        let mut visted = vec![vec![false; col]; row];
        for i in 0..row {
            for j in 0..col {
                if visted[i][j] == false && grid[i][j] == '1' {
                    Self::dfs(i as i32, j as i32, &grid, &mut visted);
                    res += 1;
                }
            }
        }
        res
    }
}