use crate::ffi::FFIBuffer;
use libc::{c_char};
use scannit_core::travelcard::TravelCard;

#[repr(C)]
pub struct FFITravelCard {
    pub application_version: u8,
    pub application_key_version: u8,
    pub application_instance_id: FFIBuffer<*mut c_char>,
    pub platform_type: u8,
    pub is_mac_protected: bool,

    /// As UNIX ticks, i.e. non-leap-seconds since Jan 1 1970. In UTC.
    pub application_issuing_date: i64,
    pub application_status: bool,
    pub application_unblocking_number: u8,
    pub application_transaction_counter: u32,
    pub action_list_counter: u32,

    pub period_pass: FFIPeriodPass,
}

impl FFITravelCard {
    pub fn from_travel_card(travel_card: TravelCard) -> FFITravelCard {
        FFITravelCard { }
    }
}

pub struct FFIPeriodPass {
    pub product_code_1_kind: ProductCodeKind,
    pub product_code_1_value: u16,
    pub validity_area_1_kind: ValidityAreaKind,
    /// This is either a single positive integer, or short list of positive integers.
    /// We'll represent it as something that's always an array.
    pub validity_area_1_value: FFIBuffer<u8>,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ProductCodeKind {
    FaresFor2010 = 0,
    FaresFor2014 = 1
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValidityAreaKind {
    OldZoneKind = 0,
    VehicleType = 1,
    NewZoneKind = 2
}