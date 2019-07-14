pub enum ProductCode {
    FaresFor2010(u16), // Code type = 0
    FaresFor2014(u16), // Code type = 1
}

impl ProductCode {
    pub(crate) fn new(code_type: u8, value: u16) -> ProductCode {
        if code_type == 0 {
            ProductCode::FaresFor2010(value)
        } else {
            ProductCode::FaresFor2014(value)
        }
    }
}

/// The number of a boarded element.
pub enum BoardingLocation {
    BusNumber(u16),
    TrainNumber(u16),
    PlatformNumber(u16),
}

impl BoardingLocation {
    pub(crate) fn new(boarding_area_type: u8, boarding_area_value: u16) -> BoardingLocation {
        match boarding_area_type {
            1 => BoardingLocation::BusNumber(boarding_area_value),
            2 => BoardingLocation::TrainNumber(boarding_area_value),
            3 => BoardingLocation::PlatformNumber(boarding_area_value),
            _ => panic!("Given value for BoardingLocation not supported."),
        }
    }
}

/// This enum is pure speculation--the underlying value is a single bit. What else _could_ it mean?
pub enum BoardingDirection {
    /// Indicates that at the time of boarding, the transit medium  was headed toward the end of its route.
    TowardEnd,
    /// Indicates that at the time of boarding, the transit medium was headed toward the start of its route.
    TowardStart,
}

impl BoardingDirection {
    pub(crate) fn new(value: u8) -> BoardingDirection {
        match value {
            0 => BoardingDirection::TowardEnd,
            1 => BoardingDirection::TowardStart,
            _ => panic!("Given value for BoardingDirection not supported."),
        }
    }
}

/// Represents an area in which, or a vehicle for which, a ticket is valid.
pub enum ValidityArea {
    Zone(Vec<ValidityZone>),
    Vehicle(VehicleType),
}

impl ValidityArea {
    pub(crate) fn new(area_type: u8, area_value: u8) -> ValidityArea {
        let mut zones: Vec<ValidityZone> = Vec::new();
        if area_type == 0 {
            let from_zone = area_value & 0b0000_0111; //rightmost 3 bits
            let to_zone = (area_value & 0b0011_1000) >> 3; // 3 bits to the left of that
            for val in from_zone..to_zone {
                zones.push(ValidityZone::from_u8(val));
            }
            ValidityArea::Zone(zones)
        } else {
            ValidityArea::Vehicle(VehicleType::from_u8(area_value))
        }
    }
}

/// The HSL fare zone(s) in which a ticket is valid.
#[derive(Clone)]
pub enum ValidityZone {
    ZoneA = 0,
    ZoneB = 1,
    ZoneC = 2,
    ZoneD = 3,
    ZoneE = 4,
    ZoneF = 5,
    ZoneG = 6,
    ZoneH = 7,
}

impl ValidityZone {
    pub(crate) fn from_u8(value: u8) -> ValidityZone {
        match value {
            0 => ValidityZone::ZoneA,
            1 => ValidityZone::ZoneB,
            2 => ValidityZone::ZoneC,
            3 => ValidityZone::ZoneD,
            4 => ValidityZone::ZoneE,
            5 => ValidityZone::ZoneF,
            6 => ValidityZone::ZoneG,
            7 => ValidityZone::ZoneH,
            _ => panic!("Given value for ValidityZone not supported."),
        }
    }
}

pub enum ValidityLength {
    Minutes(u8),
    Hours(u8),
    TwentyFourHourPeriods(u8),
    Days(u8),
}

impl ValidityLength {
    pub(crate) fn new(length_type: u8, length_value: u8) -> ValidityLength {
        match length_type {
            0 => ValidityLength::Minutes(length_value),
            1 => ValidityLength::Hours(length_value),
            2 => ValidityLength::TwentyFourHourPeriods(length_value),
            3 => ValidityLength::Days(length_value),
            _ => panic!("Given value for ValidityLength type not supported."),
        }
    }
}

/// The vehicle type on which this ticket is valid.
pub enum VehicleType {
    Undefined = 0,
    Bus = 1,
    Tram = 5,
    Metro = 6,
    Train = 7,
    Ferry = 8,
    ULine = 9,
}

impl VehicleType {
    pub(crate) fn from_u8(value: u8) -> VehicleType {
        match value {
            0 => VehicleType::Undefined,
            1 => VehicleType::Bus,
            5 => VehicleType::Tram,
            6 => VehicleType::Metro,
            7 => VehicleType::Train,
            8 => VehicleType::Ferry,
            9 => VehicleType::ULine,
            _ => panic!("Given value for VehicleType not supported."),
        }
    }
}

pub enum Language {
    Finnish = 0,
    Swedish = 1,
    English = 2,
}

impl Language {
    pub(crate) fn from_u8(value: u8) -> Language {
        match value {
            0 => Language::Finnish,
            1 => Language::Swedish,
            2 => Language::English,
            _ => panic!("Given value for Language not supported."),
        }
    }
}

pub enum SaleDeviceType {
    ServicePointSalesDevice = 0,
    DriverTicketMachine = 1,
    CardReader = 2,
    TicketMachine = 3,
    Server = 4,
    HSLSmallEquipment = 5,
    ExternalServiceEquipment = 6,
    Reserved = 7,
}

impl SaleDeviceType {
    pub(crate) fn from_u8(value: u8) -> SaleDeviceType {
        match value {
            0 => SaleDeviceType::ServicePointSalesDevice,
            1 => SaleDeviceType::DriverTicketMachine,
            2 => SaleDeviceType::CardReader,
            3 => SaleDeviceType::TicketMachine,
            4 => SaleDeviceType::Server,
            5 => SaleDeviceType::HSLSmallEquipment,
            6 => SaleDeviceType::ExternalServiceEquipment,
            7 => SaleDeviceType::Reserved,
            _ => panic!("Given value for SaleDeviceType not supported."),
        }
    }
}

pub enum BoardingArea {
    Zone(ValidityZone),
    Vehicle(VehicleType),
    ZoneCircle(u8), // Not sure what this is. One of the old-style regions?
}
