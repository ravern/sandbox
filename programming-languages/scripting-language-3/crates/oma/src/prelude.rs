use shu::Value;

pub const PRELUDE: [(&'static str, fn(Value) -> Value); 2] = [
  ("__console_info", console::info),
  ("__console_error", console::error),
];

mod console {
  use shu::Value;

  pub fn info(value: Value) -> Value {
    println!("{}", value.clone());
    value
  }

  pub fn error(value: Value) -> Value {
    eprintln!("{}", value.clone());
    value
  }
}
