pub fn mean(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        None
    } else {
        let sum: f64 = data.iter().sum();
        Some(sum / data.len() as f64)
    }
}

pub fn variance(data: &[f64]) -> Option<f64> {
    let len = data.len();
    if len < 2 {
        None
    } else {
        let mean = mean(data)?;
        let variance: f64 = data.iter().map(|value| {
            let diff = *value - mean;
            diff * diff
        }).sum();
        Some(variance / len as f64)
    }
}