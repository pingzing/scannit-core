use crate::conversion::*;
use crate::en1545date::*;
use crate::models::*;
use chrono::prelude::*;

pub struct ETicket {
    /// If ProductCodeGroup is > 0, this returns ProductCodeGroup
    pub product_code: ProductCode,
    /// If CustomerProfileGroup is > 0, this returns CustomerProfileGroup.
    pub customer_profile: u8,
    pub language: Language,
    /// If ValidityLengthGroup is > 0 this returns ValidityLengthGroup.
    pub validity_length: ValidityLength,
    pub validity_area: ValidityArea,
    pub sale_datetime: DateTime<Utc>,
    pub sale_device_type: SaleDeviceType,
    pub sale_device_number: u16,
    /// If TicketFareGroup is > 0 this returns TicketFareGroup.
    pub ticket_fare_cents: u16,
    pub group_size: u8,

    // --- Extension ticket stuff ---
    /// if true, this is an extra zone on top of a PeriodPass.
    pub extra_zone: bool,
    /// The validity area for the PeriodPass associated with the extra zone ticket.
    pub period_pass_validity_area: ValidityArea,
    pub extension_product_code: ProductCode,
    pub extension_1_validity_area: ValidityArea,
    pub extension_1_fare_cents: u16,
    pub extension_2_validity_area: ValidityArea,
    pub extension_2_fare_cents: u16,
    pub sale_status: bool,

    // --- Ticket validity info ---
    pub validity_start_datetime: DateTime<Utc>,
    /// If ValidityEndDateGroup and ValidityEndTimeGroup are > 0 this uses them instead.
    pub validity_end_datetime: DateTime<Utc>,
    /// True if the ticket is currently valid.
    pub validity_status: bool,

    // --- Boarding info ---
    pub boarding_datetime: DateTime<Utc>,
    pub boarding_vehicle: u16,
    pub boarding_location: BoardingLocation,
    pub boarding_direction: BoardingDirection,
    pub boarding_area: BoardingArea
}

pub fn create_e_ticket(e_ticket: &[u8]) -> ETicket {
    let product_code_type = get_bits_as_u8(e_ticket, 0, 1);
    let product_code_value = get_bits_as_u16(e_ticket, 1, 14);
    let product_code_group_value = get_bits_as_u16(e_ticket, 15, 14);
    let product_code = if product_code_group_value > 0 {
        product_code_group_value
    } else {
        product_code_value
    };

    let customer_profile_value = get_bits_as_u8(e_ticket, 29, 5);
    let customer_profile_group_value = get_bits_as_u8(e_ticket, 34, 5);
    let customer_profile = if customer_profile_group_value > 0 {
        customer_profile_group_value
    } else {
        customer_profile_value
    };
    let language_code = get_bits_as_u8(e_ticket, 39, 2);

    let validity_length_type_value = get_bits_as_u8(e_ticket, 41, 2);
    let validity_length_value = get_bits_as_u8(e_ticket, 43, 8);
    let validity_length_type_group_value = get_bits_as_u8(e_ticket, 51, 2);
    let validity_length_group_value = get_bits_as_u8(e_ticket, 53, 8);
    let validity_length_type = if validity_length_type_group_value > 0 {
        validity_length_type_group_value
    } else {
        validity_length_type_value
    };
    let validity_length = if validity_length_group_value > 0 {
        validity_length_group_value
    } else {
        validity_length_value
    };

    let validity_area_type = get_bits_as_u8(e_ticket, 61, 2);
    let validity_area_value = get_bits_as_u8(e_ticket, 63, 6);
    let sale_date = get_bits_as_u16(e_ticket, 69, 14);
    let sale_hour = get_bits_as_u16(e_ticket, 73, 5) * 60; // Turned into minutes so we can just stuff it into the conversion function.
    let sale_datetime = from_en1545_date_and_time(sale_date, sale_hour);

    ETicket {
        product_code: ProductCode::new(product_code_type, product_code),
        customer_profile: customer_profile,
        language: Language::from_u8(language_code),
        validity_length: ValidityLength::new(validity_length_type, validity_length),
        validity_area: ValidityArea::new(validity_area_type, validity_area_value),
        sale_datetime: sale_datetime,
    }
}

fn single_or_group<T>(single: T, group: T) -> T {
    if group > 0 {
        group
    } else {
        single
    }
}