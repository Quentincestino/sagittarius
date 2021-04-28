use super::identifier::*;
use super::MatrixErrorBody;

use actix_web::{
    get, post,
    web::{HttpResponse, Json},
    Responder,
};
use serde::{Deserialize, Serialize};

/// Represents the list of login flows returned by the serve_login_types endpoint
#[derive(Serialize)]
struct LoginFlow<'a> {
    flows: [LoginType<'a>; 2],
}

const LOGIN_FLOWS: LoginFlow = LoginFlow {
    flows: [
        LoginType::new("m.login.password"),
        LoginType::new("m.login.token"),
    ],
};

/// Represents a login type on the list returned by the serve_login_types endpoint
#[derive(Serialize)]
struct LoginType<'a> {
    r#type: &'a str,
}

impl<'a> LoginType<'a> {
    pub const fn new(r#type: &'a str) -> Self {
        Self { r#type }
    }
}

#[derive(Serialize)]
struct LoginRateLimitedBody<'a> {
    errcode: &'a str,
    error: &'a str,
    retry_after_ms: u32,
}

/// Gives the different login flows that are supported by the server
#[get("/_matrix/client/r0/login")]
pub async fn serve_login_types() -> HttpResponse {
    // TODO: Supposed to be a rate-limited endpoint
    HttpResponse::Ok().json(LOGIN_FLOWS)
}

/// On an auth request, the client sends this request in JSON format, wich we can parse using this struct
#[derive(Deserialize)]
struct AuthRequestBody {
    r#type: String,
    identifier: UserIdentifier,
    user: Option<String>,
    medium: Option<String>,
    address: Option<String>,
    password: Option<String>,
    token: Option<String>,
    device_id: Option<String>,
    initial_device_display_name: Option<String>,
}

// TODO: AUTH
#[post("/_matrix/client/r0/login")]
pub async fn auth(request_body: Json<AuthRequestBody>) -> impl Responder {
    let response = match decode_identifier_type(&request_body.identifier) {
        IdentifierType::MIdUser(identifier) => {
            format!("MIdUser !")
        }
        IdentifierType::MIdThirdparty(identifier) => {
            format!("MId3rd !")
        }
        IdentifierType::MIdPhone(identifier) => {
            format!("MIdPhone !")
        }
        IdentifierType::None => format!("fail !"), /*HttpResponse::BadRequest().json(MatrixErrorBody {
                                                       errcode: "M_UNKNOWN",
                                                       error: "Bad login type.",
                                                   })*/
    };
    response
}

#[post("/_matrix/client/r0/logout")]
pub async fn logout() -> impl Responder {
    format!("salut")
}

#[post("/_matrix/client/r0/logout/all")]
pub async fn logout_all() -> impl Responder {
    format!("salut")
}
