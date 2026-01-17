use core::fmt::{Display, Formatter};

use alloc::{fmt, string::ToString};

use super::*;

pub trait Awk {
    fn awk(&self, source: &(dyn Display + '_), fmt: &mut Formatter) -> fmt::Result;
}
pub struct AwkCommand;
impl Awk for AwkCommand {
    fn awk(&self, source: &(dyn Display + '_), fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "awk '{}'", source.to_string().replace('\'', "'\"'\"'"))
    }
}

pub trait Compile {
    fn compile(&self, awk: &(dyn Awk + '_), fmt: &mut Formatter) -> core::fmt::Result;
}
