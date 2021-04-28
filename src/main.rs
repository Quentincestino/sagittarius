use actix_web::{App, HttpServer};
use std::panic::PanicInfo;
pub mod apis;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // On panic we must do some emergency things in order to correctly shutdown the server
    std::panic::set_hook(Box::new(panic_hook));

    // Setup the actix's http server
    HttpServer::new(|| App::new().configure(apis::apis))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn panic_hook(infos: &PanicInfo) {
    println!("Panic !!!!")
}
