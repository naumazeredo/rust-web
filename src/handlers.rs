use std::sync::atomic::Ordering;
use models::{Database, Patient};
use warp::http;

pub async fn list_patients(
    db : Database
) -> Result<impl warp::Reply, std::convert::Infallible> {
    let patients = db.patients.read();
    let patients = patients.clone();
    Ok(warp::reply::json(&patients))
}

pub async fn create_patient(
    patient : Patient,
    db : Database
) -> Result<impl warp::Reply, warp::Rejection> {
    let name = patient.name.clone();

    let mut patients = db.patients.write();
    let id = db.new_id.fetch_add(1, Ordering::SeqCst);

    let mut patient = patient;
    patient.id = id;
    patients.insert(patient.id, patient);

    Ok(warp::reply::with_status(format!("Added patient: \"{}\"", name), http::StatusCode::CREATED))
}
