use dotenv::dotenv;
pub mod server_queries;

// Loads environment variables from .env file
pub fn load_env_from_file() {
    dotenv().expect("Failed to load .env file");
}
