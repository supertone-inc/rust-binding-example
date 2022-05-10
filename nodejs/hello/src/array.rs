#[napi]
mod array {
    #[napi]
    fn concat(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
        let a: Vec<f32> = a.iter().map(|v| *v as f32).collect();
        let b: Vec<f32> = b.iter().map(|v| *v as f32).collect();
        hello::array::concat(&a, &b)
            .iter()
            .map(|v| *v as f64)
            .collect()
    }
}
