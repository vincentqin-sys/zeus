#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    /// 股票基本行情
    #[prost(message, repeated, tag = "1")]
    pub basic_qot_list: ::prost::alloc::vec::Vec<super::qot_common::BasicQot>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    /// RetType,返回结果
    #[prost(int32, required, tag = "1", default = "-400")]
    pub ret_type: i32,
    #[prost(string, optional, tag = "2")]
    pub ret_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub err_code: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub s2c: ::core::option::Option<S2c>,
}
