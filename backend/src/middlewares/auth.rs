use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::IntoResponse,
    routing::head,
};
use chrono::{NaiveDateTime, Utc};

use crate::{ApiResponse, AppState};

#[derive(Debug, Clone)]
pub struct UserSession {
    id: i64,
    pub user_id: i64,
    token: String,
    expires_at: NaiveDateTime,
}

pub async fn auth(State(state): State<AppState>, request: Request, next: Next) -> ApiResponse {
    println!("Auth middleware - PRE RESPONSE");

    let bearer = get_auth_bearer(&request);

    if bearer.is_none() {
        println!("Auth middleware - NO BEARER");
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    }

    let user = check_token(&state, bearer.unwrap()).await?; // non mi piace
    if user.is_none() {
        println!("Auth middleware - BEARER TOKEN NOT VALID");
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    }
    if let Some(user_info) = user {
        // Inserisci le informazioni dell'utente come estensione della richiesta
        /* request.extensions_mut().insert(user_info); */

        // Esegui il prossimo middleware o handler
        let response = next.run(request).await;

        println!("Auth middleware - POST RESPONSE");
        Ok(response)
    } else {
        println!("Auth middleware - BEARER TOKEN NOT VALID");
        Ok(StatusCode::UNAUTHORIZED.into_response())
    }
}

pub fn get_auth_bearer(request: &Request) -> Option<&str> {
    let auth_header = request.headers().get(AUTHORIZATION)?;
    let auth_header = auth_header.to_str().ok()?;

    let token = auth_header.strip_prefix("Bearer ")?;

    Some(token)
}

pub async fn check_token(state: &AppState, bearer: &str) -> anyhow::Result<Option<UserSession>> {
    let today = Utc::now();
    let result = sqlx::query_as!(
        UserSession,
        r#"SELECT
            id,
            user_id, 
            token,
            expires_at
        FROM sessions 
        WHERE token = $1 AND expires_at > $2"#,
        bearer,
        today
    )
    .fetch_optional(&state.db)
    .await?;

    Ok(result)
}
