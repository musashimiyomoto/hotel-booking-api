use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{enums, models, utils};

pub async fn auth_middleware(
    State(app_state): State<models::AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, enums::AuthError> {
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(enums::AuthError::MissingToken)?;

    request.extensions_mut().insert(
        utils::extract_user_from_token(token, &app_state.jwt_secret)
            .map_err(|_| enums::AuthError::InvalidToken)?,
    );

    Ok(next.run(request).await)
}

impl IntoResponse for enums::AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            enums::AuthError::MissingToken => {
                (StatusCode::UNAUTHORIZED, "Missing authorization token")
            }
            enums::AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
        };
        (status, error_message).into_response()
    }
}
