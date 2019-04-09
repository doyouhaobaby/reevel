// extern crate actix_web;
// extern crate listenfd;
#![feature(custom_attribute)]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
//use actix_web::{dev::Handler, http, server, App, Error, HttpRequest, HttpResponse, Responder};
use actix_web::*;
use actix_web::{dev::Handler};
use listenfd::ListenFd;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use bytes::Bytes;
use futures::stream::once;
use futures::future::{Future, result};
use std::io;
use failure::Fail;

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

#[derive(Deserialize)]
struct Info {
    userid2: u32,
    friend2: String,
}

#[derive(Deserialize)]
struct QueryInfo {
    username: String,
}

fn index(req: &HttpRequest<AppState>) -> impl Responder {
    let count = req.state().counter.get() + 1;
    req.state().counter.set(count);

    format!("hello world,xiang min liu! you visited here {}", count)
}

fn json_test(_req: &HttpRequest<AppState>) -> impl Responder {
    MyObj { name: "小牛哥"}
}

fn async_test1(_req: &HttpRequest<AppState>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    result(Ok(HttpResponse::Ok()
              .content_type("text/html")
              .body(format!("Hello async xiaoniuge!"))))
           .responder()
}

fn async_test2(_req: &HttpRequest<AppState>) -> Box<Future<Item=&'static str, Error=Error>> {
    result(Ok("welcome async2 xiaoniuge")).
        responder()
}

fn async_test3(_req: &HttpRequest<AppState>) -> HttpResponse {
    let body = once(Ok(Bytes::from_static(b"test async xiaoniuge 3")));

    HttpResponse::Ok()
        .content_type("application/json")
        .body(Body::Streaming(Box::new(body)))
}

fn error_test(_req: &HttpRequest<AppState>) -> Result<Box<Future<Item=HttpResponse, Error=Error>>, Error> {
    //if true {
        Err(error::ErrorBadRequest("bad request"))        
    // } else {
        // Ok(
        //     Box::new(
        //         result(
        //             Ok(HttpResponse::Ok().
        //             content_type("text/html").
        //             body(format!("hello!")))
        //         )
        //     )
        // )
    // }
}

fn path_test(/*_req &HttpRequest<AppState>*/ info: Path<(u32, String)>) -> Result<String> {
    Ok(format!("welcome {} {}", info.1, info.0))
}

fn descrialize_test(info: Path<Info>) -> Result<String> {
    Ok(format!("{} welcome {}", info.userid2, info.friend2))
}

fn query_test(info: Query<QueryInfo>) -> String {
    format!("Welcome {}", info.username)
}

// #[derive(Deserialize)]
// struct FormData {
//     username: String,
// }

// fn form_test(form: Form<FormData>) -> Result<String> {
//     Ok(format!("Welcome {}!", form.username))
// }

fn path_and_query((path, query): (Path<(u32, String)>, Query<QueryInfo>)) -> String {
    format!("Welcome {} {} {}", query.username, path.0, path.1)
}

fn my_test_error(_req: &HttpRequest<AppState>) -> io::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/notFound.html")?)
}

#[derive(Fail, Debug)]
#[fail(display="my error")]
struct MyError {
    name: &'static str,
}

impl error::ResponseError for MyError {}

fn my_test_error2(_req: &HttpRequest<AppState>) -> Result<&'static str, MyError> {
    Err(MyError{name: "test"})
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
            .resource("/async-test1", |r| r.route().a(async_test1))
            .resource("/async-test2", |r| r.route().a(async_test2))
            .resource("/async-test3", |r| r.f(async_test3))
            .resource("/error-test", |r| r.f(error_test))
            .resource("/users/{userid}/{friend}", |r| r.method(http::Method::GET).with(path_test))
            //http://127.0.0.1:9888/users2/2/xianiuge
            .resource("/users2/{userid2}/{friend2}", |r| r.method(http::Method::GET).with(descrialize_test))
            //http://127.0.0.1:9888/query-test?username=xiaoniuge
            .resource("/query-test", |r| r.method(http::Method::GET).with(query_test))
            .resource("/form-test", |r| r.method(http::Method::GET).with(query_test))
            //http://127.0.0.1:9888/helloquery/444/xxxxx?username=xxx
            .resource("/helloquery/{userid}/{friend}", |r| r.method(http::Method::GET).with(path_and_query))
            .resource("/my-error-test", |r| r.f(my_test_error))
            .resource("/my-error-test2", |r| r.f(my_test_error2))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:9888").unwrap()
    };

    server.run();
}
