use std::{net::IpAddr, path::PathBuf};

pub use futures_util::SinkExt;
use interprocess::local_socket::{tokio::Listener, traits::tokio::Stream, GenericFilePath, GenericNamespaced, Name, NameType, ToFsName, ToNsName};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWrite};
pub use tokio_serde::Framed;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
pub use interprocess::local_socket::tokio::prelude::*;
use uuid::Uuid;

pub mod streams;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceName{
    name: Option<String>,
    id: Uuid
}


pub mod ipc{
    pub type Listener = interprocess::local_socket::tokio::Listener;
    pub type Stream = interprocess::local_socket::tokio::Stream;
    use std::{ffi::{OsStr, OsString}, net::IpAddr, path::PathBuf};

    use either::Either;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum HostId{
        NickName(String),
        Id(Uuid)
    }
    #[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Serialize, Deserialize)]
    pub struct  FileId{
        /// can be either a path or the nickname, if the nickname exists it will prioritise it
        inner: OsString
    }
    impl FileId {
        pub fn new(data: OsString) -> Self {
            Self {
                inner: data
            }
        }
        pub fn data(&self) -> &OsStr {
            &self.inner
        }

    }
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum ConnectionType{
        Password(String),
        Key(Uuid)
    }
    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum FetchTarget{
        Backup(String),
        File(PathBuf)
    }

    #[derive(Debug,Serialize,Deserialize)]
    pub struct ServerInfo{
        connected_count: usize,
        hosted_file_count: usize
    }
    impl ServerInfo {
        pub fn new(connected_count: usize, hosted_file_count: usize) -> Self {
            Self {
                connected_count,
                hosted_file_count
            }
        }
    }
    #[derive(Debug,Serialize,Deserialize)]
    pub struct FileInfo{
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum UserCommand{
        /// start hosting files
        Start,
        /// stop hosting files, does not shut down the deamon
        Stop,
        Reload,

        Connect {
            id: Uuid,
            connection_type: ConnectionType,
            nickname: Option<String>
        },
        Disconnect {
            id: HostId,
        },

    
        HostFile{
            path: PathBuf,
            dir: bool,
            nickname: Option<String>
        },
        UnHostFile(FileId),

        Fetch{
            host: HostId,
            target: FetchTarget
        },
        Backup{
            path: PathBuf,
            compression: Option<String>,
            schedule: String,
            host: HostId
        },
        Info {
            host: Option<HostId>
        },
        FileList{
            host: Option<HostId>
        }
    }
    
    
    #[derive(Serialize, Deserialize, Debug)]
    pub enum ServerError{
        InvalidPacket
    }
    
    #[derive(Serialize, Deserialize, Debug)]
    pub enum ServerReply{
        Started,
        Stopped,
        Reloaded,

        Connected(HostId),
        Disconnect,

        HostFile(FileId),
        UnHostFile,

        Fetched(Vec<u8>),
        Backuped,

        Info(ServerInfo),
        FileList(FileInfo),

        Error(ServerError)
    }
}


#[derive(Serialize, Deserialize)]
pub struct HostedFile{
    file: PathBuf,
    uuid: Uuid
}
impl HostedFile {
    pub fn new(file: PathBuf) -> Self {
        Self {
            file,
            uuid: Uuid::new_v4()
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct HostedFileData{
    len: usize,
}







