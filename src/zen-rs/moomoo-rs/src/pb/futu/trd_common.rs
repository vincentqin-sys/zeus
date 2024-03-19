/// 账户现金信息，目前仅用于期货账户
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccCashInfo {
    /// 货币类型，取值参考 Currency
    #[prost(int32, optional, tag = "1")]
    pub currency: ::core::option::Option<i32>,
    /// 现金结余
    #[prost(double, optional, tag = "2")]
    pub cash: ::core::option::Option<f64>,
    /// 现金可提金额
    #[prost(double, optional, tag = "3")]
    pub available_balance: ::core::option::Option<f64>,
}
/// 交易协议公共参数头
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrdHeader {
    /// 交易环境, 参见TrdEnv的枚举定义
    #[prost(int32, required, tag = "1")]
    pub trd_env: i32,
    /// 业务账号, 业务账号与交易环境、市场权限需要匹配，否则会返回错误
    #[prost(uint64, required, tag = "2")]
    pub acc_id: u64,
    /// 交易市场, 参见TrdMarket的枚举定义
    #[prost(int32, required, tag = "3")]
    pub trd_market: i32,
}
/// 交易业务账户结构
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrdAcc {
    /// 交易环境，参见TrdEnv的枚举定义
    #[prost(int32, required, tag = "1")]
    pub trd_env: i32,
    /// 业务账号
    #[prost(uint64, required, tag = "2")]
    pub acc_id: u64,
    /// 业务账户支持的交易市场权限，即此账户能交易那些市场, 可拥有多个交易市场权限，目前仅单个，取值参见TrdMarket的枚举定义
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub trd_market_auth_list: ::prost::alloc::vec::Vec<i32>,
    /// 账户类型，取值见TrdAccType
    #[prost(int32, optional, tag = "4")]
    pub acc_type: ::core::option::Option<i32>,
    /// 卡号
    #[prost(string, optional, tag = "5")]
    pub card_num: ::core::option::Option<::prost::alloc::string::String>,
    /// 所属券商，取值见SecurityFirm
    #[prost(int32, optional, tag = "6")]
    pub security_firm: ::core::option::Option<i32>,
    /// 模拟交易账号类型，取值见SimAccType
    #[prost(int32, optional, tag = "7")]
    pub sim_acc_type: ::core::option::Option<i32>,
    /// 所属综合账户卡号
    #[prost(string, optional, tag = "8")]
    pub uni_card_num: ::core::option::Option<::prost::alloc::string::String>,
}
/// 账户资金结构
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Funds {
    /// 最大购买力（做多），3位精度，下同。
    #[prost(double, required, tag = "1")]
    pub power: f64,
    /// 资产净值
    #[prost(double, required, tag = "2")]
    pub total_assets: f64,
    /// 现金
    #[prost(double, required, tag = "3")]
    pub cash: f64,
    /// 证券市值, 仅证券账户适用
    #[prost(double, required, tag = "4")]
    pub market_val: f64,
    /// 冻结资金
    #[prost(double, required, tag = "5")]
    pub frozen_cash: f64,
    /// 计息金额
    #[prost(double, required, tag = "6")]
    pub debt_cash: f64,
    /// 现金可提，仅证券账户适用
    #[prost(double, required, tag = "7")]
    pub avl_withdrawal_cash: f64,
    /// 币种，本结构体资金相关的货币类型，取值参见 Currency，期货适用
    #[prost(int32, optional, tag = "8")]
    pub currency: ::core::option::Option<i32>,
    /// 可用资金，期货适用
    #[prost(double, optional, tag = "9")]
    pub available_funds: ::core::option::Option<f64>,
    /// 未实现盈亏，期货适用
    #[prost(double, optional, tag = "10")]
    pub unrealized_pl: ::core::option::Option<f64>,
    /// 已实现盈亏，期货适用
    #[prost(double, optional, tag = "11")]
    pub realized_pl: ::core::option::Option<f64>,
    /// 风控状态，参见 CltRiskLevel, 期货适用
    #[prost(int32, optional, tag = "12")]
    pub risk_level: ::core::option::Option<i32>,
    /// 初始保证金
    #[prost(double, optional, tag = "13")]
    pub initial_margin: ::core::option::Option<f64>,
    /// 维持保证金
    #[prost(double, optional, tag = "14")]
    pub maintenance_margin: ::core::option::Option<f64>,
    /// 分币种的现金信息，期货适用
    #[prost(message, repeated, tag = "15")]
    pub cash_info_list: ::prost::alloc::vec::Vec<AccCashInfo>,
    /// 卖空购买力
    #[prost(double, optional, tag = "16")]
    pub max_power_short: ::core::option::Option<f64>,
    /// 现金购买力
    #[prost(double, optional, tag = "17")]
    pub net_cash_power: ::core::option::Option<f64>,
    /// 多头市值
    #[prost(double, optional, tag = "18")]
    pub long_mv: ::core::option::Option<f64>,
    /// 空头市值
    #[prost(double, optional, tag = "19")]
    pub short_mv: ::core::option::Option<f64>,
    /// 在途资产
    #[prost(double, optional, tag = "20")]
    pub pending_asset: ::core::option::Option<f64>,
    /// 融资可提，仅证券账户适用
    #[prost(double, optional, tag = "21")]
    pub max_withdrawal: ::core::option::Option<f64>,
    /// 风险状态，参见 \[CltRiskStatus\]，证券账户适用，共分 9 个等级，LEVEL1是最安全，LEVEL9是最危险
    #[prost(int32, optional, tag = "22")]
    pub risk_status: ::core::option::Option<i32>,
    /// 	Margin Call 保证金
    #[prost(double, optional, tag = "23")]
    pub margin_call_margin: ::core::option::Option<f64>,
    /// 是否PDT账户，仅富途证券（美国）账户适用
    #[prost(bool, optional, tag = "24")]
    pub is_pdt: ::core::option::Option<bool>,
    /// 剩余日内交易次数
    #[prost(string, optional, tag = "25")]
    pub pdt_seq: ::core::option::Option<::prost::alloc::string::String>,
    /// 初始日内交易购买力
    #[prost(double, optional, tag = "26")]
    pub beginning_dtbp: ::core::option::Option<f64>,
    /// 剩余日内交易购买力
    #[prost(double, optional, tag = "27")]
    pub remaining_dtbp: ::core::option::Option<f64>,
    /// 日内交易待缴金额
    #[prost(double, optional, tag = "28")]
    pub dt_call_amount: ::core::option::Option<f64>,
    /// 日内交易限制情况，取值见DTStatus
    #[prost(int32, optional, tag = "29")]
    pub dt_status: ::core::option::Option<i32>,
}
/// 账户持仓结构
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    /// 持仓ID，一条持仓的唯一标识
    #[prost(uint64, required, tag = "1")]
    pub position_id: u64,
    /// 持仓方向，参见PositionSide的枚举定义
    #[prost(int32, required, tag = "2")]
    pub position_side: i32,
    /// 代码
    #[prost(string, required, tag = "3")]
    pub code: ::prost::alloc::string::String,
    /// 名称
    #[prost(string, required, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// 持有数量，2位精度，期权单位是"张"，下同
    #[prost(double, required, tag = "5")]
    pub qty: f64,
    /// 可卖数量
    #[prost(double, required, tag = "6")]
    pub can_sell_qty: f64,
    /// 市价，3位精度，期货为2位精度
    #[prost(double, required, tag = "7")]
    pub price: f64,
    /// 成本价，无精度限制，期货为2位精度，如果没传，代表此时此值无效,
    #[prost(double, optional, tag = "8")]
    pub cost_price: ::core::option::Option<f64>,
    /// 市值，3位精度, 期货此字段值为0
    #[prost(double, required, tag = "9")]
    pub val: f64,
    /// 盈亏金额，3位精度，期货为2位精度
    #[prost(double, required, tag = "10")]
    pub pl_val: f64,
    /// 盈亏百分比(如plRatio等于8.8代表涨8.8%)，无精度限制，如果没传，代表此时此值无效
    #[prost(double, optional, tag = "11")]
    pub pl_ratio: ::core::option::Option<f64>,
    /// 证券所属市场，参见TrdSecMarket的枚举定义
    #[prost(int32, optional, tag = "12")]
    pub sec_market: ::core::option::Option<i32>,
    /// 以下是此持仓今日统计
    ///
    /// 今日盈亏金额，3位精度，下同, 期货为2位精度
    #[prost(double, optional, tag = "21")]
    pub td_pl_val: ::core::option::Option<f64>,
    /// 今日交易额，期货不适用
    #[prost(double, optional, tag = "22")]
    pub td_trd_val: ::core::option::Option<f64>,
    /// 今日买入总额，期货不适用
    #[prost(double, optional, tag = "23")]
    pub td_buy_val: ::core::option::Option<f64>,
    /// 今日买入总量，期货不适用
    #[prost(double, optional, tag = "24")]
    pub td_buy_qty: ::core::option::Option<f64>,
    /// 今日卖出总额，期货不适用
    #[prost(double, optional, tag = "25")]
    pub td_sell_val: ::core::option::Option<f64>,
    /// 今日卖出总量，期货不适用
    #[prost(double, optional, tag = "26")]
    pub td_sell_qty: ::core::option::Option<f64>,
    /// 未实现盈亏，期货适用
    #[prost(double, optional, tag = "28")]
    pub unrealized_pl: ::core::option::Option<f64>,
    /// 已实现盈亏，期货适用
    #[prost(double, optional, tag = "29")]
    pub realized_pl: ::core::option::Option<f64>,
    /// 货币类型，取值参考 Currency
    #[prost(int32, optional, tag = "30")]
    pub currency: ::core::option::Option<i32>,
    /// 交易市场, 参见TrdMarket的枚举定义
    #[prost(int32, optional, tag = "31")]
    pub trd_market: ::core::option::Option<i32>,
}
/// 订单结构
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    /// 交易方向, 参见TrdSide的枚举定义
    #[prost(int32, required, tag = "1")]
    pub trd_side: i32,
    /// 订单类型, 参见OrderType的枚举定义
    #[prost(int32, required, tag = "2")]
    pub order_type: i32,
    /// 订单状态, 参见OrderStatus的枚举定义
    #[prost(int32, required, tag = "3")]
    pub order_status: i32,
    /// 订单号
    #[prost(uint64, required, tag = "4")]
    pub order_id: u64,
    /// 扩展订单号(仅查问题时备用)
    #[prost(string, required, tag = "5")]
    pub order_id_ex: ::prost::alloc::string::String,
    /// 代码
    #[prost(string, required, tag = "6")]
    pub code: ::prost::alloc::string::String,
    /// 名称
    #[prost(string, required, tag = "7")]
    pub name: ::prost::alloc::string::String,
    /// 订单数量，2位精度，期权单位是"张"
    #[prost(double, required, tag = "8")]
    pub qty: f64,
    /// 订单价格，3位精度
    #[prost(double, optional, tag = "9")]
    pub price: ::core::option::Option<f64>,
    /// 创建时间，严格按YYYY-MM-DD HH:MM:SS或YYYY-MM-DD HH:MM:SS.MS格式传
    #[prost(string, required, tag = "10")]
    pub create_time: ::prost::alloc::string::String,
    /// 最后更新时间，严格按YYYY-MM-DD HH:MM:SS或YYYY-MM-DD HH:MM:SS.MS格式传
    #[prost(string, required, tag = "11")]
    pub update_time: ::prost::alloc::string::String,
    /// 成交数量，2位精度，期权单位是"张"
    #[prost(double, optional, tag = "12")]
    pub fill_qty: ::core::option::Option<f64>,
    /// 成交均价，无精度限制
    #[prost(double, optional, tag = "13")]
    pub fill_avg_price: ::core::option::Option<f64>,
    /// 最后的错误描述，如果有错误，会有此描述最后一次错误的原因，无错误为空
    #[prost(string, optional, tag = "14")]
    pub last_err_msg: ::core::option::Option<::prost::alloc::string::String>,
    /// 证券所属市场，参见TrdSecMarket的枚举定义
    #[prost(int32, optional, tag = "15")]
    pub sec_market: ::core::option::Option<i32>,
    /// 创建时间戳
    #[prost(double, optional, tag = "16")]
    pub create_timestamp: ::core::option::Option<f64>,
    /// 最后更新时间戳
    #[prost(double, optional, tag = "17")]
    pub update_timestamp: ::core::option::Option<f64>,
    /// 用户备注字符串，最大长度64字节
    #[prost(string, optional, tag = "18")]
    pub remark: ::core::option::Option<::prost::alloc::string::String>,
    /// 订单期限，参考 TimeInForce 类的定义
    #[prost(int32, optional, tag = "19")]
    pub time_in_force: ::core::option::Option<i32>,
    /// 是否允许美股订单盘前盘后成交
    #[prost(bool, optional, tag = "20")]
    pub fill_outside_rth: ::core::option::Option<bool>,
    /// 触发价格
    #[prost(double, optional, tag = "21")]
    pub aux_price: ::core::option::Option<f64>,
    /// 跟踪类型, 参见Trd_Common.TrailType的枚举定义
    #[prost(int32, optional, tag = "22")]
    pub trail_type: ::core::option::Option<i32>,
    /// 跟踪金额/百分比
    #[prost(double, optional, tag = "23")]
    pub trail_value: ::core::option::Option<f64>,
    /// 指定价差
    #[prost(double, optional, tag = "24")]
    pub trail_spread: ::core::option::Option<f64>,
    /// 货币类型，取值参考 Currency
    #[prost(int32, optional, tag = "25")]
    pub currency: ::core::option::Option<i32>,
    /// 交易市场, 参见TrdMarket的枚举定义
    #[prost(int32, optional, tag = "26")]
    pub trd_market: ::core::option::Option<i32>,
}
/// 成交结构
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderFill {
    /// 交易方向, 参见TrdSide的枚举定义
    #[prost(int32, required, tag = "1")]
    pub trd_side: i32,
    /// 成交号
    #[prost(uint64, required, tag = "2")]
    pub fill_id: u64,
    /// 扩展成交号(仅查问题时备用)
    #[prost(string, required, tag = "3")]
    pub fill_id_ex: ::prost::alloc::string::String,
    /// 订单号
    #[prost(uint64, optional, tag = "4")]
    pub order_id: ::core::option::Option<u64>,
    /// 扩展订单号(仅查问题时备用)
    #[prost(string, optional, tag = "5")]
    pub order_id_ex: ::core::option::Option<::prost::alloc::string::String>,
    /// 代码
    #[prost(string, required, tag = "6")]
    pub code: ::prost::alloc::string::String,
    /// 名称
    #[prost(string, required, tag = "7")]
    pub name: ::prost::alloc::string::String,
    /// 成交数量，2位精度，期权单位是"张"
    #[prost(double, required, tag = "8")]
    pub qty: f64,
    /// 成交价格，3位精度
    #[prost(double, required, tag = "9")]
    pub price: f64,
    /// 创建时间（成交时间），严格按YYYY-MM-DD HH:MM:SS或YYYY-MM-DD HH:MM:SS.MS格式传
    #[prost(string, required, tag = "10")]
    pub create_time: ::prost::alloc::string::String,
    /// 对手经纪号，港股有效
    #[prost(int32, optional, tag = "11")]
    pub counter_broker_id: ::core::option::Option<i32>,
    /// 对手经纪名称，港股有效
    #[prost(string, optional, tag = "12")]
    pub counter_broker_name: ::core::option::Option<::prost::alloc::string::String>,
    /// 证券所属市场，参见TrdSecMarket的枚举定义
    #[prost(int32, optional, tag = "13")]
    pub sec_market: ::core::option::Option<i32>,
    /// 创建时间戳
    #[prost(double, optional, tag = "14")]
    pub create_timestamp: ::core::option::Option<f64>,
    /// 最后更新时间戳
    #[prost(double, optional, tag = "15")]
    pub update_timestamp: ::core::option::Option<f64>,
    /// 成交状态, 参见OrderFillStatus的枚举定义
    #[prost(int32, optional, tag = "16")]
    pub status: ::core::option::Option<i32>,
}
/// 最大可交易数量
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaxTrdQtys {
    /// 因目前服务器实现的问题，卖空需要先卖掉持仓才能再卖空，是分开两步卖的，买回来同样是逆向两步；而看多的买是可以现金加融资一起一步买的，请注意这个差异
    ///
    /// 不使用融资，仅自己的现金最大可买整手股数，期货此字段值为0
    #[prost(double, required, tag = "1")]
    pub max_cash_buy: f64,
    /// 使用融资，自己的现金 + 融资资金总共的最大可买整手股数，期货不适用
    #[prost(double, optional, tag = "2")]
    pub max_cash_and_margin_buy: ::core::option::Option<f64>,
    /// 不使用融券(卖空)，仅自己的持仓最大可卖整手股数
    #[prost(double, required, tag = "3")]
    pub max_position_sell: f64,
    /// 使用融券(卖空)，最大可卖空整手股数，不包括多仓，期货不适用
    #[prost(double, optional, tag = "4")]
    pub max_sell_short: ::core::option::Option<f64>,
    /// 卖空后，需要买回的最大整手股数。因为卖空后，必须先买回已卖空的股数，还掉股票，才能再继续买多。期货不适用
    #[prost(double, optional, tag = "5")]
    pub max_buy_back: ::core::option::Option<f64>,
    /// 开多仓每张合约初始保证金。当前仅期货和期权适用（最低 FutuOpenD 版本要求：5.0.1310）
    #[prost(double, optional, tag = "6")]
    pub long_required_im: ::core::option::Option<f64>,
    /// 开空仓每张合约初始保证金。当前仅期货和期权适用（最低 FutuOpenD 版本要求：5.0.1310）
    #[prost(double, optional, tag = "7")]
    pub short_required_im: ::core::option::Option<f64>,
}
/// 过滤条件，条件组合是"与"不是"或"，用于获取订单、成交、持仓等时二次过滤
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrdFilterConditions {
    /// 代码过滤，只返回包含这些代码的数据，没传不过滤
    #[prost(string, repeated, tag = "1")]
    pub code_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// ID主键过滤，只返回包含这些ID的数据，没传不过滤，订单是orderID、成交是fillID、持仓是positionID
    #[prost(uint64, repeated, packed = "false", tag = "2")]
    pub id_list: ::prost::alloc::vec::Vec<u64>,
    /// 开始时间，严格按YYYY-MM-DD HH:MM:SS或YYYY-MM-DD HH:MM:SS.MS格式传，对持仓无效，拉历史数据必须填
    #[prost(string, optional, tag = "3")]
    pub begin_time: ::core::option::Option<::prost::alloc::string::String>,
    /// 结束时间，严格按YYYY-MM-DD HH:MM:SS或YYYY-MM-DD HH:MM:SS.MS格式传，对持仓无效，拉历史数据必须填
    #[prost(string, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost::alloc::string::String>,
}
/// 交易环境
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrdEnv {
    /// 仿真环境(模拟环境)
    Simulate = 0,
    /// 真实环境
    Real = 1,
}
impl TrdEnv {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrdEnv::Simulate => "TrdEnv_Simulate",
            TrdEnv::Real => "TrdEnv_Real",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TrdEnv_Simulate" => Some(Self::Simulate),
            "TrdEnv_Real" => Some(Self::Real),
            _ => None,
        }
    }
}
/// 交易品类
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrdCategory {
    /// 未知品类
    Unknown = 0,
    /// 证券
    Security = 1,
    /// 期货
    Future = 2,
}
impl TrdCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrdCategory::Unknown => "TrdCategory_Unknown",
            TrdCategory::Security => "TrdCategory_Security",
            TrdCategory::Future => "TrdCategory_Future",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TrdCategory_Unknown" => Some(Self::Unknown),
            "TrdCategory_Security" => Some(Self::Security),
            "TrdCategory_Future" => Some(Self::Future),
            _ => None,
        }
    }
}
/// 交易市场，是大的市场，不是具体品种
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrdMarket {
    /// 未知市场
    Unknown = 0,
    /// 香港市场
    Hk = 1,
    /// 美国市场
    Us = 2,
    /// 大陆市场
    Cn = 3,
    /// 香港A股通市场
    Hkcc = 4,
    /// 期货市场
    Futures = 5,
    /// 期货市场
    Sg = 6,
    /// 模拟交易期货市场
    FuturesSimulateHk = 10,
    FuturesSimulateUs = 11,
    FuturesSimulateSg = 12,
    FuturesSimulateJp = 13,
}
impl TrdMarket {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrdMarket::Unknown => "TrdMarket_Unknown",
            TrdMarket::Hk => "TrdMarket_HK",
            TrdMarket::Us => "TrdMarket_US",
            TrdMarket::Cn => "TrdMarket_CN",
            TrdMarket::Hkcc => "TrdMarket_HKCC",
            TrdMarket::Futures => "TrdMarket_Futures",
            TrdMarket::Sg => "TrdMarket_SG",
            TrdMarket::FuturesSimulateHk => "TrdMarket_Futures_Simulate_HK",
            TrdMarket::FuturesSimulateUs => "TrdMarket_Futures_Simulate_US",
            TrdMarket::FuturesSimulateSg => "TrdMarket_Futures_Simulate_SG",
            TrdMarket::FuturesSimulateJp => "TrdMarket_Futures_Simulate_JP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TrdMarket_Unknown" => Some(Self::Unknown),
            "TrdMarket_HK" => Some(Self::Hk),
            "TrdMarket_US" => Some(Self::Us),
            "TrdMarket_CN" => Some(Self::Cn),
            "TrdMarket_HKCC" => Some(Self::Hkcc),
            "TrdMarket_Futures" => Some(Self::Futures),
            "TrdMarket_SG" => Some(Self::Sg),
            "TrdMarket_Futures_Simulate_HK" => Some(Self::FuturesSimulateHk),
            "TrdMarket_Futures_Simulate_US" => Some(Self::FuturesSimulateUs),
            "TrdMarket_Futures_Simulate_SG" => Some(Self::FuturesSimulateSg),
            "TrdMarket_Futures_Simulate_JP" => Some(Self::FuturesSimulateJp),
            _ => None,
        }
    }
}
/// 可交易证券所属市场，目前主要是区分A股的沪市和深市，香港和美国暂不需要细分
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrdSecMarket {
    /// 未知市场
    Unknown = 0,
    /// 香港市场(股票、窝轮、牛熊、期权、期货等)
    Hk = 1,
    /// 美国市场(股票、期权、期货等)
    Us = 2,
    /// 沪股市场(股票)
    CnSh = 31,
    /// 深股市场(股票)
    CnSz = 32,
    /// 新加坡市场(期货)
    Sg = 41,
    /// 日本市场(期货)
    Jp = 51,
}
impl TrdSecMarket {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrdSecMarket::Unknown => "TrdSecMarket_Unknown",
            TrdSecMarket::Hk => "TrdSecMarket_HK",
            TrdSecMarket::Us => "TrdSecMarket_US",
            TrdSecMarket::CnSh => "TrdSecMarket_CN_SH",
            TrdSecMarket::CnSz => "TrdSecMarket_CN_SZ",
            TrdSecMarket::Sg => "TrdSecMarket_SG",
            TrdSecMarket::Jp => "TrdSecMarket_JP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TrdSecMarket_Unknown" => Some(Self::Unknown),
            "TrdSecMarket_HK" => Some(Self::Hk),
            "TrdSecMarket_US" => Some(Self::Us),
            "TrdSecMarket_CN_SH" => Some(Self::CnSh),
            "TrdSecMarket_CN_SZ" => Some(Self::CnSz),
            "TrdSecMarket_SG" => Some(Self::Sg),
            "TrdSecMarket_JP" => Some(Self::Jp),
            _ => None,
        }
    }
}
/// 交易方向
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrdSide {
    /// 客户端下单只传Buy或Sell即可，SellShort是美股订单时服务器返回有此方向，BuyBack目前不存在，但也不排除服务器会传
    ///
    /// 未知方向
    Unknown = 0,
    /// 买入
    Buy = 1,
    /// 卖出
    Sell = 2,
    /// 卖空
    SellShort = 3,
    /// 买回
    BuyBack = 4,
}
impl TrdSide {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrdSide::Unknown => "TrdSide_Unknown",
            TrdSide::Buy => "TrdSide_Buy",
            TrdSide::Sell => "TrdSide_Sell",
            TrdSide::SellShort => "TrdSide_SellShort",
            TrdSide::BuyBack => "TrdSide_BuyBack",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TrdSide_Unknown" => Some(Self::Unknown),
            "TrdSide_Buy" => Some(Self::Buy),
            "TrdSide_Sell" => Some(Self::Sell),
            "TrdSide_SellShort" => Some(Self::SellShort),
            "TrdSide_BuyBack" => Some(Self::BuyBack),
            _ => None,
        }
    }
}
/// 订单类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    /// 未知类型
    Unknown = 0,
    /// 普通订单(港股的增强限价单、港股期权的限价单，A股限价委托、美股的限价单，港股期货的限价单，CME期货的限价单)。目前港股期权只能指定此订单类型。
    Normal = 1,
    /// 市价订单(目前支持美股、港股正股、涡轮、牛熊、界内证)
    Market = 2,
    /// 绝对限价订单(目前仅港股)，只有价格完全匹配才成交，否则下单失败，比如你下价格为5元的买单，卖单价格必须也要是5元才能成交，低于5元也不能成交，下单失败。卖出同理
    AbsoluteLimit = 5,
    /// 竞价订单(目前仅港股)，仅港股早盘竞价和收盘竞价有效，A股的早盘竞价订单类型不变还是OrderType_Normal
    Auction = 6,
    /// 竞价限价订单(目前仅港股)，仅早盘竞价和收盘竞价有效，参与竞价，且要求满足指定价格才会成交
    AuctionLimit = 7,
    /// 特别限价订单(目前仅港股)，成交规则同增强限价订单，且部分成交后，交易所自动撤销订单
    SpecialLimit = 8,
    /// 特别限价且要求全部成交订单(目前仅港股)，要么全部成交，要么自动撤单
    SpecialLimitAll = 9,
    /// 止损市价单
    Stop = 10,
    /// 止损限价单
    StopLimit = 11,
    /// 触及市价单（止盈）
    MarketifTouched = 12,
    /// 触及限价单（止盈）
    LimitifTouched = 13,
    /// 跟踪止损市价单
    TrailingStop = 14,
    /// 跟踪止损限价单
    TrailingStopLimit = 15,
    /// TWAP 市价单
    TwapMarket = 16,
    /// TWAP 市价单
    TwapLimit = 17,
    /// TWAP 市价单
    VwapMarket = 18,
    /// TWAP 市价单
    VwapLimit = 19,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Unknown => "OrderType_Unknown",
            OrderType::Normal => "OrderType_Normal",
            OrderType::Market => "OrderType_Market",
            OrderType::AbsoluteLimit => "OrderType_AbsoluteLimit",
            OrderType::Auction => "OrderType_Auction",
            OrderType::AuctionLimit => "OrderType_AuctionLimit",
            OrderType::SpecialLimit => "OrderType_SpecialLimit",
            OrderType::SpecialLimitAll => "OrderType_SpecialLimit_All",
            OrderType::Stop => "OrderType_Stop",
            OrderType::StopLimit => "OrderType_StopLimit",
            OrderType::MarketifTouched => "OrderType_MarketifTouched",
            OrderType::LimitifTouched => "OrderType_LimitifTouched",
            OrderType::TrailingStop => "OrderType_TrailingStop",
            OrderType::TrailingStopLimit => "OrderType_TrailingStopLimit",
            OrderType::TwapMarket => "OrderType_TWAP_MARKET",
            OrderType::TwapLimit => "OrderType_TWAP_LIMIT",
            OrderType::VwapMarket => "OrderType_VWAP_MARKET",
            OrderType::VwapLimit => "OrderType_VWAP_LIMIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OrderType_Unknown" => Some(Self::Unknown),
            "OrderType_Normal" => Some(Self::Normal),
            "OrderType_Market" => Some(Self::Market),
            "OrderType_AbsoluteLimit" => Some(Self::AbsoluteLimit),
            "OrderType_Auction" => Some(Self::Auction),
            "OrderType_AuctionLimit" => Some(Self::AuctionLimit),
            "OrderType_SpecialLimit" => Some(Self::SpecialLimit),
            "OrderType_SpecialLimit_All" => Some(Self::SpecialLimitAll),
            "OrderType_Stop" => Some(Self::Stop),
            "OrderType_StopLimit" => Some(Self::StopLimit),
            "OrderType_MarketifTouched" => Some(Self::MarketifTouched),
            "OrderType_LimitifTouched" => Some(Self::LimitifTouched),
            "OrderType_TrailingStop" => Some(Self::TrailingStop),
            "OrderType_TrailingStopLimit" => Some(Self::TrailingStopLimit),
            "OrderType_TWAP_MARKET" => Some(Self::TwapMarket),
            "OrderType_TWAP_LIMIT" => Some(Self::TwapLimit),
            "OrderType_VWAP_MARKET" => Some(Self::VwapMarket),
            "OrderType_VWAP_LIMIT" => Some(Self::VwapLimit),
            _ => None,
        }
    }
}
/// 跟踪类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrailType {
    /// 未知类型
    Unknown = 0,
    /// 比例
    Ratio = 1,
    /// 金额
    Amount = 2,
}
impl TrailType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrailType::Unknown => "TrailType_Unknown",
            TrailType::Ratio => "TrailType_Ratio",
            TrailType::Amount => "TrailType_Amount",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TrailType_Unknown" => Some(Self::Unknown),
            "TrailType_Ratio" => Some(Self::Ratio),
            "TrailType_Amount" => Some(Self::Amount),
            _ => None,
        }
    }
}
/// 订单状态
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderStatus {
    /// 未提交
    Unsubmitted = 0,
    /// 未知状态
    Unknown = -1,
    /// 等待提交
    WaitingSubmit = 1,
    /// 提交中
    Submitting = 2,
    /// 提交失败，下单失败
    SubmitFailed = 3,
    /// 处理超时，结果未知
    TimeOut = 4,
    /// 已提交，等待成交
    Submitted = 5,
    /// 部分成交
    FilledPart = 10,
    /// 全部已成
    FilledAll = 11,
    /// 正在撤单_部分(部分已成交，正在撤销剩余部分)
    CancellingPart = 12,
    /// 正在撤单_全部
    CancellingAll = 13,
    /// 部分成交，剩余部分已撤单
    CancelledPart = 14,
    /// 全部已撤单，无成交
    CancelledAll = 15,
    /// 下单失败，服务拒绝
    Failed = 21,
    /// 已失效
    Disabled = 22,
    /// 已删除，无成交的订单才能删除
    Deleted = 23,
    /// 成交被撤销，一般遇不到，意思是已经成交的订单被回滚撤销，成交无效变为废单
    FillCancelled = 24,
}
impl OrderStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderStatus::Unsubmitted => "OrderStatus_Unsubmitted",
            OrderStatus::Unknown => "OrderStatus_Unknown",
            OrderStatus::WaitingSubmit => "OrderStatus_WaitingSubmit",
            OrderStatus::Submitting => "OrderStatus_Submitting",
            OrderStatus::SubmitFailed => "OrderStatus_SubmitFailed",
            OrderStatus::TimeOut => "OrderStatus_TimeOut",
            OrderStatus::Submitted => "OrderStatus_Submitted",
            OrderStatus::FilledPart => "OrderStatus_Filled_Part",
            OrderStatus::FilledAll => "OrderStatus_Filled_All",
            OrderStatus::CancellingPart => "OrderStatus_Cancelling_Part",
            OrderStatus::CancellingAll => "OrderStatus_Cancelling_All",
            OrderStatus::CancelledPart => "OrderStatus_Cancelled_Part",
            OrderStatus::CancelledAll => "OrderStatus_Cancelled_All",
            OrderStatus::Failed => "OrderStatus_Failed",
            OrderStatus::Disabled => "OrderStatus_Disabled",
            OrderStatus::Deleted => "OrderStatus_Deleted",
            OrderStatus::FillCancelled => "OrderStatus_FillCancelled",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OrderStatus_Unsubmitted" => Some(Self::Unsubmitted),
            "OrderStatus_Unknown" => Some(Self::Unknown),
            "OrderStatus_WaitingSubmit" => Some(Self::WaitingSubmit),
            "OrderStatus_Submitting" => Some(Self::Submitting),
            "OrderStatus_SubmitFailed" => Some(Self::SubmitFailed),
            "OrderStatus_TimeOut" => Some(Self::TimeOut),
            "OrderStatus_Submitted" => Some(Self::Submitted),
            "OrderStatus_Filled_Part" => Some(Self::FilledPart),
            "OrderStatus_Filled_All" => Some(Self::FilledAll),
            "OrderStatus_Cancelling_Part" => Some(Self::CancellingPart),
            "OrderStatus_Cancelling_All" => Some(Self::CancellingAll),
            "OrderStatus_Cancelled_Part" => Some(Self::CancelledPart),
            "OrderStatus_Cancelled_All" => Some(Self::CancelledAll),
            "OrderStatus_Failed" => Some(Self::Failed),
            "OrderStatus_Disabled" => Some(Self::Disabled),
            "OrderStatus_Deleted" => Some(Self::Deleted),
            "OrderStatus_FillCancelled" => Some(Self::FillCancelled),
            _ => None,
        }
    }
}
/// 一笔成交的状态
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderFillStatus {
    /// 正常
    Ok = 0,
    /// 成交被取消
    Cancelled = 1,
    /// 成交被更改
    Changed = 2,
}
impl OrderFillStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderFillStatus::Ok => "OrderFillStatus_OK",
            OrderFillStatus::Cancelled => "OrderFillStatus_Cancelled",
            OrderFillStatus::Changed => "OrderFillStatus_Changed",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OrderFillStatus_OK" => Some(Self::Ok),
            "OrderFillStatus_Cancelled" => Some(Self::Cancelled),
            "OrderFillStatus_Changed" => Some(Self::Changed),
            _ => None,
        }
    }
}
/// 持仓方向类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PositionSide {
    /// 多仓，默认情况是多仓
    Long = 0,
    /// 未知方向
    Unknown = -1,
    /// 空仓
    Short = 1,
}
impl PositionSide {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PositionSide::Long => "PositionSide_Long",
            PositionSide::Unknown => "PositionSide_Unknown",
            PositionSide::Short => "PositionSide_Short",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PositionSide_Long" => Some(Self::Long),
            "PositionSide_Unknown" => Some(Self::Unknown),
            "PositionSide_Short" => Some(Self::Short),
            _ => None,
        }
    }
}
/// 修改订单的操作类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModifyOrderOp {
    /// 港股支持全部操作，美股目前仅支持ModifyOrderOp_Normal和ModifyOrderOp_Cancel
    ///
    /// 未知操作
    Unknown = 0,
    /// 修改订单的价格、数量等，即以前的改单
    Normal = 1,
    /// 撤单
    Cancel = 2,
    /// 失效
    Disable = 3,
    /// 生效
    Enable = 4,
    /// 删除
    Delete = 5,
}
impl ModifyOrderOp {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ModifyOrderOp::Unknown => "ModifyOrderOp_Unknown",
            ModifyOrderOp::Normal => "ModifyOrderOp_Normal",
            ModifyOrderOp::Cancel => "ModifyOrderOp_Cancel",
            ModifyOrderOp::Disable => "ModifyOrderOp_Disable",
            ModifyOrderOp::Enable => "ModifyOrderOp_Enable",
            ModifyOrderOp::Delete => "ModifyOrderOp_Delete",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ModifyOrderOp_Unknown" => Some(Self::Unknown),
            "ModifyOrderOp_Normal" => Some(Self::Normal),
            "ModifyOrderOp_Cancel" => Some(Self::Cancel),
            "ModifyOrderOp_Disable" => Some(Self::Disable),
            "ModifyOrderOp_Enable" => Some(Self::Enable),
            "ModifyOrderOp_Delete" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// 交易账户类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrdAccType {
    /// 未知类型
    Unknown = 0,
    /// 现金账户
    Cash = 1,
    /// 保证金账户
    Margin = 2,
}
impl TrdAccType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrdAccType::Unknown => "TrdAccType_Unknown",
            TrdAccType::Cash => "TrdAccType_Cash",
            TrdAccType::Margin => "TrdAccType_Margin",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TrdAccType_Unknown" => Some(Self::Unknown),
            "TrdAccType_Cash" => Some(Self::Cash),
            "TrdAccType_Margin" => Some(Self::Margin),
            _ => None,
        }
    }
}
/// 货币种类
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Currency {
    /// 未知货币
    Unknown = 0,
    /// 港币
    Hkd = 1,
    /// 美元
    Usd = 2,
    /// 离岸人民币
    Cnh = 3,
    /// 日元
    Jpy = 4,
    /// 新币
    Sgd = 5,
    /// 澳元
    Aud = 6,
}
impl Currency {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Currency::Unknown => "Currency_Unknown",
            Currency::Hkd => "Currency_HKD",
            Currency::Usd => "Currency_USD",
            Currency::Cnh => "Currency_CNH",
            Currency::Jpy => "Currency_JPY",
            Currency::Sgd => "Currency_SGD",
            Currency::Aud => "Currency_AUD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Currency_Unknown" => Some(Self::Unknown),
            "Currency_HKD" => Some(Self::Hkd),
            "Currency_USD" => Some(Self::Usd),
            "Currency_CNH" => Some(Self::Cnh),
            "Currency_JPY" => Some(Self::Jpy),
            "Currency_SGD" => Some(Self::Sgd),
            "Currency_AUD" => Some(Self::Aud),
            _ => None,
        }
    }
}
/// 账户风险控制等级
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CltRiskLevel {
    /// 未知
    Unknown = -1,
    /// 安全
    Safe = 0,
    /// 预警
    Warning = 1,
    /// 危险
    Danger = 2,
    /// 绝对安全
    AbsoluteSafe = 3,
    /// 危险, 期权相关
    OptDanger = 4,
}
impl CltRiskLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CltRiskLevel::Unknown => "CltRiskLevel_Unknown",
            CltRiskLevel::Safe => "CltRiskLevel_Safe",
            CltRiskLevel::Warning => "CltRiskLevel_Warning",
            CltRiskLevel::Danger => "CltRiskLevel_Danger",
            CltRiskLevel::AbsoluteSafe => "CltRiskLevel_AbsoluteSafe",
            CltRiskLevel::OptDanger => "CltRiskLevel_OptDanger",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CltRiskLevel_Unknown" => Some(Self::Unknown),
            "CltRiskLevel_Safe" => Some(Self::Safe),
            "CltRiskLevel_Warning" => Some(Self::Warning),
            "CltRiskLevel_Danger" => Some(Self::Danger),
            "CltRiskLevel_AbsoluteSafe" => Some(Self::AbsoluteSafe),
            "CltRiskLevel_OptDanger" => Some(Self::OptDanger),
            _ => None,
        }
    }
}
/// 订单有效期
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TimeInForce {
    /// 当日有效
    Day = 0,
    /// 撤单前有效，最多持续90自然日。
    Gtc = 1,
}
impl TimeInForce {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TimeInForce::Day => "TimeInForce_DAY",
            TimeInForce::Gtc => "TimeInForce_GTC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TimeInForce_DAY" => Some(Self::Day),
            "TimeInForce_GTC" => Some(Self::Gtc),
            _ => None,
        }
    }
}
/// 券商
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityFirm {
    /// 未知
    Unknown = 0,
    /// 富途证券（香港）
    FutuSecurities = 1,
    /// 富途证券（美国）
    FutuInc = 2,
    /// 富途证券（新加坡）
    FutuSg = 3,
    /// 富途证券（澳洲）
    FutuAu = 4,
}
impl SecurityFirm {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SecurityFirm::Unknown => "SecurityFirm_Unknown",
            SecurityFirm::FutuSecurities => "SecurityFirm_FutuSecurities",
            SecurityFirm::FutuInc => "SecurityFirm_FutuInc",
            SecurityFirm::FutuSg => "SecurityFirm_FutuSG",
            SecurityFirm::FutuAu => "SecurityFirm_FutuAU",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SecurityFirm_Unknown" => Some(Self::Unknown),
            "SecurityFirm_FutuSecurities" => Some(Self::FutuSecurities),
            "SecurityFirm_FutuInc" => Some(Self::FutuInc),
            "SecurityFirm_FutuSG" => Some(Self::FutuSg),
            "SecurityFirm_FutuAU" => Some(Self::FutuAu),
            _ => None,
        }
    }
}
/// 模拟交易账户类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SimAccType {
    /// 未知
    Unknown = 0,
    /// 股票模拟账户（仅用于交易证券类产品，不支持交易期权）
    Stock = 1,
    /// 期权模拟账户（仅用于交易期权，不支持交易股票证券类产品）
    Option = 2,
    /// 期货模拟账户
    Futures = 3,
}
impl SimAccType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SimAccType::Unknown => "SimAccType_Unknown",
            SimAccType::Stock => "SimAccType_Stock",
            SimAccType::Option => "SimAccType_Option",
            SimAccType::Futures => "SimAccType_Futures",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SimAccType_Unknown" => Some(Self::Unknown),
            "SimAccType_Stock" => Some(Self::Stock),
            "SimAccType_Option" => Some(Self::Option),
            "SimAccType_Futures" => Some(Self::Futures),
            _ => None,
        }
    }
}
/// 风险状态，共分 9 个等级，LEVEL1是最安全，LEVEL9是最危险
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CltRiskStatus {
    /// 未知
    Unknown = 0,
    /// 非常安全
    Level1 = 1,
    /// 安全
    Level2 = 2,
    /// 较安全
    Level3 = 3,
    /// 较低风险
    Level4 = 4,
    /// 中等风险
    Level5 = 5,
    /// 较高风险
    Level6 = 6,
    /// 预警
    Level7 = 7,
    /// 预警
    Level8 = 8,
    /// 预警
    Level9 = 9,
}
impl CltRiskStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CltRiskStatus::Unknown => "CltRiskStatus_Unknown",
            CltRiskStatus::Level1 => "CltRiskStatus_Level1",
            CltRiskStatus::Level2 => "CltRiskStatus_Level2",
            CltRiskStatus::Level3 => "CltRiskStatus_Level3",
            CltRiskStatus::Level4 => "CltRiskStatus_Level4",
            CltRiskStatus::Level5 => "CltRiskStatus_Level5",
            CltRiskStatus::Level6 => "CltRiskStatus_Level6",
            CltRiskStatus::Level7 => "CltRiskStatus_Level7",
            CltRiskStatus::Level8 => "CltRiskStatus_Level8",
            CltRiskStatus::Level9 => "CltRiskStatus_Level9",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CltRiskStatus_Unknown" => Some(Self::Unknown),
            "CltRiskStatus_Level1" => Some(Self::Level1),
            "CltRiskStatus_Level2" => Some(Self::Level2),
            "CltRiskStatus_Level3" => Some(Self::Level3),
            "CltRiskStatus_Level4" => Some(Self::Level4),
            "CltRiskStatus_Level5" => Some(Self::Level5),
            "CltRiskStatus_Level6" => Some(Self::Level6),
            "CltRiskStatus_Level7" => Some(Self::Level7),
            "CltRiskStatus_Level8" => Some(Self::Level8),
            "CltRiskStatus_Level9" => Some(Self::Level9),
            _ => None,
        }
    }
}
/// 日内交易限制情况
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DtStatus {
    /// 未知
    Unknown = 0,
    /// 无限次(当前可以无限次日内交易，注意留意剩余日内交易购买力)
    Unlimited = 1,
    /// EM Call(当前状态不能新建仓位，需要补充资产净值至$25000以上，否则会被禁止新建仓位90天)
    EmCall = 2,
    /// DT Call(当前状态有未补平的日内交易追缴金额（DTCall），需要在5个交易日内足额入金来补平 DTCall，否则会被禁止新建仓位，直到足额存入资金才会解禁)
    DtCall = 3,
}
impl DtStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DtStatus::Unknown => "DTStatus_Unknown",
            DtStatus::Unlimited => "DTStatus_Unlimited",
            DtStatus::EmCall => "DTStatus_EMCall",
            DtStatus::DtCall => "DTStatus_DTCall",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DTStatus_Unknown" => Some(Self::Unknown),
            "DTStatus_Unlimited" => Some(Self::Unlimited),
            "DTStatus_EMCall" => Some(Self::EmCall),
            "DTStatus_DTCall" => Some(Self::DtCall),
            _ => None,
        }
    }
}
