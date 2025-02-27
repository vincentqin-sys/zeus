use cached::proc_macro::cached;
use std::collections::HashSet;
use std::convert::From;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::string::ToString;
use tracing::error;

use crate::client::ClientRef;
use crate::messages::{encode_option_field, IncomingMessages, ToField};
use crate::messages::{RequestMessage};
use crate::{server_versions, Error};

mod decoders;
mod encoders;

// Models

#[derive(Clone, Debug, PartialEq, Eq, Default, Hash)]
/// SecurityType enumerates available security types
pub enum SecurityType {
    /// Stock (or ETF)
    #[default]
    Stock,
    /// Option
    Option,
    /// Future
    Future,
    /// Index
    Index,
    /// Futures option
    FuturesOption,
    /// Forex pair
    ForexPair,
    /// Combo
    Spread,
    ///  Warrant
    Warrant,
    /// Bond
    Bond,
    /// Commodity
    Commodity,
    /// News
    News,
    /// Mutual fund
    MutualFund,
}

impl ToField for SecurityType {
    fn to_field(&self) -> String {
        self.to_string()
    }
}

impl ToField for Option<SecurityType> {
    fn to_field(&self) -> String {
        encode_option_field(self)
    }
}

impl ToString for SecurityType {
    fn to_string(&self) -> String {
        match self {
            SecurityType::Stock => "STK".to_string(),
            SecurityType::Option => "OPT".to_string(),
            SecurityType::Future => "FUT".to_string(),
            SecurityType::Index => "IND".to_string(),
            SecurityType::FuturesOption => "FOP".to_string(),
            SecurityType::ForexPair => "CASH".to_string(),
            SecurityType::Spread => "BAG".to_string(),
            SecurityType::Warrant => "WAR".to_string(),
            SecurityType::Bond => "BOND".to_string(),
            SecurityType::Commodity => "CMDTY".to_string(),
            SecurityType::News => "NEWS".to_string(),
            SecurityType::MutualFund => "FUND".to_string(),
        }
    }
}

impl SecurityType {
    pub fn from(name: &str) -> SecurityType {
        match name {
            "STK" => SecurityType::Stock,
            "OPT" => SecurityType::Option,
            "FUT" => SecurityType::Future,
            "IND" => SecurityType::Index,
            "FOP" => SecurityType::FuturesOption,
            "CASH" => SecurityType::ForexPair,
            "BAG" => SecurityType::Spread,
            "WAR" => SecurityType::Warrant,
            "BOND" => SecurityType::Bond,
            "CMDTY" => SecurityType::Commodity,
            "NEWS" => SecurityType::News,
            "FUND" => SecurityType::MutualFund,
            &_ => todo!(),
        }
    }
}

#[derive(Clone, Debug, Default)]
/// Contract describes an instrument's definition
pub struct Contract {
    /// The unique IB contract identifier.
    pub contract_id: i32,
    /// The underlying's asset symbol.
    pub symbol: String,
    pub security_type: SecurityType,
    /// The contract's last trading day or contract month (for Options and Futures).
    /// Strings with format YYYYMM will be interpreted as the Contract Month whereas YYYYMMDD will be interpreted as Last Trading Day.
    pub last_trade_date_or_contract_month: String,
    /// The option's strike price.
    pub strike: f64,
    /// Either Put or Call (i.e. Options). Valid values are P, PUT, C, CALL.
    pub right: String,
    /// The instrument's multiplier (i.e. options, futures).
    pub multiplier: String,
    /// The destination exchange.
    pub exchange: String,
    /// The underlying's currency.
    pub currency: String,
    /// The contract's symbol within its primary exchange For options, this will be the OCC symbol.
    pub local_symbol: String,
    /// The contract's primary exchange.
    /// For smart routed contracts, used to define contract in case of ambiguity.
    /// Should be defined as native exchange of contract, e.g. ISLAND for MSFT For exchanges which contain a period in name, will only be part of exchange name prior to period, i.e. ENEXT for ENEXT.BE.
    pub primary_exchange: String,
    /// The trading class name for this contract. Available in TWS contract description window as well. For example, GBL Dec '13 future's trading class is "FGBL".
    pub trading_class: String,
    /// If set to true, contract details requests and historical data queries can be performed pertaining to expired futures contracts. Expired options or other instrument types are not available.
    pub include_expired: bool,
    /// Security's identifier when querying contract's details or placing orders ISIN - Example: Apple: US0378331005 CUSIP - Example: Apple: 037833100.
    pub security_id_type: String,
    /// Identifier of the security type.
    pub security_id: String,
    /// Description of the combo legs.
    pub combo_legs_description: String,
    pub combo_legs: Vec<ComboLeg>,
    /// Delta and underlying price for Delta-Neutral combo orders. Underlying (STK or FUT), delta and underlying price goes into this attribute.
    pub delta_neutral_contract: Option<DeltaNeutralContract>,

