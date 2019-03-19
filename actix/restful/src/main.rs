// extern crate actix_web;
// extern crate listenfd;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use actix_web::{dev::Handler, http, server, App, Error, HttpRequest, HttpResponse, Responder};
use listenfd::ListenFd;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct AppState {
    counter: Cell<usize>,
}

struct MyHandler(Arc<AtomicUsize>);

impl<S> Handler<S> for MyHandler {
    type Result = HttpResponse;

    /// Handle request
    fn handle(&self, _req: &HttpRequest<S>) -> Self::Result {
        self.0.fetch_add(1, Ordering::Relaxed);
        HttpResponse::Ok().into()
    }
}

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

impl Responder for MyObj {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn index(req: &HttpRequest<AppState>) -> impl Responder {
    let count = req.state().counter.get() + 1;
    req.state().counter.set(count);

    format!("hello world,xiang min liu! you visited here {}", count)
}

fn json_test(_req: &HttpRequest<AppState>) -> impl Responder {
    MyObj { name: "小牛哥"}
}

fn main() {
    let inc_cloned = Arc::new(AtomicUsize::new(0));

    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(move || {
        let inc_cloned = inc_cloned.clone();

        App::with_state(AppState { counter: Cell::new(0) })
            .resource("/", |r| r.method(http::Method::GET).f(index))
            .resource("/inc-test", move |r| r.h(MyHandler(inc_cloned)))
            .resource("/json-test", |r| r.method(http::Method::GET).f(json_test))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:8888").unwrap()
    };

    server.run();
}
