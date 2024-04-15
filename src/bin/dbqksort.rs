use std::fmt::Debug;
use std::io;
use std::io::BufRead;

fn main() {
    println!("Podaj dane:");
    let mut array = io::stdin().lock().lines().next().unwrap().unwrap()
        .trim().split(" ").map(|x| x.parse::<i32>().expect("Niepoprawna wartość")).collect::<Vec<i32>>();

    let smol = if array.len() < 40 { true } else { false };
    let start;
    if smol {
        start = format!("start: {:?}", array);
        println!("{}", start)
    } else { start = String::new() };

    let stats = dual_pivot_quicksort(&mut array, smol);

    println!();
    if smol {
        println!("{}", start);
        println!("end: {:?}", array)
    }
    unsafe { println!("comparisons: {}, swaps: {}", stats.0, stats.1) }
}

pub fn dual_pivot_quicksort<T: Ord + Debug>(array: &mut [T], smol: bool) -> (usize, usize) {
    let len = array.len();
    if len <= 1 {
        return (0, 0);
    }
    let (mut comps, mut swaps) = (0, 0);

    let ((p1, p2), out) = dual_pivot_partition(array, smol);
    swaps += out.1;
    comps += out.0;

    let out = dual_pivot_quicksort(&mut array[..p1], smol);
    swaps += out.1;
    comps += out.0;

    let out = dual_pivot_quicksort(&mut array[p1 + 1..p2], smol);
    swaps += out.1;
    comps += out.0;

    let out = dual_pivot_quicksort(&mut array[p2 + 1..], smol);
    swaps += out.1;
    comps += out.0;

    (comps, swaps)
}

fn dual_pivot_partition<T: Ord + Debug>(array: &mut [T], smol: bool) -> ((usize, usize), (usize, usize)) {
    if smol {
        println!("Partition: {:?}", array);
    }

    let mut comps = 1;
    let mut swaps = 0;
    let mut smaller = 0;
    let mut bigger = 0;
    let (left_pivot, right_pivot) = (0, array.len() - 1);

    if smol {
        println!("Pivots: {:?}, {:?}", array[left_pivot], array[right_pivot]);
    }

    if array[left_pivot] > array[right_pivot] {
        swaps += 1;
        array.swap(left_pivot, right_pivot);
    }

    let mut left = left_pivot + 1;
    let mut right = right_pivot - 1;
    let mut iter = left;

    while iter <= right {
        if bigger > smaller {
            if {
                comps += 1;
                array[iter] > array[right_pivot]
            } {
                array.swap(iter, right);
                swaps += 1;

                right -= 1;
                bigger += 1;
            } else if {
                comps += 1;
                array[iter] < array[left_pivot]
            } {
                array.swap(iter, left);
                swaps += 1;

                left += 1;
                iter += 1;
                smaller += 1
            } else {
                iter += 1;
            }
        } else {
            if {
                comps += 1;
                array[iter] < array[left_pivot]
            } {
                array.swap(iter, left);
                swaps += 1;

                left += 1;
                iter += 1;
                smaller += 1;
            } else if {
                comps += 1;
                array[iter] > array[right_pivot]
            } {
                array.swap(iter, right);
                swaps += 1;

                right -= 1;
                bigger += 1;
            } else {
                iter += 1;
            }
        }
    }

    left -= 1;
    right += 1;
    array.swap(left_pivot, left);
    array.swap(right_pivot, right);

    if smol {
        println!("Done: {:?}", array);
    }

    ((left, right), (comps, swaps))
}