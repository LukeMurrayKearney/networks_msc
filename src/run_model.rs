use rand::Rng;
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Poisson};
use crate::network_structure::*;
use crate::network_properties::*;

// pub fn run_model_parallel(network_structure: &NetworkStructure, network_properties: &mut NetworkProperties, maxtime: usize, initially_infected: f64, iterations: usize) {

// }

pub fn run_model(network_structure: &NetworkStructure, network_properties: &mut NetworkProperties, maxtime: usize, initially_infected: f64) {
    network_properties.initialize_infection(initially_infected);
    let mut rng: ThreadRng = rand::thread_rng();
    for _ in 0..maxtime {
        step_model(network_structure, network_properties, &mut rng);
        if network_properties.results.last().unwrap()[1] + network_properties.results.last().unwrap()[2] == 0 {
            break;
        }
    }
}

fn step_model(network_structure: &NetworkStructure, network_properties: &mut NetworkProperties, rng: &mut ThreadRng) {
    let mut next_states: Vec<State> = vec![State::Susceptible; network_structure.degree.len()];
    let poisson_infectious_period = Poisson::new(network_properties.parameters[2]).unwrap();
    let poisson_exposed_period = Poisson::new(network_properties.parameters[1]).unwrap();
    let poisson_recovered_period = Poisson::new(network_properties.parameters[3]).unwrap();
    for (i, state) in network_properties.nodal_states.iter().enumerate() {
        match *state {
            State::Susceptible => (),
            State::Exposed(days) => {
                if days <= 0 {
                    next_states[i] = State::Infected(poisson_infectious_period.sample(rng).round() as usize);
                }
                else {
                    next_states[i] = State::Exposed(days - 1);
                }
            },
            State::Infected(days) => {
                if days <= 0 {
                    next_states[i] = State::Recovered(poisson_recovered_period.sample(rng).round() as usize);
                }
                else {
                    next_states[i] = State::Infected(days - 1);
                }
                // find connections to infected individuals
                for link in network_structure.adjacency_matrix[i].iter() {
                    match network_properties.nodal_states[link.j] {
                        State::Susceptible => {
                            if rng.gen::<f64>() < network_properties.parameters[0] {
                                next_states[link.j] = State::Exposed(poisson_exposed_period.sample(rng).round() as usize);
                                network_properties.secondary_cases[i] += 1;
                            }
                        },
                        _ => ()
                    }
                }
            },
            State::Recovered(days) => {
                if days <= 0 {
                    next_states[i] = State::Susceptible;
                }
                else {
                    next_states[i] = State::Recovered(days - 1);
                }
            }
        }
    }
    network_properties.nodal_states = next_states;
    network_properties.results.push(network_properties.count_states());
}