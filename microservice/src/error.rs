pub use anyhow::{Context, Result};
pub use thiserror::Error;

pub use thiserror;
pub use anyhow;

#[macro_export]
macro_rules! Err {
    ($err:expr $(,)?) => {{
        let error = $err;
        Err(anyhow::anyhow!(error))
    }};
}
