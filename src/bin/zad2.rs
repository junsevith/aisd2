use lista2::sort;
use lista2::chart;

pub fn main(){
    let k = 10;
    let nrange = (1000..=50000).step_by(1000);
    let mut rng = rand::thread_rng();
    let mut swapdata = Vec::new();
    let mut compdata = Vec::new();

    for n in nrange.clone() {
        println!("n: {}", n);
        let mut comps = (0, 0, 0);
        let mut swaps = (0, 0, 0);

        for _k in 0..k {
            let mut array = sort::random_array(n, &mut rng);
            // let result = insertion_sort(&mut array.clone());
            // comps.0 += result.0;
            // swaps.0 += result.1;

            let result = sort::quick_sort(&mut array.clone());
            comps.1 += result.0;
            swaps.1 += result.1;

            let result = sort::hybrid_sort(&mut array.clone(), 3);
            comps.2 += result.0;
            swaps.2 += result.1;
        }

        let k = k as f64;
        compdata.push((comps.0 as f64 / k, comps.1 as f64 / k, comps.2 as f64 / k));
        swapdata.push((swaps.0 as f64 / k, swaps.1 as f64 / k, swaps.2 as f64 / k));
    }
    let compn = chart::divide_series(compdata.clone(), nrange.clone());

    let swapn = chart::divide_series(swapdata.clone(), nrange.clone());


    chart::draw_chart(compdata, nrange.clone(), 0.0..1000000.0, "Comparisons".to_string());
    chart::draw_chart(swapdata, nrange.clone(), 0.0..200000.0, "Swaps".to_string());
    chart::draw_chart(compn, nrange.clone(), 0.0..30.0, "Comparisons per n".to_string());
    chart::draw_chart(swapn, nrange.clone(), 0.0..5.0, "Swaps per n".to_string());
}