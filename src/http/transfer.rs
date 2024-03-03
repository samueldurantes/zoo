use std::env;
use axum::Json;
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn create_transfer(
    Json(payload): Json<Value>
) -> Result<Json<Value>, String> {
    let request_payload = json!({
        "value": 300,
        "fromPixKey": payload["charge"]["subAccount"]["pixKey"],
        "fromPixKeyType": "EMAIL",
        "toPixKey": "mediator@email.com",
        "toPixKeyType": "EMAIL",
        "correlationID": Uuid::new_v4().to_string(),
    });

    let response = reqwest::Client::new()
        .post(format!("{}/api/v1/subaccount/transfer", env::var("WOOVI_API_URL").unwrap()))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", env::var("WOOVI_APP_ID").unwrap())
        .json(&request_payload)
        .send()
        .await;

    match response {
        Ok(response) => {
            let response = response.json::<Value>().await.unwrap();

            Ok(Json(response))
        },
        Err(error) => {
            Err(error.to_string())
        }
    }
}
