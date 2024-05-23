use std::sync::Arc;

//ignore unused imports
#[allow(unused_imports)]
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    model::{UserModel, UserModelResponse},
    schema::{LoginUserSchema, CreateUserSchema},
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


pub async fn create_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Query for user
    let id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query(r#"INSERT INTO users (id, username, password, email, phone, role, name) VALUES ($1, $2, $3, $4, $5, $6, $7)"#)
        .bind(id.clone())
        .bind(body.username.to_string())
        .bind(body.password.to_string())
        .bind(body.email.to_string())
        .bind(body.phone.to_string())
        .bind(body.role.to_string())
        .bind(body.name.to_string())
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    // Duplicate err check
    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "User already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", err)})),
        ));
    }

    let note = sqlx::query_as::<_, UserModel>(r#"SELECT * FROM users WHERE id = $1"#)
    .bind(id)
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", e)})),
        )
    })?;

    let note_response = serde_json::json!({
            "status": "success",
            "data": serde_json::json!({
                "note": to_user_response(&note)
        })
    });

    Ok(Json(note_response))
    }


// Convert DB Model to Response
fn to_user_response(user: &UserModel) -> UserModelResponse {
    UserModelResponse {
        id: user.id.clone(),
        username: user.username.clone(),
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}