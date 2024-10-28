// @generated
/// DecryptionKey defines the structure to store
/// the decryption key of a particular identity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptionKey {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub creator: ::prost::alloc::string::String,
}
/// EncryptedTx defines the structure to store an encrypted transaction
/// by execution height
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedTx {
    #[prost(uint64, tag = "1")]
    pub target_height: u64,
    #[prost(uint64, tag = "2")]
    pub index: u64,
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub creator: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub charged_gas: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "6")]
    pub processed_at_chain_height: u64,
    #[prost(bool, tag = "7")]
    pub expired: bool,
}
/// EncryptedTxArray defines a list of EncryptedTx
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedTxArray {
    #[prost(message, repeated, tag = "1")]
    pub encrypted_txs: ::prost::alloc::vec::Vec<EncryptedTx>,
}
/// GeneralEncryptedTx defines the structure to store a
/// general encrypted transaction by identity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralEncryptedTx {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub index: u64,
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub creator: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub charged_gas: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// GeneralEncryptedTxArray defines a list of GeneralEncryptedTx
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralEncryptedTxArray {
    #[prost(message, repeated, tag = "1")]
    pub encrypted_txs: ::prost::alloc::vec::Vec<GeneralEncryptedTx>,
}
/// IdentityExecutionEntry defines the structure to queue up
/// identities that have decryption keys available and
/// are ready to execute any associated contracts or encrypted transactions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityExecutionEntry {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub tx_list: ::core::option::Option<GeneralEncryptedTxArray>,
    #[prost(string, tag = "6")]
    pub decryption_key: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// option (gogoproto.equal) = true;
    #[prost(string, tag = "1")]
    pub keyshare_channel_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_source_chain: bool,
    #[prost(message, repeated, tag = "3")]
    pub trusted_counter_parties: ::prost::alloc::vec::Vec<TrustedCounterParty>,
    #[prost(string, repeated, tag = "4")]
    pub trusted_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub min_gas_price: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub private_decryption_key_price:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// TrustedCounterParty defines the structure to store the ibc info
