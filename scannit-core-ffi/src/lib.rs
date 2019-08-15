mod ffi;
pub mod models;

use models::FFITravelCard;
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
) -> FFITravelCard {
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

    FFITravelCard::from_travel_card(travelcard)
}

#[no_mangle]
pub extern "C" fn free_travel_card(travel_card: FFITravelCard) {
    ffi::free_string(travel_card.application_instance_id);
    ffi::free_buffer(travel_card.history);

    ffi::free_buffer(travel_card.period_pass.validity_area_1_value);
    ffi::free_buffer(travel_card.period_pass.validity_area_2_value);
    ffi::free_buffer(travel_card.period_pass.last_board_area_value);

    ffi::free_buffer(travel_card.e_ticket.validity_area_value);
    ffi::free_buffer(travel_card.e_ticket.period_pass_validity_area_value);
    ffi::free_buffer(travel_card.e_ticket.extension_1_validity_area_value);
    ffi::free_buffer(travel_card.e_ticket.extension_2_validity_area_value);
}
