use std::sync::Mutex;

use std::cell::RefCell;
use lazy_static::lazy_static;

use virt::connect::Connect;

use actix_session::Session;


use actix_web::http::StatusCode;
use actix_web::{get, http, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

thread_local!(static QEMU_CONN:RefCell<Connect> = RefCell::new(get_qemu_connection()));

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

fn get_qemu_connection() -> Connect {
    if let Ok(conn) = Connect::open("qemu:///system") {
        return conn;
    } else {
        panic!("Connection established failed.")
    }; 
}

#[get("/api/get_hostname")]
async fn api(session: Session, req: HttpRequest) -> HttpResponse {
    let hostname = QEMU_CONN.with(|qemu_conn_cell| {
        let conn = qemu_conn_cell.borrow_mut();
        conn.get_hostname().unwrap().to_string()
    });

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(hostname)
}