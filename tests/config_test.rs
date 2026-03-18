use localhost::config::{Args, Config};

#[test]
fn test_default_config() {
    let config = Config::default();
    assert_eq!(config.host, "0.0.0.0");
    assert_eq!(config.port, 8080);
}

#[test]
fn test_config_address() {
    let config = Config {
        host: "127.0.0.1".to_string(),
        port: 3000,
    };
    assert_eq!(config.address(), "127.0.0.1:3000");
}

#[test]
fn test_config_from_args_default() {
    let args = Args {
        port: 8080,
        host: "0.0.0.0".to_string(),
    };
    let config = Config::from_args(args);
    assert_eq!(config.host, "0.0.0.0");
    assert_eq!(config.port, 8080);
}

#[test]
fn test_config_from_args_custom() {
    let args = Args {
        port: 9000,
        host: "127.0.0.1".to_string(),
    };
    let config = Config::from_args(args);
    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, 9000);
}

#[test]
fn test_config_clone() {
    let config = Config::default();
    let cloned = config.clone();
    assert_eq!(config.host, cloned.host);
    assert_eq!(config.port, cloned.port);
}
