// In this we need the order of the elements in the array does not matter
// so put all in set to eleminate duplicate, then find the starting or
// the ending number of the a sequence then keep on adding or subtracting
// 1 until the sequence end, record its value in a longest variable

// Here we find the starting of the sequence
pub fn longest_consecutive(nums: vec<i32>) -> i32 {
    let (set, mut longest) = (nums.into_iter().collect::<std::collections::hashset<i32>>(), 0);
    for n in set.iter() {
        if !set.contains(&(n - 1)){
            let mut len = 1;
            while set.contains(&(n + len)) { len += 1; } 
            longest = longest.max(len);
        }
    }
    longest
}

// Here we find the ending of the sequence
pub fn longest_consecutive(nums: vec<i32>) -> i32 {
    let (set, mut longest) = (nums.into_iter().collect::<std::collections::hashset<i32>>(), 0);
    for n in set.iter() {
        if !set.contains(&(n + 1)){
            let mut len = 1;
            while set.contains(&(n - len)) { len += 1; } 
            longest = longest.max(len);
        }
    }
    longest
}


// Hence two ways to solve this problem either find the starting number then
// go the max by adding one, 
// find the ending number and go the min by subtracting one.
