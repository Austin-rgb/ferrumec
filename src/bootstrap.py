import sys
MODULE = sys.arg[1]
cargo_toml = f"""
[package]
name = "{MODULE}-service"
version = "0.1.0"
edition = "2024"

[dependencies]
actix-web = "4.13.0"
ferrumec = { git = "https://github.com/Austin-rgb/ferrumec" }
{MODULE} = { git = "https://github.com/Ferrumec/{MODULE}" }
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
dotenvy = "0.15.7"
tracing = "0.1.44"
tracing-subscriber = { version = "0.3.23", features = ["env-filter"] }
sqlx = { version = "0.8", features = [ "runtime-tokio","sqlite" ] }

[patch."https://github.com/Ferrumec/{MODULE}"]
{MODULE} = {path = "../{MODULE}"}

[patch."https://github.com/Austin-rgb/ferrumec"]
ferrumec = { path = "../ferrumec" }
"""


main_rs = f"""
     GNU nano 9.0                           src/main.rs
    // main.rs
    use dotenvy::dotenv;
    use actix_web::{ App, HttpServer, middleware};
    use ferrumec::di::inject as run;
    use notification::{Module};
    // 1. Import tracing-subscriber
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    #[actix_web::main]
    # async fn main() -> std::io::Result<()> {
    # dotenv().ok();
    #     // 2. Initialize tracing-subscriber instead of env_logger
    #     tracing_subscriber::registry()
    #         .with(fmt::layer())
    #         .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
    #             "info".into()
    #         }))
    #         .init();
    #
    #     tracing::info!("Starting Notification Preferences Service...");
    #
    #     let notifications_module = match run(Module::new).await {
    #         Ok(r) => r.await,
    #         Err(e) => {
    #             tracing::error!("Error in initializing module: {e}");
    #             panic!()
    #         }
    #     };
    # let addr = std::env::var("ADDR").unwrap_or("0.0.0.0:8080".to_owned());
    #     HttpServer::new(move || {
    #         App::new()
    #             .wrap(middleware::Logger::default()) // Logs HTTP requests
    #             .wrap(middleware::Compress::default())
    #             .configure(|cfg| {
    #                 notifications_module.config(cfg, "/api");
    #             })
    #     })
    #     .bind(addr)?
    #     .run()
    #     .await
    #  
"""

import shutil

shutil.os.mkdir(f"{MODULE}-service")
shutil.os.chdir(f"{MODULE}-service")
file = open(
    "Cargo.toml"
)
file.write(cargo_toml)
