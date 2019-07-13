use crate::conversion::*;
use crate::en1545date::{from_en1545_date, from_en1545_date_and_time};
use byteorder::{BigEndian, ByteOrder};
use chrono::prelude::*;
use std::convert::TryInto;

pub struct TravelCard {
    // Application Info
    pub application_version: u8,
    pub application_key_version: u8,
    pub application_instance_id: String,
    pub platform_type: u8,
    pub is_mac_protected: bool,

    // Control info
    pub application_issuing_date: DateTime<Utc>,
    pub application_status: bool,
    pub application_unblocking_number: u8,
    pub application_transaction_counter: u32,
    pub action_list_counter: u32,

    pub period_pass: PeriodPass
}

pub struct PeriodPass {
    pub product_code_1: ProductCode,    
    pub validity_area_1: ValidityArea,    
    pub period_start_date_1: Date<Utc>,
    pub period_end_date_1: Date<Utc>,

    pub product_code_2: ProductCode,        
    pub validity_area_2: ValidityArea,
    pub period_start_date_2: Date<Utc>,
    pub period_end_date_2: Date<Utc>,

    // Most recent card load:
    pub loaded_period_product: ProductCode,    
    pub loaded_period_datetime: DateTime<Utc>,
    pub loaded_period_length: u16,
    pub loaded_period_price: u32, // in cents
    pub loading_organization: u16,
    pub loading_device_number: u16,

    // Last use/boarding:
    pub last_board_datetime: DateTime<Utc>,
    pub last_board_vehicle_number: u16,
    pub last_board_location: BoardingLocation,
    pub last_board_direction: BoardingDirection,
    pub last_board_area: ValidityArea    
}

pub fn create_travel_card(
    app_info: &[u8],
    control_info: &[u8],
    period_pass: &[u8],
    storedValue: &[u8],
    eTicket: &[u8],
    history: &[u8],
) -> TravelCard {
    let (app_version, app_key_version, app_instance_id, platform, is_protected) =
        read_application_info(app_info);
    let (issue_date, app_status, unblock_number, transaction_counter, action_counter) =
        read_control_info(control_info);
    let period_pass = read_period_pass(period_pass);

    TravelCard {
        application_version: app_version,
        application_key_version: app_key_version,
        application_instance_id: app_instance_id,
        platform_type: platform,
        is_mac_protected: is_protected,
        application_issuing_date: issue_date,
        application_status: app_status,
        application_unblocking_number: unblock_number,
        application_transaction_counter: transaction_counter,
        action_list_counter: action_counter,
        period_pass: period_pass,
    }
}

// Notes about travel card data: All data is presented as a pile of bytes,
// and all bytes are expressed in Big Endian format.

fn read_application_info(app_info: &[u8]) -> (u8, u8, String, u8, bool) {
    (
        app_info[0] & 0xF0,                          // Application Version
        app_info[0] & 0x0F,                          // Application Key Version
        as_hex_string(&app_info[1..10]), // Application Instance ID
        app_info[10] & 0xE0,                         // Platform Type, 0 = NXP DESFire 4kB.
        (app_info[10] & 0x10) != 0, // SecurityLevel, which is a 1-bit field. 0 = open, 1 = MAC protected.
    )
}

fn read_control_info(control_info: &[u8]) -> (DateTime<Utc>, bool, u8, u32, u32) {
    let date = (((control_info[0] as u16) << 8) | control_info[1] as u16) >> 2; // Shift out the least-significant two bits, dates are only 14-bits long.
    (
        from_en1545_date(date),
        control_info[1] & 0x2 != 0,    // 1-bit app status (no idea what status *means*, but...)
        control_info[2],               // 8-bit 'unblocking number' (ditto, no idea)
        BigEndian::read_uint(&control_info[3..6], 3)
            .try_into()
            .unwrap(), // Application transaction counter, 24-bits long
        BigEndian::read_u32(&control_info[6..10]), // Action List Counter, 32-bits long
    )
}

