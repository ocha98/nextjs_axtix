use actix_web::{get, error, web, http::header::ACCEPT, Responder};
use serde::Deserialize;
use octocrab::Octocrab;
use secrecy::ExposeSecret;
use super::DbPool;

#[derive(Deserialize)]
pub struct CallbackParams {
    code: String,
}
pub struct GithubOAuthToken {
    pub client_id: String,
    pub client_secret: String,
}


#[get("/callback/github")]
pub async fn callback_github(
    params: web::Query<CallbackParams>,
    secrets: web::Data<GithubOAuthToken>,
    pool: web::Data<DbPool>
) -> actix_web::Result<impl Responder> {
    let oauth_client = Octocrab::builder()
        .base_uri("https://github.com")
        .unwrap()
        .add_header(ACCEPT, "application/json".to_string())
        .build()
        .unwrap();

    let oauth = oauth_client
        .post::<_, serde_json::Value>("/login/oauth/access_token", 
            Some(&serde_json::json!({
                "client_id": secrets.client_id,
                "client_secret": secrets.client_secret,
                "code": &params.code,
            }))
        )
        .await
        .unwrap();

    let oauth = serde_json::from_value::<octocrab::auth::OAuth>(oauth.clone())
        .unwrap_or_else(|_| panic!("couldn't parse OAuth credentials from {oauth:?}"));


    let client = Octocrab::builder()
        .user_access_token(oauth.access_token.expose_secret().clone())
        .build()
        .unwrap();

    let user = client.current().user().await.unwrap();

    let conn = pool.get().unwrap();

    let user = web::block(move || {
        let conn = pool.get()?;
        
    }).await?
    .map_err(error::ErrorInternalServerError)?;


    Ok(format!("Hello world! {}", user.login))
}