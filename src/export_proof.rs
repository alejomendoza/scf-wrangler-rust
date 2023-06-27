use worker::*;


pub async fn handler(_req:Request, _ctx:RouteContext<()>) -> Result<Response> {
   
   return Response::ok("export proof");
}