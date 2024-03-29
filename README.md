# Flowii's Castle 

Welcome to Flowii's Castle. I am what I think. If you've come this far, I see you Bean Optimus!

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

Rust v1.37 or better and Postgres v10.

* Install Rust. https://doc.rust-lang.org/book/ch01-01-installation.html
* Install Diesel. https://diesel.rs/guides/getting-started/ 

### Environment setup 

1. Create your own .env file at the project root. Example file

```
DATABASE_URL=postgres://localhost/discipline
JWT_SECRET="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
JWT_EXPIRE_HR=2
```

2. Next run the DB migration.
```
diesel migration run
```

3. Run the service
```
cargo run
```

## Running the tests
```
cargo test -- --nocapture 
```

## Deployment

coming soon!

## Built With

* [Hyper](https://docs.rs/hyper/0.12.31/hyper/) - Async http library 
* [Juniper](https://docs.rs/juniper/0.12.0/juniper/) - GraphQL library 
* [Diesel](https://diesel.rs/) - Rust ORM

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Authors

* **Club Flowii** 

## License

Use at your own risk!

## Acknowledgments

* Special thanks to Flowii's Castle and fans
