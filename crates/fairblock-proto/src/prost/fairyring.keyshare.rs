// @generated
/// AuthorizedAddress defines if an address is authorized to submit keyshares
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizedAddress {
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_authorized: bool,
    #[prost(string, tag = "3")]
    pub authorized_by: ::prost::alloc::string::String,
}
/// Commitments defines the list of commitments to verify the
/// keyshares submitted by validators
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commitments {
    #[prost(string, repeated, tag = "1")]
    pub commitments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Decryption key defines the structure and height for a decryption key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptionKey {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
/// Keyshare defines the structure for submitting
/// blockwise keyshares by validators
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keyshare {
    #[prost(string, tag = "1")]
    pub validator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
    #[prost(string, tag = "3")]
    pub keyshare: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub keyshare_index: u64,
    #[prost(uint64, tag = "5")]
    pub received_timestamp: u64,
    #[prost(uint64, tag = "6")]
    pub received_block_height: u64,
}
/// GeneralKeyshare defines the structure for submitting
/// general keyshares by validators
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralKeyshare {
    #[prost(string, tag = "1")]
    pub validator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id_value: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub keyshare: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub keyshare_index: u64,
    #[prost(uint64, tag = "6")]
    pub received_timestamp: u64,
    #[prost(uint64, tag = "7")]
    pub received_block_height: u64,
}
/// ValidatorEncryptedKeyshare defines the structure for
/// submitting encrypted keyshares by validators
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorEncryptedKeyshare {
    #[prost(string, tag = "1")]
    pub validator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub requester: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub keyshare: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub keyshare_index: u64,
    #[prost(uint64, tag = "5")]
    pub received_timestamp: u64,
    #[prost(uint64, tag = "6")]
    pub received_block_height: u64,
    #[prost(string, tag = "7")]
    pub identity: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag = "1")]
    pub key_expiry: u64,
    #[prost(uint64, tag = "2")]
    pub minimum_bonded: u64,
    #[prost(uint64, tag = "3")]
    pub max_idled_block: u64,
    #[prost(string, repeated, tag = "4")]
    pub trusted_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes = "vec", tag = "5")]
    pub slash_fraction_no_keyshare: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub slash_fraction_wrong_keyshare: ::prost::alloc::vec::Vec<u8>,
}
/// EncryptedKeyshare defines the structure for storing
/// the keyshare of the master secret key distributed to the validators
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedKeyshare {
    #[prost(string, tag = "1")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator: ::prost::alloc::string::String,
}
/// ActivePubkey defines the structure of the active public key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivePubkey {
    #[prost(string, tag = "1")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub expiry: u64,
    #[prost(uint64, tag = "4")]
    pub number_of_validators: u64,
    #[prost(message, repeated, tag = "5")]
    pub encrypted_keyshares: ::prost::alloc::vec::Vec<EncryptedKeyshare>,
}
/// QueuedPubkey defines the structure of the queued public key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueuedPubkey {
    #[prost(string, tag = "1")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub expiry: u64,
    #[prost(uint64, tag = "4")]
    pub number_of_validators: u64,
    #[prost(message, repeated, tag = "5")]
    pub encrypted_keyshares: ::prost::alloc::vec::Vec<EncryptedKeyshare>,
}
/// ValidatorSet defines the structure for storing the list of
/// validators who will be eligible to send keyshares
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSet {
    #[prost(string, tag = "1")]
    pub index: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub cons_addr: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub is_active: bool,
}
/// GenesisState defines the keyshare module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(string, tag = "2")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub validator_set_list: ::prost::alloc::vec::Vec<ValidatorSet>,
    #[prost(message, repeated, tag = "4")]
    pub keyshare_list: ::prost::alloc::vec::Vec<Keyshare>,
    #[prost(message, repeated, tag = "5")]
    pub decryption_key_list: ::prost::alloc::vec::Vec<DecryptionKey>,
    #[prost(message, optional, tag = "6")]
    pub active_pubkey: ::core::option::Option<ActivePubkey>,
    #[prost(message, optional, tag = "7")]
    pub queued_pubkey: ::core::option::Option<QueuedPubkey>,
    #[prost(message, repeated, tag = "8")]
    pub authorized_address_list: ::prost::alloc::vec::Vec<AuthorizedAddress>,
    #[prost(uint64, tag = "9")]
    pub request_count: u64,
    #[prost(message, repeated, tag = "10")]
    pub general_keyshare_list: ::prost::alloc::vec::Vec<GeneralKeyshare>,
}
/// KeysharePacketData defines all the packet types of the keyshare module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeysharePacketData {
    /// packet can be one of the following types
    #[prost(
        oneof = "keyshare_packet_data::Packet",
        tags = "1, 2, 3, 4, 5, 6, 7, 8"
    )]
    pub packet: ::core::option::Option<keyshare_packet_data::Packet>,
}
/// Nested message and enum types in `KeysharePacketData`.
pub mod keyshare_packet_data {
    /// packet can be one of the following types
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Packet {
        #[prost(message, tag = "1")]
        NoData(super::NoData),
        #[prost(message, tag = "2")]
        RequestDecryptionKeyPacket(super::RequestDecryptionKeyPacketData),
        #[prost(message, tag = "3")]
        GetDecryptionKeyPacket(super::GetDecryptionKeyPacketData),
        #[prost(message, tag = "4")]
        DecryptionKeyDataPacket(super::DecryptionKeyDataPacketData),
        #[prost(message, tag = "5")]
        PrivateDecryptionKeyDataPacket(super::PrivateDecryptionKeyDataPacketData),
        #[prost(message, tag = "6")]
        CurrentKeysPacket(super::CurrentKeysPacketData),
        #[prost(message, tag = "7")]
        RequestPrivateDecryptionKeyPacket(super::RequestPrivateDecryptionKeyPacketData),
        #[prost(message, tag = "8")]
        GetPrivateDecryptionKeyPacket(super::GetPrivateDecryptionKeyPacketData),
    }
}
/// NoData defines a blank packet
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoData {}
/// RequestDecryptionKeyPacketData defines a struct for the packet payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDecryptionKeyPacketData {
    #[prost(string, tag = "1")]
    pub requester: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub estimated_delay: ::core::option::Option<::prost_types::Duration>,
    /// id can either be a request id or a proposal id
    #[prost(oneof = "request_decryption_key_packet_data::Id", tags = "2, 3")]
    pub id: ::core::option::Option<request_decryption_key_packet_data::Id>,
}
/// Nested message and enum types in `RequestDecryptionKeyPacketData`.
pub mod request_decryption_key_packet_data {
    /// id can either be a request id or a proposal id
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        #[prost(string, tag = "2")]
        ProposalId(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        Identity(::prost::alloc::string::String),
    }
}
/// RequestPrivateDecryptionKeyPacketData defines a struct for the packet payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPrivateDecryptionKeyPacketData {
    #[prost(string, tag = "1")]
    pub requester: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
}
/// RequestPrivateDecryptionKeyPacketAck defines a struct for the packet acknowledgment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPrivateDecryptionKeyPacketAck {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
}
/// RequestDecryptionKeyPacketAck defines a struct for the packet acknowledgment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDecryptionKeyPacketAck {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
}
/// GetDecryptionKeyPacketData defines a struct for the packet payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDecryptionKeyPacketData {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
}
/// GetDecryptionKeyPacketAck defines a struct for the packet acknowledgment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDecryptionKeyPacketAck {}
/// GetPrivateDecryptionKeyPacketData defines a struct for the packet payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPrivateDecryptionKeyPacketData {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub requester: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub secp_pubkey: ::prost::alloc::string::String,
}
/// GetPrivateKeysharePacketAck defines a struct for the packet acknowledgment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPrivateDecryptionKeyPacketAck {}
/// DecryptionKeyDataPacketData defines a struct for the packet payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptionKeyDataPacketData {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub decryption_key: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub aggr_height: ::prost::alloc::string::String,
    /// used for private governance
    #[prost(string, tag = "5")]
    pub proposal_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    pub retries: u64,
}
/// DecryptionKeyPacketAck defines a struct for the packet acknowledgment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptionKeyPacketAck {}
/// PrivateDecryptionKeyDataPacketData defines a struct for the packet payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateDecryptionKeyDataPacketData {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub private_decryption_key: ::prost::alloc::vec::Vec<super::common::PrivateDecryptionKey>,
}
/// PrivateDecryptionKeyPacketAck defines a struct for the packet payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateDecryptionKeyPacketAck {}
/// CurrentKeysPacketData defines a struct for the packet payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentKeysPacketData {}
/// CurrentKeysPacketAck defines a struct for the packet acknowledgment
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentKeysPacketAck {
    #[prost(message, optional, tag = "1")]
    pub active_key: ::core::option::Option<super::common::ActivePublicKey>,
    #[prost(message, optional, tag = "2")]
    pub queued_key: ::core::option::Option<super::common::QueuedPublicKey>,
}
/// QueryVerifiableRandomnessRequest is the request type for
/// the Query/VerifiableRandomness  method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVerifiableRandomnessRequest {}
/// QueryVerifiableRandomnessResponse is the response type for
/// the Query/VerifiableRandomness  method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVerifiableRandomnessResponse {
    #[prost(string, tag = "1")]
    pub randomness: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub round: u64,
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
/// QueryCommitmentsRequest is request type for the Query/Commitments RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCommitmentsRequest {}
/// QueryCommitmentsResponse is response type for the Query/Commitments RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCommitmentsResponse {
    #[prost(message, optional, tag = "1")]
    pub active_commitments: ::core::option::Option<Commitments>,
    #[prost(message, optional, tag = "2")]
    pub queued_commitments: ::core::option::Option<Commitments>,
}
/// QueryValidatorSetRequest is request type for the Query/ValidatorSet RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorSetRequest {
    #[prost(string, tag = "1")]
    pub index: ::prost::alloc::string::String,
}
/// QueryValidatorSetResponse is response type for the Query/ValidatorSet RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorSetResponse {
    #[prost(message, optional, tag = "1")]
    pub validator_set: ::core::option::Option<ValidatorSet>,
}
/// QueryValidatorSetAllRequest is request type for the Query/ValidatorSetAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorSetAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryValidatorSetAllResponse is response type for the Query/ValidatorSetAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorSetAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub validator_set: ::prost::alloc::vec::Vec<ValidatorSet>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryKeyshareRequest is request type for the Query/Keyshare RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryKeyshareRequest {
    #[prost(string, tag = "1")]
    pub validator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
}
/// QueryKeyshareResponse is response type for the Query/Keyshare RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryKeyshareResponse {
    #[prost(message, optional, tag = "1")]
    pub keyshare: ::core::option::Option<Keyshare>,
}
/// QueryKeyshareAllRequest is request type for the Query/KeyshareAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryKeyshareAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryKeyshareAllResponse is response type for the Query/KeyshareAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryKeyshareAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub keyshare: ::prost::alloc::vec::Vec<Keyshare>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryDecryptionKeyRequest is request type for the Query/DecryptionKey RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDecryptionKeyRequest {
    #[prost(uint64, tag = "1")]
    pub height: u64,
}
/// QueryDecryptionKeyResponse is response type for the Query/DecryptionKey RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDecryptionKeyResponse {
    #[prost(message, optional, tag = "1")]
    pub decryption_key: ::core::option::Option<DecryptionKey>,
}
/// QueryDecryptionKeyAllRequest is request type for the Query/DecryptionKeyAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDecryptionKeyAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryDecryptionKeyAllResponse is response type for the Query/DecryptionKeyAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDecryptionKeyAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub decryption_keys: ::prost::alloc::vec::Vec<DecryptionKey>,
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
    pub active_pubkey: ::core::option::Option<ActivePubkey>,
    #[prost(message, optional, tag = "2")]
    pub queued_pubkey: ::core::option::Option<QueuedPubkey>,
}
/// QueryAuthorizedAddressRequest is request type for the Query/AuthorizedAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuthorizedAddressRequest {
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
}
/// QueryAuthorizedAddressResponse is response type for the Query/AuthorizedAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuthorizedAddressResponse {
    #[prost(message, optional, tag = "1")]
    pub authorized_address: ::core::option::Option<AuthorizedAddress>,
}
/// QueryAuthorizedAddressAllRequest is request type for the Query/AuthorizedAddressAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuthorizedAddressAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAuthorizedAddressAllResponse is response type for the Query/AuthorizedAddressAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuthorizedAddressAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub authorized_address: ::prost::alloc::vec::Vec<AuthorizedAddress>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGeneralKeyshareRequest is request type for the Query/GeneralKeyshare RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGeneralKeyshareRequest {
    #[prost(string, tag = "1")]
    pub validator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id_value: ::prost::alloc::string::String,
}
/// QueryGeneralKeyshareResponse is response type for the Query/GeneralKeyshare RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGeneralKeyshareResponse {
    #[prost(message, optional, tag = "1")]
    pub general_keyshare: ::core::option::Option<GeneralKeyshare>,
}
/// QueryGeneralKeyshareAllRequest is request type for the Query/GeneralKeyshareAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGeneralKeyshareAllRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGeneralKeyshareAllResponse is response type for the Query/GeneralKeyshareAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGeneralKeyshareAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub general_keyshare: ::prost::alloc::vec::Vec<GeneralKeyshare>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// DecryptionKeyRequest defines the storage structure for general keyshare requests
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptionKeyRequest {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
    /// Used only when the request is made via IBC
    #[prost(message, optional, tag = "3")]
    pub ibc_info: ::core::option::Option<IbcInfo>,
    /// Used only when the request is made via IBC
    #[prost(message, optional, tag = "4")]
    pub counterparty: ::core::option::Option<CounterPartyIbcInfo>,
    #[prost(string, tag = "5")]
    pub decryption_key: ::prost::alloc::string::String,
    /// This is only used when the request is for private governance
    #[prost(string, tag = "6")]
    pub proposal_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "7")]
    pub sent: bool,
}
/// IBCInfo defines the structure to verify request for
/// general and private keyshares in case the request was made over IBC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IbcInfo {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub port_id: ::prost::alloc::string::String,
}
/// CounterPartyIBCInfo defines the structure to send general
/// and private keyshares if the request was made over IBC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CounterPartyIbcInfo {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub port_id: ::prost::alloc::string::String,
}
/// PrivateDecryptionKeyRequest defines the stroage structure for private
/// encrypted and unaggregated decryption key requests
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateDecryptionKeyRequest {
    #[prost(string, tag = "1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
    /// Used only when the request is made via IBC
    #[prost(message, optional, tag = "3")]
    pub ibc_info: ::core::option::Option<IbcInfo>,
    /// Used only when the request is made via IBC
    #[prost(message, optional, tag = "4")]
    pub counterparty: ::core::option::Option<CounterPartyIbcInfo>,
    #[prost(message, repeated, tag = "5")]
    pub private_decryption_keys: ::prost::alloc::vec::Vec<super::common::PrivateDecryptionKey>,
    #[prost(bool, tag = "7")]
    pub sent: bool,
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
/// MsgUpdateParamsResponse defines the response structure for executing a MsgUpdateParams message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
/// MsgRegisterValidator is the Msg/RegisterValidator request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterValidator {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgRegisterValidatorResponse defines the response structure for
/// executing a MsgRegisterValidator message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterValidatorResponse {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgDeRegisterValidator is the Msg/DeRegisterValidator request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeRegisterValidator {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgDeRegisterValidatorResponse defines the response structure for
/// executing a MsgDeRegisterValidator message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeRegisterValidatorResponse {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgSendKeyshare is the Msg/SendKeyshare request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendKeyshare {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub keyshare_index: u64,
    #[prost(uint64, tag = "4")]
    pub block_height: u64,
}
/// MsgSendKeyshareResponse defines the response structure for
/// executing a MsgSendKeyshare message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendKeyshareResponse {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyshare: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub keyshare_index: u64,
    #[prost(uint64, tag = "4")]
    pub block_height: u64,
    #[prost(uint64, tag = "5")]
    pub received_block_height: u64,
    #[prost(bool, tag = "6")]
    pub success: bool,
    #[prost(string, tag = "7")]
    pub error_message: ::prost::alloc::string::String,
}
/// MsgCreateLatestPubkey is the Msg/CreateLatestPubkey request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateLatestPubkey {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub commitments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag = "4")]
    pub number_of_validators: u64,
    #[prost(message, repeated, tag = "5")]
    pub encrypted_keyshares: ::prost::alloc::vec::Vec<EncryptedKeyshare>,
}
/// MsgCreateLatestPubkeyResponse defines the response structure for
/// executing a MsgCreateLatestPubkey message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateLatestPubkeyResponse {}
/// MsgOverrideLatestPubkey is the Msg/OverrideLatestPubkey request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOverrideLatestPubkey {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub commitments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag = "4")]
    pub number_of_validators: u64,
    #[prost(message, repeated, tag = "5")]
    pub encrypted_keyshares: ::prost::alloc::vec::Vec<EncryptedKeyshare>,
}
/// MsgOverrideLatestPubkeyResponse defines the response structure for
/// executing a MsgOverrideLatestPubkey message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgOverrideLatestPubkeyResponse {}
/// MsgCreateAuthorizedAddress is the Msg/CreateAuthorizedAddress request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAuthorizedAddress {
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgCreateAuthorizedAddressResponse defines the response structure for
/// executing a MsgCreateAuthorizedAddress message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateAuthorizedAddressResponse {}
/// MsgUpdateAuthorizedAddress is the Msg/UpdateAuthorizedAddress request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAuthorizedAddress {
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_authorized: bool,
    #[prost(string, tag = "3")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgUpdateAuthorizedAddressResponse defines the response structure for
