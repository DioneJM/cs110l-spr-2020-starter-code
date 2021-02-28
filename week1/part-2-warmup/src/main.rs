/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    v.iter().map(|&x| x + n).collect()
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for num in v.iter_mut() {
        *num = *num + n
    }
}

fn dedup(v: &mut Vec<i32>) {
    let mut dup_nums: HashSet<i32> = HashSet::new();
    let mut indices_to_remove: Vec<usize> = Vec::new();
    for (index, num) in v.iter_mut().enumerate() {
        if dup_nums.contains(num) {
            indices_to_remove.push(index);
        } else {
            dup_nums.insert(*num);
        }
    }

    for (cur_index, index_to_remove) in indices_to_remove.iter().enumerate() {
        v.remove(*index_to_remove - cur_index);
    }
}

fn dedup_simplified(v: &mut Vec<i32>) {
    let mut uniques: HashSet<i32> = HashSet::new();
    v.retain(|x| uniques.insert(*x))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
        assert_eq!(add_n(vec![1,2,3], 2), vec![3, 4, 5]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
