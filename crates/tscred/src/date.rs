use crate::Error;
use jiff::civil::Date;

pub(crate) fn parse_date_from_roc_calendar(roc_calendar_date: &str) -> Result<Date, Error> {
    let date_fragments = roc_calendar_date.split('-').collect::<Vec<&str>>();
    if date_fragments.len() != 3 {
        return Err(Error::ParseDateError(roc_calendar_date.to_string()));
    }

    let year = date_fragments[0]
        .parse::<i16>()
        .map_err(|_| Error::ParseDateError(roc_calendar_date.to_string()))?
        + 1911;
    let month = date_fragments[1]
        .parse::<i8>()
        .map_err(|_| Error::ParseDateError(roc_calendar_date.to_string()))?;
    let day = date_fragments[2]
        .parse::<i8>()
        .map_err(|_| Error::ParseDateError(roc_calendar_date.to_string()))?;

    Ok(Date::new(year, month, day)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_from_roc_calendar() {
        let date = parse_date_from_roc_calendar("114-09-30").unwrap();
        assert_eq!(date.year(), 2025);
        assert_eq!(date.month(), 9);
        assert_eq!(date.day(), 30);

        let date = parse_date_from_roc_calendar("114-09-31");
        assert!(date.is_err_and(|e| matches!(e, Error::JiffError(_))));

        let date = parse_date_from_roc_calendar("114-09-30 15:25:00");
        assert!(date.is_err_and(|e| matches!(e, Error::ParseDateError(_))));
    }
}
