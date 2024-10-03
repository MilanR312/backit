use std::{collections::HashMap, ffi::OsString, io, path::PathBuf, time::Duration, u64};

use backit_core::*;
use futures::StreamExt;
use ipc::{FileId, HostId, ServerInfo, UserCommand};
use libp2p::{swarm::SwarmEvent, Multiaddr};
use streams::{server, server_codec, ServerCodec};
use tracing_subscriber::EnvFilter;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    try_join,
};


pub struct Config{}

pub(crate) fn config_path() -> PathBuf {
    todo!()
}

pub struct Server{
    user_commands: ipc::Listener,
    hosted_files: HashMap<FileId, PathBuf>,
    connected_clients: HashMap<HostId, ()>,
    active: bool,
    config: Config
}
impl Server {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            user_commands: server()?,
            hosted_files: HashMap::new(),
            active: false,
            config: Config {  }
        })
    }
    pub fn start(&mut self){
        self.active = true;
    }
    pub fn stop(&mut self){
        self.active = false;
    }
    pub fn reload(&mut self) {
        self.config = Config{};
    }
    pub fn connect(&mut self, hostid: HostId) {
        self.connected_clients.insert(hostid, ());
    }
    pub fn disconnect(&mut self, hostid: HostId) {
        self.connected_clients.remove(&hostid);
    }
    pub fn host_file(&mut self, file: PathBuf, nickname: Option<String>) -> FileId {
        let name = nickname.map(|x| x.into()).unwrap_or(file.clone().into_os_string());
        let file_id = FileId::new(name);
        dbg!(&file_id);
        self.hosted_files.insert(file_id.clone(), file);
        file_id
    }
    pub fn unhost_file(&mut self, file_id: FileId) {
        self.hosted_files.remove(&file_id);
    }
    pub fn fetch(&mut self, host_id: HostId){}
  
    pub fn backup(&mut self){}
    pub fn info(&self, host: Option<HostId>) -> ServerInfo{
        match host {
            None => {
                ServerInfo::new(self.connected_clients.len(), self.hosted_files.len())
            },
            _ => {
                todo!()
            }
        }
    }
    pub fn file_list(&self){}

    pub async fn handle_user_command(&mut self, command: UserCommand, codec: &mut ServerCodec<ipc::Stream>) -> io::Result<()>{
        match command {
            UserCommand::Start => self.start(),
            UserCommand::Stop => self.stop(),
            UserCommand::Reload => self.reload(),
            UserCommand::Connect { id, connection_type, nickname }
                => todo!(),
            UserCommand::Disconnect { id } => self.disconnect(id),
            _ => {}
        }
        Ok(())
    }

    pub async fn handle_command(&mut self) -> io::Result<()>{
        let connection = self.user_commands.accept().await?;
        let mut codec = server_codec(connection);
        while let Some(x) = codec.next().await{
            match x {
                Ok(x) => {
                    self.handle_user_command(x, &mut codec).await?;
                },
                Err(e) => {
                    codec.send(ipc::ServerReply::Error(ipc::ServerError::InvalidPacket)).await?;
                }
            };
        }
        println!("finished");
        Ok(())
    }
}


#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt().init();

    let mut server = Server::new()?;
    loop {
        server.handle_command().await?;
    }

    

    Ok(())

}