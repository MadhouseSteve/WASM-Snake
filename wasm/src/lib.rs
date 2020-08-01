extern crate console_error_panic_hook;
use std::panic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[derive(Copy, Clone, Debug)]
enum ObjectType {
    SPACE,
    BORDER,
    HEAD,
    TAIL,
    FOOD,
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Copy, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

struct Colours {
    head: String,
    tail: String,
    border: String,
    space: String,
    food: String,
}

#[wasm_bindgen]
pub struct Game {
    cb: js_sys::Function,
    score: usize,
    lives: usize,

    height: usize,
    width: usize,
    size: usize,
    grid: Vec<Vec<ObjectType>>,

    position: Position,
    tail: Vec<Position>,
    length: usize,
    direction: Direction,

    colours: Colours,

    canvas: web_sys::HtmlCanvasElement,

    tick_count: usize,

    should_tick: bool,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(
        cb: js_sys::Function,
        canvas: web_sys::HtmlCanvasElement,
        size: usize,
        height: usize,
        width: usize,
    ) -> Game {
        panic::set_hook(Box::new(console_error_panic_hook::hook));

        let mut g = Game {
            cb,
            lives: 0,
            score: 0,

            height,
            width,
            size,

            colours: Colours {
                head: String::from("#a2fead"),
                tail: String::from("#62de6d"),
                border: String::from("#ffffff"),
                space: String::from("#000000"),
                food: String::from("#db55dd"),
            },

            canvas,
            grid: vec![vec![]],
            position: Position { x: 0, y: 0 },
            tail: vec![],
            length: 2,
            direction: Direction::DOWN,

            tick_count: 0,
            should_tick: true,
        };

        g.set_score(0);
        g.set_lives(3);

        g.set_board();

        g
    }

    #[wasm_bindgen]
    pub fn handle_key_press(&mut self, key: &str) {
        match key {
            "ArrowUp" => self.direction = Direction::UP,
            "ArrowLeft" => self.direction = Direction::LEFT,
            "ArrowDown" => self.direction = Direction::DOWN,
            "ArrowRight" => self.direction = Direction::RIGHT,
            _ => {}
        }
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        if self.should_tick == false {
            return;
        }
        self.tick_count = self.tick_count + 1;

        let speed = 21 - (self.length / 2);

        if self.tick_count % speed == 0 {
            self.do_tick();
            self.emit("TICK");
        }
    }

    fn set_board(&mut self) {
        self.position = Position {
            x: self.width / 2,
            y: self.height / 2,
        };
        self.tail = vec![];
        self.tail.push(Position {
            x: self.position.x,
            y: self.position.y - 2,
        });
        self.tail.push(Position {
            x: self.position.x,
            y: self.position.y - 1,
        });
        self.length = 2;
        self.grid = vec![vec![ObjectType::SPACE; self.height as usize]; self.width as usize];
        self.grid[self.position.x][self.position.y] = ObjectType::HEAD;
        self.grid[self.position.x][self.position.y - 1] = ObjectType::TAIL;
        self.grid[self.position.x][self.position.y - 2] = ObjectType::TAIL;

        self.grid[0] = vec![ObjectType::BORDER; self.height];
        self.grid[self.width - 1] = vec![ObjectType::BORDER; self.height];
        for x in 0..self.width {
            self.grid[x][0] = ObjectType::BORDER;
            self.grid[x][self.height - 1] = ObjectType::BORDER;
        }

        self.direction = Direction::DOWN;

        self.add_food();
        self.render();
        self.should_tick = true;
    }

