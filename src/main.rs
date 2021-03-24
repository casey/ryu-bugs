use proptest::{
    num::{f32, f64, i128, i64},
    proptest,
    strategy::{Just, Strategy},
};

use itoa::Buffer;

use time::Time;

fn main() {
    println!("Hello, world!");
}

fn format_string_strategy() -> impl Strategy<Value = String> {
    Just("%r".to_string())
}

proptest! {

  #[test]
  fn doesnt_crash_f32(input in f32::ANY) {
    let mut buffer = ryu::Buffer::new();
    let text = buffer.format(input);
    let parsed: f32 = text.parse().unwrap();

    if input.is_nan() {
      assert!(parsed.is_nan());
    } else {
      assert_eq!(parsed, input);
    }
  }

  #[test]
  fn doesnt_crash_f64(input in f64::ANY) {
    let mut buffer = ryu::Buffer::new();
    let text = buffer.format(input);
    let parsed: f64 = text.parse().unwrap();

    if input.is_nan() {
      assert!(parsed.is_nan());
    } else {
      assert_eq!(parsed, input);
    }
  }

  #[test]
  fn doesnt_crash_i64(input in i64::ANY) {
    let mut buf = Buffer::new();
    let text = buf.format(input);
    let parsed: i64 = text.parse().unwrap();
    assert_eq!(parsed, input);
  }

  #[test]
  fn doesnt_crash_i128(input in i128::ANY) {
    let mut buf = Buffer::new();
    let text = buf.format(input);
    let parsed: i128 = text.parse().unwrap();
    assert_eq!(parsed, input);
  }

  #[test]
  fn doesnt_crash_time(
    input in ".*",
    format_string in "date: %r",
  ) {
    Time::parse(input, format_string).unwrap_err();
  }
}
