use env_config::environment_variables;

pub mod service;

environment_variables! {
    SERVICE_ADDRESS: String =  "[::1]:50057",
    COWORKING_ID: String = "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    CLIENT_SERVICE_ADDRESS: String = "localhost:50052",
    COWORKING_SERVICE_ADDRESS: String = "localhost:50053",
    RESERVATION_SERVICE_ADDRESS: String = "localhost:50054",
    MAIL_SERVICE_ADDRESS: String = "localhost:50056",
}
