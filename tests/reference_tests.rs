#[cfg(test)]
mod tests {
    use {chrono::prelude::*, natural_date_parser::date_parser};

    #[test]
    fn test_in_15_minutes() {
        assert_eq!(
            date_parser::from_string_with_reference(
                "in 15 minutes",
                Local.with_ymd_and_hms(2025, 9, 7, 21, 0, 0).unwrap()
            )
            .unwrap(),
            Local.with_ymd_and_hms(2025, 9, 7, 21, 15, 0).unwrap()
        )
    }

    #[test]
    fn test_in_1_hour() {
        assert_eq!(
            date_parser::from_string_with_reference(
                "in 1 hour",
                Local.with_ymd_and_hms(2025, 9, 7, 21, 0, 0).unwrap()
            )
            .unwrap(),
            Local.with_ymd_and_hms(2025, 9, 7, 22, 0, 0).unwrap()
        )
    }

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

    // #[test]
    // fn test_specific_date() {
    //     assert_eq!(
    //         date_parser::from_string("2025/Sep/27",).unwrap(),
    //         Local.with_ymd_and_hms(2025, 9, 27, 13, 33, 0).unwrap()
    //     );
    // }

    #[test]
    fn test_specific_date_and_time() {
        assert_eq!(
            date_parser::from_string("2025/Sep/27 at 1:33 PM",).unwrap(),
            Local.with_ymd_and_hms(2025, 9, 27, 13, 33, 0).unwrap()
        );
        assert_eq!(
            date_parser::from_string("2025/Sep/7 at 1 Pm",).unwrap(),
            Local.with_ymd_and_hms(2025, 9, 7, 13, 0, 0).unwrap()
        );
    }
}
