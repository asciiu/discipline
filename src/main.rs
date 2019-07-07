
extern crate diesel;
extern crate dotenv;
extern crate futures;
extern crate hyper;
extern crate jsonwebtoken as jwt;
//#[macro_use]
extern crate juniper;
extern crate juniper_hyper;
extern crate pretty_env_logger;

pub mod graphql;

use futures::future;
use graphql::{Mutation, Query, Schema};
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::{Body, Method, Response, Server, StatusCode};
use std::sync::Arc;
use discipline::*;

//fn with_auth(root_node: Arc<Schema>, context: Arc<graphql::Context>, req: Request<Body>) -> BoxFut {
//    Box::new(juniper_hyper::graphql(root_node, context, req))
//}

fn main() {
    pretty_env_logger::init();

    let cx = graphql::Context{pool: db_pool()};
    let context = Arc::new(cx);
    let root_node = Arc::new(Schema::new(Query, Mutation));

    let new_service = move || {
        let root_node = root_node.clone();
        let ctx = context.clone();
        service_fn(move |req| -> BoxFut {
            let root_node = root_node.clone();
            let ctx = ctx.clone();
            let mut response = Response::new(Body::empty());

            if req.headers().contains_key("Authorization") {
                let auth_str = req.headers()["Authorization"].to_str().unwrap();
                let jwt = String::from(auth_str).replace("Bearer ", "");

                println!("{}", jwt);
            }
            if req.headers().contains_key("Refresh") {
                let refresh = req.headers()["Refresh"].to_str().unwrap();
                println!("{}", refresh);
            }

            //(*response.headers_mut()).append("set-auth", "auth".parse().unwrap());
            //(*response.headers_mut()).append("set-refresh", "refresh".parse().unwrap());

            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => Box::new(juniper_hyper::graphiql("/graphql")),
                (&Method::GET, "/graphql") => Box::new(juniper_hyper::graphql(root_node, ctx, req)),
                (&Method::POST, "/graphql") => Box::new(juniper_hyper::graphql(root_node, ctx, req)),
                _ => {
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    Box::new(future::ok(response))
                }
            }
        })
    };

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));
    println!("Listening on http://{}", addr);

    rt::run(server);
}