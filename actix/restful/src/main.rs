// extern crate actix_web;
// extend crate listenfd;
use actix_web::{http, server, App, HttpRequest, Responder};
use listenfd::ListenFd;
use std::cell::Cell;

struct AppState {
    counter: Cell<usize>
}

fn index(req: &HttpRequest<AppState>) -> impl Responder {
    let count = req.state().counter.get() + 1;
    req.state().counter.set(count); 
    
    format!("hello world,xiang min liu! you visited here {}", count)
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App
            ::with_state(AppState { counter: Cell::new(0) })
            .resource("/", |r| r.method(http::Method::GET).f(index))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server
            .bind("127.0.0.1:8888")
            .unwrap()
    };

    server.run();
}