pub mod config;
pub mod service;
pub mod service_grpc;

use std::sync::mpsc::channel;
use std::thread;

use crate::config::Config;
use crate::service::API;

use boxfnonce::SendBoxFnOnce;
use futures::{Future, Stream};
use slog::{slog_error, slog_info};
use slog_scope::{error, info};
use structopt::StructOpt;
use tokio::executer::DefaultExecuter;
use tokio::net::TcpListener;
use tower_h2::Server;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
// const APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");
// const APP_REPO: &str = env!("CARGO_PKG_REPOSITORY");
const SHORT_GIT_HASH: &str = env!("SHORT_GIT_HASH");

pub fn app() {
    let config = Config::from_args();

    let api = API::new();

    info!("Running"; "app" => APP_NAME, "version" => SHORT_GIT_HASH);
    
    run(vec![
        catch_signals(),
        grpc_server(api, config.grpc_port),
    ]);
}

fn grpc_server(api: API, port: u16) -> RunFunc {
    SendBoxFnOnce::from(|| {
        let service = service_grpc::server::TodoApiServer::new(api);

        let mut h2 = Server::new(service, Default::default(), DefaultExecuter::current());

        let addr = format!("0.0.0.0:{}", port).parse().unwrap();
        let bind = TcpListener::bind(&addr).unwrap();

        let serve = bind
            .incoming()
            .for_each(move |sock| {
                if let Err(e) = sock.set_nodelay(true) {
                    return Err(e);
                }

                let serve = h2.serve(sock);
                tokio::spawn(serve.map_err(|e| error!("Error serving"; "err" => e.to_string())));

                Ok(())
            })
            .map_err(|e| error!("Error accepting request"; "err" => e.to_string()));

        tokio::run(serve);
    })
}

fn catch_signals() -> RunFunc {
    SendBoxFnOnce::from(|| {
        use tokio_signal::unix::{Signal, SIGINT, SIGTERM};

        let sigint = Signal::new(SIGINT).flatten_stream();
        let sigterm = Signal::new(SIGTERM).flatten_stream();

        let stream = sigint.select(sigterm);

        let (_item, _rest) = tokio::runtime::current_thread::block_on_all(stream.into_future())
            .ok()
            .unwrap();

        info!("Quitting");
    })
}

type RunFunc = SendBoxFnOnce<'static, (), ()>;

fn run(funcs: Vec<RunFunc>) {
    let (tx, rx) = channel();

    for func in funcs {
        let tx = tx.clone();
        let _ = thread::spawn(move || {
            let _ = thread::spawn(|| {
                func.call();
            })
            .join();
            tx.send(()).unwrap();
        });
    }

    rx.recv().unwrap();
}
