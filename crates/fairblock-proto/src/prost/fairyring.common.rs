// @generated
/// RequestDecryptionKey defines a struct for the data payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDecryptionKey {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub estimated_delay: ::core::option::Option<::prost_types::Duration>,
    /// id can either be a identity or a proposal id
    #[prost(oneof="request_decryption_key::Id", tags="2, 3")]
    pub id: ::core::option::Option<request_decryption_key::Id>,
}
/// Nested message and enum types in `RequestDecryptionKey`.
pub mod request_decryption_key {
    /// id can either be a identity or a proposal id
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        #[prost(string, tag="2")]
        ProposalId(::prost::alloc::string::String),
        #[prost(string, tag="3")]
        Identity(::prost::alloc::string::String),
    }
}
/// RequestDecryptionKeyResponse defines the response to the RequestDecryptionKey message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDecryptionKeyResponse {
    #[prost(string, tag="1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pubkey: ::prost::alloc::string::String,
}
/// GetDecryptionKey defines a struct for the data payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDecryptionKey {
    #[prost(bool, tag="1")]
    pub is_governance_proposal: bool,
    #[prost(string, tag="2")]
    pub proposal_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub identity: ::prost::alloc::string::String,
}
/// GetDecryptionKeyResponse defines the response to the GetDecryptionKey message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDecryptionKeyResponse {
}
/// GetPrivateDecryptionKey defines a struct for the data payload
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPrivateDecryptionKey {
    #[prost(string, tag="1")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub requester: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub secp_pubkey: ::prost::alloc::string::String,
}
/// GetPrivateDecryptionKeyResponse defines the response to the GetPrivateDecryptionKey message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPrivateDecryptionKeyResponse {
    #[prost(string, tag="1")]
    pub pubkey: ::prost::alloc::string::String,
}
/// ActivePublicKey defines the pubkey currently in use
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivePublicKey {
    #[prost(string, tag="1")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub expiry: u64,
}
/// QueuedPublicKey defines the pubkey that (when set) will replace the acive pubkey
/// when it expires
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueuedPublicKey {
    #[prost(string, tag="1")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub creator: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub expiry: u64,
}
/// RequestPrivateDecryptionKey defines the structure to request for
/// encrypted and unaggregated keyshares
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPrivateDecryptionKey {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub identity: ::prost::alloc::string::String,
}
/// PrivateDecryptionKey defines the storage structure for
/// the list of encrypted keyshares (unaggregated)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateDecryptionKey {
    #[prost(string, tag="1")]
    pub requester: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub private_keyshares: ::prost::alloc::vec::Vec<IndexedEncryptedKeyshare>,
}
/// IndexedEncryptedKeyshare defines the storage of submitted encrypted
/// keyshares along with their indices (can be decrypted and aggregated)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexedEncryptedKeyshare {
    #[prost(string, tag="1")]
    pub encrypted_keyshare_value: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub encrypted_keyshare_index: u64,
}
// @@protoc_insertion_point(module)
