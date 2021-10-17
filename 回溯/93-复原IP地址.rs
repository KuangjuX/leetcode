use std::str::FromStr;
impl Solution {
    pub fn backtrace(s: &String, res: &mut Vec<String>, tmp: &mut Vec<String>, index: &mut usize) {
        if tmp.len() == 4 && *index == s.len() {
            // 如果此时已经分完段并且没有剩余数字则将其push到结果中
            let mut ans = tmp[0].clone();
            for i in 1..4 {
                ans.push('.');
                ans.push_str(&tmp[i])
            }
            res.push(ans);
            return
        }else if (tmp.len() < 4 && *index == s.len()) || (tmp.len() == 4 && *index < s.len()){
            // 直接回溯
            return
        }
        for i in 1..=3 {
            if *index + i <= s.len() {
                let s1: String = String::from_str(&s[*index..(*index+i)]).unwrap();
                let num = s1.parse::<usize>().unwrap();
                if (i > 1 && num < 10) || (i > 2 && num < 100) {
                    continue;
                }
                if num <= 255 {
                    tmp.push(s1);
                    *index = *index + i;
                    Self::backtrace(s, res, tmp, index);
                    tmp.pop();
                    *index = *index -i;
                }else {
                    continue;
                }
            }
        }
    }

    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut res = vec![];
        let mut tmp = vec![];
        let mut index = 0;
        if s.len() < 4 || s.len() > 12 {
            return vec![]
        }
        Self::backtrace(&s, &mut res, &mut tmp, &mut index);
        res
    }
}