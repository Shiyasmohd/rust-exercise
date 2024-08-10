use time::{OffsetDateTime, PrimitiveDateTime};
fn main() {}

pub fn time_after_gigaseconds(start: PrimitiveDateTime) -> PrimitiveDateTime {
    let required_time =
        OffsetDateTime::from_unix_timestamp(start.assume_utc().unix_timestamp() + (10_i64.pow(9)))
            .unwrap();
    PrimitiveDateTime::new(required_time.date(), required_time.time())
}
