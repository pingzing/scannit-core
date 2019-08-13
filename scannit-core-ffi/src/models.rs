use crate::ffi::FFIBuffer;
use libc::c_char;
use std::ffi::CString;
use scannit_core::travelcard::{TravelCard, PeriodPass};
use scannit_core::models::{Language, ProductCode, ValidityArea, VehicleType};
use scannit_core::history::TransactionType;

pub type UnixTimestamp = i64;

/// An FFI-friendly version of a travel card. Note that all dynamically-allocated members
/// in this struct have already had their memory leaked. It is the responsibility of the
/// external consumer to manually return this to Rust code to be freed.
#[repr(C)]
pub struct FFITravelCard {
    pub application_version: u8,
    pub application_key_version: u8,
    pub application_instance_id: *const c_char,
    pub platform_type: u8,
    pub is_mac_protected: bool,
    
    pub application_issuing_date: UnixTimestamp,
    pub application_status: bool,
    pub application_unblocking_number: u8,
    pub application_transaction_counter: u32,
    pub action_list_counter: u32,

    pub period_pass: FFIPeriodPass,

    pub stored_value_cents: u32,    
    pub last_load_datetime: UnixTimestamp,
    pub last_load_value: u32,
    pub last_load_organization_id: u16,
    pub last_load_device_num: u16,

    pub e_ticket: FFIETicket,

    pub history: FFIBuffer<*mut FFIHistory>
}

impl FFITravelCard {
    pub(crate) fn from_travel_card(travel_card: TravelCard) -> FFITravelCard {
        FFITravelCard {
            application_version: travel_card.application_version,
            application_key_version: travel_card.application_key_version,
            // Note that this immediately leaks the memory of the CString.
            application_instance_id: CString::new(travel_card.application_instance_id).unwrap().into_raw(),
            platform_type: travel_card.platform_type,
            is_mac_protected: travel_card.is_mac_protected,

            application_issuing_date: travel_card.application_issuing_date.timestamp(),
            application_status: travel_card.application_status,
            application_unblocking_number: travel_card.application_unblocking_number,
            application_transaction_counter: travel_card.application_transaction_counter,
            action_list_counter: travel_card.action_list_counter,

            period_pass: FFIPeriodPass::from_period_pass(travel_card.period_pass),
         }
    }
}

#[repr(C)]
pub struct FFIPeriodPass {
    pub product_code_1_kind: ProductCodeKind,
    pub product_code_1_value: u16,
    pub validity_area_1_kind: ValidityAreaKind,
    /// This is either a single positive integer, or short list of positive integers.
    /// We'll represent it as something that's always an array.
    pub validity_area_1_value: FFIBuffer<u8>,    
    pub period_start_date_1: UnixTimestamp,

    pub product_code_2_kind: ProductCodeKind,
    pub product_code_2_value: u16,
    pub validity_area_2_kind: ValidityAreaKind,
    /// This is either a single positive integer, or short list of positive integers.
    /// We'll represent it as something that's always an array.
    pub validity_area_2_value: FFIBuffer<u8>,    
    pub period_start_date_2: UnixTimestamp,

    // Most recent card load:
    pub loaded_period_product_kind: ProductCodeKind,
    pub loaded_period_product_value: u16,    
    pub loaded_period_datetime: UnixTimestamp,
    pub loaded_period_length: u16,
    /// In cents.
    pub loaded_period_price: u32,
    pub loading_organization: u16,
    pub loading_device_number: u16,

    // Last use/boarding:    
    pub last_board_datetime: UnixTimestamp,
    pub last_board_vehicle_number: u16,
    pub last_board_location_kind: BoardingLocationKind,
    pub last_board_location_value: u16,
    pub last_board_location_direction_kind: BoardingDirectionKind,
    pub last_board_location_direction_value: u8,
    pub last_board_area_kind: ValidityAreaKind,
    pub last_board_area_value: FFIBuffer<u8>,
}

impl FFIPeriodPass {
    fn from_period_pass(period_pass: PeriodPass) -> FFIPeriodPass {        
        FFIPeriodPass {
            product_code_1_kind: ProductCodeKind::from(period_pass.product_code_1),
            product_code_1_value: u16::from(period_pass.product_code_1),
            validity_area_1_kind: ValidityAreaKind::from(period_pass.validity_area_1),
            validity_area_1_value: FFIBuffer::from(period_pass.validity_area_1),
            period_start_date_1: period_pass.period_start_date_1.and_hms(0, 0, 0).timestamp(),

            product_code_2_kind: ProductCodeKind::from(period_pass.product_code_2),
            product_code_2_value: u16::from(period_pass.product_code_2),
            validity_area_2_kind: ValidityAreaKind::from(period_pass.validity_area_2),
            validity_area_2_value: FFIBuffer::from(period_pass.validity_area_2),
            period_start_date_2: period_pass.period_start_date_2.and_hms(0, 0, 0).timestamp(),

            loaded_period_product_kind: ProductCodeKind::from(period_pass.loaded_period_product),
            loaded_period_product_value: u16::from(period_pass.loaded_period_product),
        }
    }
}

