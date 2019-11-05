use super::*;
#[test]
fn test_ws() -> Result<(), Error> {
color_backtrace::install();
let address = "ws://localhost:3401";
let connection = connect(address)?;
let hello_holo = ZomeCall::new("test-instance", "hello", "hello_holo", None);
let result = connection.call(&hello_holo);
assert_eq!(result, "hello :)");
}
