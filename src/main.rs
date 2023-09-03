use networks_msc::run_scenarios::*;
use networks_msc::network_structure::*;
use networks_msc::output_files::*;
use std::error::Error;

fn  main() -> Result<(), Box<dyn Error>> {
    // let n: usize = 100_000;
    // // test_run_model(n, "1");
    // let partitions: Vec<usize> = vec![58*n/1000, 145*n/1000, 212*n/1000, 364*n/1000, 497*n/1000, 623*n/1000, 759*n/1000, 866*n/1000, n];
    // let mut result: Vec<Vec<Vec<f64>>> = Vec::new();
    // let mut sum: Vec<Vec<f64>> = vec![vec![0.0; partitions.len()]; partitions.len()];
    // for i in 0..1 {
    //     let (_, tmp) = NetworkStructure::new_multinomial_rand(n, &partitions, "1", true);
    //     result.push(tmp.2);
    // }
    // for x in result.iter() {
    //     for (i, y) in x.iter().enumerate() {
    //         for (j, z) in y.iter().enumerate() {
    //             sum[i][j] += *z;
    //         }
    //     }
    // }
    // // println!("{:?}", sum);
    // let csv: String = write_csv(sum)?;
    // write_to_csv_file("model_output_files/unconnected_stub_breakdown1.csv", &csv)?;
    // Ok(())
    // let (_, tmp) = NetworkStructure::new_multinomial_rand(n, &partitions, "1", true);
    // println!("{:?}", tmp.2);
    let mut ns: Vec<usize> = (1..100).map(|x| x*100).collect();
    ns.append(&mut (1..=20).map(|x| x*10_000).collect());
    // ns.append(&mut (1..=100).map(|x| x*10_000).collect());
    println!("{:?}", ns);
    test_time_complexity(ns)?;
    Ok(())
}