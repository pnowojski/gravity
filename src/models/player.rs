use graphics::{Context, rectangle, polygon, Transformed};
use opengl_graphics::GlGraphics;

use color;
use geom;
use geom::Direction;
use piston::window::Size;
use super::GameObject;
use super::PhysicalObject;

const PLAYER_SPEED: f64 = 200_000_000.0;
const PLAYER_SIZE: f64 = 20.0;
// Drift for this long after movement key is released.
// You don't came to a hard stop in space!
const PLAYER_DRIFT: f64 = 0.2;

pub struct Player {
    pub physical_object: PhysicalObject,
    pub size: f64,
    pub drift_ttl: f64,
    move_offset: geom::Vector2,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Player {
        return Player {
            physical_object: PhysicalObject::new(1000_000.0, geom::Vector2::new(x, y)),
            drift_ttl: 0.0,
            move_offset: geom::Vector2::new(0.0, 0.0),
            size: PLAYER_SIZE,
        };
    }

    pub fn start_move(&mut self, dir: Direction) {
        match dir {
            Direction::WEST => self.move_offset.x = -PLAYER_SPEED,
            Direction::NORTH => self.move_offset.y = -PLAYER_SPEED,
            Direction::EAST => self.move_offset.x = PLAYER_SPEED,
            Direction::SOUTH => self.move_offset.y = PLAYER_SPEED,
        }
    }

    pub fn stop_move(&mut self, dir: Direction) {
        match dir {
            Direction::WEST => self.move_offset.x = 0.0,
            Direction::NORTH => self.move_offset.y = 0.0,
            Direction::EAST => self.move_offset.x = 0.0,
            Direction::SOUTH => self.move_offset.y = 0.0,
        }
    }
}

impl GameObject for Player {

    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        let square = rectangle::square(0.0, 0.0, self.size);
        let radius = self.get_physical_object().radius;
        let transform = ctxt.transform.trans(self.get_position().x, self.get_position().y)
            .trans(-radius, -radius);

        rectangle(color::RED, square, transform, gl);
    }

    fn render_dbg(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render collison box
        let radius = self.radius();
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

        self.physical_object.apply(dt, &self.move_offset);
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