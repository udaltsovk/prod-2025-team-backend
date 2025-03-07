use protos::{
    client::ClientsRequest,
    coworking::GetSeatsRequest,
    reservation::{GetByClientRequest, GetRequest},
};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

use crate::models::Date;

#[derive(Deserialize, IntoParams, ToSchema, Validate, Debug)]
pub struct Pagination {
    #[validate(range(min = 0))]
    #[schema(format = UInt32, minimum = 0, default = 7, examples(7))]
    pub limit: Option<u32>,

    #[validate(range(min = 0))]
    #[schema(format = UInt64, minimum = 0, default = 0, examples(1))]
    pub offset: Option<u64>,
}
impl Pagination {
    pub fn parse(self) -> (u32, u64) {
        let limit = match self.limit {
            Some(limit) if limit >= 57 => 57,
            Some(limit) => limit,
            None => 7,
        };
        let offset = match self.offset {
            Some(offset) => offset,
            None => 0,
        };
        (limit, offset)
    }
    pub fn into_proto(self, id: Uuid, is_admin: bool) -> GetByClientRequest {
        let (limit, offset) = self.parse();

        GetByClientRequest {
            client_id: id.to_string(),
            is_admin,
            limit,
            offset,
        }
    }
}
impl Into<GetSeatsRequest> for Pagination {
    fn into(self) -> GetSeatsRequest {
        let (limit, offset) = self.parse();

        GetSeatsRequest { limit, offset }
    }
}

#[derive(Deserialize, IntoParams, ToSchema, Validate, Debug)]
pub struct ByDateWithPaginationQuery {
    #[validate(range(min = 0))]
    #[schema(format = UInt32, minimum = 0, default = 7, examples(7))]
    pub limit: Option<u32>,

    #[validate(range(min = 0))]
    #[schema(format = UInt64, minimum = 0, default = 0, examples(1))]
    pub offset: Option<u64>,

    #[validate(range(min = 1, max = 31))]
    #[schema(format = UInt32, minimum = 1, maximum = 31, example = 1)]
    pub day: Option<u32>,

    #[validate(range(min = 1, max = 12))]
    #[schema(format = UInt32, minimum = 1, maximum = 12, example = 1)]
    pub month: Option<u32>,

    #[validate(range(min = 2025))]
    #[schema(format = UInt64, minimum = 2025, example = 2025)]
    pub year: Option<u64>,
}
impl ByDateWithPaginationQuery {
    pub fn fields(self) -> (u32, u64, Option<u32>, Option<u32>, Option<u64>) {
        let (limit, offset) = Pagination {
            limit: self.limit,
            offset: self.offset,
        }
        .parse();

        let date = match (self.day, self.month, self.year) {
            (Some(day), Some(month), Some(year)) => Some(Date { day, month, year }),
            _ => None,
        };

        let (day, month, year) = match date {
            Some(Date { day, month, year }) => (Some(day), Some(month), Some(year)),
            None => (None, None, None),
        };

        (limit, offset, day, month, year)
    }
}
impl Into<GetRequest> for ByDateWithPaginationQuery {
    fn into(self) -> GetRequest {
        let (limit, offset, day, month, year) = self.fields();

        GetRequest {
            limit,
            offset,
            day,
            month,
            year,
        }
    }
}
impl Into<ClientsRequest> for ByDateWithPaginationQuery {
    fn into(self) -> ClientsRequest {
        let (limit, offset, day, month, year) = self.fields();

        ClientsRequest {
            limit,
            offset,
            day,
            month,
            year,
        }
    }
}
