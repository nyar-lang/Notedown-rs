use notedown_ast::{Context, ToHTML};
use std::fs::{read_to_string, File};

use std::io::Write;

pub trait ZolaBackend {
    fn parse_source(&mut self, path_from: &str) -> Result<(), std::io::Error>;
    fn write_target(&self) -> Result<(), std::io::Error>;
}
/*
impl ZolaBackend for Context {
    fn parse_source(&mut self, path_from: &str) -> Result<(), std::io::Error> {
        let r = read_to_string(path_from)?;
        self.parse(&r);
        return Ok(());
    }
    fn write_target(&self) -> Result<(), std::io::Error> {
        let mut file = match self.get_name() {
            None => File::create("Untitled")?,
            Some(s) => File::create(s)?,
        };
        let mut cfg = HTMLConfig::default();
        cfg.target = String::from("zola");
        file.write_all(self.to_html_with(&cfg).as_bytes())?;
        return Ok(());
    }
}
*/
