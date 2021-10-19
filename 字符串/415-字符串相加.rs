impl Solution {
    pub fn add_strings(mut num1: String, mut num2: String) -> String {
        let mut flag = 0;
        let mut res: String = String::new();
        loop {
            match(num1.pop(), num2.pop()) {
                (Some(n1), Some(n2)) => {
                    let val1 = n1 as usize - '0' as usize;
                    let val2 = n2 as usize - '0' as usize;
                    let val = (val1 + val2 + flag) % 10;
                    flag = (val1 + val2 + flag) / 10;
                    let s = val.to_string();
                    res = format!("{}{}", s, res);
                },
    
                (Some(n1), None) => {
                    let val1 = n1 as usize - '0' as usize;
                    let val = (val1 + flag) % 10; 
                    flag = (val1 + flag) / 10;
                    let s = val.to_string();
                    res = format!("{}{}", s, res);
                },
    
                (None, Some(n2)) => {
                    let val2 = n2 as usize - '0' as usize;
                    let val = (val2 + flag) % 10; 
                    flag = (val2 + flag) / 10;
                    let s = val.to_string();
                    res = format!("{}{}", s, res);
                },
    
                (None, None) => {
                    if flag != 0 {
                        res = format!("{}{}", flag, res);
                    }
                    break;
                }
            }
        }
        res
    }
}