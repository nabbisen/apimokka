#[tokio::main]
async fn main() {
    #[cfg(not(feature = "napi"))]
    apimokka::run().await;
}
