// This allows to convert json to MedicineTrackingInsert struct and insert into DB
use super::schema::medicince_tracking;
use diesel::{Insertable, Queryable};
use serde_derive::Serialize;

// use rocket::FromForm;
// use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Debug)]
pub struct MedicineTrackingQuery {
    pub id: i32,
    pub ctime: Option<chrono::NaiveDateTime>,
    pub dosage: Option<f32>,
    pub dosage_count: Option<i32>
}

#[derive(Insertable)]
#[table_name = "medicince_tracking"]
pub struct MedicineTrackingInsert {
    pub ctime: Option<chrono::NaiveDateTime>,
    pub dosage: f32,
    pub dosage_count: i32 // value is always latest value + 1 -> latest value needs to be queried
}
