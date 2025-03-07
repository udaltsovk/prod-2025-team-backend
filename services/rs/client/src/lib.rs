use env_config::environment_variables;

pub mod models;
pub mod service;
pub mod utils;

environment_variables! {
    SERVICE_ADDRESS: String =  "[::1]:50052",
    DATABASE_URL: String = "postgres://postgres:postgres@localhost:5432/client",
    JWT_SECRET: String = "ohrfwahl;fhjjhawefhjaewfjhhjawfjbklbjlhjeawfjhjhwarjhjhhawhfhjhjfwahl",
    RESERVATION_SERVICE_ADDRESS: String = "localhost:50054",
}
