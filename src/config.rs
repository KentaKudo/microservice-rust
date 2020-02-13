use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    /// The grpc port to listen on for gRPC connections
    #[structopt(long = "grpc-port", env = "GRPC_PORT", default_value = "8090")]
    pub grpc_port: u16,
}
