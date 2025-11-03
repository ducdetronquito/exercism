use time::{PrimitiveDateTime as DateTime, ext::NumericalDuration};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    const GIGA : i64 = 1_000_000_000;
    start.checked_add(GIGA.seconds()).unwrap()
}
