impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut new_path: String = String::new();
        let mut stack: Vec<&str> = vec![];
        let p: Vec<&str> = path.split('/').collect();
        for dir in p {
            if dir == "" {
                continue;
            }else if dir == "." {
                continue;
            }else if dir == ".." {
                stack.pop();
            }else {
                stack.push(dir);
            }
        }
        if stack.len() == 0 {
            new_path.push('/');
        }
        for dir in stack {
            new_path.push('/');
            new_path.push_str(dir);
        }
        new_path
    }
}