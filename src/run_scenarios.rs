use crate::network_properties;
use crate::network_properties::NetworkProperties;
use crate::network_structure;
use crate::network_structure::*;
use crate::output_files::*;
use crate::useful_functions::*;
use crate::run_model::*;
use std::error::Error;

pub fn test_run_model(n: usize, period: &str) {
    let partitions: Vec<usize> = vec![58*n/1000, 145*n/1000, 212*n/1000, 364*n/1000, 497*n/1000, 623*n/1000, 759*n/1000, 866*n/1000, n];
    let (network_structure, _) = NetworkStructure::new_multinomial_rand(n, &partitions, period, true);
    let mut network_properties = NetworkProperties::new(&network_structure, &vec![0.1, 5.0, 2.0, 100.0]);
    let maxtime = 100;
    let initially_infected = 0.1;
    run_model(&network_structure, &mut network_properties, maxtime, initially_infected);
    // println!("{}", network_properties.secondary_cases.iter().sum() / network_properties.nodal_states.iter().map(f))
}

pub fn test_error_of_stubbing(ns: Vec<usize>) -> Result<(), Box<dyn Error>> {
    let period: &str = "1";
    let mut results: Vec<Vec<MeanVar>> = vec![Vec::new(); 2];
    for (i, n) in ns.iter().enumerate() { 
        let partitions: Vec<usize> = vec![58*n/1000, 145*n/1000, 212*n/1000, 364*n/1000, 497*n/1000, 623*n/1000, 759*n/1000, 866*n/1000, *n];
        println!("{}", i);
        let mut descend_tmp: Vec<(usize,usize)> = Vec::new(); 
        let mut ascend_tmp: Vec<(usize,usize)> = Vec::new();
        for _ in 0..30 {
            let (_, tmp) = NetworkStructure::new_multinomial_rand(n.clone(), &partitions, period,true);
            descend_tmp.push((tmp.0,tmp.1))
        }
        for _ in 0..30 {
            let (_, tmp) = NetworkStructure::new_multinomial_rand(n.clone(), &partitions, period,false);
            ascend_tmp.push((tmp.0,tmp.1))
        }
        let mut proportions: Vec<Vec<f64>> = Vec::new();
        proportions.push(descend_tmp.iter().map(|&(x,y)| (y as f64)/(x as f64)).collect::<Vec<f64>>());
        proportions.push(ascend_tmp.iter().map(|&(x,y)| (y as f64)/(x as f64)).collect::<Vec<f64>>());
        results[0].push(MeanVar::new(mean(&proportions[0]).unwrap(), variance(&proportions[0]).unwrap()));
        results[1].push(MeanVar::new(mean(&proportions[1]).unwrap(), variance(&proportions[1]).unwrap()));
    }
    
    write_json_mean_var(results)?;
    
    Ok(())
}