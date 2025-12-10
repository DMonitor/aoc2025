use std::fmt::{Display, Formatter};
use crate::points::Point2D;

pub struct Space<T>
{
    x_bound:u64,
    y_bound:u64,
    space:Box<[T]>

}

impl<T: Copy + PartialEq > Space<T> {
    pub fn new(x_bound:u64, y_bound:u64, default:T)  -> Space<T> {
        let space:Box<[T]> = vec!(default;((x_bound)*y_bound) as usize).into_boxed_slice();
        Space {x_bound, y_bound, space}
    }

    pub fn x(&self) -> u64 { self.x_bound }
    pub fn y(&self) -> u64 { self.y_bound }

    pub fn set(&mut self, x:u64, y:u64, value:T) -> Option<bool> {
        if(x > self.x_bound) {
            return None;
        }
        if(y > self.y_bound) {
            return None;
        }
        let ret = self.space[(self.x_bound * y + x) as usize] != value;
        self.space[(self.x_bound * y + x) as usize] = value;
        Some(ret)
    }

    pub fn get(&self, x:u64, y:u64) -> Option<T> {
        if(x > self.x_bound) {
            return None;
        }
        if(y > self.y_bound) {
            return None;
        }
        Some(self.space[(self.x_bound * y + x) as usize])
    }

    pub fn set_point(&mut self, point:Point2D, value:T) -> Option<bool> {
        Space::set(self, point.x as u64, point.y as u64,value)
    }

    pub fn draw_line(&mut self, x1:u64, y1:u64, x2:u64, y2:u64, value: T) {
        if(x1 != x2 && y1 != y2) {
            panic!("({},{}) to ({},{}): Not a straight line",x1,y1,x2,y2);
        }

        let (sx,ex) = match x1 < x2 {
            true => (x1, x2),
            false => (x2, x1),
        };
        let (sy,ey) = match y1 < y2 {
            true => (y1, y2),
            false => (y2, y1),
        };
        for x in sx..=ex {
            for y in sy..=ey {
                self.set(x,y,value);
            }
        }
    }

    pub fn draw_line_point(&mut self, p1:Point2D, p2:Point2D, value: T)
    {
        self.draw_line(p1.x as u64,p1.y as u64,p2.x as u64,p2.y as u64, value);
    }
    pub fn flood_fill(&mut self, x:u64, y:u64, value:T)
    {
        if(self.get(x,y) == Some(value)) {
            return;
        }
        self.set(x,y,value);
        if(self.get(x+1,y) != Some(value))
        {
            self.flood_fill(x+1,y,value);
        }
        if(self.get(x-1,y) != Some(value))
        {
            self.flood_fill(x-1,y,value);
        }
        if(self.get(x,y+1) != Some(value))
        {
            self.flood_fill(x,y+1,value);
        }
        if(self.get(x,y-1) != Some(value))
        {
            self.flood_fill(x,y-1,value);
        }
    }

    pub fn flood_fill_point(&mut self, point:Point2D, value:T) {
        Space::flood_fill(self, point.x as u64, point.y as u64, value);
    }
}

impl Display for Space<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.y_bound {
            for x in 0..self.x_bound {
                //s.push(' ');
                match self.get(x,y) {
                    Some(true) => s.push('#'),
                    Some(false) => s.push('.'),
                    None => (),
                }
               // s.push(' ');
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}