/// executing a MsgUpdateAuthorizedAddress message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAuthorizedAddressResponse {}
/// MsgDeleteAuthorizedAddress is the Msg/DeleteAuthorizedAddress request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAuthorizedAddress {
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
}
/// MsgDeleteAuthorizedAddressResponse defines the response structure for
/// executing a MsgDeleteAuthorizedAddress message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAuthorizedAddressResponse {}
/// MsgSubmitGeneralKeyshare is the Msg/CreateGeneralKeyshare request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitGeneralKeyshare {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id_value: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub keyshare: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub keyshare_index: u64,
    #[prost(uint64, tag = "6")]
    pub received_timestamp: u64,
    #[prost(uint64, tag = "7")]
    pub received_block_height: u64,
}
/// MsgSubmitGeneralKeyshareResponse defines the response structure for
/// executing a MsgSubmitGeneralKeyshare message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitGeneralKeyshareResponse {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id_value: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub keyshare: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub keyshare_index: u64,
    #[prost(uint64, tag = "6")]
    pub received_block_height: u64,
    #[prost(bool, tag = "7")]
    pub success: bool,
    #[prost(string, tag = "8")]
    pub error_message: ::prost::alloc::string::String,
}
/// MsgSubmitEncryptedKeyshare is the Msg/SubmitEncryptedKeyshare request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEncryptedKeyshare {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub encrypted_keyshare: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub keyshare_index: u64,
    #[prost(uint64, tag = "5")]
    pub received_timestamp: u64,
    #[prost(uint64, tag = "6")]
    pub received_block_height: u64,
    #[prost(string, tag = "7")]
    pub requester: ::prost::alloc::string::String,
}
/// MsgSubmitEncryptedKeyshareResponse defines the response structure for
/// executing a MsgSubmitEncryptedKeyshare message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEncryptedKeyshareResponse {}
#[cfg(feature = "grpc")]
include!("fairyring.keyshare.tonic.rs");
// @@protoc_insertion_point(module)

