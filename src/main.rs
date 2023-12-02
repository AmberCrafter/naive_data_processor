mod lib;
mod utils;

use std::collections::HashMap;

use clap::Parser;
use lib::ERROR;
use tonic::{transport::Server, Request, Response, Status};

use crate::{
    lib::qc_worker::QCworker,
    utils::cli::{Command::*, Operations},
};

use lib::qc::{
    qc_server::{Qc, QcServer},
    qc_client::QcClient,
    SendRequest, SendResponse,
};

#[derive(Default)]
pub struct QcDaemon {}

#[tonic::async_trait]
impl Qc for QcDaemon {
    async fn send(&self, request: Request<SendRequest>) -> Result<Response<SendResponse>, Status> {
        let mut qc = QCworker::new(HashMap::new());

        let raw_data = if let Some(fidx) = request.get_ref().protocol {
            format!("F{fidx},{}", request.get_ref().payload)
        } else {
            request.get_ref().payload.to_string()
        };

        qc.handler(&raw_data);
        qc.set_database("database");
        qc.save().unwrap();

        Ok(Response::new(SendResponse {
            status: format!("Ok"),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), ERROR> {
    let oper = Operations::parse();

    match oper.command {
        Daemon(opts) => {
            // println!("daemon: {:?}", opts);
            let addr = format!("[::1]:{}", opts.port).parse().unwrap();

            let srv = QcDaemon::default();

            Server::builder()
                .add_service(QcServer::new(srv))
                .serve(addr)
                .await?;
        }

        Qc(opts) => {
            // println!("qc: {:?}", opts);

            if let Some(ip) = opts.ip {
                let addr = format!("http://{addr}:{port}", addr = ip, port = opts.port);
                let mut client = QcClient::connect(addr).await?;
                let request = tonic::Request::new(
                    SendRequest {
                        protocol: opts.protocol,
                        payload: opts.data
                    }
                );

                let response = client.send(request).await?.into_inner();
            } else {
                let mut qc = QCworker::new(HashMap::new());
                let raw_data = if let Some(fidx) = opts.protocol {
                    format!("F{fidx},{}", opts.data)
                } else {
                    opts.data
                };
    
                qc.handler(&raw_data);
                qc.show_report();
                if opts.save {
                    qc.set_database("database");
                    qc.save().unwrap();
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {}
}
