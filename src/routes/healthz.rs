use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthzResponse {
    message: String,
}

pub async fn get() -> Json<HealthzResponse> {
    let res: HealthzResponse = HealthzResponse {
        message: "Healthz".to_string(),
    };
    Json(res)

}
