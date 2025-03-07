use utoipa::openapi::security::{Http, HttpAuthScheme};
use utoipa::{
    openapi::{security::SecurityScheme, OpenApi as OpenApiStruct},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi, Debug)]
#[openapi(
    info(
        title = "T-lounge Gateway API",
        description = "OpenAPI for the T-lounge service",
        version = "0.1.0",
        contact(
            name = "T-lounge",
            url = "https://prod-team-8-t7nj4g3c.final.prodcontest.ru"
        )
    ),
    servers(
        (
            url = "https://prod-team-8-t7nj4g3c.final.prodcontest.ru/",
            description = "PROOOOD"
        ),
        (
            url = "http://localhost:8080/",
            description = "Local server"
        )
    ),
    tags(
        (
            name = "health",
            description = "Endpoints for the monitoring"
        ),
        (
            name = "clients",
            description = "Client accounts-related endpoints"
        )
    ),
    modifiers(&Security)
)]
pub struct Swagger;
impl Swagger {
    pub fn ui_service(api: OpenApiStruct) -> SwaggerUi {
        SwaggerUi::new("/swagger-ui/{_}*").url("/openapi.json", api)
    }
}

struct Security;
impl Modify for Security {
    fn modify(&self, openapi: &mut OpenApiStruct) {
        let components = openapi.components.as_mut().unwrap();

        let scheme = SecurityScheme::Http(
            Http::builder()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        );

        components.add_security_scheme("client", scheme.clone());
        components.add_security_scheme("admin", scheme.clone());
    }
}
