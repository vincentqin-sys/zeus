/// 两个字段确定一支股票
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Security {
    /// QotMarket,股票市场
    #[prost(int32, required, tag = "1")]
    pub market: i32,
    /// 股票代码
    #[prost(string, required, tag = "2")]
    pub code: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KLine {
    /// 时间戳字符串
    #[prost(string, required, tag = "1")]
    pub time: ::prost::alloc::string::String,
    /// 是否是空内容的点,若为ture则只有时间信息
    #[prost(bool, required, tag = "2")]
    pub is_blank: bool,
    /// 最高价
    #[prost(double, optional, tag = "3")]
    pub high_price: ::core::option::Option<f64>,
    /// 开盘价
    #[prost(double, optional, tag = "4")]
    pub open_price: ::core::option::Option<f64>,
    /// 最低价
    #[prost(double, optional, tag = "5")]
    pub low_price: ::core::option::Option<f64>,
    /// 收盘价
    #[prost(double, optional, tag = "6")]
    pub close_price: ::core::option::Option<f64>,
    /// 昨收价
    #[prost(double, optional, tag = "7")]
    pub last_close_price: ::core::option::Option<f64>,
    /// 成交量
    #[prost(int64, optional, tag = "8")]
    pub volume: ::core::option::Option<i64>,
    /// 成交额
    #[prost(double, optional, tag = "9")]
    pub turnover: ::core::option::Option<f64>,
    /// 换手率（该字段为百分比字段，展示为小数表示）
    #[prost(double, optional, tag = "10")]
    pub turnover_rate: ::core::option::Option<f64>,
    /// 市盈率
    #[prost(double, optional, tag = "11")]
    pub pe: ::core::option::Option<f64>,
    /// 涨跌幅（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag = "12")]
    pub change_rate: ::core::option::Option<f64>,
    /// 时间戳
    #[prost(double, optional, tag = "13")]
    pub timestamp: ::core::option::Option<f64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionBasicQotExData {
    /// 行权价
    #[prost(double, required, tag = "1")]
    pub strike_price: f64,
    /// 每份合约数(整型数据)
    #[prost(int32, required, tag = "2")]
    pub contract_size: i32,
    /// 每份合约数（浮点型数据）
    #[prost(double, optional, tag = "17")]
    pub contract_size_float: ::core::option::Option<f64>,
    /// 未平仓合约数
    #[prost(int32, required, tag = "3")]
    pub open_interest: i32,
    /// 隐含波动率（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag = "4")]
    pub implied_volatility: f64,
    /// 溢价（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag = "5")]
    pub premium: f64,
    /// 希腊值 Delta
    #[prost(double, required, tag = "6")]
    pub delta: f64,
    /// 希腊值 Gamma
    #[prost(double, required, tag = "7")]
    pub gamma: f64,
    /// 希腊值 Vega
    #[prost(double, required, tag = "8")]
    pub vega: f64,
    /// 希腊值 Theta
    #[prost(double, required, tag = "9")]
    pub theta: f64,
    /// 希腊值 Rho
    #[prost(double, required, tag = "10")]
    pub rho: f64,
    /// 净未平仓合约数，仅港股期权适用
    #[prost(int32, optional, tag = "11")]
    pub net_open_interest: ::core::option::Option<i32>,
    /// 距离到期日天数，负数表示已过期
    #[prost(int32, optional, tag = "12")]
    pub expiry_date_distance: ::core::option::Option<i32>,
    /// 合约名义金额，仅港股期权适用
    #[prost(double, optional, tag = "13")]
    pub contract_nominal_value: ::core::option::Option<f64>,
    /// 相等正股手数，指数期权无该字段，仅港股期权适用
    #[prost(double, optional, tag = "14")]
    pub owner_lot_multiplier: ::core::option::Option<f64>,
    /// OptionAreaType，期权类型（按行权时间）
    #[prost(int32, optional, tag = "15")]
    pub option_area_type: ::core::option::Option<i32>,
    /// 合约乘数
    #[prost(double, optional, tag = "16")]
    pub contract_multiplier: ::core::option::Option<f64>,
    /// IndexOptionType，指数期权类型
    #[prost(int32, optional, tag = "18")]
    pub index_option_type: ::core::option::Option<i32>,
}
/// 美股支持盘前盘后数据
/// 科创板仅支持盘后数据：成交量，成交额
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreAfterMarketData {
    /// 盘前或盘后 - 价格
    #[prost(double, optional, tag = "1")]
    pub price: ::core::option::Option<f64>,
    /// 盘前或盘后 - 最高价
    #[prost(double, optional, tag = "2")]
    pub high_price: ::core::option::Option<f64>,
    /// 盘前或盘后 - 最低价
    #[prost(double, optional, tag = "3")]
    pub low_price: ::core::option::Option<f64>,
    /// 盘前或盘后 - 成交量
    #[prost(int64, optional, tag = "4")]
    pub volume: ::core::option::Option<i64>,
    /// 盘前或盘后 - 成交额
    #[prost(double, optional, tag = "5")]
    pub turnover: ::core::option::Option<f64>,
    /// 盘前或盘后 - 涨跌额
    #[prost(double, optional, tag = "6")]
    pub change_val: ::core::option::Option<f64>,
    /// 盘前或盘后 - 涨跌幅（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag = "7")]
    pub change_rate: ::core::option::Option<f64>,
    /// 盘前或盘后 - 振幅（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, optional, tag = "8")]
    pub amplitude: ::core::option::Option<f64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FutureBasicQotExData {
    /// 昨结
    #[prost(double, required, tag = "1")]
    pub last_settle_price: f64,
    /// 持仓量
    #[prost(int32, required, tag = "2")]
    pub position: i32,
    /// 日增仓
    #[prost(int32, required, tag = "3")]
    pub position_change: i32,
    /// 距离到期日天数
    #[prost(int32, optional, tag = "4")]
    pub expiry_date_distance: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarrantBasicQotExData {
    /// 对冲值,仅认购认沽支持该字段
    #[prost(double, optional, tag = "1")]
    pub delta: ::core::option::Option<f64>,
    /// 引申波幅,仅认购认沽支持该字段
    #[prost(double, optional, tag = "2")]
    pub implied_volatility: ::core::option::Option<f64>,
    /// 溢价（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag = "3")]
    pub premium: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicQot {
    /// 股票
    #[prost(message, required, tag = "1")]
    pub security: Security,
    /// 股票名称
    #[prost(string, optional, tag = "24")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// 是否停牌
    #[prost(bool, required, tag = "2")]
    pub is_suspended: bool,
    /// 上市日期字符串
    #[prost(string, required, tag = "3")]
    pub list_time: ::prost::alloc::string::String,
    /// 价差
    #[prost(double, required, tag = "4")]
    pub price_spread: f64,
    /// 最新价的更新时间字符串，对其他字段不适用
    #[prost(string, required, tag = "5")]
    pub update_time: ::prost::alloc::string::String,
    /// 最高价
    #[prost(double, required, tag = "6")]
    pub high_price: f64,
    /// 开盘价
    #[prost(double, required, tag = "7")]
    pub open_price: f64,
    /// 最低价
    #[prost(double, required, tag = "8")]
    pub low_price: f64,
    /// 最新价
    #[prost(double, required, tag = "9")]
    pub cur_price: f64,
    /// 昨收价
    #[prost(double, required, tag = "10")]
    pub last_close_price: f64,
    /// 成交量
    #[prost(int64, required, tag = "11")]
    pub volume: i64,
    /// 成交额
    #[prost(double, required, tag = "12")]
    pub turnover: f64,
    /// 换手率（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag = "13")]
    pub turnover_rate: f64,
    /// 振幅（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag = "14")]
    pub amplitude: f64,
    /// DarkStatus, 暗盘交易状态	
    #[prost(int32, optional, tag = "15")]
    pub dark_status: ::core::option::Option<i32>,
    /// 期权特有字段
    #[prost(message, optional, tag = "16")]
    pub option_ex_data: ::core::option::Option<OptionBasicQotExData>,
    /// 上市日期时间戳
    #[prost(double, optional, tag = "17")]
    pub list_timestamp: ::core::option::Option<f64>,
    /// 最新价的更新时间戳，对其他字段不适用
    #[prost(double, optional, tag = "18")]
    pub update_timestamp: ::core::option::Option<f64>,
    /// 盘前数据
    #[prost(message, optional, tag = "19")]
    pub pre_market: ::core::option::Option<PreAfterMarketData>,
    /// 盘后数据
    #[prost(message, optional, tag = "20")]
    pub after_market: ::core::option::Option<PreAfterMarketData>,
    /// SecurityStatus, 股票状态
    #[prost(int32, optional, tag = "21")]
    pub sec_status: ::core::option::Option<i32>,
    /// 期货特有字段
    #[prost(message, optional, tag = "22")]
    pub future_ex_data: ::core::option::Option<FutureBasicQotExData>,
    /// 窝轮特有字段
    #[prost(message, optional, tag = "23")]
    pub warrant_ex_data: ::core::option::Option<WarrantBasicQotExData>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeShare {
    /// 时间字符串
    #[prost(string, required, tag = "1")]
    pub time: ::prost::alloc::string::String,
    /// 距离0点过了多少分钟
    #[prost(int32, required, tag = "2")]
    pub minute: i32,
    /// 是否是空内容的点,若为ture则只有时间信息
    #[prost(bool, required, tag = "3")]
    pub is_blank: bool,
    /// 当前价
    #[prost(double, optional, tag = "4")]
    pub price: ::core::option::Option<f64>,
    /// 昨收价
    #[prost(double, optional, tag = "5")]
    pub last_close_price: ::core::option::Option<f64>,
    /// 均价
    #[prost(double, optional, tag = "6")]
    pub avg_price: ::core::option::Option<f64>,
    /// 成交量
    #[prost(int64, optional, tag = "7")]
    pub volume: ::core::option::Option<i64>,
    /// 成交额
    #[prost(double, optional, tag = "8")]
    pub turnover: ::core::option::Option<f64>,
    /// 时间戳
    #[prost(double, optional, tag = "9")]
    pub timestamp: ::core::option::Option<f64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityStaticBasic {
    /// 股票
    #[prost(message, required, tag = "1")]
    pub security: Security,
    /// 股票ID
    #[prost(int64, required, tag = "2")]
    pub id: i64,
    /// 每手数量,期权以及期货类型表示合约乘数
    #[prost(int32, required, tag = "3")]
    pub lot_size: i32,
    /// Qot_Common.SecurityType,股票类型
    #[prost(int32, required, tag = "4")]
    pub sec_type: i32,
    /// 股票名字
    #[prost(string, required, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// 上市时间字符串
    #[prost(string, required, tag = "6")]
    pub list_time: ::prost::alloc::string::String,
    /// 是否退市
    #[prost(bool, optional, tag = "7")]
    pub delisting: ::core::option::Option<bool>,
    /// 上市时间戳
    #[prost(double, optional, tag = "8")]
    pub list_timestamp: ::core::option::Option<f64>,
    /// Qot_Common.ExchType,所属交易所
    #[prost(int32, optional, tag = "9")]
    pub exch_type: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WarrantStaticExData {
    /// Qot_Common.WarrantType,窝轮类型
    #[prost(int32, required, tag = "1")]
    pub r#type: i32,
    /// 所属正股
    #[prost(message, required, tag = "2")]
    pub owner: Security,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionStaticExData {
    /// Qot_Common.OptionType,期权
    #[prost(int32, required, tag = "1")]
    pub r#type: i32,
    /// 标的股
    #[prost(message, required, tag = "2")]
    pub owner: Security,
    /// 行权日
    #[prost(string, required, tag = "3")]
    pub strike_time: ::prost::alloc::string::String,
    /// 行权价
    #[prost(double, required, tag = "4")]
    pub strike_price: f64,
    /// 是否停牌
    #[prost(bool, required, tag = "5")]
    pub suspend: bool,
    /// 发行市场名字
    #[prost(string, required, tag = "6")]
    pub market: ::prost::alloc::string::String,
    /// 行权日时间戳
    #[prost(double, optional, tag = "7")]
    pub strike_timestamp: ::core::option::Option<f64>,
    /// Qot_Common.IndexOptionType, 指数期权的类型，仅在指数期权有效
    #[prost(int32, optional, tag = "8")]
    pub index_option_type: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FutureStaticExData {
    /// 最后交易日，只有非主连期货合约才有该字段
    #[prost(string, required, tag = "1")]
    pub last_trade_time: ::prost::alloc::string::String,
    /// 最后交易日时间戳，只有非主连期货合约才有该字段
    #[prost(double, optional, tag = "2")]
    pub last_trade_timestamp: ::core::option::Option<f64>,
    /// 是否主连合约
    #[prost(bool, required, tag = "3")]
    pub is_main_contract: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityStaticInfo {
    /// 基本股票静态信息
    #[prost(message, required, tag = "1")]
    pub basic: SecurityStaticBasic,
    /// 窝轮额外股票静态信息
    #[prost(message, optional, tag = "2")]
    pub warrant_ex_data: ::core::option::Option<WarrantStaticExData>,
    /// 期权额外股票静态信息
    #[prost(message, optional, tag = "3")]
    pub option_ex_data: ::core::option::Option<OptionStaticExData>,
    /// 期货额外股票静态信息
    #[prost(message, optional, tag = "4")]
    pub future_ex_data: ::core::option::Option<FutureStaticExData>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Broker {
    /// 经纪ID
    #[prost(int64, required, tag = "1")]
    pub id: i64,
    /// 经纪名称
    #[prost(string, required, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 经纪档位
    #[prost(int32, required, tag = "3")]
    pub pos: i32,
    /// 以下为SF行情特有字段
    ///
    /// 交易所订单ID，与交易接口返回的订单ID并不一样
    #[prost(int64, optional, tag = "4")]
    pub order_id: ::core::option::Option<i64>,
    /// 订单股数
    #[prost(int64, optional, tag = "5")]
    pub volume: ::core::option::Option<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ticker {
    /// 时间字符串
    #[prost(string, required, tag = "1")]
    pub time: ::prost::alloc::string::String,
    /// 唯一标识
    #[prost(int64, required, tag = "2")]
    pub sequence: i64,
    /// TickerDirection, 买卖方向
    #[prost(int32, required, tag = "3")]
    pub dir: i32,
    /// 价格
    #[prost(double, required, tag = "4")]
    pub price: f64,
    /// 成交量
    #[prost(int64, required, tag = "5")]
    pub volume: i64,
    /// 成交额
    #[prost(double, required, tag = "6")]
    pub turnover: f64,
    /// 收到推送数据的本地时间戳，用于定位延迟
    #[prost(double, optional, tag = "7")]
    pub recv_time: ::core::option::Option<f64>,
    /// TickerType, 逐笔类型
    #[prost(int32, optional, tag = "8")]
    pub r#type: ::core::option::Option<i32>,
    /// 逐笔类型符号
    #[prost(int32, optional, tag = "9")]
    pub type_sign: ::core::option::Option<i32>,
    /// 用于区分推送情况
    #[prost(int32, optional, tag = "10")]
    pub push_data_type: ::core::option::Option<i32>,
    /// 时间戳
    #[prost(double, optional, tag = "11")]
    pub timestamp: ::core::option::Option<f64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookDetail {
    /// 交易所订单ID，与交易接口返回的订单ID并不一样
    #[prost(int64, required, tag = "1")]
    pub order_id: i64,
    /// 订单股数
    #[prost(int64, required, tag = "2")]
    pub volume: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBook {
    /// 委托价格
    #[prost(double, required, tag = "1")]
    pub price: f64,
    /// 委托数量
    #[prost(int64, required, tag = "2")]
    pub volume: i64,
    /// 委托订单个数
    #[prost(int32, required, tag = "3")]
    pub oreder_count: i32,
    /// 订单信息，SF行情特有
    #[prost(message, repeated, tag = "4")]
    pub detail_list: ::prost::alloc::vec::Vec<OrderBookDetail>,
}
/// 持股变动
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareHoldingChange {
    /// 持有者名称（机构名称 或 基金名称 或 高管姓名）
    #[prost(string, required, tag = "1")]
    pub holder_name: ::prost::alloc::string::String,
    /// 当前持股数量
    #[prost(double, required, tag = "2")]
    pub holding_qty: f64,
    /// 当前持股百分比（该字段为百分比字段，默认不展示%，如20实际对应20%）
    #[prost(double, required, tag = "3")]
    pub holding_ratio: f64,
    /// 较上一次变动数量
    #[prost(double, required, tag = "4")]
    pub change_qty: f64,
    /// 较上一次变动百分比（该字段为百分比字段，默认不展示%，如20实际对应20%。是相对于自身的比例，而不是总的。如总股本1万股，持有100股，持股百分比是1%，卖掉50股，变动比例是50%，而不是0.5%）
    #[prost(double, required, tag = "5")]
    pub change_ratio: f64,
    /// 发布时间(YYYY-MM-DD HH:MM:SS字符串)
    #[prost(string, required, tag = "6")]
    pub time: ::prost::alloc::string::String,
    /// 时间戳
    #[prost(double, optional, tag = "7")]
    pub timestamp: ::core::option::Option<f64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubInfo {
    /// Qot_Common.SubType,订阅类型
    #[prost(int32, required, tag = "1")]
    pub sub_type: i32,
    /// 订阅该类型行情的股票
    #[prost(message, repeated, tag = "2")]
    pub security_list: ::prost::alloc::vec::Vec<Security>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnSubInfo {
    /// 该连接订阅信息
    #[prost(message, repeated, tag = "1")]
    pub sub_info_list: ::prost::alloc::vec::Vec<SubInfo>,
    /// 该连接已经使用的订阅额度
    #[prost(int32, required, tag = "2")]
    pub used_quota: i32,
    /// 用于区分是否是自己连接的数据
    #[prost(bool, required, tag = "3")]
    pub is_own_conn_data: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlateInfo {
    /// 板块
    #[prost(message, required, tag = "1")]
    pub plate: Security,
    /// 板块名字
    #[prost(string, required, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// PlateSetType 板块类型, 仅3207（获取股票所属板块）协议返回该字段
    #[prost(int32, optional, tag = "3")]
    pub plate_type: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rehab {
    /// 时间字符串
    #[prost(string, required, tag = "1")]
    pub time: ::prost::alloc::string::String,
    /// 公司行动(CompanyAct)组合标志位,指定某些字段值是否有效
    #[prost(int64, required, tag = "2")]
    pub company_act_flag: i64,
    /// 前复权因子A
    #[prost(double, required, tag = "3")]
    pub fwd_factor_a: f64,
    /// 前复权因子B
    #[prost(double, required, tag = "4")]
    pub fwd_factor_b: f64,
    /// 后复权因子A
    #[prost(double, required, tag = "5")]
    pub bwd_factor_a: f64,
    /// 后复权因子B
    #[prost(double, required, tag = "6")]
    pub bwd_factor_b: f64,
    /// 拆股(例如，1拆5，Base为1，Ert为5)
    #[prost(int32, optional, tag = "7")]
    pub split_base: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub split_ert: ::core::option::Option<i32>,
    /// 合股(例如，50合1，Base为50，Ert为1)
    #[prost(int32, optional, tag = "9")]
    pub join_base: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub join_ert: ::core::option::Option<i32>,
    /// 送股(例如，10送3, Base为10,Ert为3)
    #[prost(int32, optional, tag = "11")]
    pub bonus_base: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub bonus_ert: ::core::option::Option<i32>,
    /// 转赠股(例如，10转3, Base为10,Ert为3)
    #[prost(int32, optional, tag = "13")]
    pub transfer_base: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "14")]
    pub transfer_ert: ::core::option::Option<i32>,
    /// 配股(例如，10送2, 配股价为6.3元, Base为10, Ert为2, Price为6.3)
    #[prost(int32, optional, tag = "15")]
    pub allot_base: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "16")]
    pub allot_ert: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "17")]
    pub allot_price: ::core::option::Option<f64>,
    /// 增发股(例如，10送2, 增发股价为6.3元, Base为10, Ert为2, Price为6.3)
    #[prost(int32, optional, tag = "18")]
    pub add_base: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "19")]
    pub add_ert: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "20")]
    pub add_price: ::core::option::Option<f64>,
    /// 现金分红(例如，每10股派现0.5元,则该字段值为0.05)
    #[prost(double, optional, tag = "21")]
    pub dividend: ::core::option::Option<f64>,
    /// 特别股息(例如，每10股派特别股息0.5元,则该字段值为0.05)
    #[prost(double, optional, tag = "22")]
    pub sp_dividend: ::core::option::Option<f64>,
    /// 时间戳
    #[prost(double, optional, tag = "23")]
    pub timestamp: ::core::option::Option<f64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QotMarket {
    /// 未知市场
    Unknown = 0,
    /// 香港市场
    HkSecurity = 1,
    /// 港期货(已废弃，使用QotMarket_HK_Security即可)
    HkFuture = 2,
    /// 美国市场
    UsSecurity = 11,
    /// 沪股市场
    CnshSecurity = 21,
    /// 深股市场
    CnszSecurity = 22,
    /// 新加坡市场
    SgSecurity = 31,
    /// 日本市场
    JpSecurity = 41,
}
impl QotMarket {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            QotMarket::Unknown => "QotMarket_Unknown",
            QotMarket::HkSecurity => "QotMarket_HK_Security",
            QotMarket::HkFuture => "QotMarket_HK_Future",
            QotMarket::UsSecurity => "QotMarket_US_Security",
            QotMarket::CnshSecurity => "QotMarket_CNSH_Security",
            QotMarket::CnszSecurity => "QotMarket_CNSZ_Security",
            QotMarket::SgSecurity => "QotMarket_SG_Security",
            QotMarket::JpSecurity => "QotMarket_JP_Security",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "QotMarket_Unknown" => Some(Self::Unknown),
            "QotMarket_HK_Security" => Some(Self::HkSecurity),
            "QotMarket_HK_Future" => Some(Self::HkFuture),
            "QotMarket_US_Security" => Some(Self::UsSecurity),
            "QotMarket_CNSH_Security" => Some(Self::CnshSecurity),
            "QotMarket_CNSZ_Security" => Some(Self::CnszSecurity),
            "QotMarket_SG_Security" => Some(Self::SgSecurity),
            "QotMarket_JP_Security" => Some(Self::JpSecurity),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityType {
    /// 未知
    Unknown = 0,
    /// 债券
    Bond = 1,
    /// 一揽子权证
    Bwrt = 2,
    /// 正股
    Eqty = 3,
    /// 信托,基金
    Trust = 4,
    /// 窝轮
    Warrant = 5,
    /// 指数
    Index = 6,
    /// 板块
    Plate = 7,
    /// 期权
    Drvt = 8,
    /// 板块集
    PlateSet = 9,
    /// 期货
    Future = 10,
}
impl SecurityType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SecurityType::Unknown => "SecurityType_Unknown",
            SecurityType::Bond => "SecurityType_Bond",
            SecurityType::Bwrt => "SecurityType_Bwrt",
            SecurityType::Eqty => "SecurityType_Eqty",
            SecurityType::Trust => "SecurityType_Trust",
            SecurityType::Warrant => "SecurityType_Warrant",
            SecurityType::Index => "SecurityType_Index",
            SecurityType::Plate => "SecurityType_Plate",
            SecurityType::Drvt => "SecurityType_Drvt",
            SecurityType::PlateSet => "SecurityType_PlateSet",
            SecurityType::Future => "SecurityType_Future",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SecurityType_Unknown" => Some(Self::Unknown),
            "SecurityType_Bond" => Some(Self::Bond),
            "SecurityType_Bwrt" => Some(Self::Bwrt),
            "SecurityType_Eqty" => Some(Self::Eqty),
            "SecurityType_Trust" => Some(Self::Trust),
            "SecurityType_Warrant" => Some(Self::Warrant),
            "SecurityType_Index" => Some(Self::Index),
            "SecurityType_Plate" => Some(Self::Plate),
            "SecurityType_Drvt" => Some(Self::Drvt),
            "SecurityType_PlateSet" => Some(Self::PlateSet),
            "SecurityType_Future" => Some(Self::Future),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlateSetType {
    /// 所有板块
    All = 0,
    /// 行业板块
    Industry = 1,
    /// 地域板块,港美股市场的地域分类数据暂为空
    Region = 2,
    /// 概念板块
    Concept = 3,
    /// 其他板块, 仅用于3207（获取股票所属板块）协议返回,不可作为其他协议的请求参数
    Other = 4,
}
impl PlateSetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlateSetType::All => "PlateSetType_All",
            PlateSetType::Industry => "PlateSetType_Industry",
            PlateSetType::Region => "PlateSetType_Region",
            PlateSetType::Concept => "PlateSetType_Concept",
            PlateSetType::Other => "PlateSetType_Other",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PlateSetType_All" => Some(Self::All),
            "PlateSetType_Industry" => Some(Self::Industry),
            "PlateSetType_Region" => Some(Self::Region),
            "PlateSetType_Concept" => Some(Self::Concept),
            "PlateSetType_Other" => Some(Self::Other),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WarrantType {
    /// 未知
    Unknown = 0,
    /// 认购
    Buy = 1,
    /// 认沽
    Sell = 2,
    /// 牛
    Bull = 3,
    /// 熊
    Bear = 4,
    /// 界内证
    InLine = 5,
}
impl WarrantType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WarrantType::Unknown => "WarrantType_Unknown",
            WarrantType::Buy => "WarrantType_Buy",
            WarrantType::Sell => "WarrantType_Sell",
            WarrantType::Bull => "WarrantType_Bull",
            WarrantType::Bear => "WarrantType_Bear",
            WarrantType::InLine => "WarrantType_InLine",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "WarrantType_Unknown" => Some(Self::Unknown),
            "WarrantType_Buy" => Some(Self::Buy),
            "WarrantType_Sell" => Some(Self::Sell),
            "WarrantType_Bull" => Some(Self::Bull),
            "WarrantType_Bear" => Some(Self::Bear),
            "WarrantType_InLine" => Some(Self::InLine),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionType {
    /// 未知
    Unknown = 0,
    /// 涨
    Call = 1,
    /// 跌
    Put = 2,
}
impl OptionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OptionType::Unknown => "OptionType_Unknown",
            OptionType::Call => "OptionType_Call",
            OptionType::Put => "OptionType_Put",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OptionType_Unknown" => Some(Self::Unknown),
            "OptionType_Call" => Some(Self::Call),
            "OptionType_Put" => Some(Self::Put),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IndexOptionType {
    /// 未知
    Unknown = 0,
    /// 正常普通的指数期权
    Normal = 1,
    /// 小型指数期权
    Small = 2,
}
impl IndexOptionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IndexOptionType::Unknown => "IndexOptionType_Unknown",
            IndexOptionType::Normal => "IndexOptionType_Normal",
            IndexOptionType::Small => "IndexOptionType_Small",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IndexOptionType_Unknown" => Some(Self::Unknown),
            "IndexOptionType_Normal" => Some(Self::Normal),
            "IndexOptionType_Small" => Some(Self::Small),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionAreaType {
    /// 未知
    Unknown = 0,
    /// 美式
    American = 1,
    /// 欧式
    European = 2,
    /// 百慕大
    Bermuda = 3,
}
impl OptionAreaType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OptionAreaType::Unknown => "OptionAreaType_Unknown",
            OptionAreaType::American => "OptionAreaType_American",
            OptionAreaType::European => "OptionAreaType_European",
            OptionAreaType::Bermuda => "OptionAreaType_Bermuda",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OptionAreaType_Unknown" => Some(Self::Unknown),
            "OptionAreaType_American" => Some(Self::American),
            "OptionAreaType_European" => Some(Self::European),
            "OptionAreaType_Bermuda" => Some(Self::Bermuda),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QotMarketState {
    /// 无交易
    None = 0,
    /// 竞价
    Auction = 1,
    /// 早盘前等待开盘
    WaitingOpen = 2,
    /// 早盘
    Morning = 3,
    /// 午间休市
    Rest = 4,
    /// 午盘
    Afternoon = 5,
    /// 收盘
    Closed = 6,
    /// 盘前
    PreMarketBegin = 8,
    /// 盘前结束
    PreMarketEnd = 9,
    /// 盘后
    AfterHoursBegin = 10,
    /// 盘后结束	
    AfterHoursEnd = 11,
    /// 夜市开盘
    NightOpen = 13,
    /// 夜市收盘
    NightEnd = 14,
    /// 期货日市开盘
    FutureDayOpen = 15,
    /// 期货日市休市
    FutureDayBreak = 16,
    /// 期货日市收盘
    FutureDayClose = 17,
    /// 期货日市等待开盘
    FutureDayWaitForOpen = 18,
    /// 盘后竞价,港股市场增加CAS机制对应的市场状态	
    HkCas = 19,
    /// 夜市等待开盘（已废弃）
    FutureNightWait = 20,
    /// 期货下午开盘（已废弃）
    FutureAfternoon = 21,
    /// 美国期货新增加状态
    ///
    /// 期货切交易日（已废弃）
    FutureSwitchDate = 22,
    /// 期货开盘
    FutureOpen = 23,
    /// 期货中盘休息
    FutureBreak = 24,
    /// 期货休息后开盘
    FutureBreakOver = 25,
    /// 期货收盘
    FutureClose = 26,
    /// 科创板新增状态
    ///
    /// 科创板的盘后撮合时段（已废弃）
    StibAfterHoursWait = 27,
    /// 科创板的盘后交易开始（已废弃）
    StibAfterHoursBegin = 28,
    /// 科创板的盘后交易结束（已废弃）
    StibAfterHoursEnd = 29,
}
impl QotMarketState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            QotMarketState::None => "QotMarketState_None",
            QotMarketState::Auction => "QotMarketState_Auction",
            QotMarketState::WaitingOpen => "QotMarketState_WaitingOpen",
            QotMarketState::Morning => "QotMarketState_Morning",
            QotMarketState::Rest => "QotMarketState_Rest",
            QotMarketState::Afternoon => "QotMarketState_Afternoon",
            QotMarketState::Closed => "QotMarketState_Closed",
            QotMarketState::PreMarketBegin => "QotMarketState_PreMarketBegin",
            QotMarketState::PreMarketEnd => "QotMarketState_PreMarketEnd",
            QotMarketState::AfterHoursBegin => "QotMarketState_AfterHoursBegin",
            QotMarketState::AfterHoursEnd => "QotMarketState_AfterHoursEnd",
            QotMarketState::NightOpen => "QotMarketState_NightOpen",
            QotMarketState::NightEnd => "QotMarketState_NightEnd",
            QotMarketState::FutureDayOpen => "QotMarketState_FutureDayOpen",
            QotMarketState::FutureDayBreak => "QotMarketState_FutureDayBreak",
            QotMarketState::FutureDayClose => "QotMarketState_FutureDayClose",
            QotMarketState::FutureDayWaitForOpen => "QotMarketState_FutureDayWaitForOpen",
            QotMarketState::HkCas => "QotMarketState_HkCas",
            QotMarketState::FutureNightWait => "QotMarketState_FutureNightWait",
            QotMarketState::FutureAfternoon => "QotMarketState_FutureAfternoon",
            QotMarketState::FutureSwitchDate => "QotMarketState_FutureSwitchDate",
            QotMarketState::FutureOpen => "QotMarketState_FutureOpen",
            QotMarketState::FutureBreak => "QotMarketState_FutureBreak",
            QotMarketState::FutureBreakOver => "QotMarketState_FutureBreakOver",
            QotMarketState::FutureClose => "QotMarketState_FutureClose",
            QotMarketState::StibAfterHoursWait => "QotMarketState_StibAfterHoursWait",
            QotMarketState::StibAfterHoursBegin => "QotMarketState_StibAfterHoursBegin",
            QotMarketState::StibAfterHoursEnd => "QotMarketState_StibAfterHoursEnd",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "QotMarketState_None" => Some(Self::None),
            "QotMarketState_Auction" => Some(Self::Auction),
            "QotMarketState_WaitingOpen" => Some(Self::WaitingOpen),
            "QotMarketState_Morning" => Some(Self::Morning),
            "QotMarketState_Rest" => Some(Self::Rest),
            "QotMarketState_Afternoon" => Some(Self::Afternoon),
            "QotMarketState_Closed" => Some(Self::Closed),
            "QotMarketState_PreMarketBegin" => Some(Self::PreMarketBegin),
            "QotMarketState_PreMarketEnd" => Some(Self::PreMarketEnd),
            "QotMarketState_AfterHoursBegin" => Some(Self::AfterHoursBegin),
            "QotMarketState_AfterHoursEnd" => Some(Self::AfterHoursEnd),
            "QotMarketState_NightOpen" => Some(Self::NightOpen),
            "QotMarketState_NightEnd" => Some(Self::NightEnd),
            "QotMarketState_FutureDayOpen" => Some(Self::FutureDayOpen),
            "QotMarketState_FutureDayBreak" => Some(Self::FutureDayBreak),
            "QotMarketState_FutureDayClose" => Some(Self::FutureDayClose),
            "QotMarketState_FutureDayWaitForOpen" => Some(Self::FutureDayWaitForOpen),
            "QotMarketState_HkCas" => Some(Self::HkCas),
            "QotMarketState_FutureNightWait" => Some(Self::FutureNightWait),
            "QotMarketState_FutureAfternoon" => Some(Self::FutureAfternoon),
            "QotMarketState_FutureSwitchDate" => Some(Self::FutureSwitchDate),
            "QotMarketState_FutureOpen" => Some(Self::FutureOpen),
            "QotMarketState_FutureBreak" => Some(Self::FutureBreak),
            "QotMarketState_FutureBreakOver" => Some(Self::FutureBreakOver),
            "QotMarketState_FutureClose" => Some(Self::FutureClose),
            "QotMarketState_StibAfterHoursWait" => Some(Self::StibAfterHoursWait),
            "QotMarketState_StibAfterHoursBegin" => Some(Self::StibAfterHoursBegin),
            "QotMarketState_StibAfterHoursEnd" => Some(Self::StibAfterHoursEnd),
            _ => None,
        }
    }
}
/// 交易日查询市场
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeDateMarket {
    /// 未知
    Unknown = 0,
    /// 港股市场
    Hk = 1,
    /// 美股市场
    Us = 2,
    /// A股市场
    Cn = 3,
    /// 深（沪）股通
    Nt = 4,
    /// 港股通（深、沪）
    St = 5,
    /// 日本期货
    JpFuture = 6,
    /// 新加坡期货
    SgFuture = 7,
}
impl TradeDateMarket {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TradeDateMarket::Unknown => "TradeDateMarket_Unknown",
            TradeDateMarket::Hk => "TradeDateMarket_HK",
            TradeDateMarket::Us => "TradeDateMarket_US",
            TradeDateMarket::Cn => "TradeDateMarket_CN",
            TradeDateMarket::Nt => "TradeDateMarket_NT",
            TradeDateMarket::St => "TradeDateMarket_ST",
            TradeDateMarket::JpFuture => "TradeDateMarket_JP_Future",
            TradeDateMarket::SgFuture => "TradeDateMarket_SG_Future",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TradeDateMarket_Unknown" => Some(Self::Unknown),
            "TradeDateMarket_HK" => Some(Self::Hk),
            "TradeDateMarket_US" => Some(Self::Us),
            "TradeDateMarket_CN" => Some(Self::Cn),
            "TradeDateMarket_NT" => Some(Self::Nt),
            "TradeDateMarket_ST" => Some(Self::St),
            "TradeDateMarket_JP_Future" => Some(Self::JpFuture),
            "TradeDateMarket_SG_Future" => Some(Self::SgFuture),
            _ => None,
        }
    }
}
/// 交易日类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeDateType {
    /// 全天交易
    Whole = 0,
    /// 上午交易，下午休市
    Morning = 1,
    /// 下午交易，上午休市
    Afternoon = 2,
}
impl TradeDateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TradeDateType::Whole => "TradeDateType_Whole",
            TradeDateType::Morning => "TradeDateType_Morning",
            TradeDateType::Afternoon => "TradeDateType_Afternoon",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TradeDateType_Whole" => Some(Self::Whole),
            "TradeDateType_Morning" => Some(Self::Morning),
            "TradeDateType_Afternoon" => Some(Self::Afternoon),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RehabType {
    /// 不复权
    None = 0,
    /// 前复权
    Forward = 1,
    /// 后复权
    Backward = 2,
}
impl RehabType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RehabType::None => "RehabType_None",
            RehabType::Forward => "RehabType_Forward",
            RehabType::Backward => "RehabType_Backward",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RehabType_None" => Some(Self::None),
            "RehabType_Forward" => Some(Self::Forward),
            "RehabType_Backward" => Some(Self::Backward),
            _ => None,
        }
    }
}
/// 枚举值兼容旧协议定义
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KlType {
    /// 未知
    Unknown = 0,
    /// 1分K
    KlType1min = 1,
    /// 日K
    Day = 2,
    /// 周K
    Week = 3,
    /// 月K	
    Month = 4,
    /// 年K
    Year = 5,
    /// 5分K
    KlType5min = 6,
    /// 15分K
    KlType15min = 7,
    /// 30分K
    KlType30min = 8,
    /// 60分K		
    KlType60min = 9,
    /// 3分K
    KlType3min = 10,
    /// 季K
    Quarter = 11,
}
impl KlType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KlType::Unknown => "KLType_Unknown",
            KlType::KlType1min => "KLType_1Min",
            KlType::Day => "KLType_Day",
            KlType::Week => "KLType_Week",
            KlType::Month => "KLType_Month",
            KlType::Year => "KLType_Year",
            KlType::KlType5min => "KLType_5Min",
            KlType::KlType15min => "KLType_15Min",
            KlType::KlType30min => "KLType_30Min",
            KlType::KlType60min => "KLType_60Min",
            KlType::KlType3min => "KLType_3Min",
            KlType::Quarter => "KLType_Quarter",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "KLType_Unknown" => Some(Self::Unknown),
            "KLType_1Min" => Some(Self::KlType1min),
            "KLType_Day" => Some(Self::Day),
            "KLType_Week" => Some(Self::Week),
            "KLType_Month" => Some(Self::Month),
            "KLType_Year" => Some(Self::Year),
            "KLType_5Min" => Some(Self::KlType5min),
            "KLType_15Min" => Some(Self::KlType15min),
            "KLType_30Min" => Some(Self::KlType30min),
            "KLType_60Min" => Some(Self::KlType60min),
            "KLType_3Min" => Some(Self::KlType3min),
            "KLType_Quarter" => Some(Self::Quarter),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KlFields {
    ///
    None = 0,
    /// 最高价
    High = 1,
    /// 开盘价
    Open = 2,
    /// 最低价
    Low = 4,
    /// 收盘价
    Close = 8,
    /// 昨收价
    LastClose = 16,
    /// 成交量
    Volume = 32,
    /// 成交额
    Turnover = 64,
    /// 换手率
    TurnoverRate = 128,
    /// 市盈率
    Pe = 256,
    /// 涨跌幅
    ChangeRate = 512,
}
impl KlFields {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KlFields::None => "KLFields_None",
            KlFields::High => "KLFields_High",
            KlFields::Open => "KLFields_Open",
            KlFields::Low => "KLFields_Low",
            KlFields::Close => "KLFields_Close",
            KlFields::LastClose => "KLFields_LastClose",
            KlFields::Volume => "KLFields_Volume",
            KlFields::Turnover => "KLFields_Turnover",
            KlFields::TurnoverRate => "KLFields_TurnoverRate",
            KlFields::Pe => "KLFields_PE",
            KlFields::ChangeRate => "KLFields_ChangeRate",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "KLFields_None" => Some(Self::None),
            "KLFields_High" => Some(Self::High),
            "KLFields_Open" => Some(Self::Open),
            "KLFields_Low" => Some(Self::Low),
            "KLFields_Close" => Some(Self::Close),
            "KLFields_LastClose" => Some(Self::LastClose),
            "KLFields_Volume" => Some(Self::Volume),
            "KLFields_Turnover" => Some(Self::Turnover),
            "KLFields_TurnoverRate" => Some(Self::TurnoverRate),
            "KLFields_PE" => Some(Self::Pe),
            "KLFields_ChangeRate" => Some(Self::ChangeRate),
            _ => None,
        }
    }
}
/// 订阅类型
/// 枚举值兼容旧协议定义
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubType {
    None = 0,
    /// 基础报价
    Basic = 1,
    /// 摆盘
    OrderBook = 2,
    /// 逐笔
    Ticker = 4,
    /// 分时
    Rt = 5,
    /// 日K
    KlDay = 6,
    /// 5分K
    Kl5min = 7,
    /// 15分K
    Kl15min = 8,
    /// 30分K
    Kl30min = 9,
    /// 60分K
    Kl60min = 10,
    /// 1分K
    Kl1min = 11,
    /// 周K
    KlWeek = 12,
    /// 月K
    KlMonth = 13,
    /// 经纪队列
    Broker = 14,
    /// 季K
    KlQurater = 15,
    /// 年K
    KlYear = 16,
    /// 3分K
    Kl3min = 17,
}
impl SubType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubType::None => "SubType_None",
            SubType::Basic => "SubType_Basic",
            SubType::OrderBook => "SubType_OrderBook",
            SubType::Ticker => "SubType_Ticker",
            SubType::Rt => "SubType_RT",
            SubType::KlDay => "SubType_KL_Day",
            SubType::Kl5min => "SubType_KL_5Min",
            SubType::Kl15min => "SubType_KL_15Min",
            SubType::Kl30min => "SubType_KL_30Min",
            SubType::Kl60min => "SubType_KL_60Min",
            SubType::Kl1min => "SubType_KL_1Min",
            SubType::KlWeek => "SubType_KL_Week",
            SubType::KlMonth => "SubType_KL_Month",
            SubType::Broker => "SubType_Broker",
            SubType::KlQurater => "SubType_KL_Qurater",
            SubType::KlYear => "SubType_KL_Year",
            SubType::Kl3min => "SubType_KL_3Min",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SubType_None" => Some(Self::None),
            "SubType_Basic" => Some(Self::Basic),
            "SubType_OrderBook" => Some(Self::OrderBook),
            "SubType_Ticker" => Some(Self::Ticker),
            "SubType_RT" => Some(Self::Rt),
            "SubType_KL_Day" => Some(Self::KlDay),
            "SubType_KL_5Min" => Some(Self::Kl5min),
            "SubType_KL_15Min" => Some(Self::Kl15min),
            "SubType_KL_30Min" => Some(Self::Kl30min),
            "SubType_KL_60Min" => Some(Self::Kl60min),
            "SubType_KL_1Min" => Some(Self::Kl1min),
            "SubType_KL_Week" => Some(Self::KlWeek),
            "SubType_KL_Month" => Some(Self::KlMonth),
            "SubType_Broker" => Some(Self::Broker),
            "SubType_KL_Qurater" => Some(Self::KlQurater),
            "SubType_KL_Year" => Some(Self::KlYear),
            "SubType_KL_3Min" => Some(Self::Kl3min),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TickerDirection {
    /// 未知
    Unknown = 0,
    /// 外盘
    Bid = 1,
    /// 内盘
    Ask = 2,
    /// 中性盘
    Neutral = 3,
}
impl TickerDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TickerDirection::Unknown => "TickerDirection_Unknown",
            TickerDirection::Bid => "TickerDirection_Bid",
            TickerDirection::Ask => "TickerDirection_Ask",
            TickerDirection::Neutral => "TickerDirection_Neutral",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TickerDirection_Unknown" => Some(Self::Unknown),
            "TickerDirection_Bid" => Some(Self::Bid),
            "TickerDirection_Ask" => Some(Self::Ask),
            "TickerDirection_Neutral" => Some(Self::Neutral),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TickerType {
    /// 未知
    Unknown = 0,
    /// 自动对盘
    Automatch = 1,
    /// 开市前成交盘
    Late = 2,
    /// 非自动对盘
    NoneAutomatch = 3,
    /// 同一证券商自动对盘
    InterAutomatch = 4,
    /// 同一证券商非自动对盘
    InterNoneAutomatch = 5,
    /// 碎股交易
    OddLot = 6,
    /// 竞价交易	
    Auction = 7,
    /// 批量交易
    Bulk = 8,
    /// 现金交易
    Crash = 9,
    /// 跨市场交易
    CrossMarket = 10,
    /// 批量卖出
    BulkSold = 11,
    /// 离价交易
    FreeOnBoard = 12,
    /// 第127条交易（纽交所规则）或第155条交易
    Rule127Or155 = 13,
    /// 延迟交易
    Delay = 14,
    /// 中央收市价
    MarketCenterClosePrice = 15,
    /// 隔日交易
    NextDay = 16,
    /// 中央开盘价交易
    MarketCenterOpening = 17,
    /// 前参考价
    PriorReferencePrice = 18,
    /// 中央开盘价
    MarketCenterOpenPrice = 19,
    /// 卖方
    Seller = 20,
    /// T类交易(盘前和盘后交易)
    T = 21,
    /// 延长交易时段
    ExtendedTradingHours = 22,
    /// 合单交易
    Contingent = 23,
    /// 平均价成交
    AvgPrice = 24,
    /// 场外售出
    OtcSold = 25,
    /// 碎股跨市场交易
    OddLotCrossMarket = 26,
    /// 衍生工具定价
    DerivativelyPriced = 27,
    /// 再开盘定价
    ReOpeningPriced = 28,
    /// 收盘定价
    ClosingPriced = 29,
    /// 综合延迟价格
    ComprehensiveDelayPrice = 30,
    /// 交易的一方不是香港交易所的成员，属于场外交易
    Overseas = 31,
}
impl TickerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TickerType::Unknown => "TickerType_Unknown",
            TickerType::Automatch => "TickerType_Automatch",
            TickerType::Late => "TickerType_Late",
            TickerType::NoneAutomatch => "TickerType_NoneAutomatch",
            TickerType::InterAutomatch => "TickerType_InterAutomatch",
            TickerType::InterNoneAutomatch => "TickerType_InterNoneAutomatch",
            TickerType::OddLot => "TickerType_OddLot",
            TickerType::Auction => "TickerType_Auction",
            TickerType::Bulk => "TickerType_Bulk",
            TickerType::Crash => "TickerType_Crash",
            TickerType::CrossMarket => "TickerType_CrossMarket",
            TickerType::BulkSold => "TickerType_BulkSold",
            TickerType::FreeOnBoard => "TickerType_FreeOnBoard",
            TickerType::Rule127Or155 => "TickerType_Rule127Or155",
            TickerType::Delay => "TickerType_Delay",
            TickerType::MarketCenterClosePrice => "TickerType_MarketCenterClosePrice",
            TickerType::NextDay => "TickerType_NextDay",
            TickerType::MarketCenterOpening => "TickerType_MarketCenterOpening",
            TickerType::PriorReferencePrice => "TickerType_PriorReferencePrice",
            TickerType::MarketCenterOpenPrice => "TickerType_MarketCenterOpenPrice",
            TickerType::Seller => "TickerType_Seller",
            TickerType::T => "TickerType_T",
            TickerType::ExtendedTradingHours => "TickerType_ExtendedTradingHours",
            TickerType::Contingent => "TickerType_Contingent",
            TickerType::AvgPrice => "TickerType_AvgPrice",
            TickerType::OtcSold => "TickerType_OTCSold",
            TickerType::OddLotCrossMarket => "TickerType_OddLotCrossMarket",
            TickerType::DerivativelyPriced => "TickerType_DerivativelyPriced",
            TickerType::ReOpeningPriced => "TickerType_ReOpeningPriced",
            TickerType::ClosingPriced => "TickerType_ClosingPriced",
            TickerType::ComprehensiveDelayPrice => "TickerType_ComprehensiveDelayPrice",
            TickerType::Overseas => "TickerType_Overseas",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TickerType_Unknown" => Some(Self::Unknown),
            "TickerType_Automatch" => Some(Self::Automatch),
            "TickerType_Late" => Some(Self::Late),
            "TickerType_NoneAutomatch" => Some(Self::NoneAutomatch),
            "TickerType_InterAutomatch" => Some(Self::InterAutomatch),
            "TickerType_InterNoneAutomatch" => Some(Self::InterNoneAutomatch),
            "TickerType_OddLot" => Some(Self::OddLot),
            "TickerType_Auction" => Some(Self::Auction),
            "TickerType_Bulk" => Some(Self::Bulk),
            "TickerType_Crash" => Some(Self::Crash),
            "TickerType_CrossMarket" => Some(Self::CrossMarket),
            "TickerType_BulkSold" => Some(Self::BulkSold),
            "TickerType_FreeOnBoard" => Some(Self::FreeOnBoard),
            "TickerType_Rule127Or155" => Some(Self::Rule127Or155),
            "TickerType_Delay" => Some(Self::Delay),
            "TickerType_MarketCenterClosePrice" => Some(Self::MarketCenterClosePrice),
            "TickerType_NextDay" => Some(Self::NextDay),
            "TickerType_MarketCenterOpening" => Some(Self::MarketCenterOpening),
            "TickerType_PriorReferencePrice" => Some(Self::PriorReferencePrice),
            "TickerType_MarketCenterOpenPrice" => Some(Self::MarketCenterOpenPrice),
            "TickerType_Seller" => Some(Self::Seller),
            "TickerType_T" => Some(Self::T),
            "TickerType_ExtendedTradingHours" => Some(Self::ExtendedTradingHours),
            "TickerType_Contingent" => Some(Self::Contingent),
            "TickerType_AvgPrice" => Some(Self::AvgPrice),
            "TickerType_OTCSold" => Some(Self::OtcSold),
            "TickerType_OddLotCrossMarket" => Some(Self::OddLotCrossMarket),
            "TickerType_DerivativelyPriced" => Some(Self::DerivativelyPriced),
            "TickerType_ReOpeningPriced" => Some(Self::ReOpeningPriced),
            "TickerType_ClosingPriced" => Some(Self::ClosingPriced),
            "TickerType_ComprehensiveDelayPrice" => Some(Self::ComprehensiveDelayPrice),
            "TickerType_Overseas" => Some(Self::Overseas),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DarkStatus {
    /// 无暗盘交易
    None = 0,
    /// 暗盘交易中
    Trading = 1,
    /// 暗盘交易结束
    End = 2,
}
impl DarkStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DarkStatus::None => "DarkStatus_None",
            DarkStatus::Trading => "DarkStatus_Trading",
            DarkStatus::End => "DarkStatus_End",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DarkStatus_None" => Some(Self::None),
            "DarkStatus_Trading" => Some(Self::Trading),
            "DarkStatus_End" => Some(Self::End),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityStatus {
    /// 未知
    Unknown = 0,
    /// 正常状态
    Normal = 1,
    /// 待上市
    Listing = 2,
    /// 申购中
    Purchasing = 3,
    /// 认购中
    Subscribing = 4,
    /// 暗盘开盘前
    BeforeDrakTradeOpening = 5,
    /// 暗盘交易中
    DrakTrading = 6,
    /// 暗盘已收盘
    DrakTradeEnd = 7,
    /// 待开盘
    ToBeOpen = 8,
    /// 停牌
    Suspended = 9,
    /// 已收回
    Called = 10,
    /// 已过最后交易日
    ExpiredLastTradingDate = 11,
    /// 已过期
    Expired = 12,
    /// 已退市
    Delisted = 13,
    /// 公司行动中，交易关闭，转至临时代码交易
    ChangeToTemporaryCode = 14,
    /// 临时买卖结束，交易关闭
    TemporaryCodeTradeEnd = 15,
    /// 已转板，旧代码交易关闭
    ChangedPlateTradeEnd = 16,
    /// 已换代码，旧代码交易关闭
    ChangedCodeTradeEnd = 17,
    /// 可恢复性熔断
    RecoverableCircuitBreaker = 18,
    /// 不可恢复性熔断
    UnRecoverableCircuitBreaker = 19,
    /// 盘后撮合
    AfterCombination = 20,
    /// 盘后交易
    AfterTransation = 21,
}
impl SecurityStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SecurityStatus::Unknown => "SecurityStatus_Unknown",
            SecurityStatus::Normal => "SecurityStatus_Normal",
            SecurityStatus::Listing => "SecurityStatus_Listing",
            SecurityStatus::Purchasing => "SecurityStatus_Purchasing",
            SecurityStatus::Subscribing => "SecurityStatus_Subscribing",
            SecurityStatus::BeforeDrakTradeOpening => {
                "SecurityStatus_BeforeDrakTradeOpening"
            }
            SecurityStatus::DrakTrading => "SecurityStatus_DrakTrading",
            SecurityStatus::DrakTradeEnd => "SecurityStatus_DrakTradeEnd",
            SecurityStatus::ToBeOpen => "SecurityStatus_ToBeOpen",
            SecurityStatus::Suspended => "SecurityStatus_Suspended",
            SecurityStatus::Called => "SecurityStatus_Called",
            SecurityStatus::ExpiredLastTradingDate => {
                "SecurityStatus_ExpiredLastTradingDate"
            }
            SecurityStatus::Expired => "SecurityStatus_Expired",
            SecurityStatus::Delisted => "SecurityStatus_Delisted",
            SecurityStatus::ChangeToTemporaryCode => {
                "SecurityStatus_ChangeToTemporaryCode"
            }
            SecurityStatus::TemporaryCodeTradeEnd => {
                "SecurityStatus_TemporaryCodeTradeEnd"
            }
            SecurityStatus::ChangedPlateTradeEnd => "SecurityStatus_ChangedPlateTradeEnd",
            SecurityStatus::ChangedCodeTradeEnd => "SecurityStatus_ChangedCodeTradeEnd",
            SecurityStatus::RecoverableCircuitBreaker => {
                "SecurityStatus_RecoverableCircuitBreaker"
            }
            SecurityStatus::UnRecoverableCircuitBreaker => {
                "SecurityStatus_UnRecoverableCircuitBreaker"
            }
            SecurityStatus::AfterCombination => "SecurityStatus_AfterCombination",
            SecurityStatus::AfterTransation => "SecurityStatus_AfterTransation",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SecurityStatus_Unknown" => Some(Self::Unknown),
            "SecurityStatus_Normal" => Some(Self::Normal),
            "SecurityStatus_Listing" => Some(Self::Listing),
            "SecurityStatus_Purchasing" => Some(Self::Purchasing),
            "SecurityStatus_Subscribing" => Some(Self::Subscribing),
            "SecurityStatus_BeforeDrakTradeOpening" => Some(Self::BeforeDrakTradeOpening),
            "SecurityStatus_DrakTrading" => Some(Self::DrakTrading),
            "SecurityStatus_DrakTradeEnd" => Some(Self::DrakTradeEnd),
            "SecurityStatus_ToBeOpen" => Some(Self::ToBeOpen),
            "SecurityStatus_Suspended" => Some(Self::Suspended),
            "SecurityStatus_Called" => Some(Self::Called),
            "SecurityStatus_ExpiredLastTradingDate" => Some(Self::ExpiredLastTradingDate),
            "SecurityStatus_Expired" => Some(Self::Expired),
            "SecurityStatus_Delisted" => Some(Self::Delisted),
            "SecurityStatus_ChangeToTemporaryCode" => Some(Self::ChangeToTemporaryCode),
            "SecurityStatus_TemporaryCodeTradeEnd" => Some(Self::TemporaryCodeTradeEnd),
            "SecurityStatus_ChangedPlateTradeEnd" => Some(Self::ChangedPlateTradeEnd),
            "SecurityStatus_ChangedCodeTradeEnd" => Some(Self::ChangedCodeTradeEnd),
            "SecurityStatus_RecoverableCircuitBreaker" => {
                Some(Self::RecoverableCircuitBreaker)
            }
            "SecurityStatus_UnRecoverableCircuitBreaker" => {
                Some(Self::UnRecoverableCircuitBreaker)
            }
            "SecurityStatus_AfterCombination" => Some(Self::AfterCombination),
            "SecurityStatus_AfterTransation" => Some(Self::AfterTransation),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HolderCategory {
    /// 未知
    Unknow = 0,
    /// 机构
    Agency = 1,
    /// 基金
    Fund = 2,
    /// 高管
    SeniorManager = 3,
}
impl HolderCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HolderCategory::Unknow => "HolderCategory_Unknow",
            HolderCategory::Agency => "HolderCategory_Agency",
            HolderCategory::Fund => "HolderCategory_Fund",
            HolderCategory::SeniorManager => "HolderCategory_SeniorManager",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HolderCategory_Unknow" => Some(Self::Unknow),
            "HolderCategory_Agency" => Some(Self::Agency),
            "HolderCategory_Fund" => Some(Self::Fund),
            "HolderCategory_SeniorManager" => Some(Self::SeniorManager),
            _ => None,
        }
    }
}
/// 推送数据的分类，目前只有逐笔在使用
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PushDataType {
    Unknow = 0,
    /// 实时推送的数据
    Realtime = 1,
    /// 对后台行情连接断开期间拉取补充的数据 最多50个
    ByDisConn = 2,
    /// 非实时非连接断开补充数据
    Cache = 3,
}
impl PushDataType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PushDataType::Unknow => "PushDataType_Unknow",
            PushDataType::Realtime => "PushDataType_Realtime",
            PushDataType::ByDisConn => "PushDataType_ByDisConn",
            PushDataType::Cache => "PushDataType_Cache",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PushDataType_Unknow" => Some(Self::Unknow),
            "PushDataType_Realtime" => Some(Self::Realtime),
            "PushDataType_ByDisConn" => Some(Self::ByDisConn),
            "PushDataType_Cache" => Some(Self::Cache),
            _ => None,
        }
    }
}
/// 窝轮排序
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SortField {
    Unknow = 0,
    /// 代码
    Code = 1,
    /// 最新价
    CurPrice = 2,
    /// 涨跌额
    PriceChangeVal = 3,
    /// 涨跌幅%
    ChangeRate = 4,
    /// 状态
    Status = 5,
    /// 买入价
    BidPrice = 6,
    /// 卖出价
    AskPrice = 7,
    /// 买量
    BidVol = 8,
    /// 卖量
    AskVol = 9,
    /// 成交量
    Volume = 10,
    /// 成交额
    Turnover = 11,
    /// 振幅%
    Amplitude = 30,
    /// 以下排序字段只支持用于Qot_GetWarrant协议
    ///
    /// 综合评分
    Score = 12,
    /// 溢价%
    Premium = 13,
    /// 有效杠杆
    EffectiveLeverage = 14,
    /// 对冲值,仅认购认沽支持该字段
    Delta = 15,
    /// 引伸波幅,仅认购认沽支持该字段
    ImpliedVolatility = 16,
    /// 类型
    Type = 17,
    /// 行权价
    StrikePrice = 18,
    /// 打和点
    BreakEvenPoint = 19,
    /// 到期日
    MaturityTime = 20,
    /// 上市日期
    ListTime = 21,
    /// 最后交易日
    LastTradeTime = 22,
    /// 杠杆比率
    Leverage = 23,
    /// 价内/价外%
    InOutMoney = 24,
    /// 收回价,仅牛熊证支持该字段
    RecoveryPrice = 25,
    /// 换股价
    ChangePrice = 26,
    /// 换股比率
    Change = 27,
    /// 街货比%
    StreetRate = 28,
    /// 街货量
    StreetVol = 29,
    /// 窝轮名称
    WarrantName = 31,
    /// 发行人
    Issuer = 32,
    /// 每手
    LotSize = 33,
    /// 发行量
    IssueSize = 34,
    /// 上限价，仅用于界内证
    UpperStrikePrice = 45,
    /// 下限价，仅用于界内证
    LowerStrikePrice = 46,
    /// 界内界外，仅用于界内证
    InLinePriceStatus = 47,
    /// 以下排序字段只支持用于Qot_GetPlateSecurity协议，并仅支持美股
    ///
    /// 盘前最新价
    PreCurPrice = 35,
    /// 盘后最新价
    AfterCurPrice = 36,
    /// 盘前涨跌额
    PrePriceChangeVal = 37,
    /// 盘后涨跌额
    AfterPriceChangeVal = 38,
    /// 盘前涨跌幅%
    PreChangeRate = 39,
    /// 盘后涨跌幅%
    AfterChangeRate = 40,
    /// 盘前振幅%
    PreAmplitude = 41,
    /// 盘后振幅%
    AfterAmplitude = 42,
    /// 盘前成交额
    PreTurnover = 43,
    /// 盘后成交额
    AfterTurnover = 44,
    /// 以下排序字段只支持用于Qot_GetPlateSecurity协议，并仅支持期货
    ///
    /// 昨结
    LastSettlePrice = 48,
    /// 持仓量
    Position = 49,
    /// 日增仓
    PositionChange = 50,
}
impl SortField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SortField::Unknow => "SortField_Unknow",
            SortField::Code => "SortField_Code",
            SortField::CurPrice => "SortField_CurPrice",
            SortField::PriceChangeVal => "SortField_PriceChangeVal",
            SortField::ChangeRate => "SortField_ChangeRate",
            SortField::Status => "SortField_Status",
            SortField::BidPrice => "SortField_BidPrice",
            SortField::AskPrice => "SortField_AskPrice",
            SortField::BidVol => "SortField_BidVol",
            SortField::AskVol => "SortField_AskVol",
            SortField::Volume => "SortField_Volume",
            SortField::Turnover => "SortField_Turnover",
            SortField::Amplitude => "SortField_Amplitude",
            SortField::Score => "SortField_Score",
            SortField::Premium => "SortField_Premium",
            SortField::EffectiveLeverage => "SortField_EffectiveLeverage",
            SortField::Delta => "SortField_Delta",
            SortField::ImpliedVolatility => "SortField_ImpliedVolatility",
            SortField::Type => "SortField_Type",
            SortField::StrikePrice => "SortField_StrikePrice",
            SortField::BreakEvenPoint => "SortField_BreakEvenPoint",
            SortField::MaturityTime => "SortField_MaturityTime",
            SortField::ListTime => "SortField_ListTime",
            SortField::LastTradeTime => "SortField_LastTradeTime",
            SortField::Leverage => "SortField_Leverage",
            SortField::InOutMoney => "SortField_InOutMoney",
            SortField::RecoveryPrice => "SortField_RecoveryPrice",
            SortField::ChangePrice => "SortField_ChangePrice",
            SortField::Change => "SortField_Change",
            SortField::StreetRate => "SortField_StreetRate",
            SortField::StreetVol => "SortField_StreetVol",
            SortField::WarrantName => "SortField_WarrantName",
            SortField::Issuer => "SortField_Issuer",
            SortField::LotSize => "SortField_LotSize",
            SortField::IssueSize => "SortField_IssueSize",
            SortField::UpperStrikePrice => "SortField_UpperStrikePrice",
            SortField::LowerStrikePrice => "SortField_LowerStrikePrice",
            SortField::InLinePriceStatus => "SortField_InLinePriceStatus",
            SortField::PreCurPrice => "SortField_PreCurPrice",
            SortField::AfterCurPrice => "SortField_AfterCurPrice",
            SortField::PrePriceChangeVal => "SortField_PrePriceChangeVal",
            SortField::AfterPriceChangeVal => "SortField_AfterPriceChangeVal",
            SortField::PreChangeRate => "SortField_PreChangeRate",
            SortField::AfterChangeRate => "SortField_AfterChangeRate",
            SortField::PreAmplitude => "SortField_PreAmplitude",
            SortField::AfterAmplitude => "SortField_AfterAmplitude",
            SortField::PreTurnover => "SortField_PreTurnover",
            SortField::AfterTurnover => "SortField_AfterTurnover",
            SortField::LastSettlePrice => "SortField_LastSettlePrice",
            SortField::Position => "SortField_Position",
            SortField::PositionChange => "SortField_PositionChange",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SortField_Unknow" => Some(Self::Unknow),
            "SortField_Code" => Some(Self::Code),
            "SortField_CurPrice" => Some(Self::CurPrice),
            "SortField_PriceChangeVal" => Some(Self::PriceChangeVal),
            "SortField_ChangeRate" => Some(Self::ChangeRate),
            "SortField_Status" => Some(Self::Status),
            "SortField_BidPrice" => Some(Self::BidPrice),
            "SortField_AskPrice" => Some(Self::AskPrice),
            "SortField_BidVol" => Some(Self::BidVol),
            "SortField_AskVol" => Some(Self::AskVol),
            "SortField_Volume" => Some(Self::Volume),
            "SortField_Turnover" => Some(Self::Turnover),
            "SortField_Amplitude" => Some(Self::Amplitude),
            "SortField_Score" => Some(Self::Score),
            "SortField_Premium" => Some(Self::Premium),
            "SortField_EffectiveLeverage" => Some(Self::EffectiveLeverage),
            "SortField_Delta" => Some(Self::Delta),
            "SortField_ImpliedVolatility" => Some(Self::ImpliedVolatility),
            "SortField_Type" => Some(Self::Type),
            "SortField_StrikePrice" => Some(Self::StrikePrice),
            "SortField_BreakEvenPoint" => Some(Self::BreakEvenPoint),
            "SortField_MaturityTime" => Some(Self::MaturityTime),
            "SortField_ListTime" => Some(Self::ListTime),
            "SortField_LastTradeTime" => Some(Self::LastTradeTime),
            "SortField_Leverage" => Some(Self::Leverage),
            "SortField_InOutMoney" => Some(Self::InOutMoney),
            "SortField_RecoveryPrice" => Some(Self::RecoveryPrice),
            "SortField_ChangePrice" => Some(Self::ChangePrice),
            "SortField_Change" => Some(Self::Change),
            "SortField_StreetRate" => Some(Self::StreetRate),
            "SortField_StreetVol" => Some(Self::StreetVol),
            "SortField_WarrantName" => Some(Self::WarrantName),
            "SortField_Issuer" => Some(Self::Issuer),
            "SortField_LotSize" => Some(Self::LotSize),
            "SortField_IssueSize" => Some(Self::IssueSize),
            "SortField_UpperStrikePrice" => Some(Self::UpperStrikePrice),
            "SortField_LowerStrikePrice" => Some(Self::LowerStrikePrice),
            "SortField_InLinePriceStatus" => Some(Self::InLinePriceStatus),
            "SortField_PreCurPrice" => Some(Self::PreCurPrice),
            "SortField_AfterCurPrice" => Some(Self::AfterCurPrice),
            "SortField_PrePriceChangeVal" => Some(Self::PrePriceChangeVal),
            "SortField_AfterPriceChangeVal" => Some(Self::AfterPriceChangeVal),
            "SortField_PreChangeRate" => Some(Self::PreChangeRate),
            "SortField_AfterChangeRate" => Some(Self::AfterChangeRate),
            "SortField_PreAmplitude" => Some(Self::PreAmplitude),
            "SortField_AfterAmplitude" => Some(Self::AfterAmplitude),
            "SortField_PreTurnover" => Some(Self::PreTurnover),
            "SortField_AfterTurnover" => Some(Self::AfterTurnover),
            "SortField_LastSettlePrice" => Some(Self::LastSettlePrice),
            "SortField_Position" => Some(Self::Position),
            "SortField_PositionChange" => Some(Self::PositionChange),
            _ => None,
        }
    }
}
/// 窝轮发行人
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Issuer {
    /// 未知
    Unknow = 0,
    /// 法兴
    Sg = 1,
    /// 法巴
    Bp = 2,
    /// 瑞信
    Cs = 3,
    /// 花旗	
    Ct = 4,
    /// 东亚
    Ea = 5,
    /// 高盛
    Gs = 6,
    /// 汇丰
    Hs = 7,
    /// 摩通	
    Jp = 8,
    /// 麦银	
    Mb = 9,
    /// 渣打
    Sc = 10,
    /// 瑞银
    Ub = 11,
    /// 中银
    Bi = 12,
    /// 德银
    Db = 13,
    /// 大和
    Dc = 14,
    /// 美林
    Ml = 15,
    /// 野村
    Nm = 16,
    /// 荷合
    Rb = 17,
    /// 苏皇	
    Rs = 18,
    /// 巴克莱
    Bc = 19,
    /// 海通
    Ht = 20,
    /// 瑞通
    Vt = 21,
    /// 比联
    Kc = 22,
    /// 摩利
    Ms = 23,
    /// 国君
    Gj = 24,
    /// 星展
    Xz = 25,
}
impl Issuer {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Issuer::Unknow => "Issuer_Unknow",
            Issuer::Sg => "Issuer_SG",
            Issuer::Bp => "Issuer_BP",
            Issuer::Cs => "Issuer_CS",
            Issuer::Ct => "Issuer_CT",
            Issuer::Ea => "Issuer_EA",
            Issuer::Gs => "Issuer_GS",
            Issuer::Hs => "Issuer_HS",
            Issuer::Jp => "Issuer_JP",
            Issuer::Mb => "Issuer_MB",
            Issuer::Sc => "Issuer_SC",
            Issuer::Ub => "Issuer_UB",
            Issuer::Bi => "Issuer_BI",
            Issuer::Db => "Issuer_DB",
            Issuer::Dc => "Issuer_DC",
            Issuer::Ml => "Issuer_ML",
            Issuer::Nm => "Issuer_NM",
            Issuer::Rb => "Issuer_RB",
            Issuer::Rs => "Issuer_RS",
            Issuer::Bc => "Issuer_BC",
            Issuer::Ht => "Issuer_HT",
            Issuer::Vt => "Issuer_VT",
            Issuer::Kc => "Issuer_KC",
            Issuer::Ms => "Issuer_MS",
            Issuer::Gj => "Issuer_GJ",
            Issuer::Xz => "Issuer_XZ",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Issuer_Unknow" => Some(Self::Unknow),
            "Issuer_SG" => Some(Self::Sg),
            "Issuer_BP" => Some(Self::Bp),
            "Issuer_CS" => Some(Self::Cs),
            "Issuer_CT" => Some(Self::Ct),
            "Issuer_EA" => Some(Self::Ea),
            "Issuer_GS" => Some(Self::Gs),
            "Issuer_HS" => Some(Self::Hs),
            "Issuer_JP" => Some(Self::Jp),
            "Issuer_MB" => Some(Self::Mb),
            "Issuer_SC" => Some(Self::Sc),
            "Issuer_UB" => Some(Self::Ub),
            "Issuer_BI" => Some(Self::Bi),
            "Issuer_DB" => Some(Self::Db),
            "Issuer_DC" => Some(Self::Dc),
            "Issuer_ML" => Some(Self::Ml),
            "Issuer_NM" => Some(Self::Nm),
            "Issuer_RB" => Some(Self::Rb),
            "Issuer_RS" => Some(Self::Rs),
            "Issuer_BC" => Some(Self::Bc),
            "Issuer_HT" => Some(Self::Ht),
            "Issuer_VT" => Some(Self::Vt),
            "Issuer_KC" => Some(Self::Kc),
            "Issuer_MS" => Some(Self::Ms),
            "Issuer_GJ" => Some(Self::Gj),
            "Issuer_XZ" => Some(Self::Xz),
            _ => None,
        }
    }
}
/// 窝轮上市日
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IpoPeriod {
    /// 未知
    Unknow = 0,
    /// 今日上市
    Today = 1,
    /// 明日上市
    Tomorrow = 2,
    /// 未来一周上市
    Nextweek = 3,
    /// 过去一周上市
    Lastweek = 4,
    /// 过去一月上市
    Lastmonth = 5,
}
impl IpoPeriod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IpoPeriod::Unknow => "IpoPeriod_Unknow",
            IpoPeriod::Today => "IpoPeriod_Today",
            IpoPeriod::Tomorrow => "IpoPeriod_Tomorrow",
            IpoPeriod::Nextweek => "IpoPeriod_Nextweek",
            IpoPeriod::Lastweek => "IpoPeriod_Lastweek",
            IpoPeriod::Lastmonth => "IpoPeriod_Lastmonth",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IpoPeriod_Unknow" => Some(Self::Unknow),
            "IpoPeriod_Today" => Some(Self::Today),
            "IpoPeriod_Tomorrow" => Some(Self::Tomorrow),
            "IpoPeriod_Nextweek" => Some(Self::Nextweek),
            "IpoPeriod_Lastweek" => Some(Self::Lastweek),
            "IpoPeriod_Lastmonth" => Some(Self::Lastmonth),
            _ => None,
        }
    }
}
/// 窝轮价外/内,界内证表示界内界外
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceType {
    Unknow = 0,
    /// 价外，界内证表示界外
    Outside = 1,
    /// 价内，界内证表示界内
    WithIn = 2,
}
impl PriceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceType::Unknow => "PriceType_Unknow",
            PriceType::Outside => "PriceType_Outside",
            PriceType::WithIn => "PriceType_WithIn",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PriceType_Unknow" => Some(Self::Unknow),
            "PriceType_Outside" => Some(Self::Outside),
            "PriceType_WithIn" => Some(Self::WithIn),
            _ => None,
        }
    }
}
/// 窝轮状态
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WarrantStatus {
    /// 未知
    Unknow = 0,
    /// 正常状态
    Normal = 1,
    /// 停牌
    Suspend = 2,
    /// 终止交易
    StopTrade = 3,
    /// 等待上市
    PendingListing = 4,
}
impl WarrantStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WarrantStatus::Unknow => "WarrantStatus_Unknow",
            WarrantStatus::Normal => "WarrantStatus_Normal",
            WarrantStatus::Suspend => "WarrantStatus_Suspend",
            WarrantStatus::StopTrade => "WarrantStatus_StopTrade",
            WarrantStatus::PendingListing => "WarrantStatus_PendingListing",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "WarrantStatus_Unknow" => Some(Self::Unknow),
            "WarrantStatus_Normal" => Some(Self::Normal),
            "WarrantStatus_Suspend" => Some(Self::Suspend),
            "WarrantStatus_StopTrade" => Some(Self::StopTrade),
            "WarrantStatus_PendingListing" => Some(Self::PendingListing),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompanyAct {
    /// 无
    None = 0,
    /// 拆股		
    Split = 1,
    /// 合股
    Join = 2,
    /// 送股
    Bonus = 4,
    /// 转赠股
    Transfer = 8,
    /// 配股	
    Allot = 16,
    /// 增发股
    Add = 32,
    /// 现金分红
    Dividend = 64,
    /// 特别股息	
    SpDividend = 128,
}
impl CompanyAct {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CompanyAct::None => "CompanyAct_None",
            CompanyAct::Split => "CompanyAct_Split",
            CompanyAct::Join => "CompanyAct_Join",
            CompanyAct::Bonus => "CompanyAct_Bonus",
            CompanyAct::Transfer => "CompanyAct_Transfer",
            CompanyAct::Allot => "CompanyAct_Allot",
            CompanyAct::Add => "CompanyAct_Add",
            CompanyAct::Dividend => "CompanyAct_Dividend",
            CompanyAct::SpDividend => "CompanyAct_SPDividend",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CompanyAct_None" => Some(Self::None),
            "CompanyAct_Split" => Some(Self::Split),
            "CompanyAct_Join" => Some(Self::Join),
            "CompanyAct_Bonus" => Some(Self::Bonus),
            "CompanyAct_Transfer" => Some(Self::Transfer),
            "CompanyAct_Allot" => Some(Self::Allot),
            "CompanyAct_Add" => Some(Self::Add),
            "CompanyAct_Dividend" => Some(Self::Dividend),
            "CompanyAct_SPDividend" => Some(Self::SpDividend),
            _ => None,
        }
    }
}
/// 行情权限
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QotRight {
    /// 未知
    Unknow = 0,
    /// Bmp，无法订阅
    Bmp = 1,
    /// Level1
    Level1 = 2,
    /// Level2
    Level2 = 3,
    /// SF高级行情
    Sf = 4,
    /// 无权限
    No = 5,
}
impl QotRight {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            QotRight::Unknow => "QotRight_Unknow",
            QotRight::Bmp => "QotRight_Bmp",
            QotRight::Level1 => "QotRight_Level1",
            QotRight::Level2 => "QotRight_Level2",
            QotRight::Sf => "QotRight_SF",
            QotRight::No => "QotRight_No",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "QotRight_Unknow" => Some(Self::Unknow),
            "QotRight_Bmp" => Some(Self::Bmp),
            "QotRight_Level1" => Some(Self::Level1),
            "QotRight_Level2" => Some(Self::Level2),
            "QotRight_SF" => Some(Self::Sf),
            "QotRight_No" => Some(Self::No),
            _ => None,
        }
    }
}
/// 提醒类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceReminderType {
    /// 未知
    Unknown = 0,
    /// 价格涨到
    PriceUp = 1,
    /// 价格跌到
    PriceDown = 2,
    /// 日涨幅超（该字段为百分比字段，设置时填 20 表示 20%）
    ChangeRateUp = 3,
    /// 日跌幅超（该字段为百分比字段，设置时填 20 表示 20%）
    ChangeRateDown = 4,
    /// 5 分钟涨幅超（该字段为百分比字段，设置时填 20 表示 20%）
    PriceReminderType5minChangeRateUp = 5,
    /// 5 分钟跌幅超（该字段为百分比字段，设置时填 20 表示 20%）
    PriceReminderType5minChangeRateDown = 6,
    /// 成交量超过
    VolumeUp = 7,
    /// 成交额超过
    TurnoverUp = 8,
    /// 换手率超过（该字段为百分比字段，设置时填 20 表示 20%）
    TurnoverRateUp = 9,
    /// 买一价高于
    BidPriceUp = 10,
    /// 卖一价低于
    AskPriceDown = 11,
    /// 买一量高于
    BidVolUp = 12,
    /// 卖一量高于
    AskVolUp = 13,
    /// 3 分钟涨幅超（该字段为百分比字段，设置时填 20 表示 20%）
    PriceReminderType3minChangeRateUp = 14,
    /// 3 分钟跌幅超（该字段为百分比字段，设置时填 20 表示 20%）
    PriceReminderType3minChangeRateDown = 15,
}
impl PriceReminderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceReminderType::Unknown => "PriceReminderType_Unknown",
            PriceReminderType::PriceUp => "PriceReminderType_PriceUp",
            PriceReminderType::PriceDown => "PriceReminderType_PriceDown",
            PriceReminderType::ChangeRateUp => "PriceReminderType_ChangeRateUp",
            PriceReminderType::ChangeRateDown => "PriceReminderType_ChangeRateDown",
            PriceReminderType::PriceReminderType5minChangeRateUp => {
                "PriceReminderType_5MinChangeRateUp"
            }
            PriceReminderType::PriceReminderType5minChangeRateDown => {
                "PriceReminderType_5MinChangeRateDown"
            }
            PriceReminderType::VolumeUp => "PriceReminderType_VolumeUp",
            PriceReminderType::TurnoverUp => "PriceReminderType_TurnoverUp",
            PriceReminderType::TurnoverRateUp => "PriceReminderType_TurnoverRateUp",
            PriceReminderType::BidPriceUp => "PriceReminderType_BidPriceUp",
            PriceReminderType::AskPriceDown => "PriceReminderType_AskPriceDown",
            PriceReminderType::BidVolUp => "PriceReminderType_BidVolUp",
            PriceReminderType::AskVolUp => "PriceReminderType_AskVolUp",
            PriceReminderType::PriceReminderType3minChangeRateUp => {
                "PriceReminderType_3MinChangeRateUp"
            }
            PriceReminderType::PriceReminderType3minChangeRateDown => {
                "PriceReminderType_3MinChangeRateDown"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PriceReminderType_Unknown" => Some(Self::Unknown),
            "PriceReminderType_PriceUp" => Some(Self::PriceUp),
            "PriceReminderType_PriceDown" => Some(Self::PriceDown),
            "PriceReminderType_ChangeRateUp" => Some(Self::ChangeRateUp),
            "PriceReminderType_ChangeRateDown" => Some(Self::ChangeRateDown),
            "PriceReminderType_5MinChangeRateUp" => {
                Some(Self::PriceReminderType5minChangeRateUp)
            }
            "PriceReminderType_5MinChangeRateDown" => {
                Some(Self::PriceReminderType5minChangeRateDown)
            }
            "PriceReminderType_VolumeUp" => Some(Self::VolumeUp),
            "PriceReminderType_TurnoverUp" => Some(Self::TurnoverUp),
            "PriceReminderType_TurnoverRateUp" => Some(Self::TurnoverRateUp),
            "PriceReminderType_BidPriceUp" => Some(Self::BidPriceUp),
            "PriceReminderType_AskPriceDown" => Some(Self::AskPriceDown),
            "PriceReminderType_BidVolUp" => Some(Self::BidVolUp),
            "PriceReminderType_AskVolUp" => Some(Self::AskVolUp),
            "PriceReminderType_3MinChangeRateUp" => {
                Some(Self::PriceReminderType3minChangeRateUp)
            }
            "PriceReminderType_3MinChangeRateDown" => {
                Some(Self::PriceReminderType3minChangeRateDown)
            }
            _ => None,
        }
    }
}
/// 提醒频率
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceReminderFreq {
    /// 未知
    Unknown = 0,
    /// 持续提醒
    Always = 1,
    /// 每日一次
    OnceADay = 2,
    /// 仅提醒一次
    OnlyOnce = 3,
}
impl PriceReminderFreq {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceReminderFreq::Unknown => "PriceReminderFreq_Unknown",
            PriceReminderFreq::Always => "PriceReminderFreq_Always",
            PriceReminderFreq::OnceADay => "PriceReminderFreq_OnceADay",
            PriceReminderFreq::OnlyOnce => "PriceReminderFreq_OnlyOnce",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PriceReminderFreq_Unknown" => Some(Self::Unknown),
            "PriceReminderFreq_Always" => Some(Self::Always),
            "PriceReminderFreq_OnceADay" => Some(Self::OnceADay),
            "PriceReminderFreq_OnlyOnce" => Some(Self::OnlyOnce),
            _ => None,
        }
    }
}
/// 资产类别
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetClass {
    /// 未知
    Unknow = 0,
    /// 股票
    Stock = 1,
    /// 债券
    Bond = 2,
    /// 商品
    Commodity = 3,
    /// 货币市场
    CurrencyMarket = 4,
    /// 期货
    Future = 5,
    /// 掉期
    Swap = 6,
}
impl AssetClass {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AssetClass::Unknow => "AssetClass_Unknow",
            AssetClass::Stock => "AssetClass_Stock",
            AssetClass::Bond => "AssetClass_Bond",
            AssetClass::Commodity => "AssetClass_Commodity",
            AssetClass::CurrencyMarket => "AssetClass_CurrencyMarket",
            AssetClass::Future => "AssetClass_Future",
            AssetClass::Swap => "AssetClass_Swap",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AssetClass_Unknow" => Some(Self::Unknow),
            "AssetClass_Stock" => Some(Self::Stock),
            "AssetClass_Bond" => Some(Self::Bond),
            "AssetClass_Commodity" => Some(Self::Commodity),
            "AssetClass_CurrencyMarket" => Some(Self::CurrencyMarket),
            "AssetClass_Future" => Some(Self::Future),
            "AssetClass_Swap" => Some(Self::Swap),
            _ => None,
        }
    }
}
/// 交割周期
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExpirationCycle {
    /// 未知
    Unknown = 0,
    /// 周期权
    Week = 1,
    /// 月期权
    Month = 2,
}
impl ExpirationCycle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExpirationCycle::Unknown => "ExpirationCycle_Unknown",
            ExpirationCycle::Week => "ExpirationCycle_Week",
            ExpirationCycle::Month => "ExpirationCycle_Month",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ExpirationCycle_Unknown" => Some(Self::Unknown),
            "ExpirationCycle_Week" => Some(Self::Week),
            "ExpirationCycle_Month" => Some(Self::Month),
            _ => None,
        }
    }
}
/// 所属交易所
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExchType {
    /// 未知
    Unknown = 0,
    /// 港交所·主板
    HkMainBoard = 1,
    /// 港交所·创业板
    HkGemBoard = 2,
    /// 港交所
    HkHkex = 3,
    /// 纽交所
    UsNyse = 4,
    /// 纳斯达克
    UsNasdaq = 5,
    /// OTC 市场
    UsPink = 6,
    /// 美交所
    UsAmex = 7,
    /// 美国（仅美股期权适用）
    UsOption = 8,
    /// NYMEX
    UsNymex = 9,
    /// COMEX
    UsComex = 10,
    /// CBOT
    UsCbot = 11,
    /// CME
    UsCme = 12,
    /// CBOE
    UsCboe = 13,
    /// 上交所
    CnSh = 14,
    /// 深交所
    CnSz = 15,
    /// 科创板
    CnStib = 16,
    /// 新交所
    SgSgx = 17,
    /// 大阪交易所
    JpOse = 18,
}
impl ExchType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExchType::Unknown => "ExchType_Unknown",
            ExchType::HkMainBoard => "ExchType_HK_MainBoard",
            ExchType::HkGemBoard => "ExchType_HK_GEMBoard",
            ExchType::HkHkex => "ExchType_HK_HKEX",
            ExchType::UsNyse => "ExchType_US_NYSE",
            ExchType::UsNasdaq => "ExchType_US_Nasdaq",
            ExchType::UsPink => "ExchType_US_Pink",
            ExchType::UsAmex => "ExchType_US_AMEX",
            ExchType::UsOption => "ExchType_US_Option",
            ExchType::UsNymex => "ExchType_US_NYMEX",
            ExchType::UsComex => "ExchType_US_COMEX",
            ExchType::UsCbot => "ExchType_US_CBOT",
            ExchType::UsCme => "ExchType_US_CME",
            ExchType::UsCboe => "ExchType_US_CBOE",
            ExchType::CnSh => "ExchType_CN_SH",
            ExchType::CnSz => "ExchType_CN_SZ",
            ExchType::CnStib => "ExchType_CN_STIB",
            ExchType::SgSgx => "ExchType_SG_SGX",
            ExchType::JpOse => "ExchType_JP_OSE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ExchType_Unknown" => Some(Self::Unknown),
            "ExchType_HK_MainBoard" => Some(Self::HkMainBoard),
            "ExchType_HK_GEMBoard" => Some(Self::HkGemBoard),
            "ExchType_HK_HKEX" => Some(Self::HkHkex),
            "ExchType_US_NYSE" => Some(Self::UsNyse),
            "ExchType_US_Nasdaq" => Some(Self::UsNasdaq),
            "ExchType_US_Pink" => Some(Self::UsPink),
            "ExchType_US_AMEX" => Some(Self::UsAmex),
            "ExchType_US_Option" => Some(Self::UsOption),
            "ExchType_US_NYMEX" => Some(Self::UsNymex),
            "ExchType_US_COMEX" => Some(Self::UsComex),
            "ExchType_US_CBOT" => Some(Self::UsCbot),
            "ExchType_US_CME" => Some(Self::UsCme),
            "ExchType_US_CBOE" => Some(Self::UsCboe),
            "ExchType_CN_SH" => Some(Self::CnSh),
            "ExchType_CN_SZ" => Some(Self::CnSz),
            "ExchType_CN_STIB" => Some(Self::CnStib),
            "ExchType_SG_SGX" => Some(Self::SgSgx),
            "ExchType_JP_OSE" => Some(Self::JpOse),
            _ => None,
        }
    }
}
/// 周期类型
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PeriodType {
    /// 未知
    Unknown = 0,
    /// 实时
    Intraday = 1,
    /// 日
    Day = 2,
    /// 周
    Week = 3,
    /// 月
    Month = 4,
}
impl PeriodType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PeriodType::Unknown => "PeriodType_Unknown",
            PeriodType::Intraday => "PeriodType_INTRADAY",
            PeriodType::Day => "PeriodType_DAY",
            PeriodType::Week => "PeriodType_WEEK",
            PeriodType::Month => "PeriodType_MONTH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PeriodType_Unknown" => Some(Self::Unknown),
            "PeriodType_INTRADAY" => Some(Self::Intraday),
            "PeriodType_DAY" => Some(Self::Day),
            "PeriodType_WEEK" => Some(Self::Week),
            "PeriodType_MONTH" => Some(Self::Month),
            _ => None,
        }
    }
}
