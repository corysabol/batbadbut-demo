use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use std::process::Command;

async fn get_server_info(path: web::Path<(String,)>) -> impl Responder {
    log::info!("{:?}", path);

    let server = path.0.clone();
    println!("{server}");

    let output = Command::new("cmd")
        .args(["/C", "serverinfo.bat", server.as_str()])
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                let result = String::from_utf8(out.stdout)
                    .unwrap_or_else(|_| String::from("Failed to read output."));
                HttpResponse::Ok().content_type("text/plain").body(result)
            } else {
                let error_message = String::from_utf8(out.stderr)
                    .unwrap_or_else(|_| String::from("Failed to read error output."));
                HttpResponse::InternalServerError().body(error_message)
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting HTTP server at http://localhost:9999");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/server_info/{server}", web::get().to(get_server_info))
    })
    .bind("127.0.0.1:9999")?
    .run()
    .await
}
