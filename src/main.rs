use concact::app;



#[tokio::main]
async fn main() {
    app::bootstrap().await;
}
