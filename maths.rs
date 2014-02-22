#[deriving(Clone)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    pub fn new() -> Vec2 {
        Vec2 { x: 0., y: 0. }
    }
}

impl Add<Vec2, Vec2> for Vec2 {
    fn add(&self, v: &Vec2) -> Vec2 {
        Vec2 { x: self.x + v.x,
               y: self.y + v.y }

    }
}

impl Sub<Vec2, Vec2> for Vec2 {
    fn sub(&self, v: &Vec2) -> Vec2 {
        Vec2 { x: self.x - v.x,
               y: self.y - v.y }

    }
}

impl Mul<f64, Vec2> for Vec2 {
    fn mul(&self, rhs: &f64) -> Vec2 {
        Vec2 { x: self.x * *rhs,
               y: self.y * *rhs }

    }
}
