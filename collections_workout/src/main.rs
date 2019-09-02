// Given a list of integers, use a vector and return the mean
// (the average value), median (when sorted, the value in the
// middle position), and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.

use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    fn mean(v: &Vec<i32>)  {
        let mut sum = 0;

        for i in v {
            sum += i;
        }

        let result = sum as f32 / v.len() as f32;

        println!("Mean is {:?}", result);
    }

    fn median(v: &Vec<i32>)  {
        let mut newvec = v.to_vec();

        newvec.sort();

        let result: f32;

         if newvec.len() % 2 == 0 {
            let mid_index = (newvec.len() / 2) - 1;
            let next_index = mid_index + 1;
            let med_sum = v[mid_index] + v[next_index];
            result = med_sum as f32 / 2.0;
        } else {
            let midindex = newvec.len() / 2;
            result = newvec[midindex] as f32;
        }

        println!("Median is {}", result);
    }

    fn mode(v: &Vec<i32>)  {
        let mut map = HashMap::new();

        for i in v {
            let count = map.entry(i).or_insert(0);
            *count += 1;
        }

        let mut mode_key = 0;
        let mut mode_value = 0;

        for (key, value) in &map {
            match value.cmp(&mode_value) {
                Ordering::Less => {},
                Ordering::Greater => {
                  mode_key = **key; // why **?
                  mode_value = *value;
                },
                Ordering::Equal => {},
            }
        }

        println!("Mode is {}", mode_key);
    }

    let v = vec![2,3,4,5,5,5];

    mode(&v);
    median(&v);
    mean(&v);
}
