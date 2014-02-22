use body::Body;
use maths::Vec2;

pub struct World {
    bodies: ~[Body],
    gravity: Vec2,
    iterations: uint,
}

impl World {
    fn step(&mut self, dt: f64) {
        
        for b in self.bodies.mut_iter() {
            if b.invMass == 0.0 {
                continue;
            }

            // patiently awaiting += operator overloading
            b.vel = b.vel + (self.gravity + b.force * b.invMass) * dt;
            b.angVel = b.angVel + b.torque * (dt * b.invI);
        }
        

        for b in self.bodies.mut_iter() {
            b.pos = b.pos + b.vel * dt;
        }

    }

    fn clear(&mut self) {
        let mut x: Option<Body>;
        loop {
            x = self.bodies.pop();
            if x.is_none() {
                break;
            }
        }
    }

    fn add(&mut self, b: Body) {
        self.bodies.push(b);
    }
}
