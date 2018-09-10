use graphics::*;
use opengl_graphics::GlGraphics;

use geom::Vector2;
use piston::window::Size;
pub mod enemy;
pub mod player;

pub struct PhysicalObject {
    pub radius: f64,
    pub mass: f64,
    pub position: Vector2,
    pub velocity: Vector2
}

impl PhysicalObject {
    pub fn new(mass: f64, position: Vector2) -> PhysicalObject {
        PhysicalObject {
            radius: 1.0,
            mass,
            position,
            velocity: Vector2::new(0.0, 0.0)
        }
    }

    pub fn update(&mut self, dt: f64) {
        // v = s / t
        // s = v * t
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
    }

    pub fn apply(&mut self, dt: f64, force: &Vector2) {
        // F = m*a
        // a = F/m

        let ax = force.x / self.mass;
        let ay = force.y / self.mass;

        // dv = a*dt

        self.velocity.x += ax * dt;
        self.velocity.y += ay * dt;
    }
}

// Every object that needs to be rendered on screen.
pub trait GameObject {

    fn interact(&mut self, dt: f64, other: &mut GameObject) -> f64 {
        // F = G*m1*m2/r^2

        let x = self.get_position().x - other.get_position().x;
        let y = self.get_position().y - other.get_position().y;
        let x2 = x.powi(2);
        let y2 = y.powi(2);

        let r2 = x2 + y2;
        let G = 6.674 / 100_000_000_000.0 * 10_000_000.0;
        let F = G * self.physical_object().mass * other.physical_object().mass / r2;

        // assume that F distributes proportionally to Fx and Fy components
        let mut Fx = F * x2 / r2;
        let mut Fy = F * y2 / r2;

        if self.get_position().x > other.get_position().x {
            Fx = -Fx
        }
        if self.get_position().y > other.get_position().y {
            Fy = -Fy
        }

//        println!("Fx = {} Fy = {}", Fx, Fy);
        self.physical_object().apply(dt, &Vector2::new(Fx, Fy));
        other.physical_object().apply(dt, &Vector2::new(-Fx, -Fy));

        F
    }

    // Used to determine whether one object has collided with another
    // object.
    fn collides(&mut self, other: &mut GameObject) -> bool {
        // Two circles intersect if the distance between their centers is
        // between the sum and the difference of their radii.
        // TODO: Bounding boxes might be more efficient.
        let distance2 = self.distance_squared(other);
        let r_start = self.radius() - other.radius();
        let r_end = self.radius() + other.radius();

        return r_start.powf(2.0) <= distance2 && distance2 <= r_end.powf(2.0);
    }

    fn distance_squared(&self, other: &GameObject) -> f64 {
        let x = self.get_position().x - other.get_position().x;
        let y = self.get_position().y - other.get_position().y;
        x.powi(2) + y.powi(2)
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
    fn update(&mut self, dt: f64, _: Size) {
        self.physical_object().update(dt)
    }
}
