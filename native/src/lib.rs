use axum::{
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use flame_macro::flame;
use serde::Serialize;
use std::{mem, net::SocketAddr};

pub struct FlameServer {
    router: Router,
    port: u16,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub body: String,
}

/// Create a new Flame server.
pub fn init() -> FlameServer {
    FlameServer {
        router: Router::new(),
        port: 3000,
    }
}

impl FlameServer {
    pub fn set_port(&mut self, port: i64) {
        self.port = port as u16;
    }
    pub fn get<H, T>(&mut self, path: &'static str, handler: H)
    where
        H: axum::handler::Handler<T, ()> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        self.router = mem::take(&mut self.router).route(path, get(handler));
    }

    pub fn post<H, T>(&mut self, path: &'static str, handler: H)
    where
        H: axum::handler::Handler<T, ()> + Clone + Send + Sync + 'static,
        T: 'static,
    {
        self.router = mem::take(&mut self.router).route(path, post(handler));
    }

    pub fn router(&mut self) -> Router {
        mem::take(&mut self.router)
    }

    #[flame(daemon)]
    pub async fn listen(self) -> std::io::Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

        println!();
        println!("🔥 Flamer");
        println!("────────────────────────────────");
        println!("✓ Running on http://localhost:{}", self.port);
        println!("✓ Environment: Development");
        println!("✓ Press Ctrl+C to stop");
        println!();

        let listener = tokio::net::TcpListener::bind(addr).await?;

        axum::serve(listener, self.router)
            .await
            .map_err(std::io::Error::other)
    }
}
