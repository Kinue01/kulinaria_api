mod handlers;
mod models;
mod errors;

use std::time::Duration;

use axum::{http::Method, routing::{delete, get, post, put}, Router};
use crate::handlers::{ get_users, get_dishes, get_types, get_bases, get_prods, get_struct_by_dish_id, add_dish, update_dish, delete_dish, get_cart_by_order_id, get_paytypes, add_order, get_orders_by_user_id };
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use tower_http::{ trace::{ self, TraceLayer }, cors::{ CorsLayer, Any } };


#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
    .with_target(false)
    .compact()
    .init();

    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_origin(Any)
    .allow_headers(Any);

    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
    .max_connections(65)
    .acquire_timeout(Duration::from_secs(20))
    .connect(&db_url)
    .await
    .unwrap();
    
    let app = Router::new()
    .route("/api/users", get(get_users))
    .route("/api/dishes", get(get_dishes))
    .route("/api/types", get(get_types))
    .route("/api/bases", get(get_bases))
    .route("/api/prods", get(get_prods))
    .route("/api/struct_by_dish", post(get_struct_by_dish_id))
    .route("/api/adddish", post(add_dish))
    .route("/api/updatedish", put(update_dish))
    .route("/api/cart_by_order", post(get_cart_by_order_id))
    .route("/api/deletedish", delete(delete_dish))
    .route("/api/order_by_user", post(get_orders_by_user_id))
    //.route("/api/addorder", post(add_order))
    .route("/api/paytypes", get(get_paytypes))
    .with_state(pool)
    .layer(
        TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new()
            .level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new()
            .level(tracing::Level::INFO)),
    ).layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
