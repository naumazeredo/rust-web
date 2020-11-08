mod models;
mod filters;
mod handlers;

use warp::Filter;
use models::Database;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let db = Database::new();

    let routes = filters::get_patient_filters(db);
    let routes = routes.with(warp::trace::request());

    let port = std::env::var("PORT")
        .map_or(5000u16, |s| s.parse::<u16>().unwrap());

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}
