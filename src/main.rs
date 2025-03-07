mod error;
mod model;

pub use self::model::*;

pub use self::error::Result;
use axum::{extract::State, routing::get, Router};
use axum_client_ip::{SecureClientIp, SecureClientIpSource};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let mm = ModelManager::new().await?;

    let app = Router::new()
        .route("/ping", get(ping))
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .with_state(mm.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let one = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    );

    let two = tokio::spawn(async move {
        let store = Arc::clone(&mm.pings_count);

        loop {
            {
                let a = store.read().await;
                println!("IPs: ");
                let mut v = Vec::from_iter(a.iter());
                v.sort_by(|&(_, a), &(_, b)| b.cmp(a));

                for (ip, count) in v.iter() {
                    println!("{}: {}", ip, count);
                }
                println!("=========== \n");
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    let _ = tokio::join!(one, two);

    Ok(())
}

pub async fn ping(State(state): State<ModelManager>, secure_ip: SecureClientIp) -> Result<String> {
    Ok(state.add_request(secure_ip.0).await?)
}
