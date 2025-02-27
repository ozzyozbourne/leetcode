// In problem where we have to find top/botton of a frequency value there we can use a bucket sort 
// technique to bring the complexicity down from nlogk to n, with creating a 2d array of frequency 
// that fold maps the frequency to the indexes of the frequency array and each frequency[i] will contains 
// the elements that have the same frequency, for this to be feasible the lenght of the input array must be 
// not large, since the frequency for an element in an array that is bounded can only be from 1 ie a single 
// instance to n ie all the elements are of the same type, use using this approach the frequency array can 
// contains alot of empty 1D arrays ie a sparse matrix so for large size of the n this would take up alot 
// of memory then the min/max heap approach is best for this type of cases;

// memory analysis the bucket sort take more memory when not ignore the constants ie 
// N for dictionary ws, N for bucket ws, but all the value will be present at the 1st index 
// since all the elements in the dictionary were unique hence all have the same frequency ie 1
// plus the result arrar so this makes it N + N + K -> 2N + K => N
// For the same the min heap will be more effecient since it will have N + K + K -> N + 2K =>
// N + K

pub fn top_k_frequent_using_min_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

    let counter = nums.into_iter().fold(HashMap::<i32, usize>::new(), | mut acc, num | {
	*acc.entry(num).or_default() += 1;
	acc
    });

    let heap = counter.into_iter().fold(BinaryHeap::<Reverse<(usize, i32)>>::new(), | mut acc, (key, val) | {
	acc.push(Reverse((val, key)));
	if acc.len() > k as usize { _= acc.pop(); }
	acc
    });

    let mut res = heap.into_iter().fold(Vec::with_capacity(k as usize), | mut acc, Reverse((_, v)) | {
	acc.push(v);
	acc
    });

   res.reverse();
   res
}

pub fn top_k_frequent_using_bucket_sort(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let counter = nums.iter().fold(std::collections::HashMap::<i32, usize>::new(), | mut acc, &num | {
	*acc.entry(num).or_default() += 1;
	acc
    });

    let (mut freq, mut res) = (vec![vec![]; nums.len() + 1], Vec::with_capacity(k as usize));
    for (key, val) in counter.into_iter() { freq[val].push(key); } 

    while let Some(bucket) = freq.pop() {
	for num in bucket {
	    res.push(num);
	    if res.len() == k as usize { return res; }
	} 
    }
    res
}

fn main() {}
