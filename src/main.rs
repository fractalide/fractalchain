use {
    fractalchain::*,
};
fn main() {
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
}
