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

    pub fn multiply(mut num1: String, num2: String) -> String {
        let val_1 = num1.chars().nth(0).unwrap() as usize - '0' as usize;
        let val_2 = num2.chars().nth(0).unwrap() as usize - '0' as usize;
        if val_1 == 0 || val_2 == 0 {
            return String::from("0")
        }
        let mut res = String::new();
        res.push('0');
        for i in 0..num1.len() {
            let c1 = num1.pop().unwrap();
            let val1 = c1 as usize - '0' as usize;
            let mut flag = 0;
            let mut tmp = String::new();
            for _ in 0..i {
                tmp.push('0');
            }
            for j in 0..num2.len() {
                let c2 = num2.chars().nth(num2.len() - 1 - j).unwrap();
                let val2 = c2 as usize - '0' as usize;
                let val = (val1 * val2 + flag) % 10;
                flag = (val1 * val2 + flag) / 10;
                tmp = format!("{}{}", val, tmp);
            }
            if flag != 0 {
                tmp = format!("{}{}", flag, tmp);
            }
            res = Self::add_strings(res, tmp);
        }
        res
    }
}