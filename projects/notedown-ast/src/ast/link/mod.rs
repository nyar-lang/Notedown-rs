use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum SmartLink {
    // from, to
    Hyperlinks { from: String, to: String, alt: Option<String>, bind: Option<String> },
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
                let from_to = if from == to { format!("[{}]", from) } else { format!("[{} > {}]", from, to) };
                match (bind, alt) {
                    (None, None) => write!(f, "{}", from_to),
                    (Some(bind), None) => write!(f, "{}[:{}]", from_to, bind),
                    (None, Some(alt)) => write!(f, "{}[{}]", from_to, alt),
                    (Some(bind), Some(alt)) => write!(f, "{}[{}:{}]", from_to, bind, alt),
                }
            }
            SmartLink::Image { img, to, alt, bind } => {
                let img_to = match to {
                    None => format!("[{}]", img),
                    Some(s) => format!("[{} > {}]", img, s),
                };
                match (bind, alt) {
                    (None, None) => write!(f, "{}", img_to),
                    (Some(bind), None) => write!(f, "{}[:{}]", img_to, bind),
                    (None, Some(alt)) => write!(f, "{}[{}]", img_to, alt),
                    (Some(bind), Some(alt)) => write!(f, "{}[{}:{}]", img_to, bind, alt),
                }
            }
            SmartLink::Reciprocal => unimplemented!(),
            SmartLink::Tag => unimplemented!(),
            SmartLink::Reference => unimplemented!(),
        }
    }
}
