#[cfg(feature = "email-service")]
mod fetch;
#[cfg(feature = "email-service")]
pub use fetch::post;
