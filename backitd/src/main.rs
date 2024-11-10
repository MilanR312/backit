use std::{collections::HashMap, ffi::OsString, io, path::PathBuf, time::Duration, u64};

use backit_core::{
    ipc::{self, to_server::*},
    streams::{client, client_codec, server, server_codec, ServerCodec, StreamExt},
    SinkExt,
};
use interprocess::local_socket::traits::tokio::Listener;
use ipc::ServerInfo;
use libp2p::{swarm::SwarmEvent, Multiaddr};
use p2p::Client;
use tracing_subscriber::EnvFilter;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    spawn, try_join,
};


pub mod p2p;

pub struct Config {}

pub(crate) fn config_path() -> PathBuf {
    todo!()
}
pub struct Server {
    user_commands: ipc::Listener,
    hosted_files: HashMap<FileTarget, PathBuf>,
    connected_clients: HashMap<HostId, ()>,
    active: bool,
    config: Config,
    client: Client
}
impl Server {
    pub fn new(client: Client) -> io::Result<Self> {
        Ok(Self {
            user_commands: server()?,
            hosted_files: HashMap::new(),
            active: false,
            config: Config {},
            connected_clients: HashMap::new(),
            client
        })
    }

    pub fn server_info(&self) -> ServerInfo {
        ServerInfo::new(self.active, self.hosted_files.len())
    }

    pub async fn handle_user_command(
        &mut self,
        backit: Backit,
        codec: &mut ServerCodec<ipc::Stream>,
    ) -> io::Result<()> {
        use ipc::ServerError as SE;
        use ipc::ServerReply as SR;
        match backit.command() {
            Command::Start => {
                self.active = true;
                codec.send(SR::Started).await?;
            }
            Command::Stop => {
                self.active = false;
                codec.send(SR::Stopped).await?;
            }
            Command::Reload => {
                todo!();
                codec.send(SR::Error(SE::NotImplemented)).await?;
            }

            Command::ServerStatus(None) => {
                codec.send(SR::Info(self.server_info())).await?;
            }
            Command::ServerStatus(Some(x)) => {
                
            }
            _ => {
                if !backit.no_confirm() {
                    codec
                        .send(ipc::ServerReply::Error(ipc::ServerError::InvalidPacket))
                        .await?;
                }
            }
        }
        Ok(())
    }

    pub async fn handle_command(&mut self) -> io::Result<()> {
        let connection = self.user_commands.accept().await?;
        let mut codec = server_codec(connection);
        while let Some(x) = codec.next().await {
            println!("got request {:?}", x);
            match x {
                Ok(x) => {
                    self.handle_user_command(x, &mut codec).await?;
                }
                Err(e) => {
                    codec
                        .send(ipc::ServerReply::Error(ipc::ServerError::InvalidPacket))
                        .await?;
                }
            };
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt().init();

    /*let mut server = Server::new()?;
    loop {
        server.handle_command().await?;
    }*/
    let (mut event_loop, client) = p2p::EventLoop::new()?;

    spawn(async move {
        event_loop.run().await;
    });
    let mut server = Server::new(client)?;
    loop {
        server.handle_command();
    }
    client.start_listening().await;

    Ok(())
}
