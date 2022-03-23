use crate::schema::medicince_tracking::{self};

use super::CustomError;
use crate::model::{MedicineTrackingInsert, MedicineTrackingQuery};

use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use rocket::serde::json::Json;

pub async fn struct_to_postgres(dosage: String) -> Result<String, CustomError> {
    let database_url = "host=localhost port=5432 user=postgres password=bla".to_string();

    let connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let dosage: f32 = dosage.parse()?;
    let medicine_struct = MedicineTrackingInsert {
        ctime: Some(Utc::now().naive_utc()),
        dosage,
    };
    diesel::insert_into(medicince_tracking::table)
        .values(medicine_struct)
        .get_results::<MedicineTrackingQuery>(&connection)
        .expect("bla");

    Ok(format! {"Successfully recorded {} mg of Cortison into log", dosage})
}

pub async fn query_data() -> Json<Vec<MedicineTrackingQuery>> {
    use crate::schema::medicince_tracking::dsl::*;

    let database_url = "host=localhost port=5432 user=postgres password=bla".to_string();

    let connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    Json(
        medicince_tracking
            .filter(dosage.eq(500.0))
            .load::<MedicineTrackingQuery>(&connection)
            .expect("Error in query"),
    )
}
