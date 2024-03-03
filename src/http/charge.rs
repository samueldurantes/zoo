use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::env;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub number: i32,
    pub city: String,
    pub state: String,
    pub neighborhood: String,
    pub zipcode: String,
}

#[derive(Serialize, Deserialize)]
pub struct Customer {
    pub name: String,
    pub tax_id: String,
    pub address: Address,
}

#[derive(Serialize, Deserialize)]
pub struct CreateChargeRequest {
    pub value: i32,
    pub sub_account: String,
    pub customer: Customer,
}

pub async fn create_charge(
    Json(payload): Json<CreateChargeRequest>
) -> Result<Json<Value>, String> {
    let payload = json!({
        "value": payload.value,
        "customer": {
            "name": payload.customer.name,
            "taxID": payload.customer.tax_id,
            "address": {
                "street": payload.customer.address.street,
                "number": payload.customer.address.number,
                "city": payload.customer.address.city,
                "state": payload.customer.address.state,
                "neighborhood": payload.customer.address.neighborhood,
                "zipcode": payload.customer.address.zipcode,
            }
        },
        "subAccount": payload.sub_account,
        "correlationID": Uuid::new_v4().to_string(),
    });

    let response = reqwest::Client::new()
        .post(format!("{}/api/v1/charge", env::var("WOOVI_API_URL").unwrap()))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", env::var("WOOVI_APP_ID").unwrap())
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap();

            if status.is_success() {
                Ok(Json(json!({
                    "message": "Charge created successfully!",
                    "data": body,
                })))
            } else {
                Err(body)
            }
        },
        Err(err) => {
            Err(err.to_string())
        }
    }
}
