use std::{net::IpAddr, path::{Path, PathBuf}, str::FromStr};

use backit_core::*;
use bpaf::{any, choice, construct, long, positional, short, OptionParser, Parser};
use ipc::{ConnectionType, FetchTarget, FileId, UserCommand};
use streams::{client, client_codec, StreamExt};




pub fn start() -> impl Parser<UserCommand>{
    construct!(UserCommand::Start{})
}
pub fn reload() -> impl Parser<UserCommand>{
    construct!(UserCommand::Reload{})
}
pub fn stop() -> impl Parser<UserCommand>{
    construct!(UserCommand::Stop{})
}


pub fn connect() -> impl Parser<UserCommand>{
    let id = positional("ID");
    let password = positional("PW")
        .map(|x| ConnectionType::Password(x));
    let key = long("key")
        .short('k')
        .argument("KEY")
        .map(|x| ConnectionType::Key(x));
    let nickname = long("nickname")
        .short('n')
        .argument("NNAME")
        .optional();

    let connection_type = construct!([key, password]);
    construct!(UserCommand::Connect{nickname, connection_type, id})
}
pub fn disconnect() -> impl Parser<UserCommand>{
    let id = positional("IP")
        .map(|x| ipc::HostId::Id(x));
    let nick = positional("NICK")
        .map(|x| ipc::HostId::NickName(x));
    let id = construct!([id, nick]);
    construct!(UserCommand::Disconnect{ id  })
} 

pub fn host() -> impl Parser<UserCommand>{
    let path = positional("FNAME")
        .help("name of the filename to host")
        .guard(|x: &PathBuf| x.exists(), "specified file|dir does not exist")
        ;
    let nickname = long("nickname")
        .short('n')
        .argument("NNAME")
        .optional()
        ;
    let dir = short('r').switch();
    construct!(UserCommand::HostFile{ dir, nickname, path})
}
pub fn unhost() -> impl Parser<UserCommand>{
    let id = positional("FID")
        .help("path or nickname of the file")
        .map(|x: String| FileId::new(x.into()))
        ;
    construct!(UserCommand::UnHostFile(id))
}

pub fn fetch() -> impl Parser<UserCommand>{
    let id = positional("IP")
        .map(|x| ipc::HostId::Id(x));
    let nick = positional("NICK")
        .map(|x| ipc::HostId::NickName(x));
    let host = construct!([id, nick]);

    let file = positional("FILE")
        .map(|x| FetchTarget::File(x));
    let backup = short('b')
        .argument("BUP")
        .map(|x| FetchTarget::Backup(x));
    let target = construct!([backup, file]);
    construct!(UserCommand::Fetch{ target, host})
}
pub fn backup() -> impl Parser<UserCommand>{
    let path = positional("PATH");
    let compression = short('c').argument("C").optional();
    let schedule = short('s').argument("S")
        .fallback(String::new());
    let id = positional("IP")
        .map(|x| ipc::HostId::Id(x));
    let nick = positional("NICK")
        .map(|x| ipc::HostId::NickName(x));
    let host = construct!([id, nick]);
    construct!(UserCommand::Backup{ compression, schedule, path, host})
}

pub fn info() -> impl Parser<UserCommand>{
    let id = positional("IP")
        .map(|x| ipc::HostId::Id(x))
        .optional();
    let nick = positional("NICK")
        .map(|x| ipc::HostId::NickName(x))
        .optional();
    let host = construct!([id, nick]);
    construct!(UserCommand::Info{ host })
}
pub fn file_list() -> impl Parser<UserCommand>{
    let id = positional("IP")
        .map(|x| ipc::HostId::Id(x))
        .optional();
    let nick = positional("NICK")
        .map(|x| ipc::HostId::NickName(x))
        .optional();
    let host = construct!([id, nick]);
    construct!(UserCommand::FileList{ host })
}

pub fn cli() -> OptionParser<UserCommand>{
    let start = start().to_options().descr("start the backup server").command("start");
    let stop = stop().to_options().descr("stop the backup server").command("stop");
    let reload = reload().to_options().descr("reload the config file").command("reload");

    let connect = connect().to_options().descr("connect to a host").command("connect");
    let disconnect = disconnect().to_options().descr("disconnect from a host").command("disconnect");

    let host = host().to_options().descr("host a file").command("add");
    let unhost = unhost().to_options().descr("unhost a file").command("remove");

    let fetch = fetch().to_options().descr("fetch a file").command("fetch");
    //TODO: add option for move to pc?
    let backup = backup().to_options().descr("backs up a file").command("backit");
    
    let info = info().to_options().descr("info for a host").command("info");
    let file_list = file_list().to_options().descr("list info for a file").command("list");


    construct!([start, stop, reload,  connect, disconnect,  host, unhost,  fetch, backup,  info, file_list]).to_options()
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = cli().run();
    println!("cmd = {cli:?}");
    let client = client().await?;
    let mut client = client_codec(client);
    client.send(cli).await?;
    let returned = client.next().await.unwrap()?;
    println!("reply = {returned:?}");
    Ok(())
}
