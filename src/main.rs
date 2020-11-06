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

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
