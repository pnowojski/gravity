use graphics::*;
use opengl_graphics::GlGraphics;

use geom::Vector2;
use piston::window::Size;
pub mod enemy;
pub mod player;

pub struct PhysicalObject {
    radius: f64,
    mass: f64,
    position: Vector2,
    velocity: Vector2
}

impl PhysicalObject {
    pub fn new(position: Vector2) -> PhysicalObject {
        PhysicalObject {
            radius: 1.0,
            mass: 1.0,
            position,
            velocity: Vector2::new(0.0, 0.0)
        }
    }
}
// Every object that needs to be rendered on screen.
pub trait GameObject {

    // Used to determine whether one object has collided with another
    // object.
    fn collides(&mut self, other: &mut GameObject) -> bool {
        // Two circles intersect if the distance between their centers is
        // between the sum and the difference of their radii.
        // TODO: Bounding boxes might be more efficient.
        let x2 = self.position().x - other.position().x;
        let y2 = self.position().y - other.position().y;
        let sum = x2.powf(2.0) + y2.powf(2.0);

        let r_start = self.radius() - other.radius();
        let r_end = self.radius() + other.radius();

        return r_start.powf(2.0) <= sum && sum <= r_end.powf(2.0);
    }

    fn physical_object(&mut self) -> &mut PhysicalObject;

    fn get_physical_object(&self) -> &PhysicalObject;

    fn position(&mut self) -> &mut Vector2 {
        &mut self.physical_object().position
    }

    fn get_position(&self) -> &Vector2 {
        &self.get_physical_object().position
    }

    fn radius(&self) -> f64 {
        self.get_physical_object().radius
    }

    // Main draw function for this GameObject.
    fn render(&self, ctxt: &Context, gl: &mut GlGraphics);
    // Only call if debug mode is turned on.
    fn render_dbg(&self, _: &Context, _: &mut GlGraphics) {}
    // Handle updates to movement/animation/etc.
    fn update(&mut self, _: f64, _: Size) {}
}
