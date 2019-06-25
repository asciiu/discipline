use juniper::{FieldResult};
use crate::DbConPool;
use crate::create_user;
use crate::models;
use diesel::prelude::*;

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

// Now, we create our root Query and Mutation types with resolvers by using the
// object macro.
// Objects can have contexts that allow accessing shared state like a database
// pool.

pub struct Context {
    // Use your real database pool here.
    //conn: std::sync::Arc<PgConnection>,
    pub pool: DbConPool,
}

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

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {

    fn signup(context: &Context, email: String, username: String, password: String) -> FieldResult<models::User> {
        let conn = context.pool.get().unwrap();
        match create_user(&conn, &email, &username, &password) {
            Ok(user) => Ok(user),
            Err(e) => Err(e)?
        }
    }

    fn login(context: &Context, email: String, password: String, remember: bool) -> FieldResult<models::User> {
        use crate::schema::users::dsl;
        use bcrypt::verify;

        let conn = context.pool.get().unwrap();
        let user = dsl::users
                   .filter(dsl::email.eq(email))
                   .first::<models::User>(&conn)
                   .optional()?;
        
        match user {
            Some(u) => {
                match verify(&password, &u.password_hash) {
                    Ok(r) if r => {
                        // TODO create jwt
                        // TODO create refresh token
                        Ok(u)
                    }
                    _ => Err("incorrect email/password")?

                }
            },
            None => Err("incorrect email/password")?
        }
    }
}

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation>;