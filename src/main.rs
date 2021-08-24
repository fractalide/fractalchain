use {
    fractalchain::{self, *, logger, Domain},
    argh::FromArgs,
    std::{
        net,
        path::PathBuf,
    },
    nakamoto::{
        client::client::Network,
    },
};
#[derive(FromArgs)]
/// A Bitcoin light client.
pub struct Options {
    /// connect to the specified peers only
    #[argh(option)]
    pub connect: Vec<net::SocketAddr>,
    /// listen on one of these addresses for peer connections.
    #[argh(option)]
    pub listen: Vec<net::SocketAddr>,
    /// use the bitcoin test network (default: false)
    #[argh(switch)]
    pub testnet: bool,
    /// only connect to IPv4 addresses (default: false)
    #[argh(switch, short = '4')]
    pub ipv4: bool,
    /// only connect to IPv6 addresses (default: false)
    #[argh(switch, short = '6')]
    pub ipv6: bool,
    /// log level (default: info)
    #[argh(option, default = "log::Level::Info")]
    pub log: log::Level,
    /// root directory for nakamoto files (default: ~)
    #[argh(option)]
    pub root: Option<PathBuf>,
}
impl Options {
    pub fn from_env() -> Self {
        argh::from_env()
    }
}
fn main() {
    let opts = Options::from_env();
    logger::init(opts.log).expect("initializing logger for the first time");

    let slot_number = SidechainSlotNumber::default();
    let name = SidechainName::default();
    let private_key = SidechainPrivateKey::default();
    let m2s_tx_req = SidechainMessages::new_main_to_side_tx_request(slot_number, name, private_key);
    let m2s_tx_req_bytes = m2s_tx_req.as_bytes();
    let m2s_tx_req_from_bytes = SidechainMessages::from_bytes(&m2s_tx_req_bytes).unwrap();
    println!("{:?}\n{:?}", m2s_tx_req_bytes, m2s_tx_req_from_bytes);

    let upvote = Upvote::default();
    let m2s_tx_req_upvote = SidechainMessages::new_main_to_side_tx_request_upvote(upvote);
    let m2s_tx_req_upvote_bytes = m2s_tx_req_upvote.as_bytes();
    let m2s_tx_req_upvote_from_bytes = SidechainMessages::from_bytes(&m2s_tx_req_upvote_bytes).unwrap();
    println!("{:?}\n{:?}", m2s_tx_req_upvote_bytes, m2s_tx_req_upvote_from_bytes);

    let slot_number = SidechainSlotNumber::default();
    let blinded_tx = BlindedTx::default();
    let s2m_tx_req = SidechainMessages::new_side_to_main_tx_request(slot_number, blinded_tx);
    let s2m_tx_req_bytes = s2m_tx_req.as_bytes();
    let s2m_tx_req_from_bytes = SidechainMessages::from_bytes(&s2m_tx_req_bytes).unwrap();
    println!("{:?}\n{:?}", s2m_tx_req_bytes, s2m_tx_req_from_bytes);

    let network = if opts.testnet {
        Network::Testnet
    } else {
        Network::Mainnet
    };
    let domains = if opts.ipv4 && opts.ipv6 {
        vec![Domain::IPV4, Domain::IPV6]
    } else if opts.ipv4 {
        vec![Domain::IPV4]
    } else if opts.ipv6 {
        vec![Domain::IPV6]
    } else {
        vec![Domain::IPV4, Domain::IPV6]
    };
    if let Err(e) = fractalchain::run(&opts.connect, &opts.listen, opts.root, &domains, network) {
        log::error!("Exiting: {}", e);
        std::process::exit(1);
    }
}


