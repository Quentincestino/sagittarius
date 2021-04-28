pub mod access;
pub mod auth;
mod identifier;
pub mod version;

#[derive(serde::Serialize)]
pub struct MatrixErrorBody<'a> {
    errcode: &'a str,
    error: &'a str,
}

use actix_web::{web::ServiceConfig, App};
use auth::{auth as authenticate, logout, logout_all, serve_login_types};
use version::version as get_version;

pub fn apis(cfg: &mut ServiceConfig) {
    // Auth API
    cfg.service(authenticate)
        .service(logout)
        .service(logout_all)
        .service(serve_login_types);

    // Version API
    cfg.service(get_version);
}