fn read_period_pass(period_pass: &[u8]) -> PeriodPass {    
    let product_code_type_1 = get_bits_as_u8(period_pass, 0, 1);
    let product_code_1 = get_bits_as_u16(period_pass, 1, 14);
    let validity_area_type_1 = get_bits_as_u8(period_pass, 15, 2);
    let validity_area_1 = get_bits_as_u8(period_pass, 17, 6);
    let start_date_1 = get_bits_as_u16(period_pass, 23, 14);
    let end_date_1 = get_bits_as_u16(period_pass, 37, 14);
    
    let product_code_type_2 = get_bits_as_u8(period_pass, 56, 1);
    let product_code_2 = get_bits_as_u16(period_pass, 57, 14);
    let validity_area_type_2 = get_bits_as_u8(period_pass, 71, 2);
    let validity_area_2 = get_bits_as_u8(period_pass, 73, 6);
    let start_date_2 = get_bits_as_u16(period_pass, 79, 14);
    let end_date_2 = get_bits_as_u16(period_pass, 93, 14);

    let loaded_period_product_type = get_bits_as_u8(period_pass, 112, 1);
    let loaded_period_product = get_bits_as_u16(period_pass, 113, 14);
    let loaded_period_date = get_bits_as_u16(period_pass, 127, 14);
    let loaded_period_time = get_bits_as_u16(period_pass, 141, 11);
    let loaded_period_length = get_bits_as_u16(period_pass, 152, 9);
    let loaded_period_price = get_bits_as_u32(period_pass, 161, 20);
    let loading_organization = get_bits_as_u16(period_pass, 181, 14);
    let loading_device_number = get_bits_as_u16(period_pass, 195, 13);

    let last_board_date = get_bits_as_u16(period_pass, 208, 14);
    let last_board_time = get_bits_as_u16(period_pass, 222, 11);
    let last_board_vehicle_number = get_bits_as_u16(period_pass, 233, 14);
    let last_board_location_num_type = get_bits_as_u8(period_pass, 247, 2);
    let last_board_location_num = get_bits_as_u16(period_pass, 249, 14);
    let last_board_direction = get_bits_as_u8(period_pass, 264, 1);
    let last_board_area_type = get_bits_as_u8(period_pass, 264, 2);
    let last_board_area = get_bits_as_u8(period_pass, 266, 6);
    PeriodPass {
        product_code_1: ProductCode::new(product_code_type_1, product_code_1),
        validity_area_1: ValidityArea::new(validity_area_type_1, validity_area_1),
        period_start_date_1: from_en1545_date(start_date_1).date(),
        period_end_date_1: from_en1545_date(end_date_1).date(),

        product_code_2: ProductCode::new(product_code_type_2, product_code_2),
        validity_area_2: ValidityArea::new(validity_area_type_2, validity_area_2),
        period_start_date_2: from_en1545_date(start_date_2).date(),
        period_end_date_2: from_en1545_date(end_date_2).date(),

        loaded_period_product: ProductCode::new(loaded_period_product_type, loaded_period_product),
        loaded_period_datetime: from_en1545_date_and_time(loaded_period_date, loaded_period_time),
        loaded_period_length: loaded_period_length,
        loaded_period_price: loaded_period_price,
        loading_organization: loading_organization,
        loading_device_number: loading_device_number,

        last_board_datetime: from_en1545_date_and_time(last_board_date, last_board_time),
        last_board_vehicle_number: last_board_vehicle_number,
        last_board_location: BoardingLocation::new(last_board_location_num_type, last_board_location_num),
        last_board_direction: BoardingDirection::new(last_board_direction),
        last_board_area: ValidityArea::new(last_board_area_type, last_board_area)
    }
}

pub enum ProductCode {
    FaresFor2010(u16), // Code type = 0
    FaresFor2014(u16), // Code type = 1
}

impl ProductCode {
    fn new(code_type: u8, value: u16) -> ProductCode {
        if code_type == 0 { ProductCode::FaresFor2010(value) }
        else { ProductCode::FaresFor2014(value) }
    }
}

/// The number of a boarded element.
pub enum BoardingLocation {
    BusNumber(u16),
    TrainNumber(u16),    
    PlatformNumber(u16),
}

impl BoardingLocation{
    fn new(boarding_area_type: u8, boarding_area_value: u16) -> BoardingLocation {
        match boarding_area_type {
            1 => BoardingLocation::BusNumber(boarding_area_value),
            2 => BoardingLocation::TrainNumber(boarding_area_value),
            3 => BoardingLocation::PlatformNumber(boarding_area_value),
            _ => panic!("Given value for BoardingLocation not supported.")
        }
    }
}

/// This enum is pure speculation--the underlying value is a single bit. What else _could_ it mean?
pub enum BoardingDirection {
    /// Indicates that at the time of boarding, the transit medium  was headed toward the end of its route.
    TowardEnd,
    /// Indicates that at the time of boarding, the transit medium was headed toward the start of its route.
    TowardStart
}

impl BoardingDirection {
    fn new(value: u8) -> BoardingDirection {
        match value {
            0 => BoardingDirection::TowardEnd,
            1 => BoardingDirection::TowardStart,
            _ => panic!("Given value for BoardingDirection not supported.")
        }
    }
}

/// Represents an area in which, or a vehicle for which, a ticket is valid.
pub enum ValidityArea {
    Zone(Vec<ValidityZone>),
    Vehicle(VehicleType)
}

impl ValidityArea {
    fn new(area_type: u8, area_value: u8) -> ValidityArea {
        let mut zones: Vec<ValidityZone> = Vec::new();
        if area_type == 0 { 
            let from_zone = area_value & 0b0000_0111; //rightmost 3 bits
            let to_zone = (area_value & 0b0011_1000) >> 3; // 3 bits to the left of that
            for val in from_zone..to_zone {
                zones.push(ValidityZone::from_u8(val));
            }
            ValidityArea::Zone(zones) 
        }
        else { ValidityArea::Vehicle(VehicleType::from_u8(area_value)) }
    }
}

/// The HSL fare zone(s) in which a ticket is valid.
#[derive(Clone)]
pub enum ValidityZone {
    ZoneA = 0,
    ZoneB = 1,
    ZoneC = 2,
    ZoneD = 3,
    ZoneE = 4,
    ZoneF = 5,
    ZoneG = 6,
    ZoneH = 7,
}

impl ValidityZone {
    fn from_u8(value: u8) -> ValidityZone {
        match value {
            0 => ValidityZone::ZoneA,
            1 => ValidityZone::ZoneB,
            2 => ValidityZone::ZoneC,
            3 => ValidityZone::ZoneD,
            4 => ValidityZone::ZoneE,
            5 => ValidityZone::ZoneF,
            6 => ValidityZone::ZoneG,
            7 => ValidityZone::ZoneH,
            _ => panic!("Given value for ValidityZone not supported.")
        }
    }
}

/// The vehicle type on which this ticket is valid.
pub enum VehicleType {
    Undefined = 0,
    Bus = 1,
    Tram = 5,
    Metro = 6,
    Train = 7,
    Ferry = 8,
    ULine = 9,
}

impl VehicleType {
    fn from_u8(value: u8) -> VehicleType {
        match value {
            0 => VehicleType::Undefined,
            1 => VehicleType::Bus,
            5 => VehicleType::Tram,
            6 => VehicleType::Metro,
            7 => VehicleType::Train,
            8 => VehicleType::Ferry,
            9 => VehicleType::ULine,
            _ => panic!("Given value for VehicleType not supported.")
        }
    }
}