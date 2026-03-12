// @generated
/// Generated client implementations.
pub mod application_service_client {
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
    pub struct ApplicationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ApplicationServiceClient<tonic::transport::Channel> {
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
    impl<T> ApplicationServiceClient<T>
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
        ) -> ApplicationServiceClient<InterceptedService<T, F>>
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
            ApplicationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_application(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateApplicationResponse>,
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
                "/zitadel.application.v2.ApplicationService/CreateApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.application.v2.ApplicationService",
                        "CreateApplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_application(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateApplicationResponse>,
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
                "/zitadel.application.v2.ApplicationService/UpdateApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.application.v2.ApplicationService",
                        "UpdateApplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_application(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetApplicationResponse>,
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
                "/zitadel.application.v2.ApplicationService/GetApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.application.v2.ApplicationService",
                        "GetApplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_application(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteApplicationResponse>,
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
                "/zitadel.application.v2.ApplicationService/DeleteApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.application.v2.ApplicationService",
                        "DeleteApplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn deactivate_application(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateApplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateApplicationResponse>,
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
                "/zitadel.application.v2.ApplicationService/DeactivateApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.application.v2.ApplicationService",
                        "DeactivateApplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reactivate_application(
            &mut self,
            request: impl tonic::IntoRequest<super::ReactivateApplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReactivateApplicationResponse>,
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
                "/zitadel.application.v2.ApplicationService/ReactivateApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.application.v2.ApplicationService",
                        "ReactivateApplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn generate_client_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateClientSecretRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateClientSecretResponse>,
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
                "/zitadel.application.v2.ApplicationService/GenerateClientSecret",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.application.v2.ApplicationService",
                        "GenerateClientSecret",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_applications(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApplicationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListApplicationsResponse>,
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
                "/zitadel.application.v2.ApplicationService/ListApplications",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.application.v2.ApplicationService",
                        "ListApplications",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_application_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApplicationKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateApplicationKeyResponse>,
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
                "/zitadel.application.v2.ApplicationService/CreateApplicationKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.application.v2.ApplicationService",
                        "CreateApplicationKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_application_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApplicationKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteApplicationKeyResponse>,
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
                "/zitadel.application.v2.ApplicationService/DeleteApplicationKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.application.v2.ApplicationService",
                        "DeleteApplicationKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_application_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApplicationKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetApplicationKeyResponse>,
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
                "/zitadel.application.v2.ApplicationService/GetApplicationKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.application.v2.ApplicationService",
                        "GetApplicationKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_application_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApplicationKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListApplicationKeysResponse>,
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
                "/zitadel.application.v2.ApplicationService/ListApplicationKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "zitadel.application.v2.ApplicationService",
                        "ListApplicationKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
