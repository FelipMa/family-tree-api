use axum::{
    http::Method,
    middleware,
    routing::{delete, get, patch, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
mod database;
mod handlers;
mod middlewares;
mod queries;

pub async fn run() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
        .allow_origin(Any);

    let app = Router::new()
        .route(
            "/individuals",
            post(handlers::individuals::post_individual::post_individual),
        )
        .route(
            "/individuals",
            get(handlers::individuals::get_all_individuals::get_all_individuals),
        )
        .route(
            "/individuals/:id",
            get(handlers::individuals::get_one_individual::get_one_individual),
        )
        .route(
            "/individuals/:id",
            patch(handlers::individuals::patch_individual::patch_individual),
        )
        .route(
            "/individuals/:id",
            delete(handlers::individuals::delete_individual::delete_individual),
        )
        .route(
            "/relationships",
            get(handlers::relationships::get_all_relationships::get_all_relationships),
        )
        .route(
            "/relationships",
            post(handlers::relationships::post_relationship::post_relationship),
        )
        .route(
            "/relationships/:from_id/:to_id",
            delete(handlers::relationships::delete_relationship::delete_relationship),
        )
        .route(
            "/relationships/:from_id/:to_id",
            patch(handlers::relationships::patch_relationship::patch_relationship),
        )
        .layer(middleware::from_fn(
            middlewares::auth::auth_middleware_user_admin,
        ))
        .route("/", get(handlers::hello_world::hello_world))
        .route("/register", post(handlers::users::register::register))
        .route("/login", post(handlers::users::login::login))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!(
        "->> Server listening on http://{:?}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
