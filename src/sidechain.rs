use {
    crate::constants::*,
    anyhow::{Result, anyhow},
};
#[derive(Debug)]
pub struct SidechainSlotNumber(pub [u8; SIDECHAIN_SLOT_NUMBER_SIZE]);
impl SidechainSlotNumber {
    pub fn default() -> Self {
        Self([0])
    }
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut s: [u8; SIDECHAIN_SLOT_NUMBER_SIZE] = [0u8; SIDECHAIN_SLOT_NUMBER_SIZE];
        s.clone_from_slice(data);
        Self(s)
    }
    pub fn as_bytes(&self) -> [u8; SIDECHAIN_SLOT_NUMBER_SIZE] {
        self.0.clone()
    }
}
#[derive(Debug)]
pub struct SidechainName(pub [u8; SIDECHAIN_NAME_SIZE]);
impl SidechainName {
    pub fn default() -> Self {
        Self([0u8; SIDECHAIN_NAME_SIZE])
    }
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut s: [u8; SIDECHAIN_NAME_SIZE] = [0u8; SIDECHAIN_NAME_SIZE];
        s.clone_from_slice(data);
        Self(s)
    }
    pub fn as_bytes(&self) -> [u8; SIDECHAIN_NAME_SIZE] {
        self.0.clone()
    }
}
#[derive(Debug)]
pub struct SidechainPrivateKey(pub [u8; SIDECHAIN_PRIVATE_KEY_SIZE]);
impl SidechainPrivateKey {
    pub fn default() -> Self {
        Self([0u8; SIDECHAIN_PRIVATE_KEY_SIZE])
    }
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut s: [u8; SIDECHAIN_PRIVATE_KEY_SIZE] = [0u8; SIDECHAIN_PRIVATE_KEY_SIZE];
        s.clone_from_slice(data);
        Self(s)
    }
    pub fn as_bytes(&self) -> [u8; SIDECHAIN_PRIVATE_KEY_SIZE] {
        self.0.clone()
    }
}
#[derive(Debug)]
pub struct CommitmentHeader(pub [u8; COMMITMENT_HEADER_SIZE]);
impl CommitmentHeader {
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut s: [u8; COMMITMENT_HEADER_SIZE] = [0u8; COMMITMENT_HEADER_SIZE];
        s.clone_from_slice(data);
        Self(s)
    }
    pub fn as_bytes(&self) -> [u8; COMMITMENT_HEADER_SIZE] {
        self.0.clone()
    }
}
#[derive(Debug)]
pub struct Upvote(pub [u8; UPVOTE_SIZE]);
impl Upvote {
    pub fn default() -> Self {
        Self([0u8; UPVOTE_SIZE])
    }
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut s: [u8; UPVOTE_SIZE] = [0u8; UPVOTE_SIZE];
        s.clone_from_slice(data);
        Self(s)
    }
    pub fn as_bytes(&self) -> [u8; UPVOTE_SIZE] {
        self.0.clone()
    }
}
#[derive(Debug)]
pub struct BlindedTx(pub [u8; BLINDED_TX_SIZE]);
impl BlindedTx {
    pub fn default() -> Self {
        Self([0u8; BLINDED_TX_SIZE])
    }
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut s: [u8; BLINDED_TX_SIZE] = [0u8; BLINDED_TX_SIZE];
        s.clone_from_slice(data);
        Self(s)
    }
    pub fn as_bytes(&self) -> [u8; BLINDED_TX_SIZE] {
        self.0.clone()
    }
}
#[derive(Debug)]
pub enum SidechainMessages {
    MainToSideTransactionRequest {
        commitment_header: CommitmentHeader,
        slot_number: SidechainSlotNumber,
        name: SidechainName,
        private_key: SidechainPrivateKey,
    },
    MainToSideTransactionRequestUpvote {
        commitment_header: CommitmentHeader,
        upvote: Upvote,
    },
    MainToSideTransaction,
    SideToMainTransactionRequest {
        commitment_header: CommitmentHeader,
        slot_number: SidechainSlotNumber,
        blinded_tx: BlindedTx,
    },
    SideToMainTransactionRequestUpvote,
    SideToMainTransaction,
}
impl SidechainMessages {
    pub fn new_main_to_side_tx_request(slot_number: SidechainSlotNumber, name: SidechainName, private_key: SidechainPrivateKey) -> Self {
        Self::MainToSideTransactionRequest {
            commitment_header: CommitmentHeader::from_bytes(&MAIN_TO_SIDE_TX_REQUEST_COMMITMENT_HEADER),
            slot_number, name, private_key
        }
    }
    pub fn new_main_to_side_tx_request_upvote(upvote: Upvote) -> Self {
        Self::MainToSideTransactionRequestUpvote {
            commitment_header: CommitmentHeader::from_bytes(&MAIN_TO_SIDE_TX_REQUEST_UPVOTE_COMMITMENT_HEADER),
            upvote
        }
    }
    pub fn new_side_to_main_tx_request(slot_number: SidechainSlotNumber, blinded_tx: BlindedTx) -> Self {
        Self::SideToMainTransactionRequest { commitment_header: CommitmentHeader::from_bytes(&SIDE_TO_MAIN_TX_REQUEST_COMMITMENT_HEADER),
            slot_number, blinded_tx,
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        match self {
            SidechainMessages::MainToSideTransactionRequest { commitment_header, slot_number, name, private_key } => {
                buf.extend_from_slice(&[OP_RETURN]);
                buf.push(MAIN_TO_SIDE_TRANSACTION_REQUEST_SIZE as u8);
                buf.extend_from_slice(&commitment_header.as_bytes());
                buf.extend_from_slice(&slot_number.as_bytes());
                buf.extend_from_slice(&name.as_bytes());
                buf.extend_from_slice(&private_key.as_bytes());
            },
            SidechainMessages::MainToSideTransactionRequestUpvote { commitment_header, upvote } => {
                buf.extend_from_slice(&[OP_RETURN]);
                buf.push(MAIN_TO_SIDE_TRANSACTION_REQUEST_UPVOTE_SIZE as u8);
                buf.extend_from_slice(&commitment_header.as_bytes());
                buf.extend_from_slice(&upvote.as_bytes());
            },
            SidechainMessages::MainToSideTransaction => { },
            SidechainMessages::SideToMainTransactionRequest { commitment_header, slot_number, blinded_tx } => {
                buf.extend_from_slice(&[OP_RETURN]);
                buf.push(SIDE_TO_MAIN_TRANSACTION_REQUEST_SIZE as u8);
                buf.extend_from_slice(&commitment_header.as_bytes());
                buf.extend_from_slice(&slot_number.as_bytes());
                buf.extend_from_slice(&blinded_tx.as_bytes());
            },
            SidechainMessages::SideToMainTransactionRequestUpvote => { },
            SidechainMessages::SideToMainTransaction => { },
        }
        buf
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes[0] != OP_RETURN {
            return Err(anyhow!("SidechainMessage serialization process failed due to no OP_RETURN code present"))
        }
        let length: u8 = bytes[LENGTH_START..LENGTH_END][0];
        let commitment_header = CommitmentHeader::from_bytes(&bytes[COMMITMENT_HEADER_START..COMMITMENT_HEADER_END]);
        let msg = match commitment_header.0 {
            MAIN_TO_SIDE_TX_REQUEST_COMMITMENT_HEADER => {
                println!("\ninbound bytes len {}, length field {}", bytes.len(), length);
                println!("slot number {} {}", M_TO_S_TX_REQ_SIDECHAIN_SLOT_NUMBER_START, M_TO_S_TX_REQ_SIDECHAIN_SLOT_NUMBER_END);
                let slot_number = SidechainSlotNumber::from_bytes(&bytes[M_TO_S_TX_REQ_SIDECHAIN_SLOT_NUMBER_START..M_TO_S_TX_REQ_SIDECHAIN_SLOT_NUMBER_END]);
                println!("sidechain name {} {}", M_TO_S_TX_REQ_SIDECHAIN_NAME_START, M_TO_S_TX_REQ_SIDECHAIN_NAME_END);
                let sidechain_name = SidechainName::from_bytes(&bytes[M_TO_S_TX_REQ_SIDECHAIN_NAME_START..M_TO_S_TX_REQ_SIDECHAIN_NAME_END]);
                println!("private key {} {}", M_TO_S_TX_REQ_SIDECHAIN_PRIVATE_KEY_START, M_TO_S_TX_REQ_SIDECHAIN_PRIVATE_KEY_END);
                let private_key = SidechainPrivateKey::from_bytes(&bytes[M_TO_S_TX_REQ_SIDECHAIN_PRIVATE_KEY_START..M_TO_S_TX_REQ_SIDECHAIN_PRIVATE_KEY_END]);
                SidechainMessages::new_main_to_side_tx_request(slot_number, sidechain_name, private_key)
            },
            MAIN_TO_SIDE_TX_REQUEST_UPVOTE_COMMITMENT_HEADER => {
                println!("\ninbound bytes len {}, length field {}", bytes.len(), length);
                let upvote = Upvote::from_bytes(&bytes[M_TO_S_TX_REQ_UPVOTE_START..M_TO_S_TX_REQ_UPVOTE_END]);
                SidechainMessages::new_main_to_side_tx_request_upvote(upvote)
            },
            SIDE_TO_MAIN_TX_REQUEST_COMMITMENT_HEADER => {
                println!("\ninbound bytes len {}, length field {}", bytes.len(), length);
                let slot_number = SidechainSlotNumber::from_bytes(&bytes[S_TO_M_TX_REQ_SIDECHAIN_SLOT_NUMBER_START..S_TO_M_TX_REQ_SIDECHAIN_SLOT_NUMBER_END]);
                let blinded_tx = BlindedTx::from_bytes(&bytes[S_TO_M_TX_REQ_BLINDED_TX_START..S_TO_M_TX_REQ_BLINDED_TX_END]);
                SidechainMessages::new_side_to_main_tx_request(slot_number, blinded_tx)
            },
            _ => return Err(anyhow!("SidechainMessage serialization process failed due to unrecognized commitment_header"))
        };
        Ok(msg)
    }
}
