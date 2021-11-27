use taskmanager::server;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter("taskmanager=trace")
        .with_target(true)
        .init();

    let signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for event");
        tracing::info!("receive sigint");
    };

    let addr = ([127, 0, 0, 1], 8001);

    server::run(addr, signal).await
}
