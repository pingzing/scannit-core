use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::Europe::Helsinki;
use lazy_static::*;

lazy_static! {
    static ref EN1545_ZERO_DATE: NaiveDateTime = NaiveDateTime::new(
        NaiveDate::from_ymd(1997, 01, 01),
        NaiveTime::from_hms(0, 0, 0)
    );
}

/// Convert from En1545 (number of days since 1997-01-01) to a standard UTC DateTime.
/// # Arguments
///  * `date` - The date in En1545 format (number of days since 1997-01-01).    
pub fn from_en1545_date(date: u16) -> DateTime<Utc> {
    from_en1545_date_and_time(date, 0u16)
}

/// Convert from En1545 (number of days since 1997-01-01, and number of minute since 00:00) to a standard UTC DateTime.
/// # Arguments
///  * `date` - The date in En1545 format (number of days since 1997-01-01).
///  * `time` - The time in En1545 format (number minutes since 00:00).
pub fn from_en1545_date_and_time(date: u16, time: u16) -> DateTime<Utc> {
    let local_datetime =
        *EN1545_ZERO_DATE + Duration::days(date as i64) + Duration::minutes(time as i64);
    // Assuming Helsinki because it's impossible to use an HSL travel card outside of Finland.
    // ...I hope.
    Helsinki
        .from_local_datetime(&local_datetime)
        .unwrap()
        .with_timezone(&Utc)
}

// --- TESTS ---
#[cfg(test)]
mod test {
    use crate::en1545date::{from_en1545_date, from_en1545_date_and_time};
    use chrono::prelude::*;

    #[test]
    fn should_handle_summer_dates() {
        let value = from_en1545_date(19514); // Should be 2050-06-06, in Helsinki time.
                                             // -3h in UTC because summer time is active.
        let expected = Utc.ymd(2050, 6, 5).and_hms(21, 0, 0);
        assert_eq!(value, expected);
    }

    #[test]
    fn should_handle_winter_dates() {
        let actual = from_en1545_date(0); // 1997-01-01, in Helsinki time.
                                          // Only -2h from UTC.
        let expected = Utc.ymd(1996, 12, 31).and_hms(22, 0, 0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_handle_summer_datetimes() {
        let value = from_en1545_date_and_time(19514, 150); // Should be 2050-06-06, 2:30AM in Helsinki time.
                                                           // Then -3h in UTC because summer time is active.
        let expected = Utc.ymd(2050, 6, 5).and_hms(23, 30, 0);
        assert_eq!(value, expected);
    }

    #[test]
    fn should_handle_winter_datetimes() {
        let value = from_en1545_date_and_time(0, 240); // should be 1997-01-01 4:00AM.
                                                       // Then -2h in UTC because winter time is active
        let expected = Utc.ymd(1997, 01, 01).and_hms(2, 0, 0);
        assert_eq!(value, expected);
    }
}
