use api::config::Config;

fn clear_env() {
    std::env::remove_var("SERVER_ADDR");
    std::env::remove_var("OLLAMA_URL");
    std::env::remove_var("DATABASE_URL");
    std::env::remove_var("JWT_SECRET");
}

use serial_test::serial;

#[test]
#[serial]
fn from_env_uses_defaults() {
    clear_env();

    let cfg = Config::from_env();
    assert_eq!(cfg.server_addr, "0.0.0.0:3001");
    assert_eq!(cfg.ollama_url, "http://ollama:11434/");
    assert_eq!(
        cfg.database_url,
        "postgresql://postgres:postgres@localhost:5432/sya_db"
    );
    // JWT_SECRET should be a 32-character random string when not set
    assert_eq!(cfg.jwt_secret.len(), 32);
    // Should only contain alphanumeric characters
    assert!(cfg.jwt_secret.chars().all(|c| c.is_alphanumeric()));
}

#[test]
#[serial]
fn from_env_reads_variables() {
    std::env::set_var("SERVER_ADDR", "1.2.3.4:8080");
    std::env::set_var("OLLAMA_URL", "http://example.com/");
    std::env::set_var("DATABASE_URL", "postgres://u:p@/db");
    std::env::set_var("JWT_SECRET", "mysecret12345678901234567890");

    let cfg = Config::from_env();
    assert_eq!(cfg.server_addr, "1.2.3.4:8080");
    assert_eq!(cfg.ollama_url, "http://example.com/");
    assert_eq!(cfg.database_url, "postgres://u:p@/db");
    assert_eq!(cfg.jwt_secret, "mysecret12345678901234567890");

    clear_env();
}
