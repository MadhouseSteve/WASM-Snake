extern crate console_error_panic_hook;
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut g = Game::init();
    g.reset();

    // LOOP HERE!
    let f = Rc::new(RefCell::new(None));
    let f1 = f.clone();

    *f1.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        g.tick();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(f1.borrow().as_ref().unwrap());
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug)]
enum ObjectType {
    SPACE,
    BODY,
    HEAD,
    FOOD,
    WALL,
}

struct Game {
    score_element: web_sys::HtmlSpanElement,
    lives_element: web_sys::HtmlSpanElement,
    play_area: web_sys::HtmlCanvasElement,

    height: usize,
    width: usize,
    score: usize,
    lives: usize,

    tick_count: usize,

    snake_position: Position,
    snake_direction: Direction,
    snake_body: Vec<Position>,

    grid: Vec<ObjectType>,
}

impl Game {
    // Fetches all the DOM elements we want
    pub fn init() -> Game {
        let document = web_sys::window().unwrap().document().unwrap();

        let mut g = Game {
            // score
            score_element: document
                .get_element_by_id("score")
                .unwrap()
                .dyn_into::<web_sys::HtmlSpanElement>()
                .unwrap(),
            // lives
            lives_element: document
                .get_element_by_id("lives")
                .unwrap()
                .dyn_into::<web_sys::HtmlSpanElement>()
                .unwrap(),
            // play area
            play_area: document
                .get_element_by_id("board")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap(),

            // size of grid
            height: 21,
            width: 11,

            // game tracking
            lives: 3,
            score: 0,
            tick_count: 0,
            snake_position: Position { x: 0, y: 0 },
            snake_direction: Direction::DOWN,
            snake_body: vec![],

            // grid contents
            grid: vec![],
        };

        g
    }

    // Draws the grid on the page
    fn draw_grid(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.draw_object(y, x, self.grid[(y * self.width) + x]);
            }
        }
        log(&format!("{:?}", self.grid));
    }

    // Draws an object to the page
    fn draw_object(&self, row: usize, col: usize, object: ObjectType) {
        let ctx = self
            .play_area
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        match object {
            ObjectType::WALL => ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#ffffff")),
            ObjectType::SPACE => ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#000000")),
            ObjectType::HEAD => ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#62de6d")),
            ObjectType::BODY => ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#62de6d")),
            ObjectType::FOOD => ctx.set_fill_style(&wasm_bindgen::JsValue::from_str("#db55dd")),
        }
        ctx.fill_rect(col as f64 * 10.0, row as f64 * 10.0, 10.0, 10.0);
    }

    fn set_cell(&mut self, row: usize, col: usize, object: ObjectType) {
        self.grid[row * self.width + col] = object;
        self.draw_object(row, col, object);
    }

    // Fill a row with a object type
    fn fill_row(&mut self, row: usize, object: ObjectType) {
        let start = self.width * row;
        let end = self.width * (row + 1);
        for i in start..end {
            self.grid[i] = object
        }
    }

    // Fill a col with a object type
    fn fill_col(&mut self, col: usize, object: ObjectType) {
        let start = 0;
        let end = self.height;
        for i in start..end {
            self.grid[(i * self.width) + col] = object
        }
    }

    // Fill in the DOM elements
    pub fn reset(&mut self) {
        self.score = 0;
        self.lives = 3;
        self.snake_position = Position {
            x: self.width / 2,
            y: self.height / 2,
        };
        self.snake_direction = Direction::DOWN;
        self.grid = vec![ObjectType::SPACE; self.height * self.width];
        self.snake_body = vec![];
        self.snake_body.push(Position {
            x: self.snake_position.x,
            y: self.snake_position.y - 2,
        });
        self.snake_body.push(Position {
            x: self.snake_position.x,
            y: self.snake_position.y - 1,
        });

        self.set_cell(
            self.snake_position.y,
            self.snake_position.x,
            ObjectType::HEAD,
        );
        self.set_cell(
            self.snake_position.y - 2,
            self.snake_position.x,
            ObjectType::BODY,
        );
        self.set_cell(
            self.snake_position.y - 1,
            self.snake_position.x,
            ObjectType::BODY,
        );

        // Add borders to grid
        self.fill_row(0, ObjectType::WALL);
        self.fill_row(self.height - 1, ObjectType::WALL);
        self.fill_col(0, ObjectType::WALL);
        self.fill_col(self.width - 1, ObjectType::WALL);

        self.tick_count = 0;

        self.draw_grid();
    }

    // Handles a tick
    pub fn tick(&mut self) {
        self.tick_count = self.tick_count + 1;
        if self.tick_count % 10 == 0 {
            // Check snakes destination
            // If Food ... grow, move, place new food
            // If Space .. move
            // If Wall ... die
            // If Body ... die
            self.snake_move();
        }

        self.draw();
    }

    fn get_square(&self, x: usize, y: usize) {}

    fn snake_grow(&self) {}

    fn snake_move(&mut self) {
        // Remove the end of the tail
        self.set_cell(
            self.snake_body[0].y,
            self.snake_body[0].x,
            ObjectType::SPACE,
        );
        self.snake_body.remove(0);

        // Add a new square
        self.set_cell(
            self.snake_position.y,
            self.snake_position.x,
            ObjectType::BODY,
        );
        self.snake_body.push(self.snake_position.clone());

        // Move the head
        match self.snake_direction {
            Direction::DOWN => self.snake_position.y = self.snake_position.y + 1,
            Direction::UP => self.snake_position.y = self.snake_position.y - 1,
            Direction::LEFT => self.snake_position.x = self.snake_position.x - 1,
            Direction::RIGHT => self.snake_position.x = self.snake_position.x + 1,
        }

        // Draw the head
        self.set_cell(
            self.snake_position.y,
            self.snake_position.x,
            ObjectType::HEAD,
        );
    }

    fn food_place(&self) {}

    fn snake_die(&self) {}

    fn draw(&self) {
        self.score_element
            .set_inner_text(&format!("{}", self.score));
        self.lives_element
            .set_inner_text(&format!("{}", self.lives));
    }
}

