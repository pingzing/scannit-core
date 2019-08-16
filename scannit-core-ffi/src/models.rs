use crate::ffi::{FFIByteBuffer, FFIHistoryBuffer};
use libc::c_char;
use scannit_core::eticket::ETicket;
use scannit_core::history::{History, TransactionType};
use scannit_core::models::{
    BoardingArea, BoardingDirection, BoardingLocation, Language, ProductCode, SaleDevice,
    ValidityArea, ValidityLength,
};
use scannit_core::travelcard::{PeriodPass, TravelCard};
use std::ffi::CString;

pub type UnixTimestamp = i64;

/// An FFI-friendly version of a travel card. Note that all dynamically-allocated members
/// in this struct have already had their memory leaked. It is the responsibility of the
/// external consumer to manually return this to Rust code to be freed.
#[repr(C)]
pub struct FFITravelCard {
    pub application_version: u8,
    pub application_key_version: u8,
    pub application_instance_id: *mut c_char,
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

    pub history: FFIHistoryBuffer,
}

impl FFITravelCard {
    pub(crate) fn from_travel_card(travel_card: TravelCard) -> FFITravelCard {
        FFITravelCard {
            application_version: travel_card.application_version,
            application_key_version: travel_card.application_key_version,
            // Note that this immediately leaks the memory of the CString.
            application_instance_id: CString::new(travel_card.application_instance_id)
                .unwrap()
                .into_raw(),
            platform_type: travel_card.platform_type,
            is_mac_protected: travel_card.is_mac_protected,

            application_issuing_date: travel_card.application_issuing_date.timestamp(),
            application_status: travel_card.application_status,
            application_unblocking_number: travel_card.application_unblocking_number,
            application_transaction_counter: travel_card.application_transaction_counter,
            action_list_counter: travel_card.action_list_counter,

            period_pass: FFIPeriodPass::from_period_pass(travel_card.period_pass),

            stored_value_cents: travel_card.stored_value_cents,
            last_load_datetime: travel_card.last_load_datetime.timestamp(),
            last_load_value: travel_card.last_load_value,
            last_load_organization_id: travel_card.last_load_organization_id,
            last_load_device_num: travel_card.last_load_device_num,

            e_ticket: FFIETicket::from_e_ticket(travel_card.e_ticket),

            history: FFIHistoryBuffer::from(travel_card.history),
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
    pub validity_area_1_value: FFIByteBuffer,
    pub period_start_date_1: UnixTimestamp,
    pub period_end_date_1: UnixTimestamp,

    pub product_code_2_kind: ProductCodeKind,
    pub product_code_2_value: u16,
    pub validity_area_2_kind: ValidityAreaKind,
    /// This is either a single positive integer, or short list of positive integers.
    /// We'll represent it as something that's always an array.
    pub validity_area_2_value: FFIByteBuffer,
    pub period_start_date_2: UnixTimestamp,
    pub period_end_date_2: UnixTimestamp,

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
    pub last_board_direction: BoardingDirection,
    pub last_board_area_kind: BoardingAreaKind,
    pub last_board_area_value: u8,
}

impl FFIPeriodPass {
    fn from_period_pass(period_pass: PeriodPass) -> FFIPeriodPass {
        FFIPeriodPass {
            product_code_1_kind: ProductCodeKind::from(&period_pass.product_code_1),
            product_code_1_value: u16::from(&period_pass.product_code_1),
            validity_area_1_kind: ValidityAreaKind::from(&period_pass.validity_area_1),
            validity_area_1_value: FFIByteBuffer::from(period_pass.validity_area_1),
            period_start_date_1: period_pass.period_start_date_1.and_hms(0, 0, 0).timestamp(),
            period_end_date_1: period_pass.period_end_date_1.and_hms(0, 0, 0).timestamp(),

            product_code_2_kind: ProductCodeKind::from(&period_pass.product_code_2),
            product_code_2_value: u16::from(&period_pass.product_code_2),
            validity_area_2_kind: ValidityAreaKind::from(&period_pass.validity_area_2),
            validity_area_2_value: FFIByteBuffer::from(period_pass.validity_area_2),
            period_start_date_2: period_pass.period_start_date_2.and_hms(0, 0, 0).timestamp(),
            period_end_date_2: period_pass.period_end_date_2.and_hms(0, 0, 0).timestamp(),

            loaded_period_product_kind: ProductCodeKind::from(&period_pass.loaded_period_product),
            loaded_period_product_value: u16::from(&period_pass.loaded_period_product),
            loaded_period_datetime: period_pass.loaded_period_datetime.timestamp(),
            loaded_period_length: period_pass.loaded_period_length,
            loaded_period_price: period_pass.loaded_period_price,
            loading_organization: period_pass.loading_organization,
            loading_device_number: period_pass.loading_device_number,

            last_board_datetime: period_pass.last_board_datetime.timestamp(),
            last_board_vehicle_number: period_pass.last_board_vehicle_number,
            last_board_location_kind: BoardingLocationKind::from(&period_pass.last_board_location),
            last_board_location_value: u16::from(&period_pass.last_board_location),
            last_board_direction: period_pass.last_board_direction,
            last_board_area_kind: BoardingAreaKind::from(&period_pass.last_board_area),
            last_board_area_value: u8::from(&period_pass.last_board_area),
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
    pub validity_area_value: FFIByteBuffer,
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
    pub period_pass_validity_area_value: FFIByteBuffer,
    pub extension_product_code_kind: ProductCodeKind,
    pub extension_product_code_value: u16,
    pub extension_1_validity_area_kind: ValidityAreaKind,
    /// This is either a single positive integer, or short list of positive integers.
    /// We'll represent it as something that's always an array.
    pub extension_1_validity_area_value: FFIByteBuffer,
    pub extension_1_fare_cents: u16,
    pub extension_2_validity_area_kind: ValidityAreaKind,
    /// This is either a single positive integer, or short list of positive integers.
    /// We'll represent it as something that's always an array.
    pub extension_2_validity_area_value: FFIByteBuffer,
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
    pub boarding_direction: BoardingDirection,
    pub boarding_area_kind: BoardingAreaKind,
    pub boarding_area_value: u8,
}

impl FFIETicket {
    fn from_e_ticket(e_ticket: ETicket) -> FFIETicket {
        FFIETicket {
            product_code_kind: ProductCodeKind::from(&e_ticket.product_code),
            product_code_value: u16::from(&e_ticket.product_code),
            customer_profile: e_ticket.customer_profile,
            language: e_ticket.language,
            validity_length_kind: ValidityLengthKind::from(&e_ticket.validity_length),
            validity_length_value: u8::from(&e_ticket.validity_length),
            validity_area_kind: ValidityAreaKind::from(&e_ticket.validity_area),
            validity_area_value: FFIByteBuffer::from(e_ticket.validity_area),
            sale_datetime: e_ticket.sale_datetime.timestamp(),
            sale_device_kind: SaleDeviceKind::from(&e_ticket.sale_device),
            sale_device_value: u16::from(&e_ticket.sale_device),
            ticket_fare_cents: e_ticket.ticket_fare_cents,
            group_size: e_ticket.group_size,

            extra_zone: e_ticket.extra_zone,
            period_pass_validity_area_kind: ValidityAreaKind::from(
                &e_ticket.period_pass_validity_area,
            ),
            period_pass_validity_area_value: FFIByteBuffer::from(
                e_ticket.period_pass_validity_area,
            ),
            extension_product_code_kind: ProductCodeKind::from(&e_ticket.extension_product_code),
            extension_product_code_value: u16::from(&e_ticket.extension_product_code),
            extension_1_validity_area_kind: ValidityAreaKind::from(
                &e_ticket.extension_1_validity_area,
            ),
            extension_1_validity_area_value: FFIByteBuffer::from(
                e_ticket.extension_1_validity_area,
            ),
            extension_1_fare_cents: e_ticket.extension_1_fare_cents,
            extension_2_validity_area_kind: ValidityAreaKind::from(
                &e_ticket.extension_2_validity_area,
            ),
            extension_2_validity_area_value: FFIByteBuffer::from(
                e_ticket.extension_2_validity_area,
            ),
            extension_2_fare_cents: e_ticket.extension_2_fare_cents,
            sale_status: e_ticket.sale_status,
            validity_start_datetime: e_ticket.validity_start_datetime.timestamp(),
            validity_end_datetime: e_ticket.validity_end_datetime.timestamp(),
            validity_status: e_ticket.validity_status,

            boarding_datetime: e_ticket.boarding_datetime.timestamp(),
            boarding_vehicle: e_ticket.boarding_vehicle,
            boarding_location_kind: BoardingLocationKind::from(&e_ticket.boarding_location),
            boarding_location_value: u16::from(&e_ticket.boarding_location),
            boarding_direction: e_ticket.boarding_direction,
            boarding_area_kind: BoardingAreaKind::from(&e_ticket.boarding_area),
            boarding_area_value: u8::from(&e_ticket.boarding_area),
        }
    }
}

#[repr(C)]
pub struct FFIHistory {
    pub transaction_type: TransactionType,
    pub boarding_datetime: UnixTimestamp,
    pub transfer_end_datetime: UnixTimestamp,
    pub ticket_fare_cents: u16,
    pub group_size: u8,
    pub remaining_value: u32,
}

impl FFIHistory {
    fn from_history(history: &History) -> FFIHistory {
        FFIHistory {
            transaction_type: history.transaction_type,
            boarding_datetime: history.boarding_datetime.timestamp(),
            transfer_end_datetime: history.transfer_end_datetime.timestamp(),
            ticket_fare_cents: history.ticket_fare_cents,
            group_size: history.group_size,
            remaining_value: history.remaining_value,
        }
    }
}

impl From<ValidityArea> for FFIByteBuffer {
    fn from(val: ValidityArea) -> Self {
        match val {
            ValidityArea::OldZone(zone_num) => {
                let mut zone_nums_vec = vec![zone_num];
                let ffi_buffer = FFIByteBuffer::from(&mut zone_nums_vec);
                std::mem::forget(zone_nums_vec);
                ffi_buffer
            }
            ValidityArea::Vehicle(vehicle_type) => {
                let mut vehicle_nums_vec = vec![u8::from(&vehicle_type)];
                let ffi_buffer = FFIByteBuffer::from(&mut vehicle_nums_vec);
                std::mem::forget(vehicle_nums_vec);
                ffi_buffer
            }
            ValidityArea::Zone(zones) => {
                let mut zones_vec: Vec<u8> = zones.iter().map(u8::from).collect();
                let ffi_buffer = FFIByteBuffer::from(&mut zones_vec);
                std::mem::forget(zones_vec);
                ffi_buffer
            }
        }
    }
}

impl From<Vec<History>> for FFIHistoryBuffer {
    fn from(val: Vec<History>) -> Self {
        let mut ffi_histories: Vec<FFIHistory> =
            val.iter().map(|x| FFIHistory::from_history(x)).collect();
        let ffi_buffer = FFIHistoryBuffer::from(&mut ffi_histories);
        std::mem::forget(ffi_histories);
        ffi_buffer
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ProductCodeKind {
    FaresFor2010 = 0,
    FaresFor2014 = 1,
}

impl From<&ProductCode> for ProductCodeKind {
    fn from(val: &ProductCode) -> Self {
        match val {
            ProductCode::FaresFor2010(_) => ProductCodeKind::FaresFor2010,
            ProductCode::FaresFor2014(_) => ProductCodeKind::FaresFor2014,
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

impl From<&ValidityArea> for ValidityAreaKind {
    fn from(val: &ValidityArea) -> Self {
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

impl From<&BoardingLocation> for BoardingLocationKind {
    fn from(val: &BoardingLocation) -> Self {
        match val {
            BoardingLocation::NoneOrReserved => BoardingLocationKind::NoneOrReserved,
            BoardingLocation::BusNumber(_) => BoardingLocationKind::BusNumber,
            BoardingLocation::TrainNumber(_) => BoardingLocationKind::TrainNumber,
            BoardingLocation::PlatformNumber(_) => BoardingLocationKind::PlatformNumber,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValidityLengthKind {
    Minutes = 0,
    Hours = 1,
    TwentyFourHourPeriods = 2,
    Days = 3,
}

impl From<&ValidityLength> for ValidityLengthKind {
    fn from(val: &ValidityLength) -> Self {
        match val {
            ValidityLength::Minutes(_) => ValidityLengthKind::Minutes,
            ValidityLength::Hours(_) => ValidityLengthKind::Hours,
            ValidityLength::TwentyFourHourPeriods(_) => ValidityLengthKind::TwentyFourHourPeriods,
            ValidityLength::Days(_) => ValidityLengthKind::Days,
        }
    }
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

impl From<&SaleDevice> for SaleDeviceKind {
    fn from(val: &SaleDevice) -> Self {
        match val {
            SaleDevice::ServicePointSalesDevice(_) => SaleDeviceKind::ServicePointSalesDevice,
            SaleDevice::DriverTicketMachine(_) => SaleDeviceKind::DriverTicketMachine,
            SaleDevice::CardReader(_) => SaleDeviceKind::CardReader,
            SaleDevice::TicketMachine(_) => SaleDeviceKind::TicketMachine,
            SaleDevice::Server(_) => SaleDeviceKind::Server,
            SaleDevice::HSLSmallEquipment(_) => SaleDeviceKind::HSLSmallEquipment,
            SaleDevice::ExternalServiceEquipment(_) => SaleDeviceKind::ExternalServiceEquipment,
            SaleDevice::Reserved(_) => SaleDeviceKind::Reserved,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BoardingAreaKind {
    Zone = 0,
    Vehicle = 1,
    ZoneCircle = 2,
}

impl From<&BoardingArea> for BoardingAreaKind {
    fn from(val: &BoardingArea) -> Self {
        match val {
            BoardingArea::Zone(_) => BoardingAreaKind::Zone,
            BoardingArea::Vehicle(_) => BoardingAreaKind::Vehicle,
            BoardingArea::ZoneCircle(_) => BoardingAreaKind::ZoneCircle,
        }
    }
}
