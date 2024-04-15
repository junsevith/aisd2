use std::fmt::Debug;
use rand::prelude::*;

pub fn random_array(size: usize, rng: &mut impl Rng) -> Vec<usize> {
    let mut arr = sorted_array(size);
    arr.shuffle(rng);
    return arr;
}

pub fn sorted_array(size: usize) -> Vec<usize> {
    (1..=size).collect::<Vec<usize>>()
}

pub fn reverse_array(size: usize) -> Vec<usize> {
    (1..=size).rev().collect::<Vec<usize>>()
}


pub fn insertion_sort<T: Ord + Debug>(array: &mut [T]) -> (usize, usize) {
    let smol = if array.len() < 40 { true } else { false };
    // let start;
    // if smol {
    //     start = format!("start: {:?}", array);
    //     println!("{}", start)
    // } else { start = String::new() }

    let mut comps = 0;
    let mut swaps = 0;


    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && {
            comps += 1;
            true
        } && array[j - 1] > array[j] {
            array.swap(j - 1, j);
            swaps += 1;
            j -= 1;
        }
        // if smol { println!("step {}: {:?}", i, array) }
    }

    // if smol {
    //     println!("{}", start);
    //     println!("end: {:?}", array)
    // } else {
    //     println!("comparisons: {}, swaps: {}", comps, swaps)
    // }
    (comps, swaps)
}

pub fn quick_sort<T: Ord + Clone + Debug>(array: &mut [T]) -> (usize, usize) {
    let smol = if array.len() < 40 { true } else { false };
    quick_sort_helper(array, smol)
}

pub fn quick_sort_helper<T: Ord + Clone + Debug>(array: &mut [T], smol: bool) -> (usize, usize) {
    if array.len() <= 1 {
        return (0, 0);
    }

    let pivot = partition(array, smol);
    let uno = quick_sort_helper(&mut array[0..=pivot.0], smol);
    let dos = quick_sort_helper(&mut array[pivot.0 + 1..], smol);
    (pivot.1 + uno.0 + dos.0, pivot.2 + uno.1 + dos.1)
}

fn partition<T: Ord + Clone + Debug>(array: &mut [T], smol: bool) -> (usize, usize, usize) {
    let mut comps = 0;
    let mut swaps = 0;
    // if smol {
    //     println!("Partition: {:?}", array);
    // }
    let pivot = array[(array.len() - 1) / 2].clone();
    // if smol {
    //     println!("Pivot: {:?}", pivot);
    // }
    let mut i = 0;
    let mut j = array.len() - 1;
    loop {
        while {
            comps += 1;
            array[i] < pivot
        } {
            i += 1;
        }
        while {
            comps += 1;
            array[j] > pivot
        } {
            j -= 1;
        }
        if i >= j {
            // if smol {
            //     println!("Done: {:?}", array);
            // }
            return (j, comps, swaps);
        }
        swaps += 1;
        array.swap(i, j);
        // if smol {
        //     //println!("Swapped: {:?}", array);
        // }
        i += 1;
        j -= 1;
    }
}

pub fn hybrid_sort<T: Ord + Clone + Debug>(array: &mut [T], breakpoint: usize) -> (usize, usize) {
    let smol = if array.len() < 40 { true } else { false };
    hybrid_sort_helper(array, breakpoint, smol)
}

pub fn hybrid_sort_helper<T: Ord + Clone + Debug>(array: &mut [T], breakpoint: usize, smol: bool) -> (usize, usize) {
    let mut swaps = 0;
    let mut comps = 0;
    if array.len() <= breakpoint {
        let res = insertion_sort(array);
        swaps += res.1;
        comps += res.0;
    } else {
        let pivot = partition(array, smol);
        let uno = hybrid_sort_helper(&mut array[0..=pivot.0], breakpoint, smol);
        let dos = hybrid_sort_helper(&mut array[pivot.0 + 1..], breakpoint, smol);
        swaps += pivot.2 + uno.1 + dos.1;
        comps += pivot.1 + uno.0 + dos.0;
    }
    (comps, swaps)
}
