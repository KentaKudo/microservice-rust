use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    /// The ops port to listen on for HTTP connections
    #[structopt(long = "ops-port", env = "OPS_PORT", default_value = "8081")]
    pub ops_port: u16,
}
