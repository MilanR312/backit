use std::{net::IpAddr, path::PathBuf};

pub use futures_util::SinkExt;
pub use interprocess::local_socket::tokio::prelude::*;
use interprocess::local_socket::{
    tokio::Listener, traits::tokio::Stream, GenericFilePath, GenericNamespaced, Name, NameType,
    ToFsName, ToNsName,
};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWrite};
pub use tokio_serde::Framed;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use uuid::Uuid;

pub mod streams;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceName {
    name: Option<String>,
    id: Uuid,
}

pub mod ipc {
    pub type Listener = interprocess::local_socket::tokio::Listener;
    pub type Stream = interprocess::local_socket::tokio::Stream;
    use std::{
        ffi::{OsStr, OsString},
        net::IpAddr,
        path::PathBuf,
    };

    use either::Either;
    use from_client::*;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    pub mod to_server {
        pub use super::from_client::*;
    }
    pub(crate) mod from_client {
        use std::path::PathBuf;

        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum Credentials {
            /// a single use key used to identify and connect to a remote client
            Key(String),
            Url(String),
            Password {
                id: String,
                password: String,
            },
        }
        impl Credentials {
            pub fn new_key(key: String) -> Self {
                Self::Key(key)
            }
            pub fn new_url(url: String) -> Self {
                Self::Url(url)
            }
            pub fn new_password(id: String, password: String) -> Self {
                Self::Password { id, password }
            }
        }
        /// an id for a host, kan be either a nickname or an id
        #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        pub struct HostId {
            nickname_or_id: String,
        }
        impl HostId {
            pub fn new_nickname(nickname: String) -> Self {
                Self {
                    nickname_or_id: nickname,
                }
            }
            pub fn new_id(id: String) -> Self {
                Self { nickname_or_id: id }
            }
        }
        #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum FileTarget {
            File {
                path: PathBuf,
                nickname: Option<String>,
            },
            Dir {
                path: PathBuf,
            },
        }
        impl FileTarget {
            pub fn new_file(path: PathBuf, nickname: Option<String>) -> Self {
                Self::File { path, nickname }
            }
            pub fn new_dir(path: PathBuf) -> Self {
                Self::Dir { path }
            }
        }

        #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum Target {
            Nickname(String),
            Tags(Vec<String>),
        }
        impl Target {
            pub fn new_nickname(nickname: String) -> Self {
                Self::Nickname(nickname)
            }
            pub fn new_tags(tags: Vec<String>) -> Self {
                Self::Tags(tags)
            }
        }
        #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum AnyHost {
            HostId(HostId),
            Credentials(Credentials),
        }
        impl AnyHost {
            pub fn new_host_id(host_id: HostId) -> Self {
                Self::HostId(host_id)
            }
            pub fn new_credentials(credentials: Credentials) -> Self {
                Self::Credentials(credentials)
            }
        }
        #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum Command {
            Start,
            Stop,
            Reload,
            /// connect to a new host and set an optional nickname
            ///
            /// priority for set name is credentials < host_name (in config) < nickname
            Connect {
                connection_type: Credentials,
                nickname: Option<String>,
            },
            Disconnect(HostId),

            Host {
                target: FileTarget,
                tags: Vec<String>,
            },
            Unhost(Vec<Target>),

            Fetch {
                host: AnyHost,
                target: Target,
            },
            Push {
                host: AnyHost,
                target: Target,
            },
            // Backup
            ServerStatus(Option<AnyHost>),
        }
        #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        pub struct Backit {
            command: Command,
            json: bool,
            no_confirm: bool,
        }
        impl Backit {
            pub fn new(command: Command, json: bool, no_confirm: bool) -> Self {
                Self {
                    command,
                    json,
                    no_confirm,
                }
            }
            pub fn no_confirm(&self) -> bool {
                self.no_confirm
            }
            pub fn command(&self) -> &Command {
                &self.command
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ServerError {
        InvalidPacket,
        NotImplemented,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ServerReply {
        Started,
        Stopped,
        Reloaded,

        Connected(Option<String>),
        Disconnect,

        HostFile(FileTarget),
        UnHostFile,

        Fetched(Vec<u8>),
        Backuped,

        Info(ServerInfo),
        //FileList(FileInfo),
        Error(ServerError),
    }
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ServerInfo {
        active: bool,
        file_count: usize,
    }
    impl ServerInfo {
        pub fn new(active: bool, file_count: usize) -> Self {
            Self { active, file_count }
        }
    }
}

pub mod tcp {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub enum SendPacket {}
    #[derive(Serialize, Deserialize, Debug)]
    pub enum ReceivePacket {}
}
