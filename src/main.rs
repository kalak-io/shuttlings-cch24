use actix_web::{get, web::{ServiceConfig,Query}, Result, HttpResponse, http::header};
use shuttle_actix_web::ShuttleActixWeb;
use serde::Deserialize;
use std::net::{Ipv4Addr, Ipv6Addr};

mod ip_utils;
#[cfg(test)]
mod tests;

#[get("/")]
async fn hello_bird() -> &'static str {
    "Hello, bird!"
}

#[get("/-1/seek")]
async fn seek() -> HttpResponse {
    HttpResponse::Found()
        .append_header((header::LOCATION, "https://www.youtube.com/watch?v=9Gc4QTqslN4"))
        .finish()
}

#[derive(Deserialize)]
struct DestV4Query {
    from: Ipv4Addr,
    key: Ipv4Addr,
}

#[derive(Deserialize)]
struct KeyV4Query {
    from: Ipv4Addr,
    to: Ipv4Addr,
}

#[derive(Deserialize)]
struct DestV6Query {
    from: Ipv6Addr,
    key: Ipv6Addr,
}

#[derive(Deserialize)]
struct KeyV6Query {
    from: Ipv6Addr,
    to: Ipv6Addr,
}

#[get("/2/dest")] 
async fn dest_v4(query: Query<DestV4Query>) -> Result<String> {
    let result = ip_utils::add_ip_addresses(query.from, query.key);
    Ok(format!("{}", result))
}

#[get("/2/key")] 
async fn key_v4(query: Query<KeyV4Query>) -> Result<String> {
    let result = ip_utils::subtract_ip_addresses(query.from, query.to);
    Ok(format!("{}", result))
}

#[get("/2/v6/dest")]
async fn dest_v6(query: Query<DestV6Query>) -> Result<String> {
    let result = ip_utils::xor_ipv6_addresses(query.from, query.key);
    Ok(format!("{}", result))
}

#[get("/2/v6/key")]
async fn key_v6(query: Query<KeyV6Query>) -> Result<String> {
    let result = ip_utils::xor_ipv6_addresses(query.from, query.to);
    Ok(format!("{}", result))
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_bird).service(seek).service(dest_v4).service(key_v4).service(dest_v6).service(key_v6);
    };

    Ok(config.into())
}
