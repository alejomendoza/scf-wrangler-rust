use worker::*;
use crate::sanity;

pub async fn handler(_req:Request, _ctx:RouteContext<()>) -> Result<Response> {
    let token = _ctx.env.secret("SANITY_API_KEY").unwrap().to_string();

    match sanity::fetch_users(&token).await {
        Ok(users) => Response::from_json(&users),
        Err(e) => {
            let error_message = format!("Error fetching users: {}", e);
            Response::error(error_message, 500)
        }
    }
}