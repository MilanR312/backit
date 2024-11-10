use std::{
    alloc::Layout,
    net::IpAddr,
    path::{Path, PathBuf},
    str::FromStr,
};

use backit_core::{
    ipc::to_server::*,
    streams::{client, client_codec, StreamExt},
    SinkExt,
};
use bpaf::{any, construct, long, positional, pure, short, Parser};

fn credentials() -> impl Parser<Credentials> {
    let key = short('k')
        .long("key")
        .argument("KEY")
        .map(|x| Credentials::new_key(x));
    let url = short('u')
        .long("url")
        .argument("URL")
        .map(|x| Credentials::new_url(x));

    let id = positional("ID");
    let password = positional("PASSWORD");
    let password = construct!(Credentials::Password { id, password });
    construct!([key, url, password])
}

fn host_id() -> impl Parser<HostId> {
    // this will always match the nickname but it doesnt really matter since both are strings
    // defining both makes the bpaf help look better
    let nickname = positional("HOST_NICK").map(|x| HostId::new_nickname(x));
    let id = positional("ID").map(|x| HostId::new_id(x));
    construct!([nickname, id])
}

fn file_target() -> impl Parser<FileTarget> {
    let dir = short('r')
        .long("recursive")
        .argument("DIR")
        .map(|x| FileTarget::Dir { path: x });
    let path = positional("FILE");
    let nickname = short('n').long("nickname").argument("NICKNAME").optional();
    let file = construct!(FileTarget::File { nickname, path });
    construct!([dir, file])
}

/*
impl Target {
    pub fn nickname_arg() -> Arg {
        Arg::new("nickname")
            .value_name("NICKNAME")
            .value_parser(ValueParser::string())
            .conflicts_with("tags")
    }
    pub fn tags_arg() -> Arg {
        Arg::new("tags")
            .value_name("TAGS")
            .short('t')
            .value_parser(ValueParser::string())
            .conflicts_with("nickname")
    }
}
impl FromArgMatches for Target {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut matches.clone())
    }
    fn from_arg_matches_mut(matches: &mut clap::ArgMatches) -> Result<Self, clap::Error> {
        if let Some(tags) = matches.remove_many::<String>("tags") {
            return Ok(Self::Tags(tags.collect()));
        }
        if let Some(nickname) = matches.remove_one("nickname") {
            return Ok(Self::Nickname(nickname));
        }
        unreachable!()
    }
    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        matches: &mut clap::ArgMatches,
    ) -> Result<(), clap::Error> {
        if matches.contains_id("tags") {
            *self = Self::Tags(matches.remove_many("tags").unwrap().collect())
        }
        if matches.contains_id("nickname") {
            *self = Self::Nickname(matches.remove_one("nickname").unwrap());
        }
        Ok(())
    }
}
impl Args for Target {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("Target"))
    }
    fn augment_args(cmd: clap::Command) -> clap::Command {
        let cmd = cmd.group(
            clap::ArgGroup::new("Target")
                .multiple(false)
                .required(true)
                .args(["nickname", "tags"]),
        );
        cmd.arg(Self::nickname_arg()).arg(Self::tags_arg())
    }
    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        cmd.arg(Self::nickname_arg().required(false))
            .arg(Self::tags_arg().required(false))
    }
}
*/
fn target() -> impl Parser<Target> {
    //let nickname = positional("NICKNAME").map(|x| Target::Nickname(x));
    let nickname = short('n')
        .long("nickname")
        .argument("NICKNAME")
        .map(|x| Target::Nickname(x));
    let tags = short('t')
        .long("tags")
        .argument("TAGS")
        .some("must be at least 1 tag supplied")
        .map(|x| Target::Tags(x));
    construct!([tags, nickname])
}

fn any_host() -> impl Parser<AnyHost> {
    let host_id = host_id().map(|x| AnyHost::HostId(x));
    let credentials = credentials().map(|x| AnyHost::Credentials(x));
    construct!([host_id, credentials])
}

fn backit() -> impl Parser<Backit> {
    struct LocalBackit {
        command: Command,
        json: bool,
        no_confirm: bool,
    }
    const _: () = assert!(size_of::<LocalBackit>() == size_of::<Backit>());
    const _: () = assert!(Layout::new::<LocalBackit>().align() == Layout::new::<Backit>().align());
    let command = command();
    let json = long("json").switch();
    let no_confirm = long("no-confirm").switch();

    construct!(LocalBackit {
        json,
        no_confirm,
        command
    })
    .map(|x| Backit::new(x.command, x.json, x.no_confirm))
}
fn command() -> impl Parser<Command> {
    let start = construct!(Command::Start {}).to_options().command("start");
    let stop = pure(Command::Stop).to_options().command("stop");
    let reload = pure(Command::Reload).to_options().command("reload");

    let connection_type = credentials();
    let nickname = short('n').long("nickname").argument("NICKNAME").optional();
    let connect = construct!(Command::Connect {
        nickname,
        connection_type
    })
    .to_options()
    .command("connect");

    let host_id = host_id();
    let disconnect = construct!(Command::Disconnect(host_id))
        .to_options()
        .command("disconnect");

    let host = {
        let target = file_target();
        let tags = short('t').long("tags").argument("TAGS").many();
        construct!(Command::Host { tags, target })
            .to_options()
            .command("host")
    };

    let unhost = target().some("must be at least 1 target supplied");
    let unhost = construct!(Command::Unhost(unhost))
        .to_options()
        .command("unhost");

    let fetch = {
        let host = any_host();
        let target = target();
        construct!(Command::Fetch { target, host })
            .to_options()
            .command("fetch")
    };

    let push = {
        let host = any_host();
        let target = target();
        construct!(Command::Push { target, host })
            .to_options()
            .command("push")
    };

    let status = {
        let host = any_host().optional();
        construct!(Command::ServerStatus(host))
            .to_options()
            .command("status")
    };

    construct!([start, stop, reload, connect, disconnect, host, unhost, fetch, push, status])
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
    let backit = backit().to_options().run();
    println!("cmd = {backit:#?}");
    let client = client().await?;
    let mut client = client_codec(client);
    let expects_reply = !backit.no_confirm();
    client.send(backit).await?;
    if expects_reply {
        let returned = client.next().await.unwrap()?;
        println!("reply = {returned:?}");
    }
    Ok(())
}
