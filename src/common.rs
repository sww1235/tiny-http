use http::{HeaderName, HeaderValue};

pub(crate) fn header_from_str(input: &str) -> Result<(HeaderName, HeaderValue), ()> {
    let (field, value) = input.split_once(':').ok_or(())?;

    let field = field.parse().map_err(|_| ())?;
    let value = HeaderValue::from_str(value.trim()).map_err(|_| ())?;

    Ok((field, value))
}

#[cfg(test)]
mod test {
    use httpdate::HttpDate;
    use std::time::{Duration, SystemTime};

    #[test]
    fn formats_date_correctly() {
        let http_date = HttpDate::from(SystemTime::UNIX_EPOCH + Duration::from_secs(420895020));

        assert_eq!(http_date.to_string(), "Wed, 04 May 1983 11:17:00 GMT")
    }
}
