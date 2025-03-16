use actix_web::{middleware::Logger, web, App, HttpServer};

mod structures;
mod api;

fn get_config() -> structures::Config {
    let file = std::fs::File::open("config.json").expect("The config file does not exist");
    let reader = std::io::BufReader::new(file);

    serde_json::from_reader(reader).expect("The config file is not properly formatted")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let config = get_config();    
    let server_string = format!("{}:{}", config.host, config.port);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(handler))
            .service(api::get_api)
            .service(api::post_api)
    })
    .bind(server_string)?
    .run()
    .await
}

async fn handler() -> &'static str {
    "Hello, world!"
}