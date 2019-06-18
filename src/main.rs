
extern crate futures;
extern crate hyper;
//#[macro_use]
extern crate juniper;
extern crate juniper_hyper;
extern crate pretty_env_logger;

mod schema;
mod model;

use futures::future;
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::Method;
use hyper::{Body, Response, Server, StatusCode};
//use model::Database;
use schema::Query;
use schema::Schema;
use schema::Mutation;
//use juniper::tests::model::Database;
//use juniper::tests::schema::Query;
//use juniper::EmptyMutation;
//use juniper::RootNode;
use std::sync::Arc;

fn main() {
    pretty_env_logger::init();

    let addr = ([127, 0, 0, 1], 3000).into();

    let cx = schema::Context{};
    let db = Arc::new(cx);
    //let root_node = Arc::new(RootNode::new(Query, EmptyMutation::<Database>::new()));
    let root_node = Arc::new(Schema::new(Query, Mutation));

    let new_service = move || {
        let root_node = root_node.clone();
        let ctx = db.clone();
        //let ctx = schema::Context{};
        service_fn(move |req| -> Box<dyn Future<Item = _, Error = _> + Send> {
            let root_node = root_node.clone();
            let ctx = ctx.clone();
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => Box::new(juniper_hyper::graphiql("/graphql")),
                (&Method::GET, "/graphql") => Box::new(juniper_hyper::graphql(root_node, ctx, req)),
                (&Method::POST, "/graphql") => {
                    Box::new(juniper_hyper::graphql(root_node, ctx, req))
                }
                _ => {
                    let mut response = Response::new(Body::empty());
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    Box::new(future::ok(response))
                }
            }
        })
    };
    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));
    println!("Listening on http://{}", addr);

    rt::run(server);
}