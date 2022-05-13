use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct QueryDataResponse {
    pub dosage: f32,
    pub dosage_count: i32,
}