/// of the source chain (fairyring) to reliably fetch active keys and
/// general/private decryption keys
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrustedCounterParty {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub channel_id: ::prost::alloc::string::String,
}
/// PepNonce defines the nonce of an account to send encrypted transactions.
/// It is incremanted seperately from the nonce maintained by the auth module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PepNonce {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub nonce: u64,
}
/// RequestId defines the structure for storing request ids
/// that have already been registered to prevent overlap
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestId {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub req_id: ::prost::alloc::string::String,
}
/// PrivateRequest defines the structure for storing private
/// decryption key requests along with the unaggregated encrypted keyshares
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateRequest {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub private_decryption_keys: ::prost::alloc::vec::Vec<super::common::PrivateDecryptionKey>,
}
/// ContractDetails defines the structure to store the details of a
/// contract that has been registered to execute automatically when
/// the identity associated with it has a decryption key available
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractDetails {
    #[prost(string, tag = "1")]
    pub registrar: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
}
/// RegisteredContract defines the structure to store the list of
/// contracts that have been registered with a particular identity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredContract {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub contracts: ::prost::alloc::vec::Vec<ContractDetails>,
}
/// ExecuteContractMsg defines the structure to callback registered contracts
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteContractMsg {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub decryption_key: ::prost::alloc::string::String,
}
/// GenesisState defines the pep module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(string, tag = "2")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub encrypted_tx_array: ::prost::alloc::vec::Vec<EncryptedTxArray>,
    #[prost(message, repeated, tag = "4")]
    pub pep_nonce_list: ::prost::alloc::vec::Vec<PepNonce>,
    #[prost(message, repeated, tag = "6")]
    pub decryption_key_list: ::prost::alloc::vec::Vec<DecryptionKey>,
    #[prost(message, optional, tag = "7")]
    pub active_pubkey: ::core::option::Option<super::common::ActivePublicKey>,
    #[prost(message, optional, tag = "8")]
    pub queued_pubkey: ::core::option::Option<super::common::QueuedPublicKey>,
    #[prost(uint64, tag = "9")]
    pub request_count: u64,
    #[prost(message, repeated, tag = "10")]
    pub request_id_list: ::prost::alloc::vec::Vec<RequestId>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryGeneralIdentityRequest is request type for the Query/GeneralIdentity RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGeneralIdentityRequest {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
}
/// QueryGeneralIdentityResponse is response type for the Query/GeneralIdentity RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGeneralIdentityResponse {
    #[prost(message, optional, tag = "1")]
    pub request_details: ::core::option::Option<IdentityExecutionEntry>,
}
/// QueryGeneralIdentityAllRequest is request type for the Query/GeneralIdentityAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGeneralIdentityAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGeneralIdentityAllResponse is response type for the Query/GeneralIdentityAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGeneralIdentityAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub request_details_list: ::prost::alloc::vec::Vec<IdentityExecutionEntry>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryEncryptedTxRequest is request type for the Query/EncryptedTx RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEncryptedTxRequest {
    #[prost(uint64, tag = "1")]
    pub target_height: u64,
    #[prost(uint64, tag = "2")]
    pub index: u64,
}
/// QueryEncryptedTxResponse is response type for the Query/EncryptedTx RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEncryptedTxResponse {
    #[prost(message, optional, tag = "1")]
    pub encrypted_tx: ::core::option::Option<EncryptedTx>,
}
/// QueryEncryptedTxAllRequest is request type for the Query/EncryptedTxAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEncryptedTxAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryEncryptedTxAllResponse is response type for the Query/EncryptedTxAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEncryptedTxAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub encrypted_tx_array: ::prost::alloc::vec::Vec<EncryptedTxArray>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryEncryptedTxAllFromHeightRequest is request type for the Query/EncryptedTxAllFromHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEncryptedTxAllFromHeightRequest {
    #[prost(uint64, tag = "1")]
    pub target_height: u64,
}
/// QueryEncryptedTxAllFromHeightResponse is response type for the Query/EncryptedTxAllFromHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEncryptedTxAllFromHeightResponse {
    #[prost(message, optional, tag = "1")]
    pub encrypted_tx_array: ::core::option::Option<EncryptedTxArray>,
}
/// QueryLatestHeightRequest is request type for the Query/LatestHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLatestHeightRequest {}
/// QueryLatestHeightResponse is response type for the Query/LatestHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLatestHeightResponse {
    #[prost(uint64, tag = "1")]
    pub height: u64,
}
/// QueryPepNonceRequest is request type for the Query/PepNonce RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPepNonceRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryPepNonceResponse is response type for the Query/PepNonce RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPepNonceResponse {
    #[prost(message, optional, tag = "1")]
    pub pep_nonce: ::core::option::Option<PepNonce>,
}
/// QueryPepNonceAllRequest is request type for the Query/PepNonceAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPepNonceAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryPepNonceAllResponse is response type for the Query/PepNonceAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPepNonceAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub pep_nonce: ::prost::alloc::vec::Vec<PepNonce>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryPubkeyRequest is request type for the Query/Pubkey RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPubkeyRequest {}
/// QueryPubkeyResponse is response type for the Query/Pubkey RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPubkeyResponse {
    #[prost(message, optional, tag = "1")]
    pub active_pubkey: ::core::option::Option<super::common::ActivePublicKey>,
    #[prost(message, optional, tag = "2")]
    pub queued_pubkey: ::core::option::Option<super::common::QueuedPublicKey>,
}
/// QueryPrivateIdentityRequest is request type for the Query/PrivateIdentity RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPrivateIdentityRequest {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
}
/// QueryPrivateIdentityResponse is response type for the Query/PrivateIdentity RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPrivateIdentityResponse {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub private_decryption_keys: ::prost::alloc::vec::Vec<super::common::PrivateDecryptionKey>,
}
/// QueryDecryptDataRequest is request type for the Query/DecryptData RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDecryptDataRequest {
    #[prost(string, tag = "1")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub decryption_key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub encrypted_data: ::prost::alloc::string::String,
}
/// QueryDecryptDataResponse is response type for the Query/DecryptData RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDecryptDataResponse {
    #[prost(string, tag = "1")]
    pub decrypted_data: ::prost::alloc::string::String,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    // params defines the module parameters to update.
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
/// MsgSubmitEncryptedTx is the Msg/SubmitEncryptedTx request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEncryptedTx {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub target_block_height: u64,
}
/// MsgSubmitEncryptedTxResponse defines the response structure for executing a
/// MsgSubmitEncryptedTx message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEncryptedTxResponse {}
/// MsgSubmitGeneralEncryptedTx is the Msg/SubmitGeneralEncryptedTx request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitGeneralEncryptedTx {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub req_id: ::prost::alloc::string::String,
}
/// MsgSubmitGeneralEncryptedTxResponse defines the response structure for executing a
/// MsgSubmitGeneralEncryptedTx message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitGeneralEncryptedTxResponse {}
/// MsgSubmitDecryptionKey is the Msg/SubmitDecryptionKey request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitDecryptionKey {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub height: u64,
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
}
/// MsgSubmitDecryptionKeyResponse defines the response structure for executing a
/// MsgSubmitDecryptionKey message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitDecryptionKeyResponse {}
/// MsgRequestGeneralIdentity is the Msg/RequestGeneralIdentity request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestGeneralIdentity {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub estimated_delay: ::core::option::Option<::prost_types::Duration>,
    #[prost(string, tag = "3")]
    pub req_id: ::prost::alloc::string::String,
}
/// MsgRequestGeneralIdentityResponse defines the response structure for executing a
/// MsgRequestGeneralIdentity message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestGeneralIdentityResponse {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
}
/// MsgRequestGeneralDecryptionKey is the Msg/RequestGeneralDecryptionKey request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestGeneralDecryptionKey {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
}
/// MsgRequestGeneralDecryptionKeyResponse defines the response structure for executing a
/// MsgRequestGeneralDecryptionKey message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestGeneralDecryptionKeyResponse {}
/// MsgRequestPrivateIdentity is the Msg/RequestPrivateIdentity request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestPrivateIdentity {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub req_id: ::prost::alloc::string::String,
}
/// MsgRequestPrivateIdentityResponse defines the response structure for executing a
/// MsgRequestPrivateIdentity message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestPrivateIdentityResponse {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
}
/// MsgRequestPrivateDecryptionKey is the Msg/RequestPrivateDecryptionKey request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestPrivateDecryptionKey {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub secp_pubkey: ::prost::alloc::string::String,
}
/// MsgRequestPrivateDecryptionKeyResponse defines the response structure for executing a
/// MsgRequestPrivateDecryptionKey message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestPrivateDecryptionKeyResponse {}
/// MsgRegisterContract is the Msg/RegisterContract request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterContract {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub identity: ::prost::alloc::string::String,
}
/// MsgRegisterContractResponse defines the response structure for executing a
/// MsgRegisterContract message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterContractResponse {}
/// MsgUnregisterContract is the Msg/UnregisterContract request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnregisterContract {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub identity: ::prost::alloc::string::String,
}
/// MsgUnregisterContractResponse defines the response structure for executing a
/// MsgUnregisterContract message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnregisterContractResponse {}
#[cfg(feature = "grpc")]
include!("fairyring.pep.tonic.rs");
// @@protoc_insertion_point(module)

