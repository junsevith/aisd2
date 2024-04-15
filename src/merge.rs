use std::cmp::{min, Ordering};
use std::fmt::Debug;
use bst_rs::{BinarySearchTree, IterativeBST};

pub fn merge_sort<T: Ord + Clone>(array: &[T]) -> (Vec<T>, usize) {
    let n = array.len();
    if n <= 1 {
        return (array.to_vec(), 0);
    }

    let mid = n / 2;
    let (left, right) = array.split_at(mid);
    let left = merge_sort(left);
    let right = merge_sort(right);
    let res = merge(&left.0, &right.0);
    (res.0, res.1 + left.1 + right.1)
}

pub fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> (Vec<T>, usize) {
    let mut comps = 0;
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < left.len() || j < right.len() {
        if j == right.len() || (i != left.len() && {
            comps += 1;
            left[i] <= right[j]
        }) {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }
    (result, comps)
}

pub fn my_sort<T: Ord + Clone>(array: &[T]) -> (Vec<T>, usize) {
    let parts = partition(array);
    let res = merge_parts(&parts.0);
    (res.0, res.1 + parts.1)
}

pub fn partition<T: Ord + Clone>(array: &[T]) -> (Vec<&[T]>, usize) {
    let mut comps = 0;
    let mut parts = Vec::new();
    let mut start = 0;
    for i in 1..array.len() {
        if {
            comps += 1;
            array[i] < array[i - 1]
        } {
            parts.push(&array[start..i]);
            start = i;
        }
    }
    parts.push(&array[start..]);
    (parts, comps)
}

pub fn merge_parts<T: Ord + Clone>(parts: &[&[T]]) -> (Vec<T>, usize) {
    let length = parts.len();
    return if length == 1 {
        (parts[0].to_vec(), 0)
    } else if length == 2 {
        merge(parts[0], parts[1])
    } else {
        let mid = length / 2;
        let (left, right) = parts.split_at(mid);
        let left = merge_parts(left);
        let right = merge_parts(right);
        let res = merge(&left.0, &right.0);
        (res.0, res.1 + left.1 + right.1)
    };
}

pub fn shuffle_sort<T: Ord + Clone>(array: &[T]) -> (Vec<T>, usize) {
    let parts = partition(array);
    let res = merge_parts2(&parts.0);
    (res.0, res.1 + parts.1)
}

fn merge_parts2<T: Ord + Clone>(parts: &[&[T]]) -> (Vec<T>, usize) {
    let mut comps = 0;
    let mut merged = Vec::new();
    for i in (2..parts.len()).step_by(4) {
        let el = merge(parts[i], parts[i - 2]);
        merged.push(el.0);
        comps += el.1;
    }
    for i in (3..parts.len()).step_by(4) {
        let el = merge(parts[i], parts[i - 2]);
        merged.push(el.0);
        comps += el.1;
    }

    let len = parts.len();
    let rest = len % 4;


    match rest {
        0 => {}
        1 => {
            let last = merge(parts[len - 1], merged.pop().unwrap().as_slice());
            merged.push(last.0);
            comps += last.1;
        }
        2 => {
            let el = merge(parts[len - 2], parts[len - 1]);
            merged.push(el.0);
            comps += el.1;
        }
        3 => {
            let last = merge(parts[len - 2], merged.pop().unwrap().as_slice());
            merged.push(last.0);
            comps += last.1;
        }
        _ => {}
    }


    // let len = merged.len();
    // return if len >= 2 {
    //     let as_slice = &merged.iter().map(|x| x.as_slice()).collect::<Vec<&[T]>>();
    //     let res = merge_parts2(as_slice);
    //     (res.0, res.1 + comps)
    // } else {
    //     (merged.pop().unwrap(), comps)
    // }
    merge_parts(&merged.iter().map(|x| x.as_slice()).collect::<Vec<&[T]>>())
}

pub fn part_sort<T: Ord + Clone + Debug>(array: &[T]) -> (Vec<T>, usize) {
    let parts = tree_sort(array);
    let res = merge_parts(parts.0.as_slice());
    (res.0, res.1 + parts.1)
}


pub fn powersort<T: Ord + Clone + Debug>(array: &[T]) -> (Vec<T>, usize) {
    let mut comps = 0;

    let mut runs: Vec<(Vec<T>, usize)> = Vec::new();
    let mut start = 0;
    let length = array.len();
    for i in 1..=length {
        if i == length || {
            comps += 1;
            array[i] < array[i - 1]
        } {
            let new_part = array[start..i].to_vec();

            let power = match runs.last() {
                None => 0,
                Some((part, _)) => get_power(part.len(), new_part.len(), i, length)
            };

            start = i;

            while runs.len() > 1 && power <= runs.last().unwrap().1 {
                comps += merge_top2(&mut runs);
            }

            runs.push((new_part, power))
        }
    }
    while runs.len() >= 2 {
        comps += merge_top2(&mut runs);
    }
    (runs.pop().unwrap().0.to_vec(), comps)
}

fn merge_top2<T: Ord + Clone + Debug>(parts: &mut Vec<(Vec<T>, usize)>) -> usize {
    let part1 = parts.pop().unwrap();
    let part2 = parts.pop().unwrap();
    // println!("merging: {:?} and {:?}", part1, part2);
    let new_part = merge(&part1.0, &part2.0);
    let new_power = min(part1.1, part2.1);
    parts.push((new_part.0, new_power));
    new_part.1
}

fn get_power(len1: usize, len2: usize, boundary: usize, whole_len: usize) -> usize {
    let start1 = (boundary - len1) as f64;
    let start2 = boundary as f64;
    let len1 = len1 as f64;
    let len2 = len2 as f64;
    let n = whole_len as f64;
    let a = (start1 + (len1 / 2.0) - 1.0) / n;
    let b = (start2 + (len2 / 2.0) - 1.0) / n;
    let mut l = 0;
    while {
        let pow = 2f64.powi(l);
        (a * pow).floor() == (b * pow).floor()
    } {
        l += 1
    }
    l as usize
}

pub fn tree_sort<T: Ord + Clone + Debug>(array: &[T]) -> (Vec<&[T]>, usize) {
    #[derive(Debug)]
    struct Node<'s, G> {
        part: &'s [G],
        length: usize,
    }

    impl<'t, G: Clone> Node<'t, G> {
        fn new(part: &'t [G]) -> Node<'t, G> {
            Node {
                part,
                length: part.len(),
            }
        }
    }

    impl<G> Eq for Node<'_, G> {}
    impl<G> PartialEq<Self> for Node<'_, G> {
        fn eq(&self, other: &Self) -> bool {
            self.length == other.length
        }
    }
    impl<G> PartialOrd<Self> for Node<'_, G> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.length.cmp(&other.length))
        }
    }
    impl<G> Ord for Node<'_, G> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.length.cmp(&other.length)
        }
    }

    let mut comps = 0;
    let mut parts = IterativeBST::new();
    let mut start = 0;
    for i in 1..array.len() {
        if {
            comps += 1;
            array[i] < array[i - 1]
        } {
            parts.insert(Node::new(&array[start..i]));
            println!("inserted: {:?}", &array[start..i]);
            start = i;
        }
    }
    parts.insert(Node::new(&array[start..]));

    // let mut sortedparts = Vec::new();
    // let mut i = 0;
    // let mut next = true;
    // while next {
    //     next = false;
    //     let mut j = 0;
    //     while j < parts.len() {
    //         if parts[j].len() == i {
    //             sortedparts.push(parts.remove(j));
    //         } else {
    //             next = true;
    //             j += 1;
    //         }
    //     }
    //     i += 1;
    // }

    let parts = parts.asc_order_vec().iter().map(|x| x.part).collect::<Vec<&[T]>>();
    println!("{:?}", parts);
    (parts, comps)
}


//nie działa :(
fn quick_merge<T: Ord>(array: &mut [T], breakpoint: usize) {
    let size = array.len();
    let mut insert = 0;
    let mut left = 0;
    let mut right = breakpoint;
    while left >= insert && left < right {
        if right != size && array[right] < array[left] {
            array.swap(insert, right);
            right += 1;
        } else {
            array.swap(insert, left);
            if right != size {
                left += 1;
            }
        }

        if left == insert {
            left = right - 1;
        } else if left == right {
            left = right - 1;
        }

        insert += 1;
    }
}
