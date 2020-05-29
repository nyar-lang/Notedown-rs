mod to_html;
pub trait ToHTML {
    fn to_html(&self) -> String;
}

// #[derive(Debug, Clone)]
// pub struct NotedownMeta {
// pub file_name: Option<String>,
// pub file_path: Option<PathBuf>,
// pub created_time: Option<NaiveDateTime>,
// pub modified_time: Option<NaiveDateTime>,
// pub title: Option<String>,
// pub tags: Vec<String>,
// pub categories: Vec<String>,
// pub series: Vec<String>,
// pub weight: usize,
// pub references: HashMap<Box<str>, String>,
// }
//
// impl Default for NotedownMeta {
// fn default() -> Self {
// Self {
// extra
// file_name: None,
// file_path: None,
// created_time: None,
// modified_time: None,
// title: None,
// tags: vec![],
// categories: vec![],
// series: vec![],
// weight: 0,
// references: Default::default(),
// }
// }
// }
