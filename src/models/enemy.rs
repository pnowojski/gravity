use graphics::{Context, rectangle, Transformed};
use opengl_graphics::GlGraphics;
use rand;
use rand::Rng;

use color;
use geom;
use piston::window::Size;
use super::GameObject;
use super::PhysicalObject;

const ENEMY_RADIUS: f64 = 10.0;

pub struct Enemy {
    pub physical_object: PhysicalObject,
    pub size: f64,
}

impl Enemy {
    pub fn new(x: f64, y: f64) -> Enemy {
        return Enemy {
            physical_object: PhysicalObject::new(1_000_000_000.0, geom::Vector2::new(x, y)),
            size: ENEMY_RADIUS * 2.0,
        };
    }

    pub fn new_rand(max_x: f64, max_y: f64) -> Enemy {
        let mut rng = rand::thread_rng();
        let randx = rng.gen_range(0.0, max_x);
        let randy = rng.gen_range(0.0, max_y);
        return Enemy::new(randx, randy);
    }
}

impl GameObject for Enemy {

    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render the player as a little square
        let square = rectangle::square(0.0, 0.0, self.size);
        let radius = self.get_physical_object().radius;
        let transform = ctxt.transform.trans(self.get_position().x, self.get_position().y)
            .trans(-radius, -radius);

        rectangle(color::GREEN, square, transform, gl);
    }

    fn render_dbg(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render collison box
        let radius = self.get_physical_object().radius;
        let diam = radius * 2.0;

        let circle = rectangle::Rectangle::new_round_border(color::WHITE, radius, 1.0);
        // Center on x/y
        let transform = ctxt.transform
            .trans(self.get_position().x, self.get_position().y)
            .trans(-radius, -radius);

        circle.draw([0.0, 0.0, diam, diam], &ctxt.draw_state, transform, gl);
    }

    fn update(&mut self, dt: f64, size: Size) {
        // TODO: Prevent movement outside of boundaries.
        let radius = self.radius();

        self.physical_object.update(dt);

        geom::restrict_to_bounds(
            &mut self.position(),
            [radius, radius, size.width as f64, size.height as f64]
        );
    }

    fn physical_object(&mut self) -> &mut PhysicalObject {
        &mut self.physical_object
    }

    fn get_physical_object(&self) -> &PhysicalObject {
        &self.physical_object
    }
}