#[repr(C)]
pub struct FFIETicket {
    pub product_code_kind: ProductCodeKind,
    pub product_code_value: u16,
    pub customer_profile: u8,
    pub language: Language,
    pub validity_length_kind: ValidityLengthKind,
    pub validity_length_value: u8,
    pub validity_area_kind: ValidityAreaKind,
    /// This is either a single positive integer, or short list of positive integers.
    /// We'll represent it as something that's always an array.
    pub validity_area_value: FFIBuffer<u8>,    
    pub sale_datetime: UnixTimestamp,
    pub sale_device_kind: SaleDeviceKind,
    pub sale_device_value: u16,
    pub ticket_fare_cents: u16,
    pub group_size: u8,

    //Extension ticket stuff    
    pub extra_zone: bool,
    /// The validity area for the PeriodPass associated with the extra zone ticket.
    pub period_pass_validity_area_kind: ValidityAreaKind,
    /// This is either a single positive integer, or short list of positive integers.
    /// We'll represent it as something that's always an array.
    pub period_pass_validity_area_value: FFIBuffer<u8>,
    pub extension_product_code_kind: ProductCodeKind,
    pub extension_product_code_value: u16,
    pub extension_1_validity_area_kind: ValidityAreaKind,
    /// This is either a single positive integer, or short list of positive integers.
    /// We'll represent it as something that's always an array.
    pub extension_1_validity_area_value: FFIBuffer<u8>,
    pub extension_1_fare_cents: u16,
    pub extension_2_validity_area_kind: ValidityAreaKind,
    /// This is either a single positive integer, or short list of positive integers.
    /// We'll represent it as something that's always an array.
    pub extension_2_validity_area_value: FFIBuffer<u8>,
    pub extension_2_fare_cents: u16,
    pub sale_status: bool,

    // Ticket validity info    
    pub validity_start_datetime: UnixTimestamp,    
    pub validity_end_datetime: UnixTimestamp,
    pub validity_status: bool,

    // Boarding info    
    pub boarding_datetime: UnixTimestamp,
    pub boarding_vehicle: u16,
    pub boarding_location_kind: BoardingLocationKind,
    pub boarding_location_value: u16,
    pub boarding_direction_kind: BoardingLocationKind,
    pub boarding_direction_value: u8,
    pub boarding_area_kind: BoardingAreaKind,
    pub boarding_area_value: u8,
}

pub struct FFIHistory {
    pub transaction_type: TransactionType,    
    pub boarding_datetime: UnixTimestamp,    
    pub transfer_end_datetime: UnixTimestamp,
    pub ticket_fare_cents: u16,
    pub group_size: u8,
    pub remaining_value: u32,
}

impl From<ValidityArea> for FFIBuffer<u8> {
    fn from(val: ValidityArea) -> Self {
        match val {
            ValidityArea::OldZone(zoneNum) => {
                let zoneNumsVec = vec!(zoneNum);
                unsafe { std::mem::forget(zoneNumsVec); }
                FFIBuffer::from(zoneNumsVec)
            },
            ValidityArea::Vehicle(vehicleType) => {
                let vehicleNumsVec = vec!(u8::from(vehicleType));
                unsafe { std::mem::forget(vehicleNumsVec); }
                FFIBuffer::from(vehicleNumsVec)
            },
            ValidityArea::Zone(zones) => {
                let zonesVec = zones.iter().map(|x| u8::from(*x)).collect();
                unsafe { std::mem::forget(zonesVec); }
                FFIBuffer::from(zonesVec)
            }        
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ProductCodeKind {
    FaresFor2010 = 0,
    FaresFor2014 = 1,
}

impl From<ProductCode> for ProductCodeKind {
    fn from(val: ProductCode) -> Self {
        match val {
            ProductCode::FaresFor2010(_) => ProductCodeKind::FaresFor2010,
            ProductCode::FaresFor2014(_) => ProductCodeKind::FaresFor2014
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValidityAreaKind {
    OldZone = 0,
    VehicleType = 1,
    NewZone = 2,
}

impl From<ValidityArea> for ValidityAreaKind {
    fn from(val: ValidityArea) -> Self {
        match val {
            ValidityArea::OldZone(_) => ValidityAreaKind::OldZone,
            ValidityArea::Vehicle(_) => ValidityAreaKind::VehicleType,
            ValidityArea::Zone(_) => ValidityAreaKind::NewZone,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BoardingLocationKind {
    NoneOrReserved = 0,
    BusNumber = 1,
    TrainNumber = 2,
    PlatformNumber = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BoardingDirectionKind {
    TowardEnd = 0,
    TowardStart = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValidityLengthKind {
    Minutes = 0 ,
    Hours = 1,
    TwentyFourHourPeriods = 2,
    Days = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SaleDeviceKind {
    ServicePointSalesDevice = 0,
    DriverTicketMachine = 1,
    CardReader = 2,
    TicketMachine = 3,
    Server = 4,
    HSLSmallEquipment = 5,
    ExternalServiceEquipment = 6,
    Reserved = 7,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BoardingAreaKind {
    Zone = 0,
    Vehicle = 1,
    ZoneCircle = 2,
}