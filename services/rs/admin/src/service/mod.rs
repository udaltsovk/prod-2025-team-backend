use error::ServiceError;
use protos::admin::{
    admin_server::Admin, AdminRequest, AdminResponse, AuthResponse, ChangePasswordRequest,
    EditRequest, LoginRequest, RegisterRequest, ValidateTokenRequest,
};
use service_helper::response::ServiceResult;
use sqlx::{Pool, Postgres};
use tonic::{async_trait, Request, Response};

mod change_password;
mod delete;
mod edit;
mod error;
mod get;
mod login;
mod register;
mod validate_token;

pub struct AdminService {
    pub postgres_pool: Pool<Postgres>,
}
impl AdminService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            postgres_pool: pool,
        }
    }
}

#[async_trait]
impl Admin for AdminService {
    async fn register(&self, request: Request<RegisterRequest>) -> ServiceResult<AuthResponse> {
        register::handle(request.into_inner(), &self.postgres_pool)
            .await
            .map(Response::new)
            .map_err(ServiceError::into)
    }

    async fn login(&self, request: Request<LoginRequest>) -> ServiceResult<AuthResponse> {
        login::handle(request.into_inner(), &self.postgres_pool)
            .await
            .map(Response::new)
            .map_err(ServiceError::into)
    }

    async fn validate_token(
        &self,
        request: Request<ValidateTokenRequest>,
    ) -> ServiceResult<AdminResponse> {
        validate_token::handle(request.into_inner(), &self.postgres_pool)
            .await
            .map(Response::new)
            .map_err(ServiceError::into)
    }

    async fn change_password(
        &self,
        request: Request<ChangePasswordRequest>,
    ) -> ServiceResult<AuthResponse> {
        change_password::handle(request.into_inner(), &self.postgres_pool)
            .await
            .map(Response::new)
            .map_err(ServiceError::into)
    }

    async fn get(&self, request: Request<AdminRequest>) -> ServiceResult<AdminResponse> {
        get::handle(request.into_inner(), &self.postgres_pool)
            .await
            .map(Response::new)
            .map_err(ServiceError::into)
    }

    async fn edit(&self, request: Request<EditRequest>) -> ServiceResult<AdminResponse> {
        edit::handle(request.into_inner(), &self.postgres_pool)
            .await
            .map(Response::new)
            .map_err(ServiceError::into)
    }

    async fn delete(&self, request: Request<AdminRequest>) -> ServiceResult<()> {
        delete::handle(request.into_inner(), &self.postgres_pool)
            .await
            .map(Response::new)
            .map_err(ServiceError::into)
    }
}
