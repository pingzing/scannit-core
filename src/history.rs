use crate::conversion::*;
use crate::en1545date::from_en1545_date_and_time;
use chrono::prelude::*;

#[derive(Debug)]
pub struct History {
    pub transaction_type: TransactionType,
    pub boarding_datetime: DateTime<Utc>,
    pub transfer_end_datetime: DateTime<Utc>,
    pub ticket_fare_cents: u16,
    pub group_size: u8,
    /// Value remaining on the card after this use. Always 0 if this was a season pass usage.
    pub remaining_value: u32,
}

pub fn create_history_entries(history_bytes: &[u8]) -> Vec<History> {
    let num_entries = history_bytes.len() / 12; // Each history entry is 12 bytes.
    let entry_size = 96; // in bits. 8 * 12.    
    let mut history_entries: Vec<History> = vec![];

    for i in 0..num_entries {
        // We're far enough away from the En1545 zero-date that one of the first three
        // bits of the date field should always be 1. This is sufficient to see if
        // there is any data in for this history entry.
        if history_bytes[i * 12 + 1] == 0
            && history_bytes[i * 12 + 2] == 0
            && history_bytes[i * 12 + 3] == 0
            && history_bytes[i * 13 + 3] == 0
        {
            continue;
        }
        let entry_offset = i * entry_size;
        let transaction_type = get_bits_as_u8(history_bytes, 0 + entry_offset, 1);
        let boarding_date = get_bits_as_u16(history_bytes, 1 + entry_offset, 14);
        let boarding_time = get_bits_as_u16(history_bytes, 15 + entry_offset, 11);
        let end_date = get_bits_as_u16(history_bytes, 26 + entry_offset, 14);
        let end_time = get_bits_as_u16(history_bytes, 40 + entry_offset, 11);
        let ticket_fare = get_bits_as_u16(history_bytes, 51 + entry_offset, 14);
        let group_size = get_bits_as_u8(history_bytes, 65 + entry_offset, 6);
        let remaining_value = get_bits_as_u32(history_bytes, 71 + entry_offset, 20);

        history_entries.push(History {
            transaction_type: TransactionType::new(transaction_type),
            boarding_datetime: from_en1545_date_and_time(boarding_date, boarding_time),
            transfer_end_datetime: from_en1545_date_and_time(end_date, end_time),
            ticket_fare_cents: ticket_fare,
            group_size: group_size,
            remaining_value: remaining_value
        });
    }

    history_entries
}

#[derive(Debug)]
pub enum TransactionType {
    SeasonPass = 0,
    ValueTicket = 1,
}

impl TransactionType {
    pub(crate) fn new(type_value: u8) -> TransactionType {
        match type_value {
            0 => TransactionType::SeasonPass,
            1 => TransactionType::ValueTicket,
            _ => panic!("Unsupported TransactionType value"),
        }
    }
}