    pub issuer_id: String,
    pub description: String,
}

impl Hash for Contract {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
        self.security_type.hash(state);
    }
}

impl PartialEq<Self> for Contract {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol && self.security_type == other.security_type
    }
}

impl Eq for Contract {}
impl Contract {
    /// Creates stock contract from specified symbol
    /// currency defaults to USD and SMART exchange.
    pub fn stock(symbol: &str) -> Contract {
        Contract {
            symbol: symbol.to_string(),
            security_type: SecurityType::Stock,
            currency: "USD".to_string(),
            exchange: "SMART".to_string(),
            ..Default::default()
        }
    }

    pub fn auto_stock(symbol: &str) -> Contract {
        let exchange_ = symbol.split(":").collect::<Vec<&str>>()[0].to_string();
        let symbol = if symbol.contains(":") {
            symbol
                .split(":")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string()
        } else {
            symbol.to_string()
        };
        if exchange_ == "SZSE" || exchange_ == "SSE" {
            if symbol.starts_with("0") {
                return Contract {
                    symbol: symbol.to_string(),
                    security_type: SecurityType::Stock,
                    currency: "CNH".to_string(),
                    exchange: "SEHKSZSE".to_string(),
                    ..Default::default()
                };
            }
            if symbol.starts_with("688") {
                return Contract {
                    symbol: symbol.to_string(),
                    security_type: SecurityType::Stock,
                    currency: "CNH".to_string(),
                    exchange: "SEHKSTAR".to_string(),
                    ..Default::default()
                };
            }
            if symbol.starts_with("6") {
                return Contract {
                    symbol: symbol.to_string(),
                    security_type: SecurityType::Stock,
                    currency: "CNH".to_string(),
                    exchange: "SEHKNTL".to_string(),
                    ..Default::default()
                };
            }
            if symbol.starts_with("3") {
                return Contract {
                    symbol: symbol.to_string(),
                    security_type: SecurityType::Stock,
                    currency: "CNH".to_string(),
                    exchange: "CHINEXT".to_string(),
                    ..Default::default()
                };
            }
        }
        Contract {
            symbol: symbol.to_string(),
            security_type: SecurityType::Stock,
            currency: "USD".to_string(),
            exchange: "SMART".to_string(),
            ..Default::default()
        }
    }
    pub fn option(
        symbol: &str,
        last_trade_date_or_contract_month: &str,
        strike: f64,
        right: &str,
        multiplier: &str,
    ) -> Contract {
        Contract {
            security_type: SecurityType::Option,
            currency: "USD".to_string(),
            exchange: "SMART".to_string(),
            symbol: symbol.to_string(),
            last_trade_date_or_contract_month: last_trade_date_or_contract_month.to_string(),
            strike,
            right: right.to_string(),
            multiplier: multiplier.to_string(),
            trading_class: symbol.to_string(),
            ..Default::default()
        }
    }

    /// Creates futures contract from specified symbol
    pub fn futures(symbol: &str) -> Contract {
        Contract {
            symbol: symbol.to_string(),
            security_type: SecurityType::Future,
            currency: "USD".to_string(),
            ..Default::default()
        }
    }

    /// Is Bag request
    pub fn is_bag(&self) -> bool {
        self.security_type == SecurityType::Spread
    }

    pub(crate) fn push_fields(&self, message: &mut RequestMessage) {
        message.push_field(&self.contract_id);
        message.push_field(&self.symbol);
        message.push_field(&self.security_type);
        message.push_field(&self.last_trade_date_or_contract_month);
        message.push_field(&self.strike);
        message.push_field(&self.right);
        message.push_field(&self.multiplier);
        message.push_field(&self.exchange);
        message.push_field(&self.primary_exchange);
        message.push_field(&self.currency);
        message.push_field(&self.local_symbol);
        message.push_field(&self.trading_class);
        message.push_field(&self.include_expired);
    }
}

