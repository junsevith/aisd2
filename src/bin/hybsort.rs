use std::fmt::Debug;
use std::{env, io};
use std::io::BufRead;

static mut COMPARES: i32 = 0;
static mut SWAPS: i32 = 0;
fn main() {
    let breakpoint = env::args().skip(1).next().expect("Za mało argumentów").parse::<usize>().expect("Niepoprawna wartość");
    println!("Podaj dane:");
    let mut array = io::stdin().lock().lines().next().unwrap().unwrap()
        .trim().split(" ").map(|x| x.parse::<i32>().expect("Niepoprawna wartość")).collect::<Vec<i32>>();

    let smol = if array.len() < 40 { true } else { false };
    let start;
    if smol {
        start = format!("start: {:?}", array);
        println!("{}", start)
    } else { start = String::new() };

    hybrid_sort(&mut array, breakpoint, smol);

    println!();
    if smol {
        println!("{}", start);
        println!("end: {:?}", array)
    }
    unsafe { println!("comparisons: {}, swaps: {}", COMPARES, SWAPS) }
}

pub fn hybrid_sort<T: Ord + Clone + Debug>(array: &mut [T], breakpoint: usize, smol: bool) {
    if array.len() <= breakpoint {
        for i in 1..array.len() {
            let mut j = i;
            while j > 0 && inc_comps() && array[j - 1] > array[j] {
                array.swap(j - 1, j);
                unsafe { SWAPS += 1; }
                j -= 1;
            }
            if smol { println!("step {}: {:?}", i, array) }
        }
    } else {
        let pivot = partition(array, smol);
        hybrid_sort(&mut array[0..=pivot], breakpoint, smol);
        hybrid_sort(&mut array[pivot + 1..], breakpoint, smol);
    }
}

fn partition<T: Ord + Clone + Debug>(array: &mut [T], smol: bool) -> usize {
    if smol {
        println!("Partition: {:?}", array);
    }
    let pivot = array[(array.len() - 1) / 2].clone();
    if smol {
        println!("Pivot: {:?}", pivot);
    }
    let mut i = 0;
    let mut j = array.len() - 1;
    loop {
        while inc_comps() && array[i] < pivot {
            i += 1;
        }
        while inc_comps() && array[j] > pivot {
            j -= 1;
        }
        if i >= j {
            if smol {
                println!("Done: {:?}", array);
            }
            return j;
        }
        array.swap(i, j);
        unsafe {
            SWAPS += 1;
        }
        // if smol {
        //     println!("Swapped: {:?}", array);
        // }
        i += 1;
        j -= 1;
    }
}

fn inc_comps() -> bool{
    unsafe {
        COMPARES += 1;
    }
    true
}