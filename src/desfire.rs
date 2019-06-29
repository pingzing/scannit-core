mod desfire {
    // --- Commands ---
    ///DESFire GetVersion command.
    const GET_VERSION_COMMAND: [u8; 5] = [144, 96, 00, 00, 00];

    ///DESFire command to return all installed application IDs on the card.
    const GET_APPLICATION_IDS_COMMAND: [u8; 5] = [144, 106, 00, 00, 00];

    ///DESFire Select Application command for selecting the HSL application on the card.
    ///Returns OkResponse on success.
    const SELECT_HSL_COMMAND: [u8; 9] = [144, 90, 00, 00, 03, 20, 32, 239, 00];

    ///Command to read app info file, which contains application version, card name, etc.
    const READ_APP_INFO_COMMAND: [u8; 13] = [144, 189, 00, 00, 07, 08, 00, 00, 00, 11, 00, 00, 00];

    ///Command to read the season pass file on the card.
    const READ_PERIOD_PASS_COMMAND: [u8; 13] =
        [144, 189, 00, 00, 07, 01, 00, 00, 00, 32, 00, 00, 00];

    ///Command to read the stored value on the card.
    const READ_STORED_VALUE_COMMAND: [u8; 13] =
        [144, 189, 00, 00, 07, 02, 00, 00, 00, 12, 00, 00, 00];

    ///Command to read the active eTicket on the card.
    const READ_E_TICKET_COMMAND: [u8; 13] = [144, 189, 00, 00, 07, 03, 00, 00, 00, 26, 00, 00, 00];

    ///Command to read the 8 most recent transactions on the card.
    const READ_HISTORY_COMMAND: [u8; 13] = [144, 189, 00, 00, 07, 04, 00, 00, 00, 00, 00, 00, 00];

    ///Reads the remaining bytes-to-be-sent if a read request returned a MoreData response.
    const READ_NEXT_COMMAND: [u8; 5] = [144, 175, 00, 00, 00];

    // --- Responses ---
    ///DESFire OPERATION_OK response. In Hex: 0x91, 0x00.
    const OK_RESPONSE: [u8; 2] = [145, 00];

    ///DESFire error response. Not sure what it's known as internally. In Hex: 0x91, 0x9D.
    const ERROR_RESPONSE: [u8; 2] = [145, 157];

    ///DESFire ADDTIONAL_FRAME response. Indicates that more data is expected to be sent. In Hex: 0x91, 0xAF.
    const MORE_DATA_RESPONSE: [u8; 2] = [145, 175];

    enum Command {
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

    enum Response {
        Ok,
        Error,
        MoreData,
    }

    impl Command {
        fn value(&self) -> &[u8] {
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
        fn value(&self) -> &[u8] {
            match *self {
                Response::Ok => &OK_RESPONSE,
                Response::Error => &ERROR_RESPONSE,
                Response::MoreData => &MORE_DATA_RESPONSE,
            }
        }
    }
}
