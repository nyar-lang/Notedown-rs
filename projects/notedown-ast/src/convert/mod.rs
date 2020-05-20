#[cfg(feature = "notedown")]
mod markdown;
#[cfg(feature = "orgize")]
mod orgize;

#[cfg(feature = "notedown")]
pub use self::markdown::markdown_parse;
#[cfg(feature = "orgize")]
pub use self::orgize::org_mode_parse;
