// pub mod config;
// pub mod app;
pub mod healthcheck;


pub fn gen_route(root: axum::Router) -> axum::Router {
    root.nest("/healthcheck", healthcheck::router::router())
}

pub async fn bootstrap() {
    let mut root_app = axum::Router::new();
    // root_app = root_app.nest("/healthcheck", healthcheck::router::router());
    root_app = gen_route(root_app);
    let listener =  match create_listener().await {
        Ok(listener) => {
            listener   
        },
        Err(err) => {
            panic!("Error creating listener {}", err)
        }
    };

    let _ = axum::serve(listener, root_app).await;
    
}

async fn create_listener() -> Result<tokio::net::TcpListener, std::io::Error> {
    tokio::net::TcpListener::bind("127.0.0.1:3000").await
}