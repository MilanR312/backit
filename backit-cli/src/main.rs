use std::{net::IpAddr, path::{Path, PathBuf}, str::FromStr};

use backit_core::*;
use ipc::{FetchTarget, FileId, UserCommand};
use streams::{client, client_codec, StreamExt};
use clap::{builder::ValueParser, command, Args, FromArgMatches, Parser, Subcommand};

#[derive(Debug)]
pub enum Credentials{
    Password(String),
    Key(String)
}
impl FromArgMatches for Credentials {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut matches.clone())
    }
    fn from_arg_matches_mut(matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
        let password: Option<String> = matches.remove_one("password");
        let key: Option<String> = matches.remove_one("key");
        let out = match (password, key) {
            (Some(x), None) => Self::Password(x),
            (None, Some(x)) => Self::Key(x),
            _ => unreachable!()
        };
        Ok(out)
    }
    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut matches.clone())
    }
    fn update_from_arg_matches_mut(&mut self, matches: &mut clap::ArgMatches) -> Result<(), clap::Error> {
        if matches.contains_id("password") {
            *self = Self::Password(matches.remove_one("password").unwrap());
        }
        if matches.contains_id("key") {
            *self = Self::Password(matches.remove_one("key").unwrap());
        }
        Ok(())
    }
}
impl Args for Credentials {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("Credentials"))
    }
    fn augment_args(cmd: clap::Command) -> clap::Command {
        let cmd = cmd
            .group(clap::ArgGroup::new("Credentials")
                .multiple(false)
                .required(true)
                .args(
                    [
                        clap::Id::from("password"),
                        clap::Id::from("key")
                    ]
                )
            );
        cmd
            .arg({
                let arg = clap::Arg::new("password")
                    .value_name("PASSWORD")
                    .value_parser({
                        ValueParser::string()
                    })  
                    .action(clap::ArgAction::Set);
                let arg = arg.short('p').long("password");
                arg
            })
            .arg({
                let arg = clap::Arg::new("key")
                    .value_name("KEY")
                    .value_parser({
                        ValueParser::string()
                    })  
                    .action(clap::ArgAction::Set);
                let arg = arg.short('k').long("key");
                arg
            })
    }
    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        cmd
            .arg({
                let arg = clap::Arg::new("password")
                    .value_name("PASSWORD")
                    .value_parser({
                        ValueParser::string()
                    })  
                    .action(clap::ArgAction::Set);
                let arg = arg.short('p').long("password");
                arg.required(false)
            })
            .arg({
                let arg = clap::Arg::new("key")
                    .value_name("KEY")
                    .value_parser({
                        ValueParser::string()
                    })  
                    .action(clap::ArgAction::Set);
                let arg = arg.short('k').long("key");
                arg.required(false)
            })
    }
}

#[derive(Parser, Debug)]
pub enum Backit{
    Start,
    Stop,
    Reload,

    Connect{
        id: String,
        #[clap(flatten)]
        connection_type: Credentials,
        nickname: Option<String>
    }
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct X{
    pass: Option<String>,
    key: Option<String>
}

/*
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
*/
#[tokio::main]
async fn main() -> eyre::Result<()> {
    /*let cli = cli().run();
    println!("cmd = {cli:?}");
    let client = client().await?;
    let mut client = client_codec(client);
    client.send(cli).await?;
    let returned = client.next().await.unwrap()?;
    println!("reply = {returned:?}");*/
    let a = Backit::parse();
    println!("{a:?}");
    Ok(())
}
