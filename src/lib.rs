pub mod constants;
pub mod logger;
mod sidechain;
mod messages;
pub use crate::{
    messages::{SidechainMessages, SidechainSlotNumber, SidechainName, SidechainPrivateKey, Upvote, BlindedTx},
};
pub use {
    nakamoto::client::Domain,
};
use {
    std::{
        net,
        path::PathBuf,
        time,
    },
    nakamoto::client::{
        client::{self, Client, Config, Network},
        error::Error,
    },
};
/// The network reactor we're going to use.
type Reactor = nakamoto::net::poll::Reactor<net::TcpStream, client::Publisher>;
/// Run the light-client. Takes an initial list of peers to connect to, a list of listen addresses,
/// the client root and the Bitcoin network to connect to.
pub fn run(
    connect: &[net::SocketAddr],
    listen: &[net::SocketAddr],
    root: Option<PathBuf>,
    domains: &[Domain],
    network: Network,
) -> Result<(), Error> {
    let mut cfg = Config {
        network,
        listen: if listen.is_empty() {
            vec![([0, 0, 0, 0], 0).into()]
        } else {
            listen.to_vec()
        },
        connect: connect.to_vec(),
        domains: domains.to_vec(),
        timeout: time::Duration::from_secs(30),
        ..Config::default()
    };
    if let Some(path) = root {
        cfg.root = path;
    }
    if !connect.is_empty() {
        cfg.target_outbound_peers = connect.len();
    }

    Client::<Reactor>::new(cfg)?.run()
}
