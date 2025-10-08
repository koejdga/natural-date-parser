# Natural Date Parser

- Crates: https://crates.io/crates/natural-date-parser
- Docs: https://docs.rs/natural-date-parser

## Brief Description

Name: natural-date-parser

A parser that converts natural language date and time expressions into Rust-compatible DateTime formats.

**Natural Date Parser** is a Rust project designed to parse human-friendly, natural language date and time expressions, such as "next Monday," "tomorrow at 5 PM," or "3 weeks from now." This parser converts these expressions into structured `DateTime` objects, making them usable in scheduling applications, reminders, or other time-based software.

## Technical Description

### Parsing Process

The parser is built using the [Pest](https://pest.rs/) parsing library and is designed to recognize a variety of natural language phrases related to dates and times. These include:

- **Simple Relative Dates**: "today," "tomorrow," "yesterday."
- **Day of the Week Expressions**: "next Monday," "last Friday."
- **Complex Relative Time Expressions**: "in 3 days," "2 weeks from now," "4 months ago."
- **Combined Date and Time Expressions**: "next Thursday at 10 AM," "tomorrow at 5:30 PM."

### How It Works

The parsing rules:

```
date_expression = { SOI ~ (relative_day_and_specific_time | relative_date | relative_term | specific_day_and_time | specific_day | specific_time | future_time ) ~ EOI }

relative_day_and_specific_time = { ( relative_date | relative_term ) ~ "at" ~ specific_time }

relative_date         = { next_or_last ~ specific_day }
relative_term         = { tomorrow | today | yesterday }
specific_day_and_time = { specific_day ~ "at" ~ specific_time }

specific_day   = { monday | tuesday | wednesday | thursday | friday | saturday | sunday }
specific_time  = { hour ~ ":" ~ minute ~ am_pm }
future_time    = { "in" ~ number ~ time_unit }

next_or_last = { next | last | this }

hour   = { ASCII_DIGIT+ }
minute = { ASCII_DIGIT+ }
am_pm  = { am | pm }

number    = { ASCII_DIGIT+ }
time_unit = { day_s | week_s | month_s | year_s }
```

### Use Cases

The output `DateTime` values can be used in a variety of applications:

- **Scheduling Apps**: Automatically parse reminders like "next Monday" and "in 3 days" into exact dates.
- **Task Management**: Set deadlines with natural language, making it easier to schedule tasks.
- **Chatbots and Virtual Assistants**: Interpret user queries about dates and times with ease.

## Getting Started

To start using **Natural Date Parser**, you can use [crate](https://crates.io/crates/natural-date-parser) on crates.io

### Example Usage

![alt text](<profile (unoptimized + debuginfo) target(s) in 0.01s.png>)
