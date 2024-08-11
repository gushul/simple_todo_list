use chrono::{DateTime, Utc, NaiveDateTime};

pub fn parse(input: &str) -> Result<DateTime<Utc>, String> {
    let date_str = &input.replace(['"', '\''], "");
    
    // Формат даты: "DD-MM-YYYY HH:MM"
    let parsed_date = NaiveDateTime::parse_from_str(date_str, "%d-%m-%Y %H:%M")
        .map_err(|e| format!("Error parsing date: {e}"))?;

    Ok(DateTime::<Utc>::from_naive_utc_and_offset(parsed_date, Utc))
}
