impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = vec![];
        for c in s.chars() {
            if c == ')' {
                let mut tmp = vec![];
                loop {
                    if let Some(a) = stack.pop() {
                        if a == '(' as u8 {
                            for ele in tmp {
                                stack.push(ele);
                            }
                            break;
                        }else {
                            tmp.push(a);
                        }
                    }
                }
            }else {
                stack.push(c as u8);
            }
        }
        String::from_utf8(stack).unwrap()
    }
}