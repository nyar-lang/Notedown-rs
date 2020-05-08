pub struct ParserConfig {
    pub tab_size: usize
}


impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            tab_size: 4
        }
    }
}