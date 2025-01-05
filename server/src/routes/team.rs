use axum::{response::{Html, IntoResponse}, Json};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TeamRequest {
    name:String,
    owner: String
}

pub async fn post(Json(body): Json<TeamRequest>) {
    println!("{:?}", body);
  
}
