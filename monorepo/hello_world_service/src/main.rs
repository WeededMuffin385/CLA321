use tokio_util::sync::CancellationToken;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod app;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    
    let shutdown = CancellationToken::new();
    let shutdown_tmp = shutdown.clone();
    let worker = tokio::spawn(async move {
        let shutdown = shutdown_tmp;
        
        loop {
            tokio::select! {
                _ = shutdown.cancelled() => {
                    info!("worker shutdown");
                    break;
                }
                
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)) => {
                    info!("tick");
                }
            }
        }
    });

    info!("Hello, World!");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app::router()).with_graceful_shutdown({
        let shutdown = shutdown.clone();
        
        async move {
            tokio::signal::ctrl_c().await.unwrap();
            info!("shutdown signal received");
            shutdown.cancel();
        }
    }).await.unwrap();

    worker.await.unwrap();
    info!("shutdown complete");
}