#[derive(Clone, Debug, Default)]
// ComboLeg represents a leg within combo orders.
pub struct ComboLeg {
    /// The Contract's IB's unique id.
    pub contract_id: i32,
    /// Select the relative number of contracts for the leg you are constructing. To help determine the ratio for a specific combination order, refer to the Interactive Analytics section of the User's Guide.
    pub ratio: i32,
    /// The side (buy or sell) of the leg:
    pub action: String,
    // The destination exchange to which the order will be routed.
    pub exchange: String,
    /// Specifies whether an order is an open or closing order.
    /// For institutional customers to determine if this order is to open or close a position.
    pub open_close: ComboLegOpenClose,
    /// For stock legs when doing short selling. Set to 1 = clearing broker, 2 = third party.
    pub short_sale_slot: i32,
    /// When ShortSaleSlot is 2, this field shall contain the designated location.
    pub designated_location: String,
    // DOC_TODO.
    pub exempt_code: i32,
}

#[derive(Clone, Copy, Debug, Default)]
/// OpenClose specifies whether an order is an open or closing order.
pub enum ComboLegOpenClose {
    /// 0 - Same as the parent security. This is the only option for retail customers.
    #[default]
    Same = 0,
    /// 1 - Open. This value is only valid for institutional customers.
    Open = 1,
    /// 2 - Close. This value is only valid for institutional customers.
    Close = 2,
    /// 3 - Unknown.
    Unknown = 3,
}

impl ToField for ComboLegOpenClose {
    fn to_field(&self) -> String {
        (*self as u8).to_string()
    }
}

