use maths::Vec2;
use world::World;
use body::Body;

mod maths;
mod body;
mod world;

fn main() {
    let dt: f64 = 0.001;
    let iters = 10u;
    let g = Vec2 { x: 0., y: -9.8 };

    let mut b1 = Body::new_mw(Vec2 { x: 60., y: 30. }, 100.);
    b1.pos = Vec2 { x: -2., y: 10. };

    let mut b2 = Body::new_mw(Vec2 { x: 60., y: 30. }, 100.);
    b2.pos = Vec2 { x: 5., y: 5. };

    let mut world = World::new(g, iters, ~[b1, b2]);

}
