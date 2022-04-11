pub type Result<T> = anyhow::Result<T>;

pub mod prelude {
    pub use super::Result;
    pub use anyhow::{anyhow, Context, Error};
}
