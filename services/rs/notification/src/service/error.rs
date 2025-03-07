use tokio::task::JoinError;
use tonic::{Code, Status};

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("{0}")]
    Service(#[from] Status),

    #[error("Failed to schedule task: {0}")]
    Schedule(#[from] JoinError),
}

impl Into<Status> for ServiceError {
    fn into(self) -> Status {
        if let Self::Service(status) = self {
            return status;
        }

        let code: Code = match self {
            Self::Service(..) => unreachable!(),
            Self::Schedule(..) => Code::Internal,
        };

        Status::new(code, self.to_string())
    }
}
