use crate::conversion;
use crate::en1545date::from_en1545_date;
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
}

pub enum ProductCode {
    FaresFor2010(u16), // Code type = 0
    FaresFor2014(u16), // Code type = 1
}

pub enum BoardingLocation {
    BusNumber(u16),
    TrainNumber(u16),    
    PlatformNumber(u16),
}

pub enum ValidityAreaType {
    Zone = 0,
    Vehicle = 1,
}

pub enum ValidityArea {
    ZoneA = 0,
    ZoneB = 1,
    ZoneC = 2,
    ZoneD = 3,
    ZoneE = 4,
    ZoneF = 5,
    ZoneG = 6,
    ZoneH = 7,
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
    }
}

struct PeriodPass {
    product_code_1: ProductCode,    
    validity_area_1_type: ValidityAreaType,
    validity_area_1: ValidityArea,
    period_start_date_1: Date<Utc>,
    period_end_date_1: Date<Utc>,

    product_code_2: ProductCode,    
    validity_area_2_type: ValidityAreaType,
    validity_area_2: ValidityArea,
    period_start_date_2: Date<Utc>,
    period_end_date_2: Date<Utc>,

    // Most recent card load:
    loaded_period_product: ProductCode,    
    period_loading_datetime: DateTime<Utc>,
    loaded_period_length: u16,
    loaded_period_price: u32, // in cents
    loading_organization: u16,
    loading_device_number: u16,

    // Last use/boarding:
    last_board_datetime: DateTime<Utc>,
    last_board_vehicle: u16, // vehicle ID
    last_board_location: BoardingLocation
}

// Notes about travel card data: All data is presented as a pile of bytes,
// and all bytes are expressed in Big Endian format.

fn read_application_info(app_info: &[u8]) -> (u8, u8, String, u8, bool) {
    (
        app_info[0] & 0xF0,                          // Application Version
        app_info[0] & 0x0F,                          // Application Key Version
        conversion::as_hex_string(&app_info[1..10]), // Application Instance ID
        app_info[10] & 0xE0,                         // Platform Type, 0 = NXP DESFire 4kB.
        (app_info[10] & 0x10) != 0, // SecurityLevel, which is a 1-bit field. 0 = open, 1 = MAC protected.
    )
}

fn read_control_info(control_info: &[u8]) -> (DateTime<Utc>, bool, u8, u32, u32) {
    let date = ((control_info[0] as u16) << 8) | control_info[1] as u16;
    (
        from_en1545_date((date) >> 2), // Shift out the least-significant two bits, dates are only 14-bits long.
        control_info[1] & 0x2 != 0,    // 1-bit app status (no idea what status *means*, but...)
        control_info[2],               // 8-bit 'unblocking number' (ditto, no idea)
        BigEndian::read_uint(&control_info[3..6], 24)
            .try_into()
            .unwrap(), // Application transaction counter, 24-bits long
        BigEndian::read_u32(&control_info[6..10]), // Action List Counter, 32-bits long
    )
}

fn read_period_pass(period_pass: &[u8]) -> PeriodPass {
    PeriodPass {        

    }
}