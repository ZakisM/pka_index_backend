use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::ApiError;

#[derive(Serialize)]
pub struct PathError {
    message: String,
    location: Option<String>,
}

pub struct CustomPath<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for CustomPath<T>
where
    T: DeserializeOwned + Send,
    B: Send,
{
    type Rejection = ApiError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err(ApiError::PathRejection(rejection)),
        }
    }
}
