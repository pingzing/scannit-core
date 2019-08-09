// --- Commands ---
///DESFire GetVersion command.
pub const GET_VERSION_COMMAND: [u8; 5] = [0x90, 0x60, 0x00, 0x00, 0x00];

///DESFire command to return all installed application IDs on the card.
pub const GET_APPLICATION_IDS_COMMAND: [u8; 5] = [0x90, 0x6A, 0x00, 0x00, 0x00];

///DESFire Select Application command for selecting the HSL application on the card.
///Returns OkResponse on success.
pub const SELECT_HSL_COMMAND: [u8; 9] = [0x90, 0x5A, 0x00, 0x00, 0x03, 0x14, 0x20, 0xEF, 0x00];

///Command to read app info file, which contains application version, card name, etc.
pub const READ_APP_INFO_COMMAND: [u8; 13] = [
    0x90, 0xBD, 0x00, 0x00, 0x07, 0x08, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x00, 0x00,
];

///Command to read the control info file from the card.
pub const READ_CONTROL_INFO_COMMAND: [u8; 13] = [
    0x90, 0xBD, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00,
];

///Command to read the season pass file on the card.
pub const READ_PERIOD_PASS_COMMAND: [u8; 13] = [
    0x90, 0xBD, 0x00, 0x00, 0x07, 0x01, 0x00, 0x00, 0x00, 0x23, 0x00, 0x00, 0x00,
];

///Command to read the stored value on the card.
pub const READ_STORED_VALUE_COMMAND: [u8; 13] = [
    0x90, 0xBD, 0x00, 0x00, 0x07, 0x02, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00,
];

///Command to read the active eTicket on the card.
pub const READ_E_TICKET_COMMAND: [u8; 13] = [
    0x90, 0xBD, 0x00, 0x00, 0x07, 0x03, 0x00, 0x00, 0x00, 0x2D, 0x00, 0x00, 0x00,
];

///Command to read the 8 most recent transactions on the card.
pub const READ_HISTORY_COMMAND: [u8; 13] = [
    0x90, 0xBB, 0x00, 0x00, 0x07, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

///Reads the remaining bytes-to-be-sent if a read request returned a MoreData response.
pub const READ_NEXT_COMMAND: [u8; 5] = [0x90, 0xAF, 0x00, 0x00, 0x00];

// --- Responses ---
///DESFire OPERATION_OK response.
pub const OK_RESPONSE: [u8; 2] = [0x91, 0x00];

///DESFire error response. Not sure what it's known as internally.
pub const ERROR_RESPONSE: [u8; 2] = [0x91, 0x9D];

///DESFire ADDTIONAL_FRAME response. Indicates that there is more data, if the caller would like to ask for it.
pub const MORE_DATA_RESPONSE: [u8; 2] = [0x91, 0xAF];

/// A DESFire APDU command that the HSL card accepts.
pub enum Command {
    ///DESFire GetVersion command.
    GetVersion,

    ///DESFire command to return all installed application IDs on the card.
    GetApplicationIds,

    ///DESFire Select Application command for selecting the HSL application on the card.
    ///Returns OkResponse on success.
    SelectHsl,

    ///Command to read app info file, which contains application version, card name, etc.
    ReadAppInfo,

    ///Command to read the control info file from the card.
    ReadControlInfo,

    ///Command to read the season pass file on the card.
    ReadPeriodPass,

    ///Command to read the stored value on the card.
    ReadStoredValue,

    ///Command to read the active eTicket on the card.
    ReadETicket,

    ///Command to read the 8 most recent transactions on the card.
    ReadHistory,

    ///Reads the remaining bytes-to-be-sent if a read request returned a MoreData response.
    ReadNext,
}

/// Possible DESFire responses to APDU command from the HSL card.
#[derive(Copy, Clone)]
pub enum Response {
    ///DESFire OPERATION_OK response.
    Ok,

    ///DESFire error response. Not sure what it's known as internally.
    Error,

    ///DESFire ADDTIONAL_FRAME response. Indicates that there is more data, if the caller would like to ask for it.
    MoreData,
}

impl Into<&[u8]> for Command {
    fn into(self) -> &'static [u8] {
        match self {
            Command::GetVersion => &GET_VERSION_COMMAND,
            Command::GetApplicationIds => &GET_APPLICATION_IDS_COMMAND,
            Command::SelectHsl => &SELECT_HSL_COMMAND,
            Command::ReadAppInfo => &READ_APP_INFO_COMMAND,
            Command::ReadControlInfo => &READ_CONTROL_INFO_COMMAND,
            Command::ReadPeriodPass => &READ_PERIOD_PASS_COMMAND,
            Command::ReadStoredValue => &READ_STORED_VALUE_COMMAND,
            Command::ReadETicket => &READ_E_TICKET_COMMAND,
            Command::ReadHistory => &READ_HISTORY_COMMAND,
            Command::ReadNext => &READ_NEXT_COMMAND,
        }
    }
}

impl Into<&[u8]> for Response {
    fn into(self) -> &'static [u8] {
        match self {
            Response::Ok => &OK_RESPONSE,
            Response::Error => &ERROR_RESPONSE,
            Response::MoreData => &MORE_DATA_RESPONSE,
        }
    }
}

impl std::cmp::PartialEq<Response> for [u8] {
    fn eq(&self, other: &Response) -> bool {
        self == Into::<&[u8]>::into(*other)
    }
}

impl std::cmp::PartialEq<Response> for &[u8] {
    fn eq(&self, other: &Response) -> bool {
        *self == Into::<&[u8]>::into(*other)
    }
}
