use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct ClientPath {
    pub client_id: Uuid,
}
