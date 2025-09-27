#[cfg(test)]
mod tests {
    use {chrono::prelude::*, natural_date_parser::date_parser};

    #[test]
    fn test_in_3_days() {
        assert_eq!(
            date_parser::from_string_with_reference(
                "in 3 days",
                Local.with_ymd_and_hms(2025, 9, 7, 21, 0, 0).unwrap()
            )
            .unwrap(),
            Local.with_ymd_and_hms(2025, 9, 10, 21, 0, 0).unwrap()
        )
    }

    #[test]
    fn test_in_5_weeks() {
        assert_eq!(
            date_parser::from_string_with_reference(
                "in 5 weeks",
                Local.with_ymd_and_hms(2025, 9, 7, 21, 0, 0).unwrap()
            )
            .unwrap(),
            Local.with_ymd_and_hms(2025, 10, 12, 21, 0, 0).unwrap()
        )
    }

    #[test]
    fn test_in_2_months() {
        assert_eq!(
            date_parser::from_string_with_reference(
                "in 2 months",
                Local.with_ymd_and_hms(2025, 9, 7, 21, 0, 0).unwrap()
            )
            .unwrap(),
            Local.with_ymd_and_hms(2025, 11, 7, 21, 0, 0).unwrap()
        )
    }

    #[test]
    fn test_in_12_years() {
        assert_eq!(
            date_parser::from_string_with_reference(
                "in 12 years",
                Local.with_ymd_and_hms(2025, 9, 7, 21, 0, 0).unwrap()
            )
            .unwrap(),
            Local.with_ymd_and_hms(2037, 9, 7, 21, 0, 0).unwrap()
        )
    }
}
