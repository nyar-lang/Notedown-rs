extern crate walkdir;
mod traits;
mod utils;

use notedown_ast::{Context, ToHTML};
pub use traits::ZolaBackend;

use crate::{traits::parse_file, utils::filter_files};

fn main() {
    let files = filter_files("./projects");
    for file in files {
        parse_file(file);
    }
}
