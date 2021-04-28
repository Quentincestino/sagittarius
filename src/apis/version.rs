use actix_web::get;
use actix_web::web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
struct VersionResponseBody<'a> {
    version: &'a str,
}

// The current Matrix version of the server.
const MATRIX_VERSION: &str = "1.0";
// The response body returned by versions endpoint.
const VERSION_RESPONSE: VersionResponseBody = VersionResponseBody {
    version: MATRIX_VERSION,
};

#[get("/_matrix/client/versions")]
pub async fn version() -> HttpResponse {
    HttpResponse::Ok().json(VERSION_RESPONSE)
}
