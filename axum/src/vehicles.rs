use axum::{
    debug_handler,
    Json,
    Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};


#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: Option<String>,
}


#[debug_handler]
#[utoipa::path(
    tag = "Vehicles",
    get,
    path = "/{id}", // Relative path (will become /vehicles/{id})
    responses(
        (status = 200, description = "Get a vehicle")
    )
)]
pub async fn get_vehicle()  -> Json<Vehicle> {
    println!("Caller retrieved a vehicle from axum");
    Json::from(Vehicle {
        manufacturer: "Dodge".to_string(),
        model: "RAM 1500".to_string(),
        year: 2021,
        id: Some(Uuid::now_v7().to_string()),
    })
}

#[debug_handler]
#[utoipa::path(
    tag = "Vehicles",
    post,
    path = "/create", // Relative path (will become /vehicles/create)
    responses(
        (status = 200, description = "Create a vehicle")
    )
)]
pub async fn create_vehicle(Json(mut payload): Json<Vehicle>) -> Json<Vehicle> {
    println!("Manufacturer: {0}, Model: {1}, Year: {2}", payload.manufacturer, payload.model, payload.year);
    payload.id = Some(Uuid::now_v7().to_string());
    Json::from(payload)
}


// Define a sub-router for Vehicle routes
pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        // .routes(routes!(list_vehicles))      // Becomes GET /vehicles/
        .routes(routes!(get_vehicle))     // Becomes GET /vehicles/{id}
        .routes(routes!(create_vehicle)) // Becomes POST /vehicles/create
}
