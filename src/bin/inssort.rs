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
        if smol { println!("step {}: {:?}", i, array) }
    }

    println!();
    if smol {
        println!("{}", start);
        println!("end: {:?}", array)
    }
    println!("comparisons: {}, swaps: {}", comps, swaps)
}