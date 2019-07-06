use crate::*;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use juniper::{FieldResult};

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

    fn login(context: &Context, email: String, password: String, remember: bool) -> FieldResult<models::AuthToken> {
        use crate::schema::users::dsl;
        use bcrypt::verify;

        let conn = context.pool.get().unwrap();
        let user_opt = dsl::users
                   .filter(dsl::email.eq(email))
                   .first::<models::User>(&conn)
                   .optional()?;
        
        match user_opt {
            Some(user) => {
                match verify(&password, &user.password_hash) {
                    Ok(is_valid) if is_valid => {
                        let mut tokies = models::AuthToken{
                            jwt: create_jwt(&user.id.to_string()),
                            refresh: String::from(""),
                        };

                        if remember {
                            let now = Utc::now();
                            let expires = (now + Duration::hours(24)).naive_utc();
                            let fresh_tokie = models::refresh::RefreshToken::new(user.id, expires);
                            tokies.refresh = String::from("refresh token");
                        }
                        // TODO create jwt
                        // TODO create refresh token
                        Ok(tokies)
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