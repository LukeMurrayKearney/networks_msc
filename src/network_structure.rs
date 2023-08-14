use rand::Rng;
// use crate::read_in::{read_params_json, DistributionParameters};
use rand::rngs::ThreadRng;
use crate::distributions::*;
use crate::read_in::read_rates_mat;
use crate::connecting_stubs::*;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::error::Error;


#[derive(Debug, Clone, Serialize)]
pub struct Link {
    pub i: usize,
    pub j: usize,
    pub weight: usize
}

#[derive(Debug, Clone, Serialize)]
pub struct NetworkStructure {
    pub adjacency_matrix: Vec<Vec<Link>>,
    pub degree: Vec<usize>,
    pub age_brackets: Vec<usize>
}

impl Link {
    pub fn new_link(i: usize, j: usize) -> Link {
        Link { i: i, j: j, weight: 1 }
    }
}

impl NetworkStructure {

    pub fn to_json(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(&self)?;
        let mut file = File::create(file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn new_multinomial_rand(n: usize, partitions: &Vec<usize>, period: &str, descending: bool) -> (NetworkStructure, (usize,usize)) {

        let mut rng: ThreadRng = rand::thread_rng();
        let mut edge_list: Vec<Vec<Link>> = vec![Vec::new(); n];
        let mut group_sizes: Vec<usize> = partitions
            .windows(2)
            .map(|pair| {
                pair[1] - pair[0]
            })
            .collect();
        group_sizes.insert(0,partitions[0]);
        // sample degrees from age degree distributions
        let mut degrees = degrees_pois_geom(&partitions, &group_sizes, period, &mut rng);
        // calculate p values for each partition to be used in multinomial
        let probs = match period {
            "1" => rates_to_row_probabilities(read_rates_mat("model_input_files/rates_matrix1.csv")),
            _ => rates_to_row_probabilities(read_rates_mat("model_input_files/rates_matrix2.csv"))
        };
        // assigning all stubs to age groups age groups
        let mut start: usize = 0;
        let mut degree_age_breakdown: Vec<Vec<usize>> = Vec::new();
        for (i, x) in partitions.iter().enumerate() {
            for j in start..*x {
                degree_age_breakdown.push(multinomial_sample(degrees[j], &probs[i], &mut rng));
            }
            start = *x;
        }
        let mut total_unconnected_stubs: usize = 0;
        //reset degrees
        degrees = vec![0;n];
        let mut start_i: usize = 0;
        for (part_i, &part_i_end) in partitions.iter().enumerate() {
            let mut start_j:usize = 0;
            // go through partitions again only lower triangular 
            for (part_j, &part_j_end) in partitions.iter().enumerate().take(part_i+1) {
                // all degrees of partition i with partition j and vice versa
                let nodes_i: Vec<(usize, usize)> = degree_age_breakdown
                    .iter()
                    .enumerate()
                    .skip(start_i)
                    .take(group_sizes[part_i])
                    .map(|(i, vec)| (i, vec[part_j]))
                    .collect();
                let nodes_j: Vec<(usize, usize)> = degree_age_breakdown
                    .iter()
                    .enumerate()
                    .skip(start_j)
                    .take(group_sizes[part_j])
                    .map(|(j, vec)| (j, vec[part_i]))
                    .collect();
                // connect stubs one partition at a time
                let (tmp_edges, missing_links): (Vec<(usize,usize)>, usize);
                if part_i == part_j {
                    match descending {
                        true => (tmp_edges, missing_links) = connect_stubs_diagonal_rand_descend(&nodes_i, &mut rng),
                        false => (tmp_edges, missing_links) = connect_stubs_diagonal_rand_ascend(&nodes_i, &mut rng)
                    }
                }
                else {
                    match descending {
                        true => (tmp_edges,missing_links) = connect_stubs_rand_descend(&nodes_i, &nodes_j, &mut rng),
                        false => (tmp_edges,missing_links) = connect_stubs_rand_ascend(&nodes_i, &nodes_j, &mut rng)
                    }
                }
                //count stubs that couldnt be joined
                total_unconnected_stubs += missing_links;
                // add edges to sparse matrix
                for pair in tmp_edges.iter() {
                    edge_list[pair.0].push(Link::new_link(pair.0, pair.1));
                    edge_list[pair.1].push(Link::new_link(pair.1, pair.0));
                    degrees[pair.0] += 1;
                    degrees[pair.1] += 1;
                }
                start_j = part_j_end;
            }
            start_i = part_i_end;
        }
        println!("connected: {}, unconnected: {}", degrees.iter().sum::<usize>(), total_unconnected_stubs);
        let connected: usize = degrees.iter().sum::<usize>();
        // define age brackets
        let mut last_idx = 0;
        let ages: Vec<usize> = partitions  
            .iter()
            .enumerate()
            .flat_map(|(i,x)| {
                let answer = vec![i; *x - last_idx];
                last_idx = *x;
                answer
            })
            .collect();

        (NetworkStructure {
            adjacency_matrix: edge_list,
            degree: degrees,
            age_brackets: ages
        },
        (connected, total_unconnected_stubs))
    }

    pub fn new_multinomial_assortative(n: usize, partitions: &Vec<usize>, period: &str) -> (NetworkStructure, (usize,usize)) {

        let mut rng: ThreadRng = rand::thread_rng();
        let mut edge_list: Vec<Vec<Link>> = vec![Vec::new(); n];
        let mut group_sizes: Vec<usize> = partitions
            .windows(2)
            .map(|pair| {
                pair[1] - pair[0]
            })
            .collect();
        group_sizes.insert(0,partitions[0]);
        // sample degrees from age degree distributions
        let mut degrees = degrees_pois_geom(&partitions, &group_sizes, period, &mut rng);
        // calculate p values for each partition to be used in multinomial
        let probs = match period {
            "1" => rates_to_row_probabilities(read_rates_mat("model_input_files/rates_matrix1.csv")),
            _ => rates_to_row_probabilities(read_rates_mat("model_input_files/rates_matrix2.csv"))
        };
        // assigning all stubs to age groups age groups
        let mut start: usize = 0;
        let mut degree_age_breakdown: Vec<Vec<usize>> = Vec::new();
        for (i, x) in partitions.iter().enumerate() {
            for j in start..*x {
                degree_age_breakdown.push(multinomial_sample(degrees[j], &probs[i], &mut rng));
            }
            start = *x;
        }
        let mut total_unconnected_stubs: usize = 0;
        //reset degrees
        degrees = vec![0;n];
        let mut start_i: usize = 0;
        for (part_i, &part_i_end) in partitions.iter().enumerate() {
            let mut start_j:usize = 0;
            // go through partitions again only lower triangular 
            for (part_j, &part_j_end) in partitions.iter().enumerate().take(part_i+1) {
                // all degrees of partition i with partition j and vice versa
                let nodes_i: Vec<(usize, usize)> = degree_age_breakdown
                    .iter()
                    .enumerate()
                    .skip(start_i)
                    .take(group_sizes[part_i])
                    .map(|(i, vec)| (i, vec[part_j]))
                    .collect();
                let nodes_j: Vec<(usize, usize)> = degree_age_breakdown
                    .iter()
                    .enumerate()
                    .skip(start_j)
                    .take(group_sizes[part_j])
                    .map(|(j, vec)| (j, vec[part_i]))
                    .collect();
                // connect stubs one partition at a time
                let (tmp_edges, missing_links): (Vec<(usize,usize)>, usize);
                if part_i == part_j {
                    (tmp_edges, missing_links) = connect_stubs_diagonal_assort(&nodes_i);
                }
                else {
                    (tmp_edges,missing_links) = connect_stubs_assort(&nodes_i, &nodes_j);
                }
                //count stubs that couldnt be joined
                total_unconnected_stubs += missing_links;
                // add edges to sparse matrix
                for pair in tmp_edges.iter() {
                    edge_list[pair.0].push(Link::new_link(pair.0, pair.1));
                    edge_list[pair.1].push(Link::new_link(pair.1, pair.0));
                    degrees[pair.0] += 1;
                    degrees[pair.1] += 1;
                }
                start_j = part_j_end;
            }
            start_i = part_i_end;
        }
        println!("connected: {}, unconnected: {}", degrees.iter().sum::<usize>(), total_unconnected_stubs);
        let connected: usize = degrees.iter().sum::<usize>();
        // define age brackets
        let mut last_idx = 0;
        let ages: Vec<usize> = partitions  
            .iter()
            .enumerate()
            .flat_map(|(i,x)| {
                let answer = vec![i; *x - last_idx];
                last_idx = *x;
                answer
            })
            .collect();

        (NetworkStructure {
            adjacency_matrix: edge_list,
            degree: degrees,
            age_brackets: ages
        },
        (connected, total_unconnected_stubs))
    }

    pub fn new_sbm(n: usize, partitions: &Vec<usize>, period: &str) -> NetworkStructure {
        let prob_mat = match period {
            "1" => rates_to_probabilities(read_rates_mat("model_input_files/rates_matrix1.csv"), partitions),
            _ => rates_to_probabilities(read_rates_mat("model_input_files/rates_matrix2.csv"), partitions)
        };
        let mut rng: ThreadRng = rand::thread_rng();
        let mut edge_list: Vec<Vec<Link>> = vec![Vec::new(); n];
        let mut degrees: Vec<usize> = vec![0; n];
        for i in 0..n {
            for j in 0..i {
                // find which block we are in
                let part_i = partitions
                    .iter()
                    .position(|&x| (i/x) < 1)
                    .unwrap();
                let part_j = partitions
                    .iter()
                    .position(|&x| (j/x) < 1)
                    .unwrap();
                // randomly generate edges with probability prob_mat
                if rng.gen::<f64>() < prob_mat[part_i][part_j] {
                    edge_list[i].push(Link::new_link(i, j));
                    edge_list[j].push(Link::new_link(j, i));
                    degrees[i] += 1;
                    degrees[j] += 1;
                }
            }
        }
        let mut last_idx = 0;
        let ages: Vec<usize> = partitions  
            .iter()
            .enumerate()
            .flat_map(|(i,x)| {
                let answer = vec![i; *x - last_idx];
                last_idx = *x;
                answer
            })
            .collect();
        NetworkStructure {
            adjacency_matrix: edge_list,
            degree: degrees,
            age_brackets: ages
        }
    }

}

