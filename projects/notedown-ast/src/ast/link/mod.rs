use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, Clone)]
pub enum SmartLink<'a> {
    // from, to
    Hyperlinks(Cow<'a, str>, Option<Cow<'a, str>>),
    // from, to
    Image(Cow<'a, str>, Option<Cow<'a, str>>, Option<Cow<'a, str>>),
    Reciprocal,
    Tag,
    Reference,
}

impl<'a> Display for SmartLink<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            SmartLink::Hyperlinks(from, None) => {
                write!(f, "[{}]", from)
            }
            SmartLink::Hyperlinks(from, Some(to)) => {
                write!(f, "[{} > {}]", from, to)
            }
            SmartLink::Image(_, _, _) => { unimplemented!() }
            SmartLink::Reciprocal => { unimplemented!() }
            SmartLink::Tag => { unimplemented!() }
            SmartLink::Reference => { unimplemented!() }
        }
    }
}