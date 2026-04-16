pub mod client;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod utils;

#[derive(Clone, Debug)]
pub struct AppState {
    pub client: crate::client::ReqwestClient,
    pub db: crate::models::db::ProfileRepo,
}

pub fn create_app(state: AppState) -> axum::Router {
    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    axum::Router::new()
        .route(
            "/api/profiles",
            axum::routing::get(handlers::list_profiles).post(handlers::create_profile),
        )
        .route(
            "/api/profiles/{id}",
            axum::routing::get(handlers::get_profile).delete(handlers::delete_profile),
        )
        .layer(cors)
        .with_state(state)
}
