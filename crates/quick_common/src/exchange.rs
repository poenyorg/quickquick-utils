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
