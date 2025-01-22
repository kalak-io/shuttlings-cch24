use actix_web::{
    get,
    http::header,
    post,
    web::{Query, ServiceConfig},
    HttpResponse, Result,
};
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::{Ipv4Addr, Ipv6Addr};

mod utils;
use utils::ip::{add_ip_addresses, subtract_ip_addresses, xor_ipv6_addresses};

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

#[derive(Deserialize)]
struct Order {
    item: String,
    quantity: u32,
}
impl Display for Order {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}: {}", self.item, self.quantity)
    }
}

#[get("/2/dest")]
async fn dest_v4(query: Query<DestQuery<Ipv4Addr>>) -> Result<String> {
    let result = add_ip_addresses(query.from, query.key);
    Ok(format!("{}", result))
}

#[get("/2/key")]
async fn key_v4(query: Query<KeyQuery<Ipv4Addr>>) -> Result<String> {
    let result = subtract_ip_addresses(query.to, query.from);
    Ok(format!("{}", result))
}

#[get("/2/v6/dest")]
async fn dest_v6(query: Query<DestQuery<Ipv6Addr>>) -> Result<String> {
    let result = xor_ipv6_addresses(query.from, query.key);
    Ok(format!("{}", result))
}

#[get("/2/v6/key")]
async fn key_v6(query: Query<KeyQuery<Ipv6Addr>>) -> Result<String> {
    let result = xor_ipv6_addresses(query.from, query.to);
    Ok(format!("{}", result))
}

#[post("/5/manifest")]
async fn manifest() -> HttpResponse {
    // this endpoint receive data from the client
    // this data could be a `Content-Type: application/toml`
    // or `Content-Type: application/json`
    // the data need to be saved in the appropriate format file
    todo!()
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
