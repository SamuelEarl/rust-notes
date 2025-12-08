use axum::{
    debug_handler,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize, Serialize)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: Option<String>,
}

#[debug_handler]
pub async fn vehicle_get()  -> Json<Vehicle> {
    println!("Caller retrieved a vehicle from axum");
    Json::from(Vehicle {
        manufacturer: "Dodge".to_string(),
        model: "RAM 1500".to_string(),
        year: 2021,
        id: Some(Uuid::now_v7().to_string()),
    })
}

#[debug_handler]
pub async fn vehicle_post(Json(mut payload): Json<Vehicle>) -> Json<Vehicle> {
    println!("Manufacturer: {0}, Model: {1}, Year: {2}", payload.manufacturer, payload.model, payload.year);
    payload.id = Some(Uuid::now_v7().to_string());
    Json::from(payload)
}
