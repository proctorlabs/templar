use super::*;
use std::any::Any;

#[derive(Debug)]
struct SendSyncContainer(Box<dyn Any + Send + Sync>);

// This will simply fail to compile if the test fails, so we just always pass.
#[test]
fn shared_context_has_send_and_sync() -> Result<()> {
    let container = SendSyncContainer(Box::new(StandardContext::new()));
    println!("{:?}", container);
    Ok(())
}
