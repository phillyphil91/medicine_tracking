use crate::schema::medicince_tracking::{self};

use super::CustomError;
use crate::model::{MedicineTrackingInsert, MedicineTrackingQuery};
use crate::common_structs::QueryDataResponse;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use serde::Serialize;

// pub async fn struct_to_postgres(dosage: String) -> Result<String, Box<dyn std::error::Error>> {
//     // Insert dosage to postgres db
//     let dosage: f32 = dosage.parse()?;

//     let query_result = query_data().await;

//     match query_result {
//         Ok(recommended_dosage_struct) => {
//             if let Some(recommended) = recommended_dosage_struct.dosage {
//                 if dosage != recommended {
//                     Err(Box::new(CustomError::DosageNotRecommend))
//                 } else {
//                     let medicine_struct = MedicineTrackingInsert {
//                         ctime: Some(Utc::now().naive_utc()),
//                         dosage,
//                         dosage_count: recommended_dosage_struct.dosage_count.unwrap_or(0) + 1,
//                     };

//                     let database_url =
//                         "host=localhost port=5432 user=postgres password=bla".to_string();

//                     let connection = PgConnection::establish(&database_url)?;

//                     diesel::insert_into(medicince_tracking::table)
//                         .values(medicine_struct)
//                         .get_results::<MedicineTrackingQuery>(&connection)
//                         .expect("bla");

//                     Ok(format! {"Successfully recorded {} mg of Cortison into log", dosage})
//                 }
//             } else {
//                 Err(Box::new(CustomError::DosageNotRecommend))
//             }
//         }
//         Err(e) => Err(e),
//     }
// }

pub async fn query_data() -> Result<QueryDataResponse, Box<dyn std::error::Error>> {
    // recommends dosage and dosage_count

    // use crate::medicine_logic::return_recommended_dosage;
    use crate::schema::medicince_tracking::dsl::*;

    let database_url = "host=localhost port=5432 user=postgres password=bla".to_string();

    let connection = PgConnection::establish(&database_url)?;

    let mut current_medicine_struct =
        medicince_tracking.load::<MedicineTrackingQuery>(&connection)?;

    let current_medicine_struct = current_medicine_struct.remove(0);

    Ok(QueryDataResponse{
        dosage: current_medicine_struct.dosage.unwrap(),
        dosage_count: current_medicine_struct.dosage_count.unwrap() })
}

#[cfg(test)]
mod test_postgres_custom {
    use super::*;
    use chrono::NaiveDate;

    // #[test]
    // fn test_insert() {
    //     let mock_query_result = MedicineTrackingQuery {
    //         id: 1,
    //         ctime: Some(NaiveDate::from_ymd(2017, 31, 1).and_hms(9, 20, 20)),
    //         dosage: Some(1.25),
    //         dosage_count: Some(5),
    //     };

    // }
}

