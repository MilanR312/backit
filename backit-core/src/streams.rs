pub use futures_util::{SinkExt, StreamExt};
use interprocess::local_socket::{tokio::Listener, traits::tokio::Stream, GenericFilePath, GenericNamespaced, Name, NameType, ToFsName, ToNsName};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWrite};
pub use tokio_serde::Framed;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
pub use interprocess::local_socket::tokio::prelude::*;
use uuid::Uuid;


use crate::ipc::{ServerReply, UserCommand};

pub type ClientCodec<T> = Framed<tokio_util::codec::Framed<T, LengthDelimitedCodec>, ServerReply, UserCommand, tokio_serde::formats::Cbor<ServerReply, UserCommand>>;
pub type ServerCodec<T> = Framed<tokio_util::codec::Framed<T, LengthDelimitedCodec>, UserCommand, ServerReply, tokio_serde::formats::Cbor<UserCommand, ServerReply>>;

pub fn client_codec<T: AsyncRead + AsyncWrite>(socket: T) -> ClientCodec<T> {
    let a = tokio_util::codec::Framed::new(socket, LengthDelimitedCodec::new());
    tokio_serde::Framed::new(
        a, tokio_serde::formats::Cbor::default()
    )
}
pub fn server_codec<T: AsyncRead + AsyncWrite>(socket: T) -> ServerCodec<T> {
    let a = tokio_util::codec::Framed::new(socket, LengthDelimitedCodec::new());
    tokio_serde::Framed::new(
        a, tokio_serde::formats::Cbor::default()
    )
}

fn ipc_name() -> std::io::Result<Name<'static>>{
    if GenericNamespaced::is_supported() {
        "backit.sock".to_ns_name::<GenericNamespaced>()
    } else {
        "/tmp/backit.sock".to_fs_name::<GenericFilePath>()
    }
}

pub fn server() -> std::io::Result<Listener>{
    interprocess::local_socket::ListenerOptions::new()
        .name(ipc_name()?)
        .create_tokio()
}
pub async fn client() -> std::io::Result<impl Stream > {
    interprocess::local_socket::tokio::Stream::connect(ipc_name()?).await
}