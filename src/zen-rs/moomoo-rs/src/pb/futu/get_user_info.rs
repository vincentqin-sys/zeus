#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct C2s {
    /// UserInfoField集合，不设置默认返回全部信息
    #[prost(int32, optional, tag = "2")]
    pub flag: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S2c {
    /// 用户昵称
    #[prost(string, optional, tag = "1")]
    pub nick_name: ::core::option::Option<::prost::alloc::string::String>,
    /// 用户头像url
    #[prost(string, optional, tag = "2")]
    pub avatar_url: ::core::option::Option<::prost::alloc::string::String>,
    /// api用户等级描述, 已在2.10版本之后废弃
    #[prost(string, optional, tag = "3")]
    pub api_level: ::core::option::Option<::prost::alloc::string::String>,
    /// 港股行情权限, QotRight
    #[prost(int32, optional, tag = "4")]
    pub hk_qot_right: ::core::option::Option<i32>,
    /// 美股行情权限, QotRight
    #[prost(int32, optional, tag = "5")]
    pub us_qot_right: ::core::option::Option<i32>,
    /// A股行情权限, QotRight // 废弃，使用shQotRight和szQotRight
    #[prost(int32, optional, tag = "6")]
    pub cn_qot_right: ::core::option::Option<i32>,
    /// 已开户用户需要同意免责声明，未开户或已同意的用户返回false
    #[prost(bool, optional, tag = "7")]
    pub is_need_agree_disclaimer: ::core::option::Option<bool>,
    /// 用户牛牛号
    #[prost(int64, optional, tag = "8")]
    pub user_id: ::core::option::Option<i64>,
    /// 升级类型，UpdateType
    #[prost(int32, optional, tag = "9")]
    pub update_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "10")]
    pub web_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "18")]
    pub web_jump_url_head: ::core::option::Option<::prost::alloc::string::String>,
    /// 港股期权行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag = "11")]
    pub hk_option_qot_right: ::core::option::Option<i32>,
    /// 是否有美股期权行情权限
    #[prost(bool, optional, tag = "12")]
    pub has_us_option_qot_right: ::core::option::Option<bool>,
    /// 港股期货行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag = "13")]
    pub hk_future_qot_right: ::core::option::Option<i32>,
    /// 订阅额度
    #[prost(int32, optional, tag = "14")]
    pub sub_quota: ::core::option::Option<i32>,
    /// 历史K线额度
    #[prost(int32, optional, tag = "15")]
    pub history_kl_quota: ::core::option::Option<i32>,
    /// 美股期货行情权限, Qot_Common.QotRight(已废弃)
    #[prost(int32, optional, tag = "16")]
    pub us_future_qot_right: ::core::option::Option<i32>,
    /// 美股期货行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag = "17")]
    pub us_option_qot_right: ::core::option::Option<i32>,
    /// 用户注册归属地, Common.UserAttribution
    #[prost(int32, optional, tag = "19")]
    pub user_attribution: ::core::option::Option<i32>,
    /// 升级提示
    #[prost(string, optional, tag = "20")]
    pub update_whats_new: ::core::option::Option<::prost::alloc::string::String>,
    /// 美股指数行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag = "21")]
    pub us_index_qot_right: ::core::option::Option<i32>,
    /// 美股OTC市场行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag = "22")]
    pub us_otc_qot_right: ::core::option::Option<i32>,
    /// 美股CME期货行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag = "23")]
    pub us_cme_future_qot_right: ::core::option::Option<i32>,
    /// 美股CBOT期货行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag = "24")]
    pub us_cbot_future_qot_right: ::core::option::Option<i32>,
    /// 美股NYMEX期货行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag = "25")]
    pub us_nymex_future_qot_right: ::core::option::Option<i32>,
    /// 美股COMEX期货行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag = "26")]
    pub us_comex_future_qot_right: ::core::option::Option<i32>,
    /// 美股CBOE期货行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag = "27")]
    pub us_cboe_future_qot_right: ::core::option::Option<i32>,
    /// 新加坡市场期货行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag = "28")]
    pub sg_future_qot_right: ::core::option::Option<i32>,
    /// 日本市场期货行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag = "29")]
    pub jp_future_qot_right: ::core::option::Option<i32>,
    /// true:NN false:MM
    #[prost(bool, optional, tag = "30")]
    pub is_app_nn_or_mm: ::core::option::Option<bool>,
    /// 上海市场行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag = "31")]
    pub sh_qot_right: ::core::option::Option<i32>,
    /// 深圳市场行情权限, Qot_Common.QotRight
    #[prost(int32, optional, tag = "32")]
    pub sz_qot_right: ::core::option::Option<i32>,
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
    /// 返回结果，参见Common.RetType的枚举定义
    #[prost(int32, required, tag = "1", default = "-400")]
    pub ret_type: i32,
    /// 返回结果描述
    #[prost(string, optional, tag = "2")]
    pub ret_msg: ::core::option::Option<::prost::alloc::string::String>,
    /// 错误码，客户端一般通过retType和retMsg来判断结果和详情，errCode只做日志记录，仅在个别协议失败时对账用
    #[prost(int32, optional, tag = "3")]
    pub err_code: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub s2c: ::core::option::Option<S2c>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpdateType {
    /// 无需升级
    None = 0,
    /// 建议升级
    Advice = 1,
    /// 强制升级
    Force = 2,
}
impl UpdateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UpdateType::None => "UpdateType_None",
            UpdateType::Advice => "UpdateType_Advice",
            UpdateType::Force => "UpdateType_Force",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UpdateType_None" => Some(Self::None),
            "UpdateType_Advice" => Some(Self::Advice),
            "UpdateType_Force" => Some(Self::Force),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserInfoField {
    /// 昵称，用户头像，牛牛号
    Basic = 1,
    /// API权限信息
    Api = 2,
    /// 市场的行情权限
    QotRight = 4,
    /// 免责
    Disclaimer = 8,
    /// 升级类型
    Update = 16,
    WebKey = 2048,
}
impl UserInfoField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserInfoField::Basic => "UserInfoField_Basic",
            UserInfoField::Api => "UserInfoField_API",
            UserInfoField::QotRight => "UserInfoField_QotRight",
            UserInfoField::Disclaimer => "UserInfoField_Disclaimer",
            UserInfoField::Update => "UserInfoField_Update",
            UserInfoField::WebKey => "UserInfoField_WebKey",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UserInfoField_Basic" => Some(Self::Basic),
            "UserInfoField_API" => Some(Self::Api),
            "UserInfoField_QotRight" => Some(Self::QotRight),
            "UserInfoField_Disclaimer" => Some(Self::Disclaimer),
            "UserInfoField_Update" => Some(Self::Update),
            "UserInfoField_WebKey" => Some(Self::WebKey),
            _ => None,
        }
    }
}
