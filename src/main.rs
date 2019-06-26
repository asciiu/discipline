
extern crate diesel;
extern crate dotenv;
extern crate futures;
extern crate hyper;
extern crate jsonwebtoken as jwt;
//#[macro_use]
extern crate juniper;
extern crate juniper_hyper;
extern crate pretty_env_logger;

#[macro_use] 
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod graphql;

use futures::future;
use graphql::{Mutation, Query, Schema};
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::{Body, Method, Response, Server, StatusCode};
use jwt::{encode, decode, Header, Algorithm, Validation};
use std::sync::Arc;
use discipline::*;


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

fn main() {
    pretty_env_logger::init();

    dotenv::dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT secret not set");
    let my_claims =
        Claims { sub: "b@b.com".to_owned(), company: "ACME".to_owned(), exp: 10000000000 };
    let token = match encode(&Header::default(), &my_claims, secret.as_ref()) {
        Ok(t) => t,
        Err(_) => panic!("could not encode jwt"),
    };
    let validation = Validation { sub: Some("b@b.com".to_string()), ..Validation::default() };
    let token_data = match decode::<Claims>(&token, secret.as_ref(), &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            jwt::errors::ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            jwt::errors::ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            _ => panic!("Some other errors"),
        },
    };
    println!("{:?}", token_data.claims);
    println!("{:?}", token_data.header);

    let addr = ([127, 0, 0, 1], 3000).into();
    let cx = graphql::Context{pool: db_pool()};
    let context = Arc::new(cx);
    let root_node = Arc::new(Schema::new(Query, Mutation));

    let new_service = move || {
        let root_node = root_node.clone();
        let ctx = context.clone();
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