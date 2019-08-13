#[derive(Debug)]

/// Indicates whether a PeriodPass or Ticket uses the old-style fares and zones, or the new.
/// 2010 is the old-style, while 2014 is the new-style.
pub enum ProductCode {
    FaresFor2010(u16), // Code type = 0
    FaresFor2014(u16), // Code type = 1
}

impl ProductCode {
    pub const FARES_2010_TYPE: u8 = 0;
    pub const FARES_2014_TYPE: u8 = 1;

    pub(crate) fn new(code_type: u8, value: u16) -> ProductCode {
        if code_type == ProductCode::FARES_2010_TYPE {
            ProductCode::FaresFor2010(value)
        } else {
            ProductCode::FaresFor2014(value)
        }
    }
}

impl From<ProductCode> for u16 {
    fn from(val: ProductCode) -> Self {
        match val { 
            ProductCode::FaresFor2010(v) | ProductCode::FaresFor2014(v) => v 
        }
    }
}

/// The number of a boarded element.
#[derive(Debug)]
pub enum BoardingLocation {
    NoneOrReserved,
    BusNumber(u16),
    TrainNumber(u16),
    PlatformNumber(u16),
}

impl BoardingLocation {
    pub(crate) fn new(boarding_area_type: u8, boarding_area_value: u16) -> BoardingLocation {
        match boarding_area_type {
            0 => BoardingLocation::NoneOrReserved,
            1 => BoardingLocation::BusNumber(boarding_area_value),
            2 => BoardingLocation::TrainNumber(boarding_area_value),
            3 => BoardingLocation::PlatformNumber(boarding_area_value),
            e => panic!("Given value ({}) for BoardingLocation not supported.", e),
        }
    }
}

#[derive(Debug)]
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
            e => panic!("Given value ({}) for BoardingDirection not supported.", e),
        }
    }
}

/// Represents an area in which, or a vehicle for which, a ticket is valid.
#[derive(Debug)]
pub enum ValidityArea {
    OldZone(u8),
    Zone(Vec<ValidityZone>),
    Vehicle(VehicleType),
}

impl ValidityArea {
    pub const OLD_ZONE_TYPE: u8 = 0;
    pub const VEHICLE_TYPE: u8 = 1;
    pub const NEW_ZONE_TYPE: u8 = 2; // The docs LIE, and don't include this value. But it's there!

    pub(crate) fn new(area_type: u8, area_value: u8) -> ValidityArea {
        let mut zones: Vec<ValidityZone> = Vec::new();
        match area_type {
            // TODO: Wrap this a bit more nicely. It represents an old zone (i.e. Zone 1, Zone 2, Region etc)
            ValidityArea::OLD_ZONE_TYPE => ValidityArea::OldZone(area_value),
            ValidityArea::VEHICLE_TYPE => ValidityArea::Vehicle(VehicleType::from(area_value)),
            ValidityArea::NEW_ZONE_TYPE => {
                let from_zone = (area_value & 0b0011_1000) >> 3; // leftmost 3 bits
                let to_zone = area_value & 0b0000_0111; // 3 bits to the right of that
                for val in from_zone..=to_zone {
                    zones.push(ValidityZone::from(val));
                }
                ValidityArea::Zone(zones)
            }
            e => panic!("Unsupported area type: {}", e),
        }
    }
}

/// The HSL fare zone(s) in which a ticket is valid.
#[derive(Clone, Debug)]
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

impl From<u8> for ValidityZone {
    fn from(value: u8) -> Self {
        match value {
            0 => ValidityZone::ZoneA,
            1 => ValidityZone::ZoneB,
            2 => ValidityZone::ZoneC,
            3 => ValidityZone::ZoneD,
            4 => ValidityZone::ZoneE,
            5 => ValidityZone::ZoneF,
            6 => ValidityZone::ZoneG,
            7 => ValidityZone::ZoneH,
            e => panic!("Given value ({}) for ValidityZone not supported.", e),
        }
    }
}

