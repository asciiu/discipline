
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


// redis example
extern crate redis;
use redis::Commands;

fn example_redis() -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _ : () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}


fn main() {
    pretty_env_logger::init();

    match example_redis() {
        Ok(r) => println!("{}", r),
        Err(_) => println!("nope"),
    }

    // let token = create_jwt("test");
    // println!("{}: ", token);
    // let token_data = match validate_jwt(token) {
    //     Ok(c) => c,
    //     Err(err) => match *err.kind() {
    //         jwt::errors::ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
    //         jwt::errors::ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
    //         jwt::errors::ErrorKind::ExpiredSignature => panic!("token expired"),
    //         _ => panic!("Some other errors"),
    //     },
    // };
    // println!("{:?}", token_data.claims);
    // println!("{:?}", token_data.header);

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