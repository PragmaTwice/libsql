#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogOffset {
    #[prost(uint64, tag = "1")]
    pub next_offset: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(uint64, optional, tag = "1")]
    pub handshake_version: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloResponse {
    /// / Uuid of the current generation
    #[prost(string, tag = "1")]
    pub generation_id: ::prost::alloc::string::String,
    /// / First frame_no in the current generation
    #[prost(uint64, tag = "2")]
    pub generation_start_index: u64,
    /// / id of the replicated log
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    /// / string-encoded Uuid v4 token for the current session, changes on each restart, and must be passed in subsequent requests header.string
    /// / If the header session token fails to match the current session token, a NO_HELLO error is returned
    #[prost(bytes = "bytes", tag = "4")]
    pub session_token: ::prost::bytes::Bytes,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Frame {
    #[prost(bytes = "bytes", tag = "1")]
    pub data: ::prost::bytes::Bytes,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Frames {
    #[prost(message, repeated, tag = "1")]
    pub frames: ::prost::alloc::vec::Vec<Frame>,
}
/// Generated client implementations.
pub mod replication_log_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ReplicationLogClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReplicationLogClient<tonic::transport::Channel> {
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
    impl<T> ReplicationLogClient<T>
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
        ) -> ReplicationLogClient<InterceptedService<T, F>>
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
            ReplicationLogClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn hello(
            &mut self,
            request: impl tonic::IntoRequest<super::HelloRequest>,
        ) -> std::result::Result<tonic::Response<super::HelloResponse>, tonic::Status> {
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
                "/wal_log.ReplicationLog/Hello",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("wal_log.ReplicationLog", "Hello"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn log_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::LogOffset>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::Frame>>,
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
                "/wal_log.ReplicationLog/LogEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("wal_log.ReplicationLog", "LogEntries"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn batch_log_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::LogOffset>,
        ) -> std::result::Result<tonic::Response<super::Frames>, tonic::Status> {
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
                "/wal_log.ReplicationLog/BatchLogEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("wal_log.ReplicationLog", "BatchLogEntries"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::LogOffset>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::Frame>>,
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
                "/wal_log.ReplicationLog/Snapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("wal_log.ReplicationLog", "Snapshot"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod replication_log_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ReplicationLogServer.
    #[async_trait]
    pub trait ReplicationLog: Send + Sync + 'static {
        async fn hello(
            &self,
            request: tonic::Request<super::HelloRequest>,
        ) -> std::result::Result<tonic::Response<super::HelloResponse>, tonic::Status>;
        /// Server streaming response type for the LogEntries method.
        type LogEntriesStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::Frame, tonic::Status>,
            >
            + Send
            + 'static;
        async fn log_entries(
            &self,
            request: tonic::Request<super::LogOffset>,
        ) -> std::result::Result<tonic::Response<Self::LogEntriesStream>, tonic::Status>;
        async fn batch_log_entries(
            &self,
            request: tonic::Request<super::LogOffset>,
        ) -> std::result::Result<tonic::Response<super::Frames>, tonic::Status>;
        /// Server streaming response type for the Snapshot method.
        type SnapshotStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::Frame, tonic::Status>,
            >
            + Send
            + 'static;
        async fn snapshot(
            &self,
            request: tonic::Request<super::LogOffset>,
        ) -> std::result::Result<tonic::Response<Self::SnapshotStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ReplicationLogServer<T: ReplicationLog> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ReplicationLog> ReplicationLogServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ReplicationLogServer<T>
    where
        T: ReplicationLog,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/wal_log.ReplicationLog/Hello" => {
                    #[allow(non_camel_case_types)]
                    struct HelloSvc<T: ReplicationLog>(pub Arc<T>);
                    impl<
                        T: ReplicationLog,
                    > tonic::server::UnaryService<super::HelloRequest> for HelloSvc<T> {
                        type Response = super::HelloResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HelloRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReplicationLog>::hello(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HelloSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/wal_log.ReplicationLog/LogEntries" => {
                    #[allow(non_camel_case_types)]
                    struct LogEntriesSvc<T: ReplicationLog>(pub Arc<T>);
                    impl<
                        T: ReplicationLog,
                    > tonic::server::ServerStreamingService<super::LogOffset>
                    for LogEntriesSvc<T> {
                        type Response = super::Frame;
                        type ResponseStream = T::LogEntriesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogOffset>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReplicationLog>::log_entries(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LogEntriesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/wal_log.ReplicationLog/BatchLogEntries" => {
                    #[allow(non_camel_case_types)]
                    struct BatchLogEntriesSvc<T: ReplicationLog>(pub Arc<T>);
                    impl<T: ReplicationLog> tonic::server::UnaryService<super::LogOffset>
                    for BatchLogEntriesSvc<T> {
                        type Response = super::Frames;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogOffset>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReplicationLog>::batch_log_entries(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BatchLogEntriesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/wal_log.ReplicationLog/Snapshot" => {
                    #[allow(non_camel_case_types)]
                    struct SnapshotSvc<T: ReplicationLog>(pub Arc<T>);
                    impl<
                        T: ReplicationLog,
                    > tonic::server::ServerStreamingService<super::LogOffset>
                    for SnapshotSvc<T> {
                        type Response = super::Frame;
                        type ResponseStream = T::SnapshotStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogOffset>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ReplicationLog>::snapshot(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SnapshotSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ReplicationLog> Clone for ReplicationLogServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: ReplicationLog> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ReplicationLog> tonic::server::NamedService for ReplicationLogServer<T> {
        const NAME: &'static str = "wal_log.ReplicationLog";
    }
}
