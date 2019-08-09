use crate::conversion::*;
use crate::en1545date::*;
use crate::models::*;
use chrono::prelude::*;
use num_traits::Zero;

#[derive(Debug)]
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
    pub sale_device: SaleDevice,
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
    pub boarding_area: BoardingArea,
}

pub fn create_e_ticket(e_ticket: &[u8]) -> ETicket {
    let product_code_type = get_bits_as_u8(e_ticket, 0, 1);
    let product_code_value = get_bits_as_u16(e_ticket, 1, 14);
    let product_code_group_value = get_bits_as_u16(e_ticket, 15, 14);
    let product_code = single_or_group(product_code_value, product_code_group_value);

    let customer_profile_value = get_bits_as_u8(e_ticket, 29, 5);
    let customer_profile_group_value = get_bits_as_u8(e_ticket, 34, 5);
    let customer_profile = single_or_group(customer_profile_value, customer_profile_group_value);
    let language_code = get_bits_as_u8(e_ticket, 39, 2);

    let validity_length_type_value = get_bits_as_u8(e_ticket, 41, 2);
    let validity_length_value = get_bits_as_u8(e_ticket, 43, 8);
    let validity_length_type_group_value = get_bits_as_u8(e_ticket, 51, 2);
    let validity_length_group_value = get_bits_as_u8(e_ticket, 53, 8);
    let validity_length_type =
        single_or_group(validity_length_type_value, validity_length_type_group_value);
    let validity_length = single_or_group(validity_length_value, validity_length_group_value);
    let validity_area_type = get_bits_as_u8(e_ticket, 61, 2);
    let validity_area_value = get_bits_as_u8(e_ticket, 63, 6);

    let sale_date = get_bits_as_u16(e_ticket, 69, 14);
    let sale_hour_as_minutes = get_bits_as_u16(e_ticket, 83, 5) * 60; // Turned into minutes so we can just stuff it into the conversion function.
    let sale_datetime = from_en1545_date_and_time(sale_date, sale_hour_as_minutes);
    let sale_device_type = get_bits_as_u8(e_ticket, 88, 3);
    let sale_device_number = get_bits_as_u16(e_ticket, 91, 14);

    let ticket_fare_value = get_bits_as_u16(e_ticket, 105, 14);
    let ticket_fare_group_value = get_bits_as_u16(e_ticket, 119, 14);
    let ticket_fare = single_or_group(ticket_fare_value, ticket_fare_group_value);
    let group_size = get_bits_as_u8(e_ticket, 133, 6);
    let extra_zone = get_bits_as_u8(e_ticket, 139, 1) != 0;

    let period_pass_validity_area = get_bits_as_u8(e_ticket, 140, 6);
    let extension_product_code = get_bits_as_u16(e_ticket, 146, 14);
    let extension_1_validity_area = get_bits_as_u8(e_ticket, 160, 6);
    let extension_1_fare_cents = get_bits_as_u16(e_ticket, 166, 14);
    let extension_2_validity_area = get_bits_as_u8(e_ticket, 180, 6);
    let extension_2_fare_cents = get_bits_as_u16(e_ticket, 186, 14);
    let sale_status = get_bits_as_u8(e_ticket, 200, 1) != 0;

    let validity_start_date = get_bits_as_u16(e_ticket, 205, 14);
    let validity_start_time = get_bits_as_u16(e_ticket, 219, 11);
    let validity_end_date_value = get_bits_as_u16(e_ticket, 230, 14);
    let validity_end_time_value = get_bits_as_u16(e_ticket, 244, 11);
    let validity_end_date_group_value = get_bits_as_u16(e_ticket, 255, 14);
    let valdiity_end_time_group_value = get_bits_as_u16(e_ticket, 269, 11);
    let validity_end_datetime = from_en1545_date_and_time(
        single_or_group(validity_end_date_value, validity_end_date_group_value),
        single_or_group(validity_end_time_value, valdiity_end_time_group_value),
    );
    let validity_status = get_bits_as_u8(e_ticket, 285, 1) != 0;

    let boarding_date = get_bits_as_u16(e_ticket, 286, 14);
    let boarding_time = get_bits_as_u16(e_ticket, 300, 11);
    let boarding_vehicle = get_bits_as_u16(e_ticket, 311, 14);
    let boarding_location_num_type = get_bits_as_u8(e_ticket, 325, 2);
    let boarding_location_num = get_bits_as_u16(e_ticket, 327, 14);
    let boarding_direction = get_bits_as_u8(e_ticket, 341, 1);
    let boarding_area_type = get_bits_as_u8(e_ticket, 342, 2);
    let boarding_area = get_bits_as_u8(e_ticket, 344, 6);

    ETicket {
        product_code: ProductCode::new(product_code_type, product_code),
        customer_profile,
        language: Language::from_u8(language_code),
        validity_length: ValidityLength::new(validity_length_type, validity_length),
        validity_area: ValidityArea::new(validity_area_type, validity_area_value),
        sale_datetime,
        sale_device: SaleDevice::new(sale_device_type, sale_device_number),
        ticket_fare_cents: ticket_fare,
        group_size,

        extra_zone,
        period_pass_validity_area: ValidityArea::new(
            ValidityArea::OLD_ZONE_TYPE,
            period_pass_validity_area,
        ),
        extension_product_code: ProductCode::new(
            ProductCode::FARES_2014_TYPE,
            extension_product_code,
        ),
        extension_1_validity_area: ValidityArea::new(
            ValidityArea::OLD_ZONE_TYPE,
            extension_1_validity_area,
        ),
        extension_1_fare_cents,
        extension_2_validity_area: ValidityArea::new(
            ValidityArea::OLD_ZONE_TYPE,
            extension_2_validity_area,
        ),
        extension_2_fare_cents,
        sale_status,

        validity_start_datetime: from_en1545_date_and_time(
            validity_start_date,
            validity_start_time,
        ),
        validity_end_datetime,
        validity_status,
        boarding_datetime: from_en1545_date_and_time(boarding_date, boarding_time),
        boarding_vehicle,
        boarding_location: BoardingLocation::new(boarding_location_num_type, boarding_location_num),
        boarding_direction: BoardingDirection::new(boarding_direction),
        boarding_area: BoardingArea::new(boarding_area_type, boarding_area),
    }
}

fn single_or_group<T: Zero + PartialOrd>(single: T, group: T) -> T {
    if group > T::zero() {
        group
    } else {
        single
    }
}
