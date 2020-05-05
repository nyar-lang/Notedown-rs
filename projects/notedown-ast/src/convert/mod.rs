#[cfg(feature = "markdown")]
mod markdown;
#[cfg(feature = "orgize")]
mod orgize;

#[cfg(feature = "markdown")]
pub use self::markdown::markdown_parse;
#[cfg(feature = "orgize")]
pub use self::orgize::org_mode_parse;
