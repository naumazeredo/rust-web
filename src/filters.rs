use warp::Filter;
use crate::models::Database;
use crate::handlers;

pub fn get_patient_filters(
    db : Database
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(
            list_patients(db.clone())
            .or(create_patient(db))
        )
}


fn list_patients(
    db : Database
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path!("patients"))
        .and(with_db(db))
        .and_then(handlers::list_patients)
}

fn create_patient(
    db : Database
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("patients"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::create_patient)
}

fn with_db(
    db : Database
) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
