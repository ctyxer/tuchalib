pub struct APIKeys {
    pub api_id: i32,
    pub api_hash: String,
}

impl APIKeys {
    pub fn new(api_id: i32, api_hash: &str) -> Self {
        Self {
            api_id,
            api_hash: api_hash.to_string(), 
        }
    }
}
