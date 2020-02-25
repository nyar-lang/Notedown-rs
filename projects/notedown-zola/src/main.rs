mod traits;

use notedown_ast::{Context, ToHTML};
pub use traits::ZolaBackend;

fn main() {
    let mut c = Context::default();
    c.parse("# a");
    println!("{}", c.to_html())
}
