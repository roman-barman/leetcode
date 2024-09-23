impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut index_1 = m - 1;
        let mut index_2 = n - 1;

        while index_2 >= 0 && index_1 >= 0{
            if nums2[index_2 as usize] < nums1[index_1 as usize] {
                nums1[(index_1 + index_2 +1) as usize] = nums1[index_1 as usize];
                index_1 = index_1 - 1;
            }
            else {
                let temp = nums1[index_1 as usize];
                nums1[(index_1 + index_2 +1) as usize] = nums2[index_2 as usize];
                index_2 = index_2 - 1;
            }
        }

        if index_2 >= 0 {
            for index in 0..=index_2 {
                nums1[index as usize] = nums2[index as usize];
            }
        }
    }
}