// extern crate console_error_panic_hook;
// use std::panic;
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;

// #[wasm_bindgen]
// pub enum Direction {
//     UP,
//     DOWN,
//     LEFT,
//     RIGHT,
// }

// #[derive(Copy, Clone)]
// struct Position {
//     x: usize,
//     y: usize,
// }

// #[derive(Copy, Clone)]
// enum ObjectType {
//     SPACE,
//     BODY,
//     HEAD,
//     FOOD,
//     WALL,
//     BORDER,
// }

// #[wasm_bindgen]
// pub struct Game {
//     board: Vec<Vec<ObjectType>>,
//     canvas: web_sys::CanvasRenderingContext2d,
//     position: Position,
//     tail: Vec<Position>,
//     direction: Direction,
//     do_tick: bool,
//     length: usize,
// }

// #[wasm_bindgen]
// impl Game {
//     #[wasm_bindgen(constructor)]
//     pub fn new(board: web_sys::Node) -> Game {
//         panic::set_hook(Box::new(console_error_panic_hook::hook));

//         let canvas = board.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
//         let board = canvas
//             .get_context("2d")
//             .unwrap()
//             .unwrap()
//             .dyn_into::<web_sys::CanvasRenderingContext2d>()
//             .unwrap();

//         Game {
//             canvas: board,
//             position: Position {
//                 x: (canvas.width() / 10) as usize,
//                 y: (canvas.height() / 10) as usize,
//             },
//             direction: Direction::DOWN,
//             board: vec![
//                 vec![ObjectType::SPACE; (canvas.width() / 5) as usize];
//                 (canvas.height() / 5) as usize
//             ],
//             do_tick: true,
//             length: 2,
//             tail: Vec::with_capacity(0),
//         }
//     }

//     #[wasm_bindgen]
//     pub fn get_do_tick(self: &Game) -> JsValue {
//         wasm_bindgen::JsValue::from(self.do_tick)
//     }

//     #[wasm_bindgen]
//     pub fn set_board(self: &mut Game, place_food: bool) {
//         // Resets the game
//         self.board = vec![
//             vec![ObjectType::SPACE; self.board[0].iter().count()];
//             self.board.iter().count() as usize
//         ];
//         self.position = Position {
//             x: self.board[0].iter().count() / 2,
//             y: self.board.iter().count() / 2,
//         };
//         self.do_tick = true;
//         self.direction = Direction::DOWN;
//         self.length = 2;
//         self.tail = Vec::with_capacity(0);
//         if place_food == true {
//             self.place_food();
//         }

//         // Draws the board
//         self.fill_row(0, ObjectType::BORDER);
//         self.fill_row(self.board.iter().count() - 1, ObjectType::BORDER);
//         self.fill_col(0, ObjectType::BORDER);
//         self.fill_col(self.board[0].iter().count() - 1, ObjectType::BORDER);
//         self.board[self.position.y][self.position.x] = ObjectType::HEAD;
//         self.render_board();
//     }

