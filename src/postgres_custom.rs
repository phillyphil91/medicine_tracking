use crate::schema::medicince_tracking::{self};

use super::model::{MedicineTrackingInsert, MedicineTrackingQuery};
use super::CustomError;
use tokio_postgres::NoTls;

use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub async fn struct_to_postgres(dosage: String) -> Result<String, CustomError> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost port=5432 user=postgres password=bla", NoTls)
            .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let dosage: f32 = dosage.parse()?;
    client
        .execute(
            "INSERT INTO medicince_tracking (dosage) VALUES ($1)",
            &[&dosage],
        )
        .await?;
    Ok(format! {"Successfully recorded {} mg of Cortison into log", dosage})
}

pub async fn struct_to_postgres_diesel(dosage: String) -> Result<String, CustomError> {
    let database_url = "host=localhost port=5432 user=postgres password=bla".to_string();

    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

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