    fn do_tick(&mut self) {
        let current_position = self.position;

        // Move head
        match self.direction {
            Direction::DOWN => self.position.y = self.position.y + 1,
            Direction::UP => self.position.y = self.position.y - 1,
            Direction::RIGHT => self.position.x = self.position.x + 1,
            Direction::LEFT => self.position.x = self.position.x - 1,
        }

        match self.grid[self.position.x][self.position.y] {
            ObjectType::BORDER => {
                self.has_died();
                return;
            }
            ObjectType::TAIL => {
                self.has_died();
                return;
            }
            ObjectType::FOOD => {
                self.emit("FOOD");
                self.set_score(self.score + 1);
                self.length = self.length + 1;
                self.add_food();
            }
            _ => {}
        }
        self.draw_cell(self.position.x, self.position.y, ObjectType::HEAD);

        // Move tail
        self.draw_cell(current_position.x, current_position.y, ObjectType::TAIL);
        self.tail.push(current_position);

        if self.tail.len() > self.length {
            self.draw_cell(self.tail[0].x, self.tail[0].y, ObjectType::SPACE);
            self.tail.remove(0);
        }
    }

    fn has_died(&mut self) {
        self.should_tick = false;
        self.set_lives(self.lives - 1);
        if self.lives > 0 {
            self.emit("DIED");
            self.set_board();
        } else {
            self.emit("GAME_OVER");
        }
    }

    fn set_score(&mut self, score: usize) {
        self.score = score;
        self.emit_f64("SET_SCORE", self.score as f64);
    }

    fn set_lives(&mut self, lives: usize) {
        self.lives = lives;
        self.emit_f64("SET_LIVES", self.lives as f64);
    }
    fn emit(&self, message: &str) {
        self.cb
            .call1(
                wasm_bindgen::JsValue::from(wasm_bindgen::JsValue::NULL).as_ref(),
                wasm_bindgen::JsValue::from(message).as_ref(),
            )
            .expect("Error calling cb");
    }

    fn emit_f64(&self, message: &str, value: f64) {
        self.cb
            .call2(
                wasm_bindgen::JsValue::from(wasm_bindgen::JsValue::NULL).as_ref(),
                wasm_bindgen::JsValue::from(message).as_ref(),
                wasm_bindgen::JsValue::from_f64(value).as_ref(),
            )
            .expect("Error calling cb");
    }

    fn render(&mut self) {
        self.emit("RENDER.PRE");

        for x in 0..self.grid.iter().count() {
            for y in 0..self.grid[x].iter().count() {
                self.draw_cell(x, y, self.grid[x][y]);
            }
        }

        self.emit("RENDER.POST");
    }

    fn draw_cell(&mut self, x: usize, y: usize, object: ObjectType) {
        self.emit("DRAW_CELL");
        self.grid[x][y] = object;

        let ctx = self
            .canvas
            .get_context("2d")
            .expect("Unable to get rendering context")
            .expect("Unable to get rendering context")
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("Unable to get rendering context");

        match object {
            ObjectType::SPACE => ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(
                self.colours.space.as_str(),
            )),
            ObjectType::BORDER => ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(
                self.colours.border.as_str(),
            )),
            ObjectType::HEAD => {
                ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(self.colours.head.as_str()))
            }
            ObjectType::TAIL => {
                ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(self.colours.tail.as_str()))
            }
            ObjectType::FOOD => {
                ctx.set_fill_style(&wasm_bindgen::JsValue::from_str(self.colours.food.as_str()))
            }
        }

        ctx.fill_rect(
            (x * self.size) as f64,
            (y * self.size) as f64,
            self.size as f64,
            self.size as f64,
        );
    }

    fn add_food(&mut self) {
        let mut open = vec![];

        for x in 0..self.grid.iter().count() {
            for y in 0..self.grid[x].iter().count() {
                match self.grid[x][y] {
                    ObjectType::SPACE => open.push(Position { x, y }),
                    _ => {}
                }
            }
        }

        let pos = open[(random() * open.len() as f64) as usize];
        log(&format!("{} {}", pos.x, pos.y));
        self.draw_cell(pos.x, pos.y, ObjectType::FOOD);
        self.emit("FOOD_ADDED");
    }
}