impl From<i32> for ComboLegOpenClose {
    // TODO - verify these values
    fn from(val: i32) -> Self {
        match val {
            0 => Self::Same,
            1 => Self::Open,
            2 => Self::Close,
            3 => Self::Unknown,
            _ => panic!("unsupported value: {val}"),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
/// Delta and underlying price for Delta-Neutral combo orders.
/// Underlying (STK or FUT), delta and underlying price goes into this attribute.
pub struct DeltaNeutralContract {
    /// The unique contract identifier specifying the security. Used for Delta-Neutral Combo contracts.
    pub contract_id: i32,
    /// The underlying stock or future delta. Used for Delta-Neutral Combo contracts.
    pub delta: f64,
    /// The price of the underlying. Used for Delta-Neutral Combo contracts.
    pub price: f64,
}

/// ContractDetails provides extended contract details.
#[derive(Debug, Default, Clone)]
pub struct ContractDetails {
    /// A fully-defined Contract object.
    pub contract: Contract,
    /// The market name for this product.
    pub market_name: String,
    /// The minimum allowed price variation. Note that many securities vary their minimum tick size according to their price. This value will only show the smallest of the different minimum tick sizes regardless of the product's price. Full information about the minimum increment price structure can be obtained with the reqMarketRule function or the IB Contract and Security Search site.
    pub min_tick: f64,
    /// Allows execution and strike prices to be reported consistently with market data, historical data and the order price, i.e. Z on LIFFE is reported in Index points and not GBP. In TWS versions prior to 972, the price magnifier is used in defining future option strike prices (e.g. in the API the strike is specified in dollars, but in TWS it is specified in cents). In TWS versions 972 and higher, the price magnifier is not used in defining futures option strike prices so they are consistent in TWS and the API.
    pub price_magnifier: i32,
    /// Supported order types for this product.
    pub order_types: String,
    /// Valid exchange fields when placing an order for this contract.
    /// The list of exchanges will is provided in the same order as the corresponding MarketRuleIds list.
    pub valid_exchanges: String,
    /// For derivatives, the contract ID (conID) of the underlying instrument.
    pub under_contract_id: i32,
    /// Descriptive name of the product.
    pub long_name: String,
    /// Typically the contract month of the underlying for a Future contract.
    pub contract_month: String,
    /// The industry classification of the underlying/product. For example, Financial.
    pub industry: String,
    /// The industry category of the underlying. For example, InvestmentSvc.
    pub category: String,
    /// The industry subcategory of the underlying. For example, Brokerage.
    pub subcategory: String,
    /// The time zone for the trading hours of the product. For example, EST.
    pub time_zone_id: String,
    /// The trading hours of the product. This value will contain the trading hours of the current day as well as the next's. For example, 20090507:0700-1830,1830-2330;20090508:CLOSED. In TWS versions 965+ there is an option in the Global Configuration API settings to return 1 month of trading hours. In TWS version 970+, the format includes the date of the closing time to clarify potential ambiguity, ex: 20180323:0400-20180323:2000;20180326:0400-20180326:2000 The trading hours will correspond to the hours for the product on the associated exchange. The same instrument can have different hours on different exchanges.
    pub trading_hours: String,
    /// The liquid hours of the product. This value will contain the liquid hours (regular trading hours) of the contract on the specified exchange. Format for TWS versions until 969: 20090507:0700-1830,1830-2330;20090508:CLOSED. In TWS versions 965+ there is an option in the Global Configuration API settings to return 1 month of trading hours. In TWS v970 and above, the format includes the date of the closing time to clarify potential ambiguity, e.g. 20180323:0930-20180323:1600;20180326:0930-20180326:1600.
    pub liquid_hours: String,
    /// Contains the Economic Value Rule name and the respective optional argument. The two values should be separated by a colon. For example, aussieBond:YearsToExpiration=3. When the optional argument is not present, the first value will be followed by a colon.
    pub ev_rule: String,
    /// Tells you approximately how much the market value of a contract would change if the price were to change by 1. It cannot be used to get market value by multiplying the price by the approximate multiplier.
    pub ev_multiplier: f64,
    /// Aggregated group Indicates the smart-routing group to which a contract belongs. contracts which cannot be smart-routed have aggGroup = -1.
    pub agg_group: i32,
    /// A list of contract identifiers that the customer is allowed to view. CUSIP/ISIN/etc. For US stocks, receiving the ISIN requires the CUSIP market data subscription. For Bonds, the CUSIP or ISIN is input directly into the symbol field of the Contract class.
    pub sec_id_list: Vec<TagValue>,
    /// For derivatives, the symbol of the underlying contract.
    pub under_symbol: String,
    /// For derivatives, returns the underlying security type.
    pub under_security_type: String,
    /// The list of market rule IDs separated by comma Market rule IDs can be used to determine the minimum price increment at a given price.
    pub market_rule_ids: String,
    /// Real expiration date. Requires TWS 968+ and API v973.04+. Python API specifically requires API v973.06+.
    pub real_expiration_date: String,
    /// Last trade time.
    pub last_trade_time: String,
    /// Stock type.
    pub stock_type: String,
    /// The nine-character bond CUSIP. For Bonds only. Receiving CUSIPs requires a CUSIP market data subscription.
    pub cusip: String,
    /// Identifies the credit rating of the issuer. This field is not currently available from the TWS API. For Bonds only. A higher credit rating generally indicates a less risky investment. Bond ratings are from Moody's and S&P respectively. Not currently implemented due to bond market data restrictions.
    pub ratings: String,
    /// A description string containing further descriptive information about the bond. For Bonds only.
    pub desc_append: String,
    /// The type of bond, such as "CORP.".
    pub bond_type: String,
    /// The type of bond coupon. This field is currently not available from the TWS API. For Bonds only.
    pub coupon_type: String,
    /// If true, the bond can be called by the issuer under certain conditions. This field is currently not available from the TWS API. For Bonds only.
    pub callable: bool,
    /// Values are True or False. If true, the bond can be sold back to the issuer under certain conditions. This field is currently not available from the TWS API. For Bonds only.
    pub putable: bool,
    /// The interest rate used to calculate the amount you will receive in interest payments over the course of the year. This field is currently not available from the TWS API. For Bonds only.
    pub coupon: f64,
    /// Values are True or False. If true, the bond can be converted to stock under certain conditions. This field is currently not available from the TWS API. For Bonds only.
    pub convertible: bool,
    /// The date on which the issuer must repay the face value of the bond. This field is currently not available from the TWS API. For Bonds only. Not currently implemented due to bond market data restrictions.
    pub maturity: String,
    /// The date the bond was issued. This field is currently not available from the TWS API. For Bonds only. Not currently implemented due to bond market data restrictions.
    pub issue_date: String,
    /// Only if bond has embedded options. This field is currently not available from the TWS API. Refers to callable bonds and puttable bonds. Available in TWS description window for bonds.
    pub next_option_date: String,
    /// Type of embedded option. This field is currently not available from the TWS API. Only if bond has embedded options.
    pub next_option_type: String,
    /// Only if bond has embedded options. This field is currently not available from the TWS API. For Bonds only.
    pub next_option_partial: bool,
    /// If populated for the bond in IB's database. For Bonds only.
    pub notes: String,
    /// Order's minimal size.
    pub min_size: f64,
    /// Order's size increment.
    pub size_increment: f64,
    /// Order's suggested size increment.
    pub suggested_size_increment: f64,
}

/// TagValue is a convenience struct to define key-value pairs.
#[derive(Clone, Debug)]
pub struct TagValue {
    pub tag: String,
    pub value: String,
}

impl ToField for Vec<TagValue> {
    fn to_field(&self) -> String {
        let mut values = Vec::new();
        for tag_value in self {
            values.push(format!("{}={};", tag_value.tag, tag_value.value))
        }
        values.concat()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub struct ReqSecDefOptParams {
    pub underlying_symbol: String,
    pub fut_fop_exchange: String,
    pub underlying_sec_type: String,
    pub underlying_con_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SecDefOptParam {
    pub exchange: String,
    pub underlying_con_id: i32,
    pub trading_class: String,
    pub multiplier: String,
    pub expirations: HashSet<String>,
    pub strikes: Vec<f64>,
}
// === API ===

// Requests contract information.
//
// Provides all the contracts matching the contract provided. It can also be used to retrieve complete options and futures chains. Though it is now (in API version > 9.72.12) advised to use reqSecDefOptParams for that purpose.
//
// # Arguments
// * `client` - [Client] with an active connection to gateway.
// * `contract` - The [Contract] used as sample to query the available contracts. Typically, it will contain the [Contract]'s symbol, currency, security_type, and exchange.
#[cached(
    result = true,
    time = 7200,
    key = "String",
    convert = r##"{ format!("{:?}", contract) }"##
)]
pub async fn contract_details(
    client: &ClientRef,
    contract: &Contract,
) -> Result<Vec<ContractDetails>, Error> {
    verify_contract(client, contract)?;

    let request_id = client.next_request_id();
    let packet = encoders::request_contract_data(client.server_version(), request_id, contract)?;

    let mut responses = client.send(request_id, packet).await?;

    let mut contract_details: Vec<ContractDetails> = Vec::default();

    loop {
        match responses.receiver.as_mut().unwrap().recv().await {
            Some(mut message) => match message.message_type() {
                IncomingMessages::ContractData => {
                    let decoded =
                        decoders::contract_details(client.server_version(), &mut message)?;
                    contract_details.push(decoded);
                }
                IncomingMessages::ContractDataEnd => {
                    break;
                }
                IncomingMessages::Error => {
                    error!("error: {message:?}");
                    return Err(Error::Simple(format!("contract_details {message:?}")));
                }
                _ => {
                    error!("unexpected message: {:?}", message);
                }
            },
            _ => {
                break;
            }
        }
    }

    Ok(contract_details)
}

fn verify_contract(client: &ClientRef, contract: &Contract) -> Result<(), Error> {
    if !contract.security_id_type.is_empty() || !contract.security_id.is_empty() {
        client.check_server_version(
            server_versions::SEC_ID_TYPE,
            "It does not support security_id_type or security_id attributes",
        )?
    }

    if !contract.trading_class.is_empty() {
        client.check_server_version(
            server_versions::TRADING_CLASS,
            "It does not support the trading_class parameter when requesting contract details.",
        )?
    }

    if !contract.primary_exchange.is_empty() {
        client.check_server_version(
            server_versions::LINKING,
            "It does not support primary_exchange parameter when requesting contract details.",
        )?
    }

    if !contract.issuer_id.is_empty() {
        client.check_server_version(
            server_versions::BOND_ISSUERID,
            "It does not support issuer_id parameter when requesting contract details.",
        )?
    }

    Ok(())
}

/// Contract data and list of derivative security types
#[derive(Debug)]
pub struct ContractDescription {
    pub contract: Contract,
    pub derivative_security_types: Vec<String>,
}

#[derive(Debug, Default)]
pub struct MarketRule {
    pub market_rule_id: i32,
    pub price_increments: Vec<PriceIncrement>,
}

#[derive(Debug, Default)]
pub struct PriceIncrement {
    pub low_edge: f64,
    pub increment: f64,
}

#[cached(
    result = true,
    time = 7200,
    key = "String",
    convert = r##"{ format!("{:?}", req) }"##
)]
pub async fn sec_def_opt(
    client: &ClientRef,
    req: &ReqSecDefOptParams,
) -> Result<Vec<SecDefOptParam>, Error> {
    let request_id = client.next_request_id();
    let packet = encoders::req_sec_def_opt(client.server_version(), request_id, req)?;

    let mut responses = client.send(request_id, packet).await?;

    let mut result = vec![];
    loop {
        match responses.receiver.as_mut().unwrap().recv().await {
            Some(mut message) => match message.message_type() {
                IncomingMessages::SecurityDefinitionOptionParameter => {
                    let decoded = decoders::sec_def_opt(&mut message)?;
                    result.push(decoded);
                }
                IncomingMessages::SecurityDefinitionOptionParameterEnd => {
                    break;
                }
                IncomingMessages::Error => {
                    error!("error: {message:?}");
                    return Err(Error::Simple(format!(" {message:?}")));
                }
                _ => {
                    error!("unexpected message: {:?}", message);
                    return Err(Error::Simple(format!(" {message:?}")));
                }
            },
            _ => {
                return Err(Error::Simple(format!("unexpected message")));
            }
        }
    }
    Ok(result)
}