//     #[wasm_bindgen]
//     pub fn key_press(self: &mut Game, key: &str) {
//         match key {
//             "ArrowLeft" => self.direction = Direction::LEFT,
//             "ArrowDown" => self.direction = Direction::DOWN,
//             "ArrowRight" => self.direction = Direction::RIGHT,
//             "ArrowUp" => self.direction = Direction::UP,
//             _ => {}
//         }
//     }

//     #[wasm_bindgen]
//     pub fn tick(self: &mut Game) {
//         if self.do_tick == true {
//             self.move_snake();
//             self.render_board();
//         }
//     }

//     // Fills a row with an object
//     fn fill_row(self: &mut Game, row_index: usize, object: ObjectType) {
//         for i in 0..self.board[row_index].iter().count() {
//             self.board[row_index][i] = object;
//         }
//     }

//     // Fills a column with an object
//     fn fill_col(self: &mut Game, col_index: usize, object: ObjectType) {
//         for i in 0..self.board.iter().count() {
//             self.board[i][col_index] = object;
//         }
//     }

//     fn move_snake(self: &mut Game) {
//         let initial_position = self.position;
//         self.board[self.position.y][self.position.x] = ObjectType::SPACE;

//         match self.direction {
//             Direction::DOWN => self.position.y = self.position.y + 1,
//             Direction::UP => self.position.y = self.position.y - 1,
//             Direction::LEFT => self.position.x = self.position.x - 1,
//             Direction::RIGHT => self.position.x = self.position.x + 1,
//         }

//         match self.board[self.position.y][self.position.x] {
//             ObjectType::SPACE => self.board[self.position.y][self.position.x] = ObjectType::HEAD,
//             ObjectType::FOOD => {
//                 self.board[self.position.y][self.position.x] = ObjectType::HEAD;
//                 self.length = self.length + 1;
//                 self.place_food();
//             }
//             _ => self.do_tick = false,
//         }

//         if self.tail.iter().count() >= self.length {
//             self.board[self.tail[0].y][self.tail[0].x] = ObjectType::SPACE;
//             self.tail.remove(0);
//         }
//         self.tail.push(initial_position);

//         for pos in self.tail.iter() {
//             self.board[pos.y][pos.x] = ObjectType::BODY;
//         }
//     }

//     fn place_food(self: &mut Game) {
//         let x = (random() * (self.board[0].iter().count() - 1) as f32) as usize;
//         let y = (random() * (self.board.iter().count() - 1) as f32) as usize;
//         self.board[y][x] = ObjectType::FOOD;
//     }

//     fn render_board(self: &Game) {
//         self.canvas.clear_rect(
//             0.0,
//             0.0,
//             (self.board[0].iter().count() * 5) as f64,
//             (self.board.iter().count() * 5) as f64,
//         );

//         for row in 0..self.board.iter().count() {
//             for cell in 0..self.board[row].iter().count() {
//                 match self.board[row][cell] {
//                     ObjectType::WALL => {
//                         self.canvas
//                             .set_fill_style(&wasm_bindgen::JsValue::from_str("#000000"));
//                         self.canvas
//                             .fill_rect((cell * 5) as f64, (row * 5) as f64, 4.0, 4.0)
//                     }
//                     ObjectType::BORDER => {
//                         self.canvas
//                             .set_fill_style(&wasm_bindgen::JsValue::from_str("#000000"));
//                         self.canvas
//                             .fill_rect((cell * 5) as f64, (row * 5) as f64, 5.0, 5.0)
//                     }
//                     ObjectType::HEAD => {
//                         self.canvas
//                             .set_fill_style(&wasm_bindgen::JsValue::from_str("#FF0000"));
//                         self.canvas
//                             .fill_rect((cell * 5) as f64, (row * 5) as f64, 4.0, 4.0)
//                     }
//                     ObjectType::BODY => {
//                         self.canvas
//                             .set_fill_style(&wasm_bindgen::JsValue::from_str("#FF00FF"));
//                         self.canvas
//                             .fill_rect((cell * 5) as f64, (row * 5) as f64, 4.0, 4.0)
//                     }
//                     ObjectType::FOOD => {
//                         self.canvas
//                             .set_fill_style(&wasm_bindgen::JsValue::from_str("#0000FF"));
//                         self.canvas
//                             .fill_rect((cell * 5) as f64, (row * 5) as f64, 4.0, 4.0)
//                     }
//                     ObjectType::SPACE => {
//                         self.canvas
//                             .set_fill_style(&wasm_bindgen::JsValue::from_str("#dddddd"));
//                         self.canvas
//                             .fill_rect((cell * 5) as f64, (row * 5) as f64, 4.0, 4.0)
//                     }
//                 }
//             }
//         }
//     }
// }
