use time::PrimitiveDateTime as DateTime;
use time::Duration as Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let duration = Duration::seconds(1_000_000_000);
    return start + duration;
}
