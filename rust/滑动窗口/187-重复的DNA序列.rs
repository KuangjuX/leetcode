use std::collections::HashMap;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut hash: HashMap<String, usize> = HashMap::new();
        let mut res: Vec<String> = vec![];
        let s = s.as_bytes();
        let mut slow = 0;
        let mut fast = 10;
        if s.len() < 10 {
            return res
        }
        let mut s1 = unsafe{ String::from_utf8_unchecked(s[slow..fast].to_vec()) };
        hash.insert(s1, 1);
        while fast < s.len() {
            slow += 1;
            fast += 1;
            s1 = unsafe{ String::from_utf8_unchecked(s[slow..fast].to_vec()) };
            match hash.get(&s1) {
                None => {
                    hash.insert(s1.clone(), 1);
                },

                Some(1) => {
                    hash.insert(s1.clone(), 2);
                    res.push(s1);
                },

                _ => {
                    continue;
                }
            }
        }
        res
    }
}