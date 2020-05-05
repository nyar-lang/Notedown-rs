use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum SmartLink {
    // from, to
    Hyperlinks { from: String, to: Option<String>, alt: Option<String>, bind: Option<String> },
    // from, to, alt (, , Option<String>)
    Image { img: String, to: Option<String>, alt: Option<String>, bind: Option<String> },
    Reciprocal,
    Tag,
    Reference,
}

impl Display for SmartLink {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            SmartLink::Hyperlinks { from, to, alt, bind } => {
                let to = match to {
                    None => String::new(),
                    Some(s) => format!(" > {}", s),
                };
                match (bind, alt) {
                    (None, None) => write!(f, "[{}{}]", from, to),
                    (Some(bind), None) => write!(f, "[{}{}][:{}]", from, to, bind),
                    (None, Some(alt)) => write!(f, "[{}{}][{}]", from, to, alt),
                    (Some(bind), Some(alt)) => write!(f, "[{}{}][{}:{}]", from, to, bind, alt),
                }
            }
            SmartLink::Image { img, to, alt, bind } => {
                let to = match to {
                    None => format!(""),
                    Some(s) => format!(" > {}", s),
                };
                match (bind, alt) {
                    (None, None) => write!(f, "[{}{}]", img, to),
                    (Some(bind), None) => write!(f, "[{}{}][:{}]", img, to, bind),
                    (None, Some(alt)) => write!(f, "[{}{}][{}]", img, to, alt),
                    (Some(bind), Some(alt)) => write!(f, "[{}{}][{}:{}]", img, to, bind, alt),
                }
            }
            SmartLink::Reciprocal => unimplemented!(),
            SmartLink::Tag => unimplemented!(),
            SmartLink::Reference => unimplemented!(),
        }
    }
}
