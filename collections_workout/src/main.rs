// Given a list of integers, use a vector and return the mean
// (the average value), median (when sorted, the value in the
// middle position), and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.

use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    let v = vec![2,3,4,5,5,5];

    mode(&v);
    median(&v);
    mean(&v);
}

fn mean(v: &Vec<i32>) -> f32  {
    let mut sum = 0;

    for i in v {
        sum += i;
    }

    let result = sum as f32 / v.len() as f32;

    println!("Mean is {:?}", result);

    result
}

fn median(v: &Vec<i32>) -> f32  {
    let mut newvec = v.to_vec();

    newvec.sort();

    let result: f32;

     if newvec.len() % 2 == 0 {
        let mid_left_index = (newvec.len() / 2) - 1;
        let mid_right_index = mid_left_index + 1;
        let med_sum = v[mid_left_index] + v[mid_right_index];
        result = med_sum as f32 / 2.0;
    } else {
        let midindex = newvec.len() / 2;
        result = newvec[midindex] as f32;
    }

    println!("Median is {}", result);

    result
}

fn mode(v: &Vec<i32>) -> i32  {
    let mut map = HashMap::new();

    for i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut vec_item = 0;
    let mut vec_item_count = 0;

    for (key, value) in &map {
        match value.cmp(&vec_item_count) {
            Ordering::Less => {},
            Ordering::Greater => {
              vec_item = **key;
              vec_item_count = *value;
            },
            Ordering::Equal => {},
        }
    }

    println!("Mode is {}", vec_item);

    vec_item
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_returns_the_mean() {
        let mean1 = mean(&vec![1, 2, 3]);
        assert_eq!(mean1, 2.0);
        let mean2 = mean(&vec![1, 2]);
        assert_eq!(mean2, 1.5);
        let mean3 = mean(&vec![1]);
        assert_eq!(mean3, 1.0);
    }

    #[test]
    fn median_returns_the_median() {
        let median1 = median(&vec![1, 2, 3, 1, 3]);
        assert_eq!(median1, 2.0);
        let median2 = median(&vec![12, 3, 5]);
        assert_eq!(median2, 5.0);
        let median3 = median(&vec![1]);
        assert_eq!(median3, 1.0);
        let median4 = median(&vec![2, 1]);
        assert_eq!(median4, 1.5);
    }

    #[test]
    fn mode_returns_the_mode() {
        let mode1 = mode(&vec![1, 2, 3, 1]);
        assert_eq!(mode1, 1);
        let mode2 = mode(&vec![2, 1, 2, 3, 1, 2]);
        assert_eq!(mode2, 2);
        let mode3 = mode(&vec![1]);
        assert_eq!(mode3, 1);
    }
}
