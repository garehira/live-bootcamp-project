use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
lazy_static! {
    pub static ref JWT_SECRET: String = "8w8Pu987O+mvZq5573gvwMNfMSzF6QX6ZIxhjuYtT91iD0UGN9U+GOi2LU4hUA0PGkoizJgOlgU1JyWKQRPweg==".to_string();
    pub static ref DATABASE_URL:String = "Postgres://postgres:POST123@postgres:5432".to_string();
     // pub static ref DATABASE_URL:String = "Postgres://postgres:POST123@127.0.0.1:5434".to_string();
    // #set_token();
}

fn _set_token() -> String {
    dotenv().ok(); // Load environment variables
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    secret
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
}

pub const JWT_COOKIE_NAME: &str = "jwt";
pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}
// #[test]
// pub fn testit() {
//     let token = _set_token();
//     assert!(token.len() > 0);
// }
