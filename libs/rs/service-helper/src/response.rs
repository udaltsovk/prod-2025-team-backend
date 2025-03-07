use tonic::{Response, Status};

pub type ServiceResult<T> = Result<Response<T>, Status>;
