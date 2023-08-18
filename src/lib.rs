use dotenv::dotenv;

// Loads environment variables from .env file
pub fn load_env_from_file() {
    dotenv().expect("Failed to load .env file 🔴");
}
