// The world's laziest example of using of the scannit-core library.
// The absolute bare minimum to get it working, and test that it works.

use pcsc::*;
use scannit_core::desfire;
use scannit_core::travelcard::create_travel_card;

fn main() {
    let ctx = match Context::establish(Scope::User) {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("Failed to establish context: {}", err);
            std::process::exit(1);
        }
    };

    let mut readers_buf = [0; 2048];
    let mut readers = match ctx.list_readers(&mut readers_buf) {
        Ok(readers) => readers,
        Err(err) => {
            eprintln!("Failed to list readers: {}", err);
            std::process::exit(1);
        }
    };

    // Use the first reader:
    let reader = match readers.next() {
        Some(reader) => reader,
        None => {
            println!("No readers connected or found.");
            return;
        }
    };

    // Connect to the card
    let card = match ctx.connect(reader, ShareMode::Shared, Protocols::ANY) {
        Ok(card) => card,
        Err(Error::NoSmartcard) => {
            println!("No smart card present.");
            return;
        }
        Err(err) => {
            eprintln!("Failed to connect to card: {}", err);
            std::process::exit(1);
        }
    };

    let open_hsl_command = desfire::SELECT_HSL_COMMAND;
    println!("Sending APDU: {:X?}", open_hsl_command);
    let mut response_buffer = [0; MAX_BUFFER_SIZE];

    let result = transcieve(&card, &open_hsl_command, &mut response_buffer).unwrap();

    if result[0..2] != desfire::OK_RESPONSE {
        println!("Failed. Received {:X?}", result);
        return;
    }

    println!("Success! Reading data...");

    let app_info =
        transcieve(&card, &desfire::READ_APP_INFO_COMMAND, &mut response_buffer).unwrap();

    let mut response_buffer = [0; MAX_BUFFER_SIZE];
    let control_info = transcieve(
        &card,
        &desfire::READ_CONTROL_INFO_COMMAND,
        &mut response_buffer,
    )
    .unwrap();

    let mut response_buffer = [0; MAX_BUFFER_SIZE];
    let period_pass = transcieve(
        &card,
        &desfire::READ_PERIOD_PASS_COMMAND,
        &mut response_buffer,
    )
    .unwrap();

    let mut response_buffer = [0; MAX_BUFFER_SIZE];
    let stored_value = transcieve(
        &card,
        &desfire::READ_STORED_VALUE_COMMAND,
        &mut response_buffer,
    )
    .unwrap();

    let mut response_buffer = [0; MAX_BUFFER_SIZE];
    let e_ticket =
        transcieve(&card, &desfire::READ_E_TICKET_COMMAND, &mut response_buffer).unwrap();

    let mut response_buffer = [0; MAX_BUFFER_SIZE];
    let history_bytes =
        transcieve(&card, &desfire::READ_HISTORY_COMMAND, &mut response_buffer).unwrap();
    let mut history_bytes_2: Vec<u8>;

    // Check for extra history data. It comes in chunks of four.
    let len = history_bytes.len();
    // If the last two bytes of history contain MORE_DATA:
    let all_history: &[u8] = if &history_bytes[len - 2..len] == &desfire::MORE_DATA_RESPONSE {
        let mut response_buffer = [0; MAX_BUFFER_SIZE];
        let bytes = transcieve(&card, &desfire::READ_NEXT_COMMAND, &mut response_buffer).unwrap();
        let len_2 = bytes.len();
        history_bytes_2 = [&history_bytes[0..len - 2], &bytes[0..len_2 - 2]].concat();
        &history_bytes_2
    } else {
        &history_bytes[0..len - 2]
    };

    let travel_card = create_travel_card(
        app_info,
        control_info,
        period_pass,
        stored_value,
        e_ticket,
        all_history,
    );

    println!("Travel card all read! Values: {:?}", travel_card);
}

fn transcieve<'a>(
    card: &pcsc::Card,
    command: &[u8],
    response_buffer: &'a mut [u8],
) -> Result<&'a [u8], Error> {
    match card.transmit(command, response_buffer) {
        Ok(res) => Ok(res),
        Err(err) => {
            eprintln!("Failed to transmit command to card: {}", err);
            Err(err)
        }
    }
}
