impl Solution {

    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num = num_courses as usize;
        let mut queue: Vec<usize> = Vec::new();
        let mut indegree: Vec<usize> = vec![0; num];
        let mut ans: Vec<i32> = Vec::new();
        for i in 0..prerequisites.len() {
            indegree[prerequisites[i][0] as usize] += 1;
        }

        for i in 0..num {
            if indegree[i] == 0 {
                queue.push(i);
            }
        }
        let mut count = queue.len();
        while queue.len() > 0 {
            let p = queue.pop().unwrap();
            ans.push(p as i32);
            for i in 0..prerequisites.len() {
                if prerequisites[i][1] == p as i32 {
                    indegree[prerequisites[i][0] as usize] -= 1;
                    if indegree[prerequisites[i][0] as usize] == 0 {
                        queue.push(prerequisites[i][0] as usize);
                        count += 1;
                    }
                }
            }
        }
        if count != num {
            return Vec::new()
        }
        ans
    }
}