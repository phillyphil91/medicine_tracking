use crate::schema::medicince_tracking::{self};

use super::model::{MedicineTrackingInsert, MedicineTrackingQuery};
use super::CustomError;
// use tokio_postgres::NoTls;

use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;

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