impl From<ValidityZone> for u8 {
    fn from(value: ValidityZone) -> Self {
        match value {
            ValidityZone::ZoneA => 0,
            ValidityZone::ZoneB => 1,
            ValidityZone::ZoneC => 2,
            ValidityZone::ZoneD => 3,
            ValidityZone::ZoneE => 4,
            ValidityZone::ZoneF => 5,
            ValidityZone::ZoneG => 6,
            ValidityZone::ZoneH => 7,            
        }
    }
}

#[derive(Debug)]
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
            e => panic!("Given value ({}) for ValidityLength type not supported.", e),
        }
    }
}

/// The vehicle type on which this ticket is valid.
#[derive(Debug)]
pub enum VehicleType {
    Undefined = 0,
    Bus = 1,
    Tram = 5,
    Metro = 6,
    Train = 7,
    Ferry = 8,
    ULine = 9,
}

impl From<u8> for VehicleType {
    fn from(value: u8) -> Self {
        match value {
            0 => VehicleType::Undefined,
            1 => VehicleType::Bus,
            5 => VehicleType::Tram,
            6 => VehicleType::Metro,
            7 => VehicleType::Train,
            8 => VehicleType::Ferry,
            9 => VehicleType::ULine,
            e => panic!("Given value ('{:?}') for VehicleType not supported.", e),
        }
    }
}

impl From<VehicleType> for u8 {
    fn from(value: VehicleType) -> Self {
        match value {
            VehicleType::Undefined => 0,
            VehicleType::Bus => 1,
            VehicleType::Tram => 5,
            VehicleType::Metro => 6,
            VehicleType::Train => 7,
            VehicleType::Ferry => 8,
            VehicleType::ULine => 9,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Finnish = 0,
    Swedish = 1,
    English = 2,
}

impl From<u8> for Language {
    fn from(value: u8) -> Self {
        match value {
            0 => Language::Finnish,
            1 => Language::Swedish,
            2 => Language::English,
            e => panic!("Given value ({}) for Language not supported.", e),
        }
    }
}

/// The type of device that sold the ticket, or recharged the card.
#[derive(Debug)]
pub enum SaleDevice {
    ServicePointSalesDevice(u16),
    DriverTicketMachine(u16),
    CardReader(u16),
    TicketMachine(u16),
    Server(u16),
    HSLSmallEquipment(u16),
    ExternalServiceEquipment(u16),
    Reserved(u16),
}

impl SaleDevice {
    pub(crate) fn new(device_type: u8, device_number: u16) -> SaleDevice {
        match device_type {
            0 => SaleDevice::ServicePointSalesDevice(device_number),
            1 => SaleDevice::DriverTicketMachine(device_number),
            2 => SaleDevice::CardReader(device_number),
            3 => SaleDevice::TicketMachine(device_number),
            4 => SaleDevice::Server(device_number),
            5 => SaleDevice::HSLSmallEquipment(device_number),
            6 => SaleDevice::ExternalServiceEquipment(device_number),
            7 => SaleDevice::Reserved(device_number),
            e => panic!("Given value ({}) for SaleDeviceType not supported.", e),
        }
    }
}

/// The type and value of area where boarding last happened.
#[derive(Debug)]
pub enum BoardingArea {
    Zone(ValidityZone),
    Vehicle(VehicleType),
    ZoneCircle(u8), // Not sure what this is. One of the old-style regions?
}

impl BoardingArea {
    pub(crate) fn new(area_type: u8, area_value: u8) -> BoardingArea {
        match area_type {
            0 => BoardingArea::Zone(ValidityZone::from(area_value)),
            1 => BoardingArea::Vehicle(VehicleType::from(area_type)),
            2 => BoardingArea::ZoneCircle(area_value),
            e => panic!("Given value ({}) for BoardingArea type not supported.", e),
        }
    }
}
