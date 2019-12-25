use crate::conduit::users;
use crate::middleware::ContextExt;
use crate::web::diesel_error;
use crate::Repo;
use log::info;

use crate::web::users::responses::UserResponse;
use http::status::StatusCode;
use tide::{Request, Response};

pub async fn get_current_user(cx: Request<Repo>) -> tide::Result<Response> {
    let auth = cx.get_claims().map_err(|_| StatusCode::UNAUTHORIZED)?;
    let repo = cx.state();
    info!("Get user {}", auth.user_id());

    let results = users::find(repo, auth.user_id());
    match results {
        Ok(user) => {
            let response = UserResponse { user };
            Ok(Response::new(200).body_json(&response).unwrap())
        }
        Err(e) => Err(diesel_error(&e)),
    }
}
