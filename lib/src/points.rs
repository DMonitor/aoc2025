use std::collections::HashMap;
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


    // returns compressed point space (retaining order) and mapping from compressed to real space
    pub fn compress2d(points: Vec<Point2D>) -> (Vec<Point2D>,HashMap<Point2D,Point2D>)
    {
        fn compress_space(input_vec: Vec<i64>) -> HashMap<i64,usize>
        {
            let mut workspace = input_vec;
            workspace.sort();
            workspace.dedup();
            workspace.iter().enumerate().map(|(i,x)| (*x,i) ).collect()
        }

        let x_map = compress_space(points.iter().map(|p| p.x).collect::<Vec<_>>());
        let y_map = compress_space(points.iter().map(|p| p.y).collect::<Vec<_>>());
        let mapping = points.iter().map( |p| (Point2D { x: *x_map.get(&p.x).unwrap() as i64, y:*y_map.get(&p.y).unwrap() as i64},*p)).collect::<HashMap<_,_>>();

        let compressed_points = points.iter().map( |p| Point2D { x: *x_map.get(&p.x).unwrap() as i64, y:*y_map.get(&p.y).unwrap() as i64}).collect::<Vec<_>>();

        (compressed_points,mapping)
    }
}

impl Display for Point2D
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}