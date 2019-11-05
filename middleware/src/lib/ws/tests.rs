use super::*;
use serde_json::Value;
#[test]
fn test_ws() -> Result<(), Error> {
color_backtrace::install();
let address = "ws://localhost:3401";
let connection = connect(address)?;
let hello_holo = ZomeCall::new("test-instance", "hello", "hello_holo", None);
let result = connection.call(&hello_holo)?;
if let Value::String(result) = result.inner()? {
  assert_eq!(result, "Hello Holo");
}
Ok(())
}
