use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JsonResponse<T: Serialize> {
    pub status: u16,
    pub data: T,
}

impl<T: Serialize> JsonResponse<T> {
    pub fn success(data: T) -> Self {
        Self { status: 200, data }
    }

    pub fn with_status(status: u16, data: T) -> Self {
        Self { status, data }
    }
}

impl<T: Serialize> IntoResponse for JsonResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
