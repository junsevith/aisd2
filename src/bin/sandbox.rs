use lista2::{merge, sort, double_pivot};
use lista2::merge::partition;

fn main() {
    // let array = [24,25,26,27,28,21,22,23,18,19,20,4,5,6,7,8,9,10,11,12,13,15,16,17,3,1,2];
    let rng = &mut rand::thread_rng();

    let range = (1000..50000).step_by(1000);

    let mut avg_min = 0.0;
    for j in range.clone() {
        let mut array = sort::random_array(j, rng);

        let mut min = usize::MAX;
        let mut min_num = 0;
        for i in 1..50 {
            let res  = sort::hybrid_sort(&mut array.clone(), i);
            let sum = res.0 + res.1;
            if sum < min {
                min = sum;
                min_num = i;
            }
            // println!("{}: {:?}", i, res.0 + res.1);
        }
        println!("{}: {:?}", j, min_num);
        avg_min += min_num as f64;
    }
    avg_min /= (range.count() as f64);
    println!("Average: {}", avg_min);

}