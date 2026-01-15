use std::time::{SystemTime, UNIX_EPOCH};

use crate::types::Timestamp;
#[deprecated(since = "0.1.9", note = "please use `now_ms` instead")]
pub fn get_timestamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis() as i64
}
#[deprecated(since = "0.1.9", note = "please use `now_sec` instead")]
pub fn get_timestamp_as_sec() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_secs() as i64
}

pub fn now_ms() -> Timestamp {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis() as i64
}

pub fn now_sec() -> Timestamp {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_secs() as i64
}

pub fn str_2_decimal_value(s: &str) -> i128 {
    let mut parts = s.split('.');
    let int = parts.next().unwrap_or("0");
    let frac = parts.next().unwrap_or("");

    let mut value = int.parse::<i128>().unwrap() * 10_i128.pow(8);

    let frac_scaled = format!("{:0<width$}", frac, width = 8_usize);
    let frac_part = &frac_scaled[..8_usize];

    value += frac_part.parse::<i128>().unwrap();
    value
}

pub fn decimal_value_2_str(v: i128) -> String {
    let sign = if v < 0 { "-" } else { "" };
    let v = v.abs();

    let int = v / 100_000_000;
    let frac = v % 100_000_000;

    format!("{}{}.{:08}", sign, int, frac)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_2_decimal_value() {
        let s = "123.45678901";
        let s_dec = str_2_decimal_value(s);
        println!("s_dec: {:?}", s_dec);

        println!("s_dec: {:?}", str_2_decimal_value("123"));
        println!("s_dec: {:?}", str_2_decimal_value("0.00000001"));
        println!("s_dec: {:?}", str_2_decimal_value("0.1"));
        println!("s_dec: {:?}", str_2_decimal_value("200000.1"));
        println!("s_dec: {:?}", str_2_decimal_value("200000.1111"));
        println!("s_dec: {:?}", str_2_decimal_value("2000.1111"));
    }

    #[test]
    fn test_decimal_value_2_str() {
        println!(
            "s_dec: {:?}",
            decimal_value_2_str(str_2_decimal_value("123"))
        );
        println!(
            "s_dec: {:?}",
            decimal_value_2_str(str_2_decimal_value("0.00000001"))
        );
        println!(
            "s_dec: {:?}",
            decimal_value_2_str(str_2_decimal_value("0.1"))
        );
        println!(
            "s_dec: {:?}",
            decimal_value_2_str(str_2_decimal_value("200000.1"))
        );
        println!(
            "s_dec: {:?}",
            decimal_value_2_str(str_2_decimal_value("200000.1111"))
        );
        println!(
            "s_dec: {:?}",
            decimal_value_2_str(str_2_decimal_value("2000.1111"))
        );
    }
}
