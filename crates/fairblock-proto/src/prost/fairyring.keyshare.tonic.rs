// @generated
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryParamsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Query/Params",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("fairyring.keyshare.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn commitments(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCommitmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryCommitmentsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Query/Commitments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("fairyring.keyshare.Query", "Commitments"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn validator_set(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorSetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidatorSetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Query/ValidatorSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("fairyring.keyshare.Query", "ValidatorSet"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn validator_set_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorSetAllRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidatorSetAllResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Query/ValidatorSetAll",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("fairyring.keyshare.Query", "ValidatorSetAll"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn keyshare(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryKeyshareRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryKeyshareResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Query/Keyshare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("fairyring.keyshare.Query", "Keyshare"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn keyshare_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryKeyshareAllRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryKeyshareAllResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Query/KeyshareAll",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("fairyring.keyshare.Query", "KeyshareAll"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn decryption_key(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDecryptionKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDecryptionKeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Query/DecryptionKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("fairyring.keyshare.Query", "DecryptionKey"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn decryption_key_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDecryptionKeyAllRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDecryptionKeyAllResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Query/DecryptionKeyAll",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("fairyring.keyshare.Query", "DecryptionKeyAll"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pubkey(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPubkeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPubkeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Query/Pubkey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("fairyring.keyshare.Query", "Pubkey"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn authorized_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAuthorizedAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAuthorizedAddressResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Query/AuthorizedAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("fairyring.keyshare.Query", "AuthorizedAddress"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn authorized_address_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAuthorizedAddressAllRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAuthorizedAddressAllResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Query/AuthorizedAddressAll",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("fairyring.keyshare.Query", "AuthorizedAddressAll"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn general_keyshare(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGeneralKeyshareRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGeneralKeyshareResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Query/GeneralKeyshare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("fairyring.keyshare.Query", "GeneralKeyshare"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn general_keyshare_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGeneralKeyshareAllRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGeneralKeyshareAllResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Query/GeneralKeyshareAll",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("fairyring.keyshare.Query", "GeneralKeyshareAll"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn verifiable_randomness(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVerifiableRandomnessRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryVerifiableRandomnessResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Query/VerifiableRandomness",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("fairyring.keyshare.Query", "VerifiableRandomness"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn update_params(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateParams>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateParamsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Msg/UpdateParams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("fairyring.keyshare.Msg", "UpdateParams"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn register_validator(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRegisterValidator>,
        ) -> std::result::Result<
            tonic::Response<super::MsgRegisterValidatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Msg/RegisterValidator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("fairyring.keyshare.Msg", "RegisterValidator"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn de_register_validator(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeRegisterValidator>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDeRegisterValidatorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Msg/DeRegisterValidator",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("fairyring.keyshare.Msg", "DeRegisterValidator"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_keyshare(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSendKeyshare>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSendKeyshareResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Msg/SendKeyshare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("fairyring.keyshare.Msg", "SendKeyshare"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_latest_pubkey(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateLatestPubkey>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateLatestPubkeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Msg/CreateLatestPubkey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("fairyring.keyshare.Msg", "CreateLatestPubkey"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn override_latest_pubkey(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgOverrideLatestPubkey>,
        ) -> std::result::Result<
            tonic::Response<super::MsgOverrideLatestPubkeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Msg/OverrideLatestPubkey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("fairyring.keyshare.Msg", "OverrideLatestPubkey"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_authorized_address(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateAuthorizedAddress>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateAuthorizedAddressResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Msg/CreateAuthorizedAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("fairyring.keyshare.Msg", "CreateAuthorizedAddress"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_authorized_address(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateAuthorizedAddress>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateAuthorizedAddressResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Msg/UpdateAuthorizedAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("fairyring.keyshare.Msg", "UpdateAuthorizedAddress"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_authorized_address(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteAuthorizedAddress>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDeleteAuthorizedAddressResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Msg/DeleteAuthorizedAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("fairyring.keyshare.Msg", "DeleteAuthorizedAddress"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn submit_general_keyshare(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSubmitGeneralKeyshare>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSubmitGeneralKeyshareResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Msg/SubmitGeneralKeyshare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("fairyring.keyshare.Msg", "SubmitGeneralKeyshare"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn submit_encrypted_keyshare(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSubmitEncryptedKeyshare>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSubmitEncryptedKeyshareResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/fairyring.keyshare.Msg/SubmitEncryptedKeyshare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("fairyring.keyshare.Msg", "SubmitEncryptedKeyshare"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
