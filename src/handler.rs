use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    model::{UserModel, UserModelResponse},
    schema::LoginUserSchema,
    AppState,
};

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn login_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Query for user
    let query_result = sqlx::query_as::<_, UserModel>(r#"SELECT * FROM users WHERE username = $1"#)
        .bind(body.username.to_string())
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(user) => {
            // Check password
            if user.password == body.password {
                let login_response = serde_json::json!({
                    "status": "success",
                    "message": "Login successful",
                    "user": user
                });

                Ok(Json(login_response))
            } else {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Invalid password"
                });
                Err((StatusCode::UNAUTHORIZED, Json(error_response)))
            }
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "User not found"
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ))
        }
    }
}


// Convert DB Model to Response
// fn to_user_response(user: &UserModel) -> UserModelResponse {
//     UserModelResponse {
//         id: user.id.clone(),
//         username: user.username.clone(),
//         created_at: user.created_at,
//         updated_at: user.updated_at,
//     }
// }