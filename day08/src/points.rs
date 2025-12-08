use std::fmt::{Display, Formatter};
#[derive(PartialEq,Eq,Hash,Debug,Copy,Clone)]
pub struct Point
{
    pub x:i64,
    pub y:i64,
    pub z:i64,
}

impl Point
{
    pub fn distance(self, other:Point) -> i64
    {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl Display for Point
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}
