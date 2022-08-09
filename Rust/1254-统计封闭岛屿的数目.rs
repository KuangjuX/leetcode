const X: [i32; 4] = [1, -1, 0, 0];
const Y: [i32; 4] = [0, 0, 1, -1];

impl Solution {
    pub fn dfs(x: i32, y: i32, grid: &mut Vec<Vec<i32>>, is_closed: &mut bool) -> bool {
        let row = grid.len();
        let col = grid[0].len();
        for i in 0..4 {
            let dx = x + X[i];
            let dy = y + Y[i];
            if dx < 0 || dy < 0 || dx >= row as i32 || dy >= col as i32 {
                *is_closed = false;
                continue;
            }else if grid[dx as usize][dy as usize] == 1{
                continue;
            }else {
                grid[dx as usize][dy as usize] = 1;
                Self::dfs(dx, dy, grid, is_closed);
            }
        }
        *is_closed
    }

    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let row = grid.len();
        let col = grid[0].len();
        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == 0 {
                    let mut is_closed = true;
                    Self::dfs(i as i32, j as i32, &mut grid, &mut is_closed);
                    if is_closed {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}