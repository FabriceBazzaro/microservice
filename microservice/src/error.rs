pub use anyhow::{anyhow as anyhow_macro, Context, Result};
pub use thiserror::Error;

#[macro_export]
macro_rules! Err {
    ($err:expr $(,)?) => {{
        let error = $err;
        Err(anyhow::anyhow!(error))
    }};
}
