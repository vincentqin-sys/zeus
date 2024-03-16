#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarginRatioInfo {
    /// 股票
    #[prost(message, required, tag = "1")]
    pub security: super::qot_common::Security,
    /// 是否允许融资
    #[prost(bool, optional, tag = "2")]
    pub is_long_permit: ::core::option::Option<bool>,
    /// 是否允许融券
    #[prost(bool, optional, tag = "3")]
    pub is_short_permit: ::core::option::Option<bool>,
    /// 卖空池剩余量
    #[prost(double, optional, tag = "4")]
    pub short_pool_remain: ::core::option::Option<f64>,
    /// 融券参考利率
    #[prost(double, optional, tag = "5")]
    pub short_fee_rate: ::core::option::Option<f64>,
    /// 融资预警比率
    #[prost(double, optional, tag = "6")]
    pub alert_long_ratio: ::core::option::Option<f64>,
    /// 融券预警比率
    #[prost(double, optional, tag = "7")]
    pub alert_short_ratio: ::core::option::Option<f64>,
    /// 融资初始保证金率
    #[prost(double, optional, tag = "8")]
    pub im_long_ratio: ::core::option::Option<f64>,
    /// 融券初始保证金率
    #[prost(double, optional, tag = "9")]
    pub im_short_ratio: ::core::option::Option<f64>,
    /// 融资 margin call 保证金率
    #[prost(double, optional, tag = "10")]
    pub mcm_long_ratio: ::core::option::Option<f64>,
    /// 融券 margin call 保证金率
    #[prost(double, optional, tag = "11")]
    pub mcm_short_ratio: ::core::option::Option<f64>,
    /// 融资维持保证金率
    #[prost(double, optional, tag = "12")]
    pub mm_long_ratio: ::core::option::Option<f64>,
    /// 融券维持保证金率
    #[prost(double, optional, tag = "13")]
    pub mm_short_ratio: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    /// 交易公共参数头
    #[prost(message, required, tag = "1")]
    pub header: super::trd_common::TrdHeader,
    /// 股票
    #[prost(message, repeated, tag = "2")]
    pub security_list: ::prost::alloc::vec::Vec<super::qot_common::Security>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    /// 交易公共参数头
    #[prost(message, required, tag = "1")]
    pub header: super::trd_common::TrdHeader,
    /// 账户资金
    #[prost(message, repeated, tag = "2")]
    pub margin_ratio_info_list: ::prost::alloc::vec::Vec<MarginRatioInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(message, required, tag = "1")]
    pub c2s: C2s,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    /// 以下3个字段每条协议都有，注释说明在InitConnect.proto中
    #[prost(int32, required, tag = "1", default = "-400")]
    pub ret_type: i32,
    #[prost(string, optional, tag = "2")]
    pub ret_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "3")]
    pub err_code: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub s2c: ::core::option::Option<S2c>,
}
