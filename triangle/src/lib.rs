use std::ops::Add;

pub struct Triangle<T>(T, T, T)
where
    T: PartialEq + PartialOrd + Add<Output = T> + Copy;

impl<T> Triangle<T>
where
    T: PartialEq + PartialOrd + Add<Output = T> + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let [a, b, c] = sides;
        if a + b <= c || b + c <= a || a + c <= b {
            return None;
        }

        Some(Triangle(a, b, c))
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.0 == self.2
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2 || self.0 == self.2
    }

    pub fn is_scalene(&self) -> bool {
        self.0 != self.1 && self.1 != self.2 && self.0 != self.2
    }
}
