use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum OrderStatus {
    Unknow = 0,
    New = 1,
    Filled = 2,
    PartiallyFilled = 3,
    Canceled = 4,
    Expired = 5,
    Rejected = 6,
}

impl OrderStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknow => "UNKNOW",
            Self::New => "NEW",
            Self::Filled => "FILLED",
            Self::PartiallyFilled => "PARTIALLY_FILLED",
            Self::Canceled => "CANCELED",
            Self::Expired => "EXPIRED",
            Self::Rejected => "REJECTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOW" => Some(Self::Unknow),
            "NEW" => Some(Self::New),
            "FILLED" => Some(Self::Filled),
            "PARTIALLY_FILLED" => Some(Self::PartiallyFilled),
            "CANCELED" => Some(Self::Canceled),
            "EXPIRED" => Some(Self::Expired),
            "REJECTED" => Some(Self::Rejected),
            _ => None,
        }
    }

    pub fn from_i32(value: i32) -> ::core::option::Option<Self> {
        match value {
            0 => Some(Self::Unknow),
            1 => Some(Self::New),
            2 => Some(Self::Filled),
            3 => Some(Self::PartiallyFilled),
            4 => Some(Self::Canceled),
            5 => Some(Self::Expired),
            6 => Some(Self::Rejected),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[repr(i32)]
pub enum TimeInForce {
    Ioc = 0,
    Gtc = 1,
    Gtd = 2,
    Fok = 3,
    /// / (ALO/POST ONLY)
    Gtx = 4,
    Hiden = 5,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[repr(i32)]
pub enum OrderType {
    Limit = 0,
    Market = 1,
    StopLimit = 2,
    StopMarket = 3,
    TakeProfitLimit = 4,
    TakeProfitMarket = 5,
}



impl TimeInForce {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Fok => "FOK",
            Self::Gtc => "GTC",
            Self::Gtd => "GTD",
            Self::Gtx => "GTX",
            Self::Hiden => "HIDEN",
            Self::Ioc => "IOC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FOK" => Some(Self::Fok),
            "GTC" => Some(Self::Gtc),
            "GTD" => Some(Self::Gtd),
            "GTX" => Some(Self::Gtx),
            "HIDEN" => Some(Self::Hiden),
            "IOC" => Some(Self::Ioc),
            _ => None,
        }
    }

    pub fn from_i32(value: i32) -> ::core::option::Option<Self> {
        match value {
            0 => Some(Self::Ioc),
            1 => Some(Self::Gtc),
            2 => Some(Self::Gtd),
            3 => Some(Self::Fok),
            4 => Some(Self::Gtx),
            5 => Some(Self::Hiden),
            _ => None,
        }
    }
}


impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Limit => "LIMIT",
            Self::Market => "MARKET",
            Self::StopLimit => "STOP_LIMIT",
            Self::StopMarket => "STOP_MARKET",
            Self::TakeProfitLimit => "TAKE_PROFIT_LIMIT",
            Self::TakeProfitMarket => "TAKE_PROFIT_MARKET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LIMIT" => Some(Self::Limit),
            "MARKET" => Some(Self::Market),
            "STOP_LIMIT" => Some(Self::StopLimit),
            "STOP_MARKET" => Some(Self::StopMarket),
            "TAKE_PROFIT_LIMIT" => Some(Self::TakeProfitLimit),
            "TAKE_PROFIT_MARKET" => Some(Self::TakeProfitMarket),
            _ => None,
        }
    }

    pub fn from_i32(value: i32) -> ::core::option::Option<Self> {
        match value {
            0 => Some(Self::Limit),
            1 => Some(Self::Market),
            2 => Some(Self::StopLimit),
            3 => Some(Self::StopMarket),
            4 => Some(Self::TakeProfitLimit),
            5 => Some(Self::TakeProfitMarket),
            _ => None,
        }
    }
}