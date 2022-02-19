use tokio_postgres::NoTls;
use super::CustomError;

pub async fn struct_to_postgres(dosage: String) -> Result<String, CustomError> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost port=5432 user=postgres password=bla",
        NoTls,
    )
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
        .execute("INSERT INTO taro_medicine (dosage) VALUES ($1)", &[&dosage])
        .await?;
    Ok(format!{"Successfully recorded {} mg of Cortison into log", dosage})
}
