// --- Commands ---
///DESFire GetVersion command.
const GET_VERSION_COMMAND: [u8; 5] = [0x90, 0x60, 0x00, 0x00, 0x00];

///DESFire command to return all installed application IDs on the card.
const GET_APPLICATION_IDS_COMMAND: [u8; 5] = [0x90, 0x6A, 0x00, 0x00, 0x00];

///DESFire Select Application command for selecting the HSL application on the card.
///Returns OkResponse on success.
const SELECT_HSL_COMMAND: [u8; 9] = [0x90, 0x5A, 0x00, 0x00, 0x03, 0x14, 0x20, 0xEF, 0x00];

///Command to read app info file, which contains application version, card name, etc.
const READ_APP_INFO_COMMAND: [u8; 13] = [
    0x90, 0xBD, 0x00, 0x00, 0x07, 0x08, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x00, 0x00,
];

///Command to read the season pass file on the card.
const READ_PERIOD_PASS_COMMAND: [u8; 13] = [
    0x90, 0xBD, 0x00, 0x00, 0x07, 0x01, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
];

///Command to read the stored value on the card.
const READ_STORED_VALUE_COMMAND: [u8; 13] = [
    0x90, 0xBD, 0x00, 0x00, 0x07, 0x02, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00,
];

///Command to read the active eTicket on the card.
const READ_E_TICKET_COMMAND: [u8; 13] = [
    0x90, 0xBD, 0x00, 0x00, 0x07, 0x03, 0x00, 0x00, 0x00, 0x1A, 0x00, 0x00, 0x00,
];

///Command to read the 8 most recent transactions on the card.
const READ_HISTORY_COMMAND: [u8; 13] = [
    0x90, 0xBB, 0x00, 0x00, 0x07, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

///Reads the remaining bytes-to-be-sent if a read request returned a MoreData response.
const READ_NEXT_COMMAND: [u8; 5] = [0x90, 0xAF, 0x00, 0x00, 0x00];

// --- Responses ---
///DESFire OPERATION_OK response. In Hex: 0x91, 0x00.
const OK_RESPONSE: [u8; 2] = [0x91, 0x00];

///DESFire error response. Not sure what it's known as internally. In Hex: 0x91, 0x9D.
const ERROR_RESPONSE: [u8; 2] = [0x91, 0x9D];

///DESFire ADDTIONAL_FRAME response. Indicates that more data is expected to be sent. In Hex: 0x91, 0xAF.
const MORE_DATA_RESPONSE: [u8; 2] = [0x91, 0xAF];

pub enum Command {
    GetVersion,
    GetApplicationIds,
    SelectHsl,
    ReadAppInfo,
    ReadPeriodPass,
    ReadStoredValue,
    ReadETicket,
    ReadHistory,
    ReadNext,
}

pub enum Response {
    Ok,
    Error,
    MoreData,
}

impl Command {
    pub fn value(&self) -> &[u8] {
        match *self {
            Command::GetVersion => &GET_VERSION_COMMAND,
            Command::GetApplicationIds => &GET_APPLICATION_IDS_COMMAND,
            Command::SelectHsl => &SELECT_HSL_COMMAND,
            Command::ReadAppInfo => &READ_APP_INFO_COMMAND,
            Command::ReadPeriodPass => &READ_PERIOD_PASS_COMMAND,
            Command::ReadStoredValue => &READ_STORED_VALUE_COMMAND,
            Command::ReadETicket => &READ_E_TICKET_COMMAND,
            Command::ReadHistory => &READ_HISTORY_COMMAND,
            Command::ReadNext => &READ_NEXT_COMMAND,
        }
    }
}

impl Response {
    pub fn value(&self) -> &[u8] {
        match *self {
            Response::Ok => &OK_RESPONSE,
            Response::Error => &ERROR_RESPONSE,
            Response::MoreData => &MORE_DATA_RESPONSE,
        }
    }
}
