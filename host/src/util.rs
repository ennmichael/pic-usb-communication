use std::thread;
use std::time::Duration;

pub fn exponential_backoff<F, E>(mut f: F) -> Result<(), E>
where
    F: FnMut() -> Result<(), E>,
{
    let mut timeout: Duration = Duration::from_millis(10);
    while let Err(e) = f() {
        if timeout > Duration::from_secs(1) {
            return Err(e);
        }

        thread::sleep(timeout);
        timeout *= 10;
    }

    Ok(())
}
