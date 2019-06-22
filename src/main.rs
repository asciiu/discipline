
extern crate diesel;
extern crate dotenv;
extern crate futures;
extern crate hyper;
//#[macro_use]
extern crate juniper;
extern crate juniper_hyper;
extern crate pretty_env_logger;

pub mod graphql;
//pub mod schema;

use futures::future;
use graphql::{Mutation, Query, Schema};
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::{Body, Method, Response, Server, StatusCode};
use std::sync::Arc;
use discipline::*;
//use self::diesel::prelude::*;

//use models::*;
//use schema::users::dsl::*;

fn main() {
    //use discipline::schema::users::dsl::*;
    //let connection = establish_connection();
    //let user = create_user(&connection, "flowy", "flow@email", "password");
    //println!("{}", user.id);

    //let results = users
    //    .limit(5)
    //    .load::<User>(&connection)
    //    .expect("Error loading users");

    //println!("Displaying {} users", results.len());
    //for user in results {
    //    println!("{}", user.username);
    //    println!("{}", user.email);
    //    println!("{}", user.created_on);
    //    println!("{}", user.updated_on);
    //    match user.deleted_on {
    //        None => println!("not deleted"),
    //        Some(date) => println!("{}", date),
    //    }
    //}

    pretty_env_logger::init();

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