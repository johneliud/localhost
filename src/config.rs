use clap::Parser;

/**
 * Command-line arguments for the server.
 */
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /**
     * The port number to listen on.
     */
    #[arg(short, long, default_value = "8080")]
    pub port: u16,

    /**
     * The host address to bind to.
     */
    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,
}

/**
 * Server configuration containing host address and port number settings.
 */
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    /**
     * Creates a new Config from command-line arguments.
     *
     * # Arguments
     * * `args` - The parsed command-line arguments
     *
     * Returns a Config instance.
     */
    pub fn from_args(args: Args) -> Self {
        Self {
            host: args.host,
            port: args.port,
        }
    }

    /**
     * Returns the address in format "host:port".
     */
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Config {
    /**
     * Returns the default configuration with host "0.0.0.0" and port 8080.
     */
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}
