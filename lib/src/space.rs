use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use crate::points::Point2D;
use crate::space;

pub struct Space<T>
{
    x_bound:u64,
    y_bound:u64,
    space:Box<[T]>,
    default:T
}

impl<T: Copy + PartialEq > Space<T> {
    pub fn new(x_bound:u64, y_bound:u64, default:T)  -> Space<T> {
        let space:Box<[T]> = vec!(default;((x_bound)*y_bound) as usize).into_boxed_slice();
        Space {x_bound, y_bound, space, default}
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
        if(x >= self.x_bound) {
            return None;
        }
        if(y >= self.y_bound) {
            return None;
        }
        Some(self.space[(self.x_bound * y + x) as usize])
    }

    pub fn set_point(&mut self, point:&Point2D, value:T) -> Option<bool> {
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

    pub fn draw_line_point(&mut self, p1:&Point2D, p2:&Point2D, value: T)
    {
        self.draw_line(p1.x as u64,p1.y as u64,p2.x as u64,p2.y as u64, value);
    }

    pub fn flood_fill(&mut self, x:u64, y:u64, replace:T, value:T)
    {
        let mut queue = VecDeque::<(u64,u64)>::new();
        queue.push_back((x,y));

        while !queue.is_empty() {
            let (x,y) = queue.pop_front().unwrap();
            if self.get(x, y) != Some(replace) {
                continue;
            }
            self.set(x,y,value);
            if self.get(x+1, y) == Some(replace)
            {
                queue.push_back((x+1,y));
            }
            if x!= 0 && self.get(x-1, y) == Some(replace)
            {
                queue.push_back((x-1,y));
            }
            if self.get(x, y+1) == Some(replace)
            {
                queue.push_back((x,y+1));
            }
            if y!=0 && self.get(x, y-1) == Some(replace)
            {
                queue.push_back((x,y-1));
            }
        }
    }

    pub fn flood_fill_point(&mut self, point:&Point2D, replace:T, value:T) {
        Space::flood_fill(self, point.x as u64, point.y as u64, replace, value);
    }

    pub fn get_rectangle(&mut self, x1:u64, y1:u64, x2:u64, y2:u64) -> Option<Space<T>>
    {
        if ! (x1 < self.x_bound && y1 < self.y_bound && x2 < self.x_bound && y2 < self.y_bound) {
            return None;
        }

        let x_size = u64::abs_diff(x1, x2) + 1;
        let y_size = u64::abs_diff(y1, y2) + 1;

        let mut new_space = Space::new(x_size, y_size, self.default);

        let (sx,ex) = match x1 < x2 {
            true => (x1, x2),
            false => (x2, x1),
        };
        let (sy,ey) = match y1 < y2 {
            true => (y1, y2),
            false => (y2, y1),
        };
        for x in (sx..=ex).enumerate() {
            for y in (sy..=ey).enumerate() {
                new_space.set(x.0 as u64,y.0 as u64,self.get(x.1,y.1).unwrap());
            }
        }

        Some(new_space)
    }

    pub fn get_rectangle_points(&mut self, point1:&Point2D, point2:&Point2D) -> Option<Space<T>>
    {
        self.get_rectangle(point1.x as u64, point1.y as u64, point2.x as u64, point2.y as u64)
    }

    pub fn any(&mut self, value:T) -> bool
    {
        self.space.contains(&value)
    }

    pub fn any_inside(&mut self, value:T, x1:u64, y1:u64, x2:u64, y2:u64) -> Option<bool> {

        if ! (x1 < self.x_bound && y1 < self.y_bound && x2 < self.x_bound && y2 < self.y_bound) {
            return None;
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
                if self.get(x, y) == Some(value) {
                    return Some(true);
                }
            }
        }
        Some(false)
    }

    pub fn any_inside_points(&mut self, value:T, point1:&Point2D, point2:&Point2D) -> Option<bool>
    {
        self.any_inside(value, point1.x as u64, point1.y as u64, point2.x as u64, point2.y as u64)
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


impl Display for Space<char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.y_bound {
            for x in 0..self.x_bound {
                //s.push(' ');
                s.push(self.get(x,y).unwrap())
                // s.push(' ');
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}