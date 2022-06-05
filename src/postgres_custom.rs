use crate::medicine_logic::return_recommended_dosage_and_count;
use crate::schema::medicince_tracking::{self};

use super::CustomError;
use crate::common_structs::QueryDataResponse;
use crate::model::{MedicineTrackingInsert, MedicineTrackingQuery};

use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use serde::Serialize;

pub async fn dosage_to_postgres(dosage: String) -> Result<String, Box<dyn std::error::Error>> {
    // Insert dosage to postgres db

    let dosage: f32 = dosage.parse()?;

    let query_result = query_data().await;

    match query_result {
        Ok(current_dosage) => match return_recommended_dosage_and_count(&current_dosage) {
            Ok(recommended_dosage) => {
                if dosage != recommended_dosage.dosage {
                    Err(Box::new(CustomError::DosageNotRecommend))
                } else {
                    let medicine_struct = MedicineTrackingInsert {
                        ctime: Some(Utc::now().naive_utc()),
                        dosage: current_dosage.dosage,
                        dosage_count: recommended_dosage.dosage_count + 1,
                    };

                    let database_url =
                        "host=localhost port=5432 user=postgres password=bla".to_string();

                    let connection = PgConnection::establish(&database_url)?;

                    diesel::insert_into(medicince_tracking::table)
                        .values(medicine_struct)
                        .get_results::<MedicineTrackingQuery>(&connection)
                        .expect("bla");

                    return Ok(format! {"Successfully recorded {} mg of Cortison into log", dosage});
                }
            }
            Err(_e) => Err(Box::new(CustomError::DosageNotRecommend)),
        },
        Err(_e) => Err(Box::new(CustomError::DosageNotRecommend)),
    }
}

pub async fn query_data() -> Result<QueryDataResponse, Box<dyn std::error::Error>> {
    // returns current dosage and dosage_count

    // use crate::medicine_logic::return_recommended_dosage;
    use crate::schema::medicince_tracking::dsl::*;

    let database_url = "host=localhost port=5432 user=postgres password=bla".to_string();

    let connection = PgConnection::establish(&database_url)?;

    let mut current_medicine_struct =
        medicince_tracking.load::<MedicineTrackingQuery>(&connection)?;

    let current_medicine_struct = current_medicine_struct.remove(0);

    Ok(QueryDataResponse {
        dosage: current_medicine_struct.dosage.unwrap(),
        dosage_count: current_medicine_struct.dosage_count.unwrap(),
    })
}

#[cfg(test)]
mod test_postgres_custom {
    use super::dosage_to_postgres;

    #[tokio::test]
    async fn test_dosage_to_postgres() {
        assert_eq!(
            "Successfully recorded 1.25 mg of Cortison into log",
            dosage_to_postgres("1.25".to_string()).await.unwrap()
        )
    }

    #[tokio::test]
    #[should_panic]
    async fn test_wrong_dosage_to_postgres() {
            dosage_to_postgres("5.0".to_string()).await.unwrap();
    }
}
