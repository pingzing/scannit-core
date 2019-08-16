use crate::conversion::*;
use crate::en1545date::{from_en1545_date, from_en1545_date_and_time};
use crate::eticket::*;
use crate::history::*;
use crate::models::*;
use chrono::prelude::*;

#[derive(Debug)]
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

    // Period pass
    pub period_pass: PeriodPass,

    // Last load info
    pub stored_value_cents: u32,
    pub last_load_datetime: DateTime<Utc>,
    pub last_load_value: u32,
    pub last_load_organization_id: u16,
    pub last_load_device_num: u16,

    // E-Ticket
    pub e_ticket: ETicket,

    // History
    pub history: Vec<History>,
}

#[derive(Debug)]
pub struct PeriodPass {
    pub product_code_1: ProductCode,
    pub validity_area_1: ValidityArea,
    pub period_start_date_1: Date<Utc>,
    pub period_end_date_1: Date<Utc>,

    // This _seems_ to be the last-known season pass before the switchover to the new card format.
    // Probably part of the migration path when they were doing the changeover.
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
    pub last_board_area: BoardingArea,
}

pub fn create_travel_card(
    app_info: &[u8],
    control_info: &[u8],
    period_pass: &[u8],
    stored_value: &[u8],
    e_ticket: &[u8],
    history: &[u8],
) -> TravelCard {
    println!(
        "Lengths: app_info: {:?}, period_pass: {:?}, history: {:?}",
        app_info.len(),
        period_pass.len(),
        history.len()
    );

    let (app_version, app_key_version, app_instance_id, platform, is_protected) =
        read_application_info(app_info);
    let (issue_date, app_status, unblock_number, transaction_counter, action_counter) =
        read_control_info(control_info);
    let period_pass = read_period_pass(period_pass);
    let stored_value = read_stored_value(stored_value);
    let e_ticket = create_e_ticket(e_ticket);
    let history = create_history_entries(history);

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

        period_pass,

        stored_value_cents: stored_value.cents,
        last_load_datetime: stored_value.last_load_datetime,
        last_load_value: stored_value.last_load_value,
        last_load_organization_id: stored_value.last_load_organization_id,
        last_load_device_num: stored_value.last_load_device_num,

        e_ticket,
        history,
    }
}

// Notes about travel card data: All data is presented as a pile of bytes,
// and all bytes are expressed in Big Endian format.

fn read_application_info(app_info: &[u8]) -> (u8, u8, String, u8, bool) {
    (
        get_bits_as_u8(app_info, 0, 4),       // Application Version
        get_bits_as_u8(app_info, 4, 4), // Application Key Version (though the spec sheet marks it as "reserved")
        as_hex_string(&app_info[1..10]), // Application Instance ID (aka the card's unique ID number)
        get_bits_as_u8(app_info, 80, 3), // Platform Type, 0 = NXP DESFire 4kB.
        get_bits_as_u8(app_info, 83, 1) != 0, // SecurityLevel, which is a 1-bit field. 0 = open, 1 = MAC protected.
    )
}

fn read_control_info(control_info: &[u8]) -> (DateTime<Utc>, bool, u8, u32, u32) {
    let issuing_date = get_bits_as_u16(control_info, 0, 14);
    (
        from_en1545_date(issuing_date),
        get_bits_as_u8(control_info, 14, 1) != 0, // 1-bit app status (no idea what status *means*, but...)
        // Skip a single reserved bit here
        get_bits_as_u8(control_info, 16, 8), // 8-bit 'unblocking number' (ditto, no idea)
        get_bits_as_u32(control_info, 24, 24), // Application transaction counter, 24-bits long
        get_bits_as_u32(control_info, 48, 32), // Action List Counter, 32-bits long
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
    let last_board_direction = get_bits_as_u8(period_pass, 263, 1);
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
        loaded_period_length,
        loaded_period_price,
        loading_organization,
        loading_device_number,

        last_board_datetime: from_en1545_date_and_time(last_board_date, last_board_time),
        last_board_vehicle_number,
        last_board_location: BoardingLocation::new(
            last_board_location_num_type,
            last_board_location_num,
        ),
        last_board_direction: BoardingDirection::from(last_board_direction),
        last_board_area: BoardingArea::new(last_board_area_type, last_board_area),
    }
}

fn read_stored_value(stored_value: &[u8]) -> StoredValue {
    let last_load_date = get_bits_as_u16(stored_value, 20, 14);
    let last_load_time = get_bits_as_u16(stored_value, 34, 11);

    StoredValue {
        cents: get_bits_as_u32(stored_value, 0, 20),
        last_load_datetime: from_en1545_date_and_time(last_load_date, last_load_time),
        last_load_value: get_bits_as_u32(stored_value, 45, 20),
        last_load_organization_id: get_bits_as_u16(stored_value, 65, 14),
        last_load_device_num: get_bits_as_u16(stored_value, 79, 14),
    }
}

struct StoredValue {
    cents: u32,
    last_load_datetime: DateTime<Utc>,
    last_load_value: u32,
    last_load_organization_id: u16,
    last_load_device_num: u16,
}
