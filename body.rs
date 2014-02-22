use maths::Vec2;
use std::f64;

pub struct Body {
    pos: Vec2,
    vel: Vec2,
    force: Vec2,
    rotation: f64,
    angVel: f64,
    torque: f64,

    width: Vec2,

    mass: f64,
    invMass: f64,
    I: f64,
    invI: f64,
    friction: f64,
}

impl Body {
    fn new() -> Body {
        Body { pos: Vec2::new(),
               vel: Vec2::new(),
               force: Vec2::new(),
               rotation: 0.,
               angVel: 0.,
               torque: 0.,
               width: Vec2 { x: 1., y: 1. },
               mass: f64::MAX_VALUE,
               invMass: 0.,
               I: f64::MAX_VALUE,
               invI: 0.,
               friction: 0.1, 
        }
    }

    fn new_mw(w: Vec2, m: f64) -> Body {
       let I = (w.x * w.x + w.y * w.y) / 12.0;
        Body { pos: Vec2::new(),
               vel: Vec2::new(),
               force: Vec2::new(),
               rotation: 0.,
               angVel: 0.,
               torque: 0.,
               width: w,
               mass: m,
               invMass: 1. / m,
               I: I,
               invI: 1. / I,
               friction: 0.1, 
        }

    }
}
