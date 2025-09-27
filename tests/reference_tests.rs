#[cfg(test)]
mod tests {
    use {chrono::prelude::*, natural_date_parser::date_parser};

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
