use crate::conversion;

pub struct TravelCard {
    pub application_version: u8,
    pub application_instance_id: String,
    pub platform_type: u8,
}

pub fn create_travel_card(
    app_info: &[u8],
    period_pass: &[u8],
    storedValue: &[u8],
    eTicket: &[u8],
    history: &[u8],
) -> TravelCard {
    let (app_version, app_instance_id, platform) = read_application_info(app_info);

    TravelCard {
        application_version: app_version,
        application_instance_id: app_instance_id,
        platform_type: platform,
    }
}

fn read_application_info(app_info: &[u8]) -> (u8, String, u8) {
    (
        app_info[0] & 0xF0,
        conversion::as_hex_string(&app_info[1..10]),
        app_info[10] & 0xE0,
    )
}
