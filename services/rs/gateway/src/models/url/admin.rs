use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct AdminPath {
    pub admin_id: Uuid,
}
