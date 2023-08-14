use serde::Deserialize;
use serde_json;
use std::io::Read;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct DistributionParameters {
    pub lambda: Vec<Vec<f64>>,
    pub p_geom: Vec<Vec<f64>>,
    pub p: Vec<Vec<f64>>
}

impl DistributionParameters {
    pub fn new() -> DistributionParameters {
        DistributionParameters { lambda: Vec::new(), p_geom: Vec::new(), p: Vec::new() }
    }
}

fn params_json(file_path: &str) -> Result<DistributionParameters, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let my_struct: DistributionParameters = serde_json::from_str(&content)?;

    Ok(my_struct)
}

pub fn read_params_json(file_path: &str) -> DistributionParameters {
    // let file_path = "model_input_files/fitting_parameters1.json";
    let my_struct = match params_json(&file_path) {
        Ok(my_struct) => my_struct,
        Err(err) => {
            eprintln!("Error: {}", err);
            DistributionParameters::new()
        }
    };
    my_struct
}

pub fn read_rates_mat(file_path: &str) -> Vec<Vec<f64>> {
    // let file_path = "model_input_files/rates_matrix.csv";
    let rates_mat = match read_csv_file(file_path) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error {}", err);
            Vec::new()
        }
    };
    rates_mat
}

fn read_csv_file(file_path: &str) -> Result<Vec<Vec<f64>>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut reader = csv::ReaderBuilder::new().from_reader(content.as_bytes());
    let mut data: Vec<Vec<f64>> = Vec::new();

    let headers = reader.headers()?.clone();
    data.push(headers.iter().map(|header| header.parse::<f64>().unwrap()).collect());

    for result in reader.records() {
        let record = result?;
        let row: Vec<f64> = record
            .iter()
            .map(|value| value.parse::<f64>())
            .collect::<Result<Vec<f64>, _>>()?;
        data.push(row);
    }

    Ok(data)
}