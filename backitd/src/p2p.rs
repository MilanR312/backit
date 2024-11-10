use std::{collections::HashMap, time::Duration};

use eyre::eyre;
use futures::{
    channel::{mpsc, oneshot},
    SinkExt, StreamExt,
};
use libp2p::{
    identity, kad, noise,
    request_response::{self, ProtocolSupport},
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId, StreamProtocol, Swarm, SwarmBuilder,
};

use backit_core::tcp::*;
use signals::{ToSwarm};

pub mod signals {
    use std::fmt::Display;

    use futures::channel::{mpsc::SendError, oneshot};

    #[derive(Debug)]
    pub enum ToSwarm {
        StartListening { tx: oneshot::Sender<bool> },
    }
    
}

pub struct Client {
    tx: mpsc::Sender<ToSwarm>,
}
impl Client {
    pub fn new(tx: mpsc::Sender<ToSwarm>) -> Self {
        Self { tx }
    }
    pub async fn start_listening(&mut self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.tx.send(ToSwarm::StartListening { tx }).await.expect("receiver not to be dropped");
        let result = rx.await.expect("sender not be dropped");
        result
    }
}

pub struct EventLoop {
    swarm: Swarm<Behaviour>,
    command_queue: mpsc::Receiver<ToSwarm>,
}
impl EventLoop {
    pub fn new() -> eyre::Result<(Self, Client)> {
        let (to_swarm_tx, to_swarm_rx) = mpsc::channel(16);

        let swarm = new()?;
        let out = Self {
            swarm,
            command_queue: to_swarm_rx,
        };
        Ok((out, Client::new(to_swarm_tx)))
    }
    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => self.handle_event(event).await,
                command = self.command_queue.next() => match command {
                    Some(x) => self.handle_command(x),
                    None => {}
                }
            };
        }
    }
    pub async fn handle_event(&mut self, event: SwarmEvent<BehaviourEvent>) {
        dbg!(event);
    }
    pub fn handle_command(&mut self, command: ToSwarm) {
        dbg!(command);
    }
}

fn new() -> eyre::Result<Swarm<Behaviour>> {
    let key = identity::Keypair::generate_ed25519();
    let peer_id = key.public().to_peer_id();

    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| Behaviour {
            kademlia: kad::Behaviour::new(
                peer_id,
                kad::store::MemoryStore::new(key.public().to_peer_id()),
            ),
            request_response: request_response::cbor::Behaviour::new(
                [(StreamProtocol::new("/backit"), ProtocolSupport::Full)],
                request_response::Config::default(),
            ),
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    swarm
        .behaviour_mut()
        .kademlia
        .set_mode(Some(kad::Mode::Server));

    Ok(swarm)
}

#[derive(NetworkBehaviour)]
pub struct Behaviour {
    request_response: request_response::cbor::Behaviour<SendPacket, ReceivePacket>,
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
}
