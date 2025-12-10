#![allow(unused)] // For beginning only.

use axum::{
    extract::{
        Path,
        Query,
    }, 
    http::StatusCode, 
    Json,
    response::{
        Html,
        IntoResponse,
    },
    Router,
    routing::{
        MethodRouter, 
        get,
        get_service,
        post,
    }
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use utoipa::{
    IntoParams,
    OpenApi,
    ToSchema,
};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_redoc::{Redoc, Servable};

mod vehicles;


// 1. Define the OpenAPI struct
// Note: We leave 'paths' empty! utoipa-axum fills them in.
#[derive(OpenApi)]
#[openapi(
    components(
        schemas(User, vehicles::Vehicle)
    ),
    info(description = "Demo Utoipa-Axum API", title = "Demo API", version = "1.0")
)]
struct ApiDoc;


#[tokio::main]
async fn main() {
    // 1. Create the OpenApiRouter
    // We pass our base 'ApiDoc' so it knows about the schemas/title.
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        // 2. Register routes using the routes! macro
        // Syntax: routes!(handler_function)
        .routes(routes!(create_user))
        .routes(routes!(get_user))
        .routes(routes!(query_params_example))
        .routes(routes!(path_params_example))
        // Nest the vehicles routes under "/api/vehicles"
        .nest("/api/vehicles", vehicles::router())
        // .routes(routes!(get_vehicle))
        // .routes(routes!(create_vehicle))

        // 3. Split it!
        // 'router' is a standard Axum Router
        // 'api' is the generated OpenAPI JSON spec
        .split_for_parts();

    // 5. Merge Redoc into the standard router
    // First arg: The path to serve the Redoc UI
    // Second arg: The OpenAPI spec object
    let app = router.merge(Redoc::with_url("/api-docs", api));
    // let app = Router::new()
    //     .route(
    //         "/query-params-route",
    //         get(query_params_example),
    //     )
    //     .route(
    //         "/path-params-route/{name}",
    //         get(path_params_example),
    //     )
    //     .route("/vehicle", get(get_vehicle))
    //     .route("/vehicle", post(create_vehicle))
    //     // 4. Merge the Redoc router
    //     // First arg: The path to serve the Redoc UI
    //     // Second arg: The OpenAPI spec object
    //     .merge(Redoc::with_url("/redoc", doc));
    //     // .merge(app_router());
    //     // .fallback_service(static_routes());

    // // Define the address to bind to.
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // println!(">>> LISTENING on {addr}\n");

    // // Create a TCP listener and bind it to the address.
    // let listener = TcpListener::bind(&addr).await.unwrap();

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    println!("Docs available at http://localhost:3000/api-docs");

    // Serve the Axum application with the listener.
    axum::serve(listener, app).await.unwrap();
}

// fn app_router() -> Router {
//     return Router::new()
//         .route(
//             "/query-params-route",
//             get(query_params_example),
//         )
//         .route(
//             "/path-params-route/{name}",
//             get(path_params_example),
//         );
// }

// fn static_routes() -> MethodRouter {
//     async fn handle_404() -> (StatusCode, &'static str) {
//         return (StatusCode::NOT_FOUND, "Resource not found.");
//     }

//     return get_service(ServeDir::new("/").not_found_service(handle_404.into_service()));
// }


// Traits are used to define shared behaviors — like printing, comparing, cloning, or serializing.
// The `#[derive(...)]` macro auto-implements traits for the data structure that follows this macro.
#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema, Clone)]
// Optional: Define default location for all fields (if not specified per field)
#[into_params(parameter_in = Query)]
pub struct User {
    id: Option<String>,
    first_name: String,
    last_name: String,
}

// #[derive(Deserialize, ToSchema)]
// pub struct CreateUserReq {
//     username: String,
// }


#[utoipa::path(
    tag = "Users",
    post,
    path = "/api/users/create",
    responses(
        (status = 200, description = "Create a user", body = User)
    )
)]
async fn create_user(Json(payload): Json<User>) -> Json<User> {
    Json(User { id: Some("123-xyz".to_string()), first_name: payload.first_name, last_name: payload.last_name })
}

#[utoipa::path(
    tag = "Users",
    get,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "Get user by ID", body = User),
        (status = 404, description = "User not found")
    )
)]
async fn get_user(Path(id): Path<i32>) -> Json<User> {
    Json(User { id: Some("123-xyz".to_string()), first_name: "John".to_string(), last_name: "Doe".to_string() })
}



// #[derive(Debug, Deserialize)]
// struct QueryParams {
//     name: Option<String>,
// }

// NOTE: If you have more data to parse from the request than is defined in a single struct, then you can use multiple extractors where each extractor will parse the data that is associated with a different struct.

// You can destructure query params by passing `Query(params)` (instead of simply `params` or `{ params }` as in JavaScript) to the route handler function.
// Then you can reference the params in your function like this: `params.name`.
#[utoipa::path(
    tag = "Demos",
    get,
    path = "/api/query-params-route",
    params(
        // Utoipa expands this into multiple query parameters automatically
        User
    ),
    responses(
        (status = 200, description = "Example of a route that takes query params", body = User)
    ),
)]
async fn query_params_example(Query(params): Query<User>) -> impl IntoResponse {
    // The `Debug` trait allows you to use the `{:?}` formatter to print the internal state of a struct or enum.
    // Without the `Debug` trait, the following line wouldn’t compile.
    println!(">>> {:<12} - query_params_example - {params:?}", "HANDLER");
    // println!(">>> {:<12} - query_params_example - {1:?}", "HANDLER", params.first_name);

    // unwrap_or() is a method used on Option and Result types in Rust. It allows you to safely extract the value inside if it exists, or provide a fallback default value if it doesn't.
    let id = params.id.unwrap_or("123-xyz".to_string());
    let first_name = params.first_name;
    let last_name = params.last_name;
    Html(format!("Query Params: <strong>{first_name}</strong>"))
}

#[utoipa::path(
    tag = "Demos",
    get,
    path = "/api/path-params-route/{name}",
    params(
        ("name" = String, Path, description = "Name of user"),
    ),
    responses(
        (status = 200, description = "Example of a route that takes path params", body = User)
    ),
)]
async fn path_params_example(Path(name): Path<String>) -> impl IntoResponse {
    println!(">>> {:<12} - path_params_example - {name:?}", "HANDLER");

    Html(format!("Path Params: <strong>{name}</strong>"))
}
