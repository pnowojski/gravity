extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use opengl_graphics::{GlyphCache, TextureSettings};
use piston::input::*;
use piston::window::Window;

mod color;
pub mod config;
mod geom;
mod gfx;

mod models;
use models::{GameObject};
use models::enemy::Enemy;
use models::player::Player;

const FIRE_COOLDOWN: f64 = 0.1; // Only allow user to shoot 10 bullets/sec.

pub struct App<'a> {
    pub window: config::GraphicsConfig,
    glyph_cache: GlyphCache<'a>,
    player: Player,
    enemies: Vec<Enemy>,
    debug_mode: bool,
}

impl<'a> App<'a> {
    pub fn new(window: config::GraphicsConfig) -> App<'a> {
        let size = window.settings.size();

        let (x, y) = ((size.width / 2) as f64,
                      (size.height / 2) as f64);

        let player = Player::new(x, y);

        // Load font(s) used in the game.
        let glyph_cache = GlyphCache::new("./assets/fonts/PxPlus_IBM_VGA8.ttf", (), TextureSettings::new())
            .expect("Unable to load font");

        return App {
            glyph_cache,
            player,
            window,
            enemies: Vec::new(),
            debug_mode: false,
        };
    }

    fn reset(&mut self) {
    }

    pub fn input(&mut self, button: &Button, is_press: bool) {
        if is_press {
            if let Button::Keyboard(key) = *button {
                match key {
                    Key::Up => self.player.start_move(geom::Direction::NORTH),
                    Key::Down => self.player.start_move(geom::Direction::SOUTH),
                    Key::Left => self.player.start_move(geom::Direction::WEST),
                    Key::Right => self.player.start_move(geom::Direction::EAST),
                    // Toggle debug mode.
                    Key::D => {
                        self.debug_mode = !self.debug_mode;
                        println!("Debug mode: {}", self.debug_mode);
                    },
                    // Reset game
                    Key::Return => {
                        self.reset();
                    }
                    _ => (),
                }
            }
        } else {
            if let Button::Keyboard(key) = *button {
                match key {
                    Key::Up => self.player.stop_move(geom::Direction::NORTH),
                    Key::Down => self.player.stop_move(geom::Direction::SOUTH),
                    Key::Left => self.player.stop_move(geom::Direction::WEST),
                    Key::Right => self.player.stop_move(geom::Direction::EAST),
                    // Toggle debug mode.
                    Key::D => {
                        if is_press {
                            self.debug_mode = !self.debug_mode;
                            println!("Debug mode: {}", self.debug_mode);
                        }
                    },
                    _ => (),
                }
            }
        }
    }

    // Render stuff on the screen.
    pub fn render(&mut self, args: &RenderArgs) {
        // Grab list of objects to render.
        let enemies = &self.enemies;
        let player = &self.player;
        let _gc = &mut self.glyph_cache;

        let debug_mode = self.debug_mode;
        let _size = self.window.settings.size();

        // Render stuff.
        self.window.gl.draw(args.viewport(), |c, gl| {
            use graphics::*;

            // Clear the screen.
            clear(::color::BLACK, gl);

            for enemy in enemies.iter() {
                enemy.render(&c, gl);
            }

            player.render(&c, gl);

            if debug_mode {
                player.render_dbg(&c, gl);
            }
        });
    }

    // Update any animation, etc.
    // dt is the delta since the last update.
    pub fn update(&mut self, args: &UpdateArgs) {
        let size = self.window.settings.size();

        // Update player & enemies
        self.player.update(args.dt, size);
        // If number of enemies is zero... spawn more!
        if self.enemies.len() == 0 {
            let size = self.window.settings.size();
            for _ in 0..10 {
                self.enemies.push(Enemy::new_rand(size.width as f64, size.height as f64));
            }
        }

        for enemy in self.enemies.iter_mut() {
            enemy.update(args.dt, size);
            // If the player collides with an enemy, game over!
            if enemy.collides(&mut self.player) {
            }
        }
    }
}