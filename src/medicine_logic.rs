// Logic
// Each time someone opens the app, it should show the user the recommended dosage for that day.
//
// How much each dosage should be given, should be fixed in Key:Value store e.g:
//
// -------------
// 75mg : 2 days
// 50mg : 3 days
// ...
// 0.5mg : 14 days
// ...
// -------------
//
// Query what much the current value
use indexmap::IndexMap;

use crate::{error_custom::CustomError, model::MedicineTrackingQuery};
use crate::common_structs::QueryDataResponse;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Hash)]
enum DosageLogic {
    mg75,
    mg50,
    mg25,
    mg12_5,
    mg5,
    mg2_5,
    mg1_25,
    Invalid,
}

fn check_dosage_count_limit(
    // Check if current dosage count is over max count. Return current dosage or next appropriate dosage
    max_count: i32,
    current_count: i32,
    dosage_variant: DosageLogic,
    logic_index_map: IndexMap<DosageLogic, f32>,
) -> (f32, i32) {
    if max_count - current_count >= 0 {
        let value = logic_index_map.get(&dosage_variant).unwrap_or(&1.25);
        (*value, current_count) //Dosage if current_count dosage count is lower than or equal to max_count
    } else {
        let curent_dosage_index = logic_index_map
            .get_index_of(&dosage_variant)
            .unwrap_or_else(|| logic_index_map.len());
        let next_index = curent_dosage_index + 1;
        let next_value = logic_index_map
            .get_index(next_index)
            .unwrap_or((&DosageLogic::mg1_25, &1.25));
        let next_value = next_value.1;
        (*next_value, 0) //Dosage if current_count dosage count is higher than max_count
    }
}

pub fn return_recommended_dosage_and_count(
    query_results: &QueryDataResponse,
) -> Result<QueryDataResponse, CustomError> {
    // Return recommended dosage and dosage_count in a serialize struct

    let mut logic_index_map: IndexMap<DosageLogic, f32> = IndexMap::new();

    logic_index_map.insert(DosageLogic::mg75, 75.0);
    logic_index_map.insert(DosageLogic::mg50, 50.0);
    logic_index_map.insert(DosageLogic::mg25, 25.0);
    logic_index_map.insert(DosageLogic::mg12_5, 12.5);
    logic_index_map.insert(DosageLogic::mg5, 5.0);
    logic_index_map.insert(DosageLogic::mg2_5, 2.5);
    logic_index_map.insert(DosageLogic::mg1_25, 1.25);

    // Assign DosageLogic variant to dosage
    let dosage_variant = match query_results.dosage {
        75.0 => DosageLogic::mg75,
        50.0 => DosageLogic::mg50,
        25.0 => DosageLogic::mg25,
        12.5 => DosageLogic::mg12_5,
        5.0 => DosageLogic::mg5,
        2.5 => DosageLogic::mg2_5,
        1.25 => DosageLogic::mg1_25,
        _ => DosageLogic::Invalid,
    };

    let recommendation =  match dosage_variant {
            DosageLogic::mg75 => Ok(check_dosage_count_limit(
                2,
                query_results.dosage_count,
                DosageLogic::mg75,
                logic_index_map,
            )),
            DosageLogic::mg50 => Ok(check_dosage_count_limit(
                3,
                query_results.dosage_count,
                DosageLogic::mg50,
                logic_index_map,
            )),
            DosageLogic::mg25 => Ok(check_dosage_count_limit(
                5,
                query_results.dosage_count,
                DosageLogic::mg25,
                logic_index_map,
            )),
            DosageLogic::mg12_5 => Ok(check_dosage_count_limit(
                7,
                query_results.dosage_count,
                DosageLogic::mg12_5,
                logic_index_map,
            )),
            DosageLogic::mg5 => Ok(check_dosage_count_limit(
                14,
                query_results.dosage_count,
                DosageLogic::mg5,
                logic_index_map,
            )),
            DosageLogic::mg2_5 => Ok(check_dosage_count_limit(
                28,
                query_results.dosage_count,
                DosageLogic::mg2_5,
                logic_index_map,
            )),
            DosageLogic::mg1_25 => Ok(check_dosage_count_limit(
                14,
                query_results.dosage_count,
                DosageLogic::mg1_25,
                logic_index_map,
            )),
            DosageLogic::Invalid => Err(CustomError::DosageNotRecommend),
    };

    match recommendation {
        Ok(rec_tup) => Ok(QueryDataResponse {
            dosage: rec_tup.0,
            dosage_count: rec_tup.1,
        }),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test_medicine_logic {
    use super::*;

    #[test]
    fn test_current_less_than_max() {
        let mock_query_result = QueryDataResponse {
            dosage: 12.5,
            dosage_count: 5,
        };

        let mock_query_data_response = QueryDataResponse {
            dosage: 12.5,
            dosage_count: 5,
        };
        assert_eq!(
            mock_query_data_response,
            return_recommended_dosage_and_count(&mock_query_result).unwrap()
        )
    }

    #[test]
    fn test_current_more_than_max() {
        let mock_query_result = QueryDataResponse {
            dosage: 12.5,
            dosage_count: 10,
        };

        let mock_query_data_response = QueryDataResponse {
            dosage: 5.0,
            dosage_count: 0,
        };

        assert_eq!(
            mock_query_data_response,
            return_recommended_dosage_and_count(&mock_query_result).unwrap()
        )
    }

    #[test]
    #[should_panic]
    fn test_non_existing_dosage() {
        let mock_query_result = QueryDataResponse {
            dosage: 200.0,
            dosage_count: 10,
        };


        return_recommended_dosage_and_count(&mock_query_result).unwrap();
    }

    #[test]
    fn test_current_more_than_1_25_max() {
        let mock_query_result = QueryDataResponse {
            dosage: 1.25,
            dosage_count: 20,
        };


        let mock_query_data_response = QueryDataResponse {
            dosage: 1.25,
            dosage_count: 0,
        };

        assert_eq!(
            mock_query_data_response,
            return_recommended_dosage_and_count(&mock_query_result).unwrap()
        )

    }
}
