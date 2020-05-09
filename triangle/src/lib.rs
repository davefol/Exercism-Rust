pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T: Copy + std::cmp::PartialOrd + From<u8> + std::ops::Add<Output = T>> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if Triangle::is_valid(sides) {
            Some(Triangle { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[0] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1]
            && self.sides[0] != self.sides[2]
            && self.sides[1] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[0] == self.sides[2]
            || self.sides[1] == self.sides[2]
    }

    fn is_valid(sides: [T; 3]) -> bool {
        for i in 0..=2 {
            if sides[i] <= T::from(0) || sides[i] > sides[(i + 1) % 3] + sides[(i + 2) % 3] {
                return false;
            }
        }
        true
    }
}
