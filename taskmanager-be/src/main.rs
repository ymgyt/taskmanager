use taskmanager::server;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(true)
        .init();

    /*
    let signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for event");
        tracing::info!("receive sigint");
    };

    let addr = ([127, 0, 0, 1], 8001);

    server::run(addr, signal).await
     */

    db_handson().await
}

async fn db_handson() -> Result<(), anyhow::Error> {
    use sea_orm::{Database,ConnectOptions};

    let mut opt = ConnectOptions::new("postgresql://taskmanager:secret@localhost:5432/taskmanager".to_owned());

    opt.sqlx_logging(true);

    let db = Database::connect(opt).await?;

    use taskmanager::db::entity::prelude::*;

    Ok(())
}
