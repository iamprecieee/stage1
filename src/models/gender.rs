use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GenderizeResponse {
    pub gender: Option<String>,
    #[serde(rename = "probability")]
    pub gender_probability: f64,
    #[serde(rename = "count")]
    pub sample_size: u64,
}
