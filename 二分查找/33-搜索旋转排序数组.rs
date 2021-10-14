impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0 as i32;
        let mut high = (nums.len() - 1) as i32;
        while low <= high && low >=0 && high >= 0{
            let mid = (low + high) / 2;
            if nums[mid as usize] == target {
                return mid
            }else if nums[mid as usize] < nums[high as usize] {
                // 右半部分有序
                if nums[mid as usize] < target && target <= nums[high as usize] {
                    low = mid + 1;
                }else {
                    high = mid - 1;
                }
            }else {
                // 左半部分有序
                if nums[low as usize] <= target && target < nums[mid as usize] {
                    high = mid - 1;
                }else {
                    low = mid + 1;
                }
            }
        }
        -1
    }
}