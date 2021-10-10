use std::cmp;
impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_by(|a, b|{
            if a[0] > b[0] {
                return cmp::Ordering::Less
            }else if a[0] == b[0] {
                if a[1] < b[1] {
                    return cmp::Ordering::Less
                } else {
                    return cmp::Ordering::Greater
                }
            }else {
                return cmp::Ordering::Greater
            }
        });
        let mut max_def = -1;
        let mut res = 0;
        for each in properties.iter() {
            if each[1] < max_def {
                res += 1;
            }else {
                max_def = each[1]
            }
        }
        res
    }
}