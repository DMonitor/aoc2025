use std::fmt::{Display, Formatter};
#[derive(PartialEq,Eq,Hash,Debug,Copy,Clone)]
pub struct Point3D
{
    pub x:i64,
    pub y:i64,
    pub z:i64,
}

impl Point3D
{
    // distance without sqrt, good enough for comparisons
    pub fn distance_cmp(self, other:Point3D) -> i64
    {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl Display for Point3D
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}



#[derive(PartialEq,Eq,Hash,Debug,Copy,Clone)]
pub struct Point2D
{
    pub x:i64,
    pub y:i64,
}

impl Point2D {
    pub fn area_rectangle(&self, other: &Point2D) -> u64 {
        (i64::abs_diff(self.x,other.x) + 1) * (i64::abs_diff(self.y,other.y) + 1)
    }
}

impl Display for Point2D
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}