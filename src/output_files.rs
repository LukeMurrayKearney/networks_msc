// use std::clone;
use std::fmt::Display;
use std::error::Error;
use csv::Writer;
use std::fs::File;
use std::io::Write;
use serde::Serialize;

pub fn write_json_mean_var(data: Vec<Vec<MeanVar>>) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string(&data)?;

    let mut file = File::create("model_output_files/mean_variance_stubbing1de.json")?;
    file.write_all(json.as_bytes())?;
    
    Ok(())
}

pub  fn write_csv<T: Display>(data: Vec<Vec<T>>) -> Result<String, Box<dyn Error>> {
    let mut wtr = Writer::from_writer(Vec::new());

    for row in data {
        let string_row: Vec<String> = row.into_iter().map(|item| format!("{}", item)).collect();
        wtr.write_record(&string_row)?;
    }

    wtr.flush()?;
    Ok(String::from_utf8(wtr.into_inner()?)?)
}

pub fn write_to_csv_file(filename: &str, contents: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

#[derive(Debug,Clone,Serialize)]
pub struct MeanVar {
    mean: f64,
    var: f64
} 

impl MeanVar {
    pub fn new(mean: f64, var: f64) -> MeanVar {
        MeanVar { mean: mean, var: var}
    }
}
