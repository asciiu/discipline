use juniper::{FieldResult};
use crate::DbConPool;
use crate::create_user;
use crate::models;

#[derive(juniper::GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description="A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

// There is also a custom derive for mapping GraphQL input objects.
#[derive(juniper::GraphQLInputObject)]
#[graphql(description="New user")]
struct NewUser {
    username: String,
    email: String,
    password: String,
}

// Now, we create our root Query and Mutation types with resolvers by using the
// object macro.
// Objects can have contexts that allow accessing shared state like a database
// pool.

pub struct Context {
    // Use your real database pool here.
    //conn: std::sync::Arc<PgConnection>,
    pub pool: DbConPool,
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    Context = Context,
)]
impl Query {

    fn apiVersion() -> &str {
        "1.0"
    }

    // Arguments to resolvers can either be simple types or input objects.
    // To gain access to the context, we specify a argument
    // that is a reference to the Context type.
    // Juniper automatically injects the correct context here.
    fn human(context: &Context, id: String) -> FieldResult<Human> {
        // Get a db connection.
        //let connection = context.pool.get_connection()?;
        // Execute a db query.
        // Note the use of `?` to propagate errors.
        //let human = connection.find_human(&id)?;
        let human = Human{
            id: String::from("123"),
            name: String::from("Luke"),
            appears_in: vec![Episode::NewHope],
            home_planet: String::from("hoth"),
        };
        // Return the result.
        Ok(human)
    }
}

// Now, we do the same for our Mutation type.

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {

    fn createUser(context: &Context, new_user: NewUser) -> FieldResult<models::User> {
        let conn = context.pool.get().unwrap();
        let user = create_user(&conn, &new_user.username, &new_user.email, &new_user.password);
        //let db = executor.context().pool.get_connection()?;
        //let human: Human = db.insert_human(&new_human)?;
        //let human = Human{
        //    id: String::from("123"),
        //    name: new_human.name,
        //    appears_in: new_human.appears_in,
        //    home_planet: new_human.home_planet,
        //};
        Ok(user)
    }
}

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation>;