// use std::ops::Add;

// use time::PrimitiveDateTime as DateTime;
// use time::ext::NumericalDuration;
use time::{Duration, PrimitiveDateTime as DateTime};

// const STD_GIGA_SECOND: std::time::Duration = std::time::Duration::from_secs(1_000_000_000);
// const GIGA_SECOND: Duration = Duration::new(1_000_000_000, 0);
const GIGA_SECOND: Duration = Duration::seconds(1_000_000_000);

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime
{
    // let giga_second = 1e9.seconds();

    // start.add(STD_GIGA_SECOND)
    // start.add(GIGA_SECOND)

    // start + STD_GIGA_SECOND
    // start + GIGA_SECOND

    // start.saturating_add(GIGA_SECOND)

    start.checked_add(GIGA_SECOND).unwrap()
}
