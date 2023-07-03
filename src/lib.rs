use worker::*;
mod auth;
mod user;
mod users;
mod projects;
mod export_proof;
mod tiers;
mod needs_work;
mod index_submittable;
mod submit;
mod sanity;

impl Route {
    fn to_str(&self) -> &'static str {
        match self {
            Route::Auth => "/auth",
            Route::User => "/user",
            Route::Users => "/users",
            Route::Projects => "/projects",
            Route::ExportProof => "/export-proof",
            Route::Tiers => "/tiers",
            Route::NeedsWork => "/needs-work",
            Route::IndexSubmittable => "/index-submittable",
            Route::Submit => "/submit"
        }
    }
}

enum Route {
    Auth,
    User,
    Users,
    Projects,
    ExportProof,
    Tiers,
    NeedsWork,
    IndexSubmittable,
    Submit
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    
    let router = Router::new();
    
    router
    .get_async(Route::Auth.to_str(), auth::handler)
    .get_async(Route::User.to_str(), user::handler)
    .get_async(Route::Users.to_str(), users::handler)
    .get_async(Route::Projects.to_str(), projects::handler)
    .get_async(Route::ExportProof.to_str(), export_proof::handler)
    .post_async(Route::User.to_str(), user::handler)
    .post_async(Route::Tiers.to_str(), tiers::handler)
    .post_async(Route::NeedsWork.to_str(), needs_work::handler)
    .post_async(Route::IndexSubmittable.to_str(), index_submittable::handler)
    .post_async(Route::Submit.to_str(), submit::handler)
    .run(req, env).await
}
