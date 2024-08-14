use std::str::FromStr;

use cosmos_sdk_proto::{
    cosmos::{
        auth::v1beta1::{BaseAccount, QueryAccountRequest},
        tx::v1beta1::{BroadcastMode, BroadcastTxRequest},
    },
    prost::Message,
};
use cosmrs::{
    bank::MsgSend,
    tx::{Body, Fee, Msg, SignDoc, SignerInfo},
    Coin,
};
use fairblock_proto::fairyring::{keyshare::MsgSendKeyshare, pep::QueryGetPepNonceRequest};
use tonic::transport::Channel;

const CHAIN_ID: &str = "fairyring-testnet-2";

#[tokio::main]
async fn main() {
    let mnemonic_string = "test test test test test test test test test test test junk";
    let mnemonic = bip39::Mnemonic::from_str(mnemonic_string).unwrap();
    let signing_key = cosmrs::crypto::secp256k1::SigningKey::derive_from_path(
        mnemonic.to_seed(""),
        &"m/44'/118'/0'/0/0".parse().unwrap(),
    )
    .unwrap();

    let address = signing_key.public_key().account_id("fairy").unwrap();
    println!("{:?}", address);

    let amount = Coin {
        amount: 1_000_000u128,
        denom: "ufairy".parse().unwrap(),
    };

    let msg_send = MsgSend {
        from_address: address.clone(),
        to_address: address.clone(),
        amount: vec![amount],
    };

    let channel = Channel::from_static("http://54.210.57.197:9090")
        .connect()
        .await
        .unwrap();

    let mut keyshare_query_client =
        fairblock_proto::fairyring::keyshare::query_client::QueryClient::new(channel.clone());
    let mut pep_query_client =
        fairblock_proto::fairyring::pep::query_client::QueryClient::new(channel.clone());
    let mut auth_query_client =
        cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient::new(channel.clone());

    let account_query_request = QueryAccountRequest {
        address: address.clone().to_string(),
    };

    let account_query_response = auth_query_client
        .account(account_query_request)
        .await
        .unwrap()
        .into_inner();

    let account = account_query_response.account.expect("account not found");
    let account = BaseAccount::decode(account.value.as_slice()).unwrap();

    let account_number = account.account_number;
    let sequence_number = account.sequence;

    let pep_sequence_request = QueryGetPepNonceRequest {
        address: address.to_string(),
    };
    let pep_sequence_response = pep_query_client
        .pep_nonce(pep_sequence_request)
        .await
        .unwrap()
        .into_inner();

    let pep_sequence_number = pep_sequence_response
        .pep_nonce
        .expect("account not found")
        .nonce;

    let tx_body = Body::new(
        vec![msg_send.to_any().unwrap()],
        "testing rust client",
        0_u16,
    );
    let signer_info = SignerInfo::single_direct(Some(signing_key.public_key()), sequence_number);
    let gas_info = Coin {
        amount: 0_u128,
        denom: "ufairy".parse().unwrap(),
    };
    let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(gas_info, 2_000_000_000u64));
    let sign_doc = SignDoc::new(
        &tx_body,
        &auth_info,
        &CHAIN_ID.parse().unwrap(),
        account_number,
    )
    .unwrap();
    let tx_signed = sign_doc.sign(&signing_key).unwrap();
    let tx_bytes = tx_signed.to_bytes().unwrap();

    let mut tx_client =
        cosmrs::proto::cosmos::tx::v1beta1::service_client::ServiceClient::new(channel.clone());

    let request = BroadcastTxRequest {
        tx_bytes,
        mode: BroadcastMode::Sync.into(),
    };
    let response = tx_client.broadcast_tx(request).await;
    println!("{:?}", response);
}
