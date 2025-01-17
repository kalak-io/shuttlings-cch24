use actix_web::{
    get,
    http::header,
    web::{Query, ServiceConfig},
    HttpResponse, Result,
};
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;
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
        .append_header((
            header::LOCATION,
            "https://www.youtube.com/watch?v=9Gc4QTqslN4",
        ))
        .finish()
}

#[derive(Deserialize)]
struct DestQuery<T> {
    from: T,
    key: T,
}

#[derive(Deserialize)]
struct KeyQuery<T> {
    from: T,
    to: T,
}

#[get("/2/dest")]
async fn dest_v4(query: Query<DestQuery<Ipv4Addr>>) -> Result<String> {
    let result = ip_utils::add_ip_addresses(query.from, query.key);
    Ok(format!("{}", result))
}

#[get("/2/key")]
async fn key_v4(query: Query<KeyQuery<Ipv4Addr>>) -> Result<String> {
    let result = ip_utils::subtract_ip_addresses(query.to, query.from);
    Ok(format!("{}", result))
}

#[get("/2/v6/dest")]
async fn dest_v6(query: Query<DestQuery<Ipv6Addr>>) -> Result<String> {
    let result = ip_utils::xor_ipv6_addresses(query.from, query.key);
    Ok(format!("{}", result))
}

#[get("/2/v6/key")]
async fn key_v6(query: Query<KeyQuery<Ipv6Addr>>) -> Result<String> {
    let result = ip_utils::xor_ipv6_addresses(query.from, query.to);
    Ok(format!("{}", result))
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_bird)
            .service(seek)
            .service(dest_v4)
            .service(key_v4)
            .service(dest_v6)
            .service(key_v6);
    };

    Ok(config.into())
}
