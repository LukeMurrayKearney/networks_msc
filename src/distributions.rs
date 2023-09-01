use crate::read_in::*;
use statrs::distribution::{Poisson, Geometric};
use rand::{distributions::Distribution, rngs::ThreadRng};
use rand_distr::Binomial;

pub fn degrees_pois_geom(partitions: &Vec<usize>, group_sizes: &Vec<usize>, period: &str, rng: &mut ThreadRng) -> Vec<usize> {
    // import parameters to sample
    let dist_params: DistributionParameters  = match period {
        "1" => read_params_json("model_input_files/fixed_multinomial_params1.json"),
        _ => read_params_json("model_input_files/fixed_multinomial_params2.json")
        // "1" => read_params_json("model_input_files/multinomial_params1.json"),
        // _ => read_params_json("model_input_files/multinomial_params2.json")
    };
    let mut degrees: Vec<usize> = Vec::new();
    // assign degrees for each individual
    for (i, _) in partitions.iter().enumerate() {
        let poisson = Poisson::new(dist_params.lambda[i][0]);
        let geometric = Geometric::new(dist_params.p_geom[i][0]);
        let p = dist_params.p[i][0];
        degrees.append(&mut (0..group_sizes[i])
            .map(|_| {
                (p*poisson.as_ref().unwrap().sample(rng) + (1.0-p)*geometric.as_ref().unwrap().sample(rng)).round() as usize
            })
            .collect());
    }
    degrees
}

pub fn multinomial_sample(n: usize, ps: &Vec<f64>, rng: &mut ThreadRng) -> Vec<usize> {
    let mut x_sum: usize = 0;
    let mut p_sum: f64 = 0.0;
    ps.iter()
        .map(|&p| {
            // check if p or n equal 0
            if n - x_sum == 0 || p == 0.0 {
                return 0
            }
            let mut p_cond: f64 = p / (1.0 - p_sum);
            if p_cond > 1.0 {
                p_cond = 1.0;
            }
            // make each binomial sample conditional on the last
            let bin = Binomial::new((n - x_sum) as u64, p_cond).unwrap();
            p_sum += p;
            let x: usize = bin.sample(rng) as usize;
            x_sum += x;
            x
        })
        .collect()
}

pub fn rates_to_row_probabilities(rates_mat: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    rates_mat
        .iter()
        .map(|row| {
            let sum_row: f64 = row.iter().sum();
            row.iter().map(|&val| val/sum_row).collect()
        })
        .collect()
}

pub fn rates_to_probabilities(rates_mat: Vec<Vec<f64>>, partitions: &Vec<usize>) -> Vec<Vec<f64>> {
    
    // find consecutive group sizes to turn rates to probabilities
    let mut group_sizes: Vec<usize> = partitions
        .windows(2)
        .map(|pair| {
            pair[1] - pair[0]
        })
        .collect();
    group_sizes.insert(0,partitions[0]);
    
    // transform rates matrix to probability matrix 
    rates_mat
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().map(|rate| {
                rate / (group_sizes[i] as f64)
            })
            .collect()
        })
        .collect()
}
