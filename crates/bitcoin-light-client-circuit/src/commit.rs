use bitcoin::{Amount, TxOut};
use bitcoin_script::script;
use goat::transactions::base::DUST_AMOUNT;

pub fn generate_data_commitment_outputs_except_opreturn(data: &[u8]) -> Vec<TxOut> {
    let data_len = data.len();
        let mut txouts = vec![];
        let mut data = data.to_vec();
        for chunk in data.chunks(32) {
            txouts.push(TxOut {
                value: Amount::from_sat(DUST_AMOUNT),
                script_pubkey: script! {
                    OP_0
                    { chunk.to_vec() }
                }
                .compile(),
            });
        }
        txouts
}

pub fn extract_data_from_commitment_outputs_except_opreturn(txouts: &[TxOut]) -> Vec<u8> {
    let mut data = vec![];
    for txout in txouts {
        let script = &txout.script_pubkey;
        let instructions = script
            .instructions_minimal()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        if txout.value != Amount::from_sat(DUST_AMOUNT) {
            continue
        }
        if let bitcoin::blockdata::script::Instruction::PushBytes(bytes) = &instructions[1] {
            data.extend_from_slice(bytes.as_bytes());
        }
    }
    data
}
