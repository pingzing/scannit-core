mod en1545date {
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
    fn from_en1545_date(date: u16) -> DateTime<Utc> {
        from_en1545_date_and_time(date, 0u16)
    }

    /// Convert from En1545 (number of days since 1997-01-01, and number of minute since 00:00) to a standard UTC DateTime.
    /// # Arguments
    ///  * `date` - The date in En1545 format (number of days since 1997-01-01).
    ///  * `time` - The time in En1545 format (number minutes since 00:00).
    fn from_en1545_date_and_time(date: u16, time: u16) -> DateTime<Utc> {
        let local_datetime =
            *EN1545_ZERO_DATE + Duration::days(date as i64) + Duration::minutes(time as i64);
        // Assuming Helsinki because it's impossible to use an HSL travel card outside of Finland.
        // ...I hope.
        Helsinki
            .from_local_datetime(&local_datetime)
            .unwrap()
            .with_timezone(&Utc)
    }
}
