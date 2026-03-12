// @generated
/// Generated client implementations.
pub mod action_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ActionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ActionServiceClient<tonic::transport::Channel> {
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
    impl<T> ActionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> ActionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ActionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_target(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTargetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTargetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/CreateTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.action.v2.ActionService", "CreateTarget"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_target(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTargetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateTargetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/UpdateTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.action.v2.ActionService", "UpdateTarget"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_target(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTargetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteTargetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/DeleteTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.action.v2.ActionService", "DeleteTarget"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_target(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTargetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTargetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/GetTarget",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("zitadel.action.v2.ActionService", "GetTarget"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTargetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTargetsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/ListTargets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.action.v2.ActionService", "ListTargets"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::AddPublicKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddPublicKeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/AddPublicKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.action.v2.ActionService", "AddPublicKey"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn activate_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivatePublicKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ActivatePublicKeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/ActivatePublicKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.action.v2.ActionService",
                        "ActivatePublicKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deactivate_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivatePublicKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivatePublicKeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/DeactivatePublicKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.action.v2.ActionService",
                        "DeactivatePublicKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::RemovePublicKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemovePublicKeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/RemovePublicKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.action.v2.ActionService", "RemovePublicKey"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_public_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPublicKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPublicKeysResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/ListPublicKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.action.v2.ActionService", "ListPublicKeys"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::SetExecutionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetExecutionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/SetExecution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.action.v2.ActionService", "SetExecution"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_executions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExecutionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExecutionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/ListExecutions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("zitadel.action.v2.ActionService", "ListExecutions"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_execution_functions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExecutionFunctionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExecutionFunctionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/ListExecutionFunctions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.action.v2.ActionService",
                        "ListExecutionFunctions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_execution_methods(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExecutionMethodsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExecutionMethodsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/ListExecutionMethods",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.action.v2.ActionService",
                        "ListExecutionMethods",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_execution_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExecutionServicesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExecutionServicesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/zitadel.action.v2.ActionService/ListExecutionServices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.action.v2.ActionService",
                        "ListExecutionServices",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
