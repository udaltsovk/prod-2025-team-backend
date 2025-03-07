pub mod response;

pub fn make_url(address: &str) -> String {
    format!("http://{address}")
}
