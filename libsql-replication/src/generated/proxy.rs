#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Queries {
    #[prost(message, repeated, tag = "1")]
    pub queries: ::prost::alloc::vec::Vec<Query>,
    /// Uuid
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Query {
    #[prost(string, tag = "1")]
    pub stmt: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub skip_rows: bool,
    #[prost(oneof = "query::Params", tags = "2, 3")]
    pub params: ::core::option::Option<query::Params>,
}
/// Nested message and enum types in `Query`.
pub mod query {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Params {
        #[prost(message, tag = "2")]
        Positional(super::Positional),
        #[prost(message, tag = "3")]
        Named(super::Named),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Positional {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Named {
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResult {
    #[prost(oneof = "query_result::RowResult", tags = "1, 2")]
    pub row_result: ::core::option::Option<query_result::RowResult>,
}
/// Nested message and enum types in `QueryResult`.
pub mod query_result {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RowResult {
        #[prost(message, tag = "1")]
        Error(super::Error),
        #[prost(message, tag = "2")]
        Row(super::ResultRows),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(enumeration = "error::ErrorCode", tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub extended_code: i32,
}
/// Nested message and enum types in `Error`.
pub mod error {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ErrorCode {
        SqlError = 0,
        TxBusy = 1,
        TxTimeout = 2,
        Internal = 3,
    }
    impl ErrorCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ErrorCode::SqlError => "SQL_ERROR",
                ErrorCode::TxBusy => "TX_BUSY",
                ErrorCode::TxTimeout => "TX_TIMEOUT",
                ErrorCode::Internal => "INTERNAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SQL_ERROR" => Some(Self::SqlError),
                "TX_BUSY" => Some(Self::TxBusy),
                "TX_TIMEOUT" => Some(Self::TxTimeout),
                "INTERNAL" => Some(Self::Internal),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultRows {
    #[prost(message, repeated, tag = "1")]
    pub column_descriptions: ::prost::alloc::vec::Vec<Column>,
    #[prost(message, repeated, tag = "2")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
    #[prost(uint64, tag = "3")]
    pub affected_row_count: u64,
    #[prost(int64, optional, tag = "4")]
    pub last_insert_rowid: ::core::option::Option<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeRequest {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub stmt: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeResult {
    #[prost(oneof = "describe_result::DescribeResult", tags = "1, 2")]
    pub describe_result: ::core::option::Option<describe_result::DescribeResult>,
}
/// Nested message and enum types in `DescribeResult`.
pub mod describe_result {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DescribeResult {
        #[prost(message, tag = "1")]
        Error(super::Error),
        #[prost(message, tag = "2")]
        Description(super::Description),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Description {
    #[prost(message, repeated, tag = "1")]
    pub column_descriptions: ::prost::alloc::vec::Vec<Column>,
    #[prost(string, repeated, tag = "2")]
    pub param_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag = "3")]
    pub param_count: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// / bincode encoded Value
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Column {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub decltype: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectMessage {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ack {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteResults {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<QueryResult>,
    /// / State after executing the queries
    #[prost(enumeration = "State", tag = "2")]
    pub state: i32,
    /// / Primary frame_no after executing the request.
    #[prost(uint64, optional, tag = "3")]
    pub current_frame_no: ::core::option::Option<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Program {
    #[prost(message, repeated, tag = "1")]
    pub steps: ::prost::alloc::vec::Vec<Step>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Step {
    #[prost(message, optional, tag = "1")]
    pub cond: ::core::option::Option<Cond>,
    #[prost(message, optional, tag = "2")]
    pub query: ::core::option::Option<Query>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cond {
    #[prost(oneof = "cond::Cond", tags = "1, 2, 3, 4, 5, 6")]
    pub cond: ::core::option::Option<cond::Cond>,
}
/// Nested message and enum types in `Cond`.
pub mod cond {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Cond {
        #[prost(message, tag = "1")]
        Ok(super::OkCond),
        #[prost(message, tag = "2")]
        Err(super::ErrCond),
        #[prost(message, tag = "3")]
        Not(::prost::alloc::boxed::Box<super::NotCond>),
        #[prost(message, tag = "4")]
        And(super::AndCond),
        #[prost(message, tag = "5")]
        Or(super::OrCond),
        #[prost(message, tag = "6")]
        IsAutocommit(super::IsAutocommitCond),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OkCond {
    #[prost(int64, tag = "1")]
    pub step: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrCond {
    #[prost(int64, tag = "1")]
    pub step: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotCond {
    #[prost(message, optional, boxed, tag = "1")]
    pub cond: ::core::option::Option<::prost::alloc::boxed::Box<Cond>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndCond {
    #[prost(message, repeated, tag = "1")]
    pub conds: ::prost::alloc::vec::Vec<Cond>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrCond {
    #[prost(message, repeated, tag = "1")]
    pub conds: ::prost::alloc::vec::Vec<Cond>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsAutocommitCond {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgramReq {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pgm: ::core::option::Option<Program>,
}
/// / Streaming exec request
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecReq {
    /// / id of the request. The response will contain this id.
    #[prost(uint32, tag = "1")]
    pub request_id: u32,
    #[prost(oneof = "exec_req::Request", tags = "2, 3")]
    pub request: ::core::option::Option<exec_req::Request>,
}
/// Nested message and enum types in `ExecReq`.
pub mod exec_req {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "2")]
        Execute(super::StreamProgramReq),
        #[prost(message, tag = "3")]
        Describe(super::StreamDescribeReq),
    }
}
/// / Describe request for the streaming protocol
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamProgramReq {
    #[prost(message, optional, tag = "1")]
    pub pgm: ::core::option::Option<Program>,
}
/// / descibre request for the streaming protocol
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamDescribeReq {
    #[prost(string, tag = "1")]
    pub stmt: ::prost::alloc::string::String,
}
/// / Request response types
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Init {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginStep {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishStep {
    #[prost(uint64, tag = "1")]
    pub affected_row_count: u64,
    #[prost(int64, optional, tag = "2")]
    pub last_insert_rowid: ::core::option::Option<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepError {
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<Error>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColsDescription {
    #[prost(message, repeated, tag = "1")]
    pub columns: ::prost::alloc::vec::Vec<Column>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowValue {
    #[prost(oneof = "row_value::Value", tags = "1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<row_value::Value>,
}
/// Nested message and enum types in `RowValue`.
pub mod row_value {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "1")]
        Text(::prost::alloc::string::String),
        #[prost(int64, tag = "2")]
        Integer(i64),
        #[prost(double, tag = "3")]
        Real(f64),
        #[prost(bytes, tag = "4")]
        Blob(::prost::alloc::vec::Vec<u8>),
        /// null if present
        #[prost(bool, tag = "5")]
        Null(bool),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginRows {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginRow {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRowValue {
    #[prost(message, optional, tag = "1")]
    pub val: ::core::option::Option<RowValue>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishRow {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishRows {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Finish {
    #[prost(uint64, optional, tag = "1")]
    pub last_frame_no: ::core::option::Option<u64>,
    #[prost(enumeration = "State", tag = "2")]
    pub state: i32,
}
/// / Stream execx dexcribe response messages
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeParam {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeCol {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub decltype: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeResp {
    #[prost(message, repeated, tag = "1")]
    pub params: ::prost::alloc::vec::Vec<DescribeParam>,
    #[prost(message, repeated, tag = "2")]
    pub cols: ::prost::alloc::vec::Vec<DescribeCol>,
    #[prost(bool, tag = "3")]
    pub is_explain: bool,
    #[prost(bool, tag = "4")]
    pub is_readonly: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RespStep {
    #[prost(oneof = "resp_step::Step", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub step: ::core::option::Option<resp_step::Step>,
}
/// Nested message and enum types in `RespStep`.
pub mod resp_step {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Step {
        #[prost(message, tag = "1")]
        Init(super::Init),
        #[prost(message, tag = "2")]
        BeginStep(super::BeginStep),
        #[prost(message, tag = "3")]
        FinishStep(super::FinishStep),
        #[prost(message, tag = "4")]
        StepError(super::StepError),
        #[prost(message, tag = "5")]
        ColsDescription(super::ColsDescription),
        #[prost(message, tag = "6")]
        BeginRows(super::BeginRows),
        #[prost(message, tag = "7")]
        BeginRow(super::BeginRow),
        #[prost(message, tag = "8")]
        AddRowValue(super::AddRowValue),
        #[prost(message, tag = "9")]
        FinishRow(super::FinishRow),
        #[prost(message, tag = "10")]
        FinishRows(super::FinishRows),
        #[prost(message, tag = "11")]
        Finish(super::Finish),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgramResp {
    #[prost(message, repeated, tag = "1")]
    pub steps: ::prost::alloc::vec::Vec<RespStep>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecResp {
    #[prost(uint32, tag = "1")]
    pub request_id: u32,
    #[prost(oneof = "exec_resp::Response", tags = "2, 3, 4")]
    pub response: ::core::option::Option<exec_resp::Response>,
}
/// Nested message and enum types in `ExecResp`.
pub mod exec_resp {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "2")]
        ProgramResp(super::ProgramResp),
        #[prost(message, tag = "3")]
        DescribeResp(super::DescribeResp),
        #[prost(message, tag = "4")]
        Error(super::Error),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    Init = 0,
    Invalid = 1,
    Txn = 2,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::Init => "INIT",
            State::Invalid => "INVALID",
            State::Txn => "TXN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INIT" => Some(Self::Init),
            "INVALID" => Some(Self::Invalid),
            "TXN" => Some(Self::Txn),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod proxy_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ProxyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProxyClient<tonic::transport::Channel> {
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
    impl<T> ProxyClient<T>
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
        ) -> ProxyClient<InterceptedService<T, F>>
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
            ProxyClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn stream_exec(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ExecReq>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ExecResp>>,
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
            let path = http::uri::PathAndQuery::from_static("/proxy.Proxy/StreamExec");
            let mut req = request.into_streaming_request();
            req.extensions_mut().insert(GrpcMethod::new("proxy.Proxy", "StreamExec"));
            self.inner.streaming(req, path, codec).await
        }
        /// Deprecated:
        pub async fn execute(
            &mut self,
            request: impl tonic::IntoRequest<super::ProgramReq>,
        ) -> std::result::Result<tonic::Response<super::ExecuteResults>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/proxy.Proxy/Execute");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("proxy.Proxy", "Execute"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn describe(
            &mut self,
            request: impl tonic::IntoRequest<super::DescribeRequest>,
        ) -> std::result::Result<tonic::Response<super::DescribeResult>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/proxy.Proxy/Describe");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("proxy.Proxy", "Describe"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn disconnect(
            &mut self,
            request: impl tonic::IntoRequest<super::DisconnectMessage>,
        ) -> std::result::Result<tonic::Response<super::Ack>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/proxy.Proxy/Disconnect");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("proxy.Proxy", "Disconnect"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod proxy_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ProxyServer.
    #[async_trait]
    pub trait Proxy: Send + Sync + 'static {
        /// Server streaming response type for the StreamExec method.
        type StreamExecStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::ExecResp, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_exec(
            &self,
            request: tonic::Request<tonic::Streaming<super::ExecReq>>,
        ) -> std::result::Result<tonic::Response<Self::StreamExecStream>, tonic::Status>;
        /// Deprecated:
        async fn execute(
            &self,
            request: tonic::Request<super::ProgramReq>,
        ) -> std::result::Result<tonic::Response<super::ExecuteResults>, tonic::Status>;
        async fn describe(
            &self,
            request: tonic::Request<super::DescribeRequest>,
        ) -> std::result::Result<tonic::Response<super::DescribeResult>, tonic::Status>;
        async fn disconnect(
            &self,
            request: tonic::Request<super::DisconnectMessage>,
        ) -> std::result::Result<tonic::Response<super::Ack>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ProxyServer<T: Proxy> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Proxy> ProxyServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ProxyServer<T>
    where
        T: Proxy,
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
                "/proxy.Proxy/StreamExec" => {
                    #[allow(non_camel_case_types)]
                    struct StreamExecSvc<T: Proxy>(pub Arc<T>);
                    impl<T: Proxy> tonic::server::StreamingService<super::ExecReq>
                    for StreamExecSvc<T> {
                        type Response = super::ExecResp;
                        type ResponseStream = T::StreamExecStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::ExecReq>>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Proxy>::stream_exec(&inner, request).await
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
                        let method = StreamExecSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proxy.Proxy/Execute" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteSvc<T: Proxy>(pub Arc<T>);
                    impl<T: Proxy> tonic::server::UnaryService<super::ProgramReq>
                    for ExecuteSvc<T> {
                        type Response = super::ExecuteResults;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProgramReq>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Proxy>::execute(&inner, request).await
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
                        let method = ExecuteSvc(inner);
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
                "/proxy.Proxy/Describe" => {
                    #[allow(non_camel_case_types)]
                    struct DescribeSvc<T: Proxy>(pub Arc<T>);
                    impl<T: Proxy> tonic::server::UnaryService<super::DescribeRequest>
                    for DescribeSvc<T> {
                        type Response = super::DescribeResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DescribeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Proxy>::describe(&inner, request).await
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
                        let method = DescribeSvc(inner);
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
                "/proxy.Proxy/Disconnect" => {
                    #[allow(non_camel_case_types)]
                    struct DisconnectSvc<T: Proxy>(pub Arc<T>);
                    impl<T: Proxy> tonic::server::UnaryService<super::DisconnectMessage>
                    for DisconnectSvc<T> {
                        type Response = super::Ack;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DisconnectMessage>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Proxy>::disconnect(&inner, request).await
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
                        let method = DisconnectSvc(inner);
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
    impl<T: Proxy> Clone for ProxyServer<T> {
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
    impl<T: Proxy> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Proxy> tonic::server::NamedService for ProxyServer<T> {
        const NAME: &'static str = "proxy.Proxy";
    }
}
