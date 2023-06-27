use worker::*;
use crate::sanity;

pub async fn handler(_req:Request, _ctx:RouteContext<()>) -> Result<Response> {

    let data = sanity::fetch_users().await.unwrap();

   
   return Response::from_json(&data);
}