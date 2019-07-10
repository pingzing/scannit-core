use crate::conversion;
use crate::en1545date::from_en1545_date;
use std::convert::TryInto;
use byteorder::{BigEndian, ByteOrder};
use chrono::prelude::*;

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
        BigEndian::read_uint(&control_info[3..6], 24).try_into().unwrap(), // Application transaction counter, 24-bits long
        BigEndian::read_u32(&control_info[6..10]), // Action List Counter, 32-bits long
    )
}
