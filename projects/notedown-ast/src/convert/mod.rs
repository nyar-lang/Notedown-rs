#[cfg(feature = "markdown")]
mod markdown;

#[cfg(feature = "markdown")]
pub use self::markdown::markdown_parse;
