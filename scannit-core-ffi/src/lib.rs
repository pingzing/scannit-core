mod ffi;
pub mod models;

use models::FFITravelCard;
use scannit_core::desfire::{
    ERROR_RESPONSE, GET_APPLICATION_IDS_COMMAND, GET_VERSION_COMMAND, MORE_DATA_RESPONSE,
    OK_RESPONSE, READ_APP_INFO_COMMAND, READ_CONTROL_INFO_COMMAND, READ_E_TICKET_COMMAND,
    READ_HISTORY_COMMAND, READ_NEXT_COMMAND, READ_PERIOD_PASS_COMMAND, READ_STORED_VALUE_COMMAND,
    SELECT_HSL_COMMAND,
};
use scannit_core::travelcard;

#[no_mangle]
pub unsafe extern "C" fn create_travel_card(
    app_info_ptr: *const u8,
    app_info_size: usize,
    control_info_ptr: *const u8,
    control_info_size: usize,
    period_pass_ptr: *const u8,
    period_pass_size: usize,
    stored_value_ptr: *const u8,
    stored_value_size: usize,
    e_ticket_ptr: *const u8,
    e_ticket_size: usize,
    history_ptr: *const u8,
    history_size: usize,
) -> *mut FFITravelCard {
    let app_info;
    let control_info;
    let period_pass;
    let stored_value;
    let e_ticket;
    let history;    

    // Actual unsafety begins here
    app_info = std::slice::from_raw_parts(app_info_ptr, app_info_size);
    control_info = std::slice::from_raw_parts(control_info_ptr, control_info_size);
    period_pass = std::slice::from_raw_parts(period_pass_ptr, period_pass_size);
    stored_value = std::slice::from_raw_parts(stored_value_ptr, stored_value_size);
    e_ticket = std::slice::from_raw_parts(e_ticket_ptr, e_ticket_size);
    history = std::slice::from_raw_parts(history_ptr, history_size);
    // Unsafety ends here

    let travelcard = travelcard::create_travel_card(
        app_info,
        control_info,
        period_pass,
        stored_value,
        e_ticket,
        history,
    );

    let ffi_travel_card = FFITravelCard::from_travel_card(travelcard);
    Box::into_raw(Box::from(ffi_travel_card))
}

#[no_mangle]
pub unsafe extern "C" fn free_travel_card(travel_card_ptr: *mut FFITravelCard) {
    let travel_card = Box::from_raw(travel_card_ptr);
    ffi::free_string(travel_card.application_instance_id);
    ffi::free_history_buffer(travel_card.history);

    ffi::free_byte_buffer(travel_card.period_pass.validity_area_1_value);
    ffi::free_byte_buffer(travel_card.period_pass.validity_area_2_value);

    ffi::free_byte_buffer(travel_card.e_ticket.validity_area_value);
    ffi::free_byte_buffer(travel_card.e_ticket.period_pass_validity_area_value);
    ffi::free_byte_buffer(travel_card.e_ticket.extension_1_validity_area_value);
    ffi::free_byte_buffer(travel_card.e_ticket.extension_2_validity_area_value);
}

// The following don't need free() functions, because they're constant--
// they never get freed anyway.

#[no_mangle]
pub extern "C" fn get_GET_VERSION_COMMAND() -> *const u8 {
    GET_VERSION_COMMAND.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_GET_APPLICATION_IDS_COMMAND() -> *const u8 {
    GET_APPLICATION_IDS_COMMAND.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_SELECT_HSL_COMMAND() -> *const u8 {
    SELECT_HSL_COMMAND.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_READ_APP_INFO_COMMAND() -> *const u8 {
    READ_APP_INFO_COMMAND.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_READ_CONTROL_INFO_COMMAND() -> *const u8 {
    READ_CONTROL_INFO_COMMAND.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_READ_PERIOD_PASS_COMMAND() -> *const u8 {
    READ_PERIOD_PASS_COMMAND.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_READ_STORED_VALUE_COMMAND() -> *const u8 {
    READ_STORED_VALUE_COMMAND.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_READ_E_TICKET_COMMAND() -> *const u8 {
    READ_E_TICKET_COMMAND.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_READ_HISTORY_COMMAND() -> *const u8 {
    READ_HISTORY_COMMAND.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_READ_NEXT_COMMAND() -> *const u8 {
    READ_NEXT_COMMAND.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_OK_RESPONSE() -> *const u8 {
    OK_RESPONSE.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_ERROR_RESPONSE() -> *const u8 {
    ERROR_RESPONSE.as_ptr()
}

#[no_mangle]
pub extern "C" fn get_MORE_DATA_RESPONSE() -> *const u8 {
    MORE_DATA_RESPONSE.as_ptr()
}