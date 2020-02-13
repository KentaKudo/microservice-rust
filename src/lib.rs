// pub mod config;

use std::sync::mpsc::channel;
use std::thread;

// use crate::config::Config;

use boxfnonce::SendBoxFnOnce;
use futures::{Future, Stream};
use slog_scope::{info};
// use structopt::StructOpt;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
// const APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");
// const APP_REPO: &str = env!("CARGO_PKG_REPOSITORY");
const SHORT_GIT_HASH: &str = env!("SHORT_GIT_HASH");

pub fn app() {
    // let config = Config::from_args();

    info!("Running"; "app" => APP_NAME, "version" => SHORT_GIT_HASH);
    
    run(vec![
        catch_signals(),
    ]);
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
