use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, Clone)]
pub enum SmartLink<'a> {
    // from, to
    Hyperlinks {
        from: Cow<'a, str>,
        to: Option<Cow<'a, str>>,
        alt: Option<Cow<'a, str>>,
        bind: Option<Cow<'a, str>>,
    },
    // from, to, alt (, , Option<Cow<'a, str>>)
    Image {
        img: Cow<'a, str>,
        to: Option<Cow<'a, str>>,
        alt: Option<Cow<'a, str>>,
        bind: Option<Cow<'a, str>>,
    },
    Reciprocal,
    Tag,
    Reference,
}

impl<'a> Display for SmartLink<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            SmartLink::Hyperlinks { from, to, alt, bind } => {
                let to = match to {
                    None => String::new(),
                    Some(s) => { format!(" > {}", s) }
                };
                match (bind, alt) {
                    (None, None) => {
                        write!(f, "[{}{}]", from, to)
                    }
                    (Some(bind), None) => {
                        write!(f, "[{}{}][:{}]", from, to, bind)
                    }
                    (None, Some(alt)) => {
                        write!(f, "[{}{}][{}]", from, to, alt)
                    }
                    (Some(bind), Some(alt)) => {
                        write!(f, "[{}{}][{}:{}]", from, to, bind, alt)
                    }
                }
            }
            SmartLink::Image { img, to, alt, bind } => {
                let to = match to {
                    None => { format!("") }
                    Some(s) => { format!(" > {}", s) }
                };
                match (bind, alt) {
                    (None, None) => {
                        write!(f, "[{}{}]", img, to)
                    }
                    (Some(bind), None) => {
                        write!(f, "[{}{}][:{}]", img, to, bind)
                    }
                    (None, Some(alt)) => {
                        write!(f, "[{}{}][{}]", img, to, alt)
                    }
                    (Some(bind), Some(alt)) => {
                        write!(f, "[{}{}][{}:{}]", img, to, bind, alt)
                    }
                }
            }
            SmartLink::Reciprocal => { unimplemented!() }
            SmartLink::Tag => { unimplemented!() }
            SmartLink::Reference => { unimplemented!() }
        }
    }
}