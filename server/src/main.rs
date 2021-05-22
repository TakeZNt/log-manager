use actix_web::http::Method;
use actix_web::App;

mod handler;

const PORT: u16 = 3000;

#[derive(Clone)]
pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }
}

// ルーティング
pub fn app(server: Server) -> App<Server> {
    use crate::handler::*;
    let app: App<Server> = App::with_state(server)
        .route("/logs", Method::POST, handle_post_logs)
        .route("/csv", Method::POST, handle_post_csv)
        .route("/logs", Method::GET, handle_get_logs)
        .route("/csv", Method::GET, handle_get_csv);

    app
}

fn main() {
    env_logger::init();

    let server = Server::new();
    actix_web::server::new(move || app(server.clone()))
        .bind(&format!("localhost:{}", PORT))
        .expect(&format!("Cannot bind to port {}", PORT))
        .run();
}
