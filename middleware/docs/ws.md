\#S:INCLUDE,MODE=test
# Call a zome over websocket
Make a connection to the websocket then
create a call and call it from the connection.
\#S:HIDE
```rust
use super::*;
use serde_json::Value;
#[test]
fn test_ws() -> Result<(), Error> {
color_backtrace::install();
```
```rust
let address = "ws://localhost:3401";
let connection = connect(address)?;
let hello_holo = ZomeCall::new("test-instance", "hello", "hello_holo", None);
let result = connection.call(&hello_holo)?;
```
Now pull out the inner string:
```rust
if let Value::String(result) = result.inner()? {
  assert_eq!(result, "Hello Holo");
}
```
\#S:HIDE
```rust
Ok(())
}
```
