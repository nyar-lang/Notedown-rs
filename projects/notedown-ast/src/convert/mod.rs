#[cfg(feature = "markdown")]
mod markdown;

#[cfg(feature = "markdown")]
pub use markdown::markdown_parse;