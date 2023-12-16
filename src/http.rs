use core::net::{IpAddr, Ipv4Addr, SocketAddr};
use esp_async_http_server::Response;

pub async fn start_http_server() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);
    let listener = esp_async_tcp::TcpListener::bind(&addr).unwrap();
    if let Err(err) = esp_async_http_server::serve(listener, router).await {
        log::error!("serve error: {err:?}")
    }
}

async fn router(
    addr: SocketAddr,
    req: esp_async_http_server::Request<'_, '_>,
    body: &[u8],
) -> Response {
    match req.path {
        None => "400".into(),
        Some(p) if p.starts_with("/api/start") => start(addr, req, body).await.into(),
        Some(p) if p.starts_with("/api/stop") => stop(addr, req, body).await.into(),
        Some(_) => index().await.into(),
    }
}

async fn start(
    _addr: SocketAddr,
    _req: esp_async_http_server::Request<'_, '_>,
    _body: &[u8],
) -> &'static str {
    log::info!("start");
    crate::ble::start();
    "start"
}

async fn stop(
    _addr: SocketAddr,
    _req: esp_async_http_server::Request<'_, '_>,
    _body: &[u8],
) -> &'static str {
    log::info!("stop");
    crate::ble::stop();
    "stop"
}

async fn index() -> &'static str {
    include_str!("../vue-ui/dist/index.html")
}
