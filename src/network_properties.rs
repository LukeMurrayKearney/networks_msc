use crate::network_structure::NetworkStructure;
use rand::prelude::SliceRandom;

#[derive(Clone, Debug)]
pub struct NetworkProperties {
    pub nodal_states: Vec<State>,
    pub parameters: Vec<f64>,
    pub results: Vec<Vec<usize>>,
    pub secondary_cases: Vec<usize>
}

#[derive(Clone,Debug)]
pub enum State {
    Susceptible,
    Exposed(usize),
    Infected(usize),
    Recovered(usize)
}

impl NetworkProperties {

    pub fn new(network: &NetworkStructure, params: &Vec<f64>) -> NetworkProperties {
        NetworkProperties {
            nodal_states: vec![State::Susceptible; network.degree.len()],
            parameters: params.clone(),
            results: Vec::new(),
            secondary_cases: vec![0; network.degree.len()]
        }
    }

    pub fn initialize_infection(&mut self, proportion_of_population: f64) {
        let number_of_infecteds: usize = match proportion_of_population as usize {
            0..=1 => {
                ((self.nodal_states.len() as f64) * proportion_of_population) as usize
            },
            _ => {
                println!("The proportion infected must be between 0 and 1");
                0
            }
        };
        // define random number generator
        let mut rng = rand::thread_rng();
        // shuffle indices and choose
        let mut indices: Vec<usize> = (0..self.nodal_states.len()).collect();
        indices.shuffle(&mut rng);
        for i in 0..number_of_infecteds {
            self.nodal_states[indices[i]] = State::Infected(0)
        }
        self.results.push(self.count_states());
    }

    pub fn count_states(&self) -> Vec<usize> {
        let mut result: Vec<usize> = vec![0; 4];
        for state in self.nodal_states.iter() {
            match state {
                State::Susceptible => result[0] += 1,
                State::Exposed(_) => result[1] += 1,
                State::Infected(_) => result[2] += 1,
                State::Recovered(_) => result[3] += 1
            }
        }
        result
    }

}