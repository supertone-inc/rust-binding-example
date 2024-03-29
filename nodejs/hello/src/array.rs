#[napi]
pub mod array {
    #[napi]
    pub fn concat(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
        let a: Vec<f32> = a.into_iter().map(|v| v as f32).collect();
        let b: Vec<f32> = b.into_iter().map(|v| v as f32).collect();
        hello::array::concat(&a, &b)
            .into_iter()
            .map(|v| v as f64)
            .collect()
    }
}
