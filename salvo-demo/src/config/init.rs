use crate::config::mysql::init_mysql;
use crate::config::router::init_router;
use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::conn::{QuinnListener, TcpListener};
use salvo::{Listener, Router, Server};

#[inline]
async fn use_http1(router: Router) {
    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}

#[allow(dead_code)]
async fn use_http3(router: Router) {
    let cert = include_bytes!("../../cert/cert.pem").to_vec();
    let key = include_bytes!("../../cert/key.pem").to_vec();
    let config = RustlsConfig::new(Keycert::new().cert(cert.as_slice()).key(key.as_slice()));
    let listener = TcpListener::new(("0.0.0.0", 5800)).rustls(config.clone());
    let acceptor = QuinnListener::new(config, "0.0.0.0:5800")
        .join(listener)
        .bind()
        .await;
    Server::new(acceptor).serve(router).await;
}

#[inline]
pub async fn init_salvo_framework() {
    tracing_subscriber::fmt().init();
    // init_mysql().await;

    // router
    let router = init_router();

    tracing::info!("Listening on http://127.0.0.1:5800");
    use_http1(router).await;
}
