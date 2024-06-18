use actix_files as fs;
use actix_web::{App, HttpServer, middleware::Logger};
use std::env;
use std::io::{self, Write};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Get the current directory
    let current_dir = env::current_dir().unwrap();
    println!("Serving files from: {:?}", current_dir);

    // Prompt the user for the port
    let port = loop {
        let port = get_port_from_user();
        if port_available(port) {
            break port;
        } else {
            eprintln!("Port {} is already in use. Please specify a different port.", port);
        }
    };

    // Start the server
    println!("Server running at http://0.0.0.0:{}", port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(fs::Files::new("/", &current_dir).show_files_listing())
    })
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}

fn get_port_from_user() -> u16 {
    print!("Enter port to bind to (default 8000): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(8000)
}

fn port_available(port: u16) -> bool {
    std::net::TcpListener::bind(("0.0.0.0", port)).is_ok()
}
