use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct QueryDataResponse {
    pub dosage: f32,
    pub dosage_count: i32,
}

#[derive(Serialize, Default)]
pub struct TemplateContext {
    pub current_dosage: f32,
    pub current_dosage_count: i32,
    pub recommended_dosage: f32,
}
