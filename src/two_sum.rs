//https://leetcode.com/problems/two-sum/

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  if nums.len() < 2 {
    return Vec::new() as Vec<i32>;
  }

  for i in 0..(nums.len()-1) {
    for j in (i+1)..nums.len() {
      if (nums[i] + nums[j]) == target {
        return vec![i as i32, j as i32];
      }
    }
  }

  Vec::new() as Vec<i32>
}

#[test]
fn two_sum_test() {
  assert_eq!([0, 1].to_vec(), two_sum(vec![1, 2, 3], 3));
  assert_eq!([2, 3].to_vec(), two_sum(vec![1, 2, 3, 4], 7));
  assert_eq!(Vec::new() as Vec<i32>, two_sum(vec![1], 3));
  assert_eq!([3, 4].to_vec(), two_sum(vec![1, 3, 5, 7, 9], 16));
  assert_eq!(Vec::new() as Vec<i32>, two_sum(vec![1, 3, 5, 7, 9], 20));
}