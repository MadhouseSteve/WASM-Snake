extern crate console_error_panic_hook;
use std::panic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Position {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
enum ObjectType {
    SPACE,
    SNAKE_BODY,
    SNAKE_HEAD,
    FOOD,
    WALL,
}

#[wasm_bindgen]
pub struct Game {
    board: Vec<Vec<ObjectType>>,
    canvas: web_sys::CanvasRenderingContext2d,
    position: Position,
    direction: Direction,
    doTick: bool,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(board: web_sys::Node) -> Game {
        panic::set_hook(Box::new(console_error_panic_hook::hook));

        let canvas = board.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let board = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Game {
            canvas: board,
            position: Position { x: 10, y: 10 },
            direction: Direction::DOWN,
            board: vec![
                vec![ObjectType::SPACE; (canvas.width() / 5) as usize];
                (canvas.height() / 5) as usize
            ],
            doTick: true,
        }
    }

    #[wasm_bindgen]
    pub fn set_board(self: &mut Game) {
        self.board = vec![
            vec![ObjectType::SPACE; self.board[0].iter().count()];
            self.board.iter().count() as usize
        ];

        self.fill_row(0, ObjectType::WALL);
        self.fill_row(self.board.iter().count() - 1, ObjectType::WALL);
        self.fill_col(0, ObjectType::WALL);
        self.fill_col(self.board[0].iter().count() - 1, ObjectType::WALL);

        self.board[self.position.y][self.position.x] = ObjectType::SNAKE_HEAD;
    }

    #[wasm_bindgen]
    pub fn key_press(self: &mut Game, key: &str) {
        match key {
            "ArrowLeft" => self.direction = Direction::LEFT,
            "ArrowDown" => self.direction = Direction::DOWN,
            "ArrowRight" => self.direction = Direction::RIGHT,
            "ArrowUp" => self.direction = Direction::UP,
            _ => {}
        }
    }

    #[wasm_bindgen]
    pub fn tick(self: &mut Game) {
        if self.doTick == true {
            self.move_snake();
            self.render_board();
        }
    }

    // Fills a row with an object
    fn fill_row(self: &mut Game, row_index: usize, object: ObjectType) {
        for i in 0..self.board[row_index].iter().count() {
            self.board[row_index][i] = object;
        }
    }

    // Fills a column with an object
    fn fill_col(self: &mut Game, col_index: usize, object: ObjectType) {
        for i in 0..self.board.iter().count() {
            self.board[i][col_index] = object;
        }
    }
    fn move_snake(self: &mut Game) {
        self.board[self.position.y][self.position.x] = ObjectType::SPACE;
        match self.direction {
            Direction::DOWN => self.position.y = self.position.y + 1,
            Direction::UP => self.position.y = self.position.y - 1,
            Direction::LEFT => self.position.x = self.position.x - 1,
            Direction::RIGHT => self.position.x = self.position.x + 1,
        }

        match self.board[self.position.y][self.position.x] {
            ObjectType::SPACE => {
                self.board[self.position.y][self.position.x] = ObjectType::SNAKE_HEAD
            }
            ObjectType::FOOD => {
                self.board[self.position.y][self.position.x] = ObjectType::SNAKE_HEAD
            }
            _ => self.doTick = false,
        }
    }

    fn render_board(self: &Game) {
        self.canvas.clear_rect(
            0.0,
            0.0,
            (self.board[0].iter().count() * 5) as f64,
            (self.board.iter().count() * 5) as f64,
        );

        for row in 0..self.board.iter().count() {
            for cell in 0..self.board[row].iter().count() {
                match self.board[row][cell] {
                    ObjectType::WALL => {
                        self.canvas
                            .set_fill_style(&wasm_bindgen::JsValue::from_str("#000000"));
                        self.canvas
                            .fill_rect((cell * 5) as f64, (row * 5) as f64, 5.0, 5.0)
                    }
                    ObjectType::SNAKE_HEAD => {
                        self.canvas
                            .set_fill_style(&wasm_bindgen::JsValue::from_str("#FF0000"));
                        self.canvas
                            .fill_rect((cell * 5) as f64, (row * 5) as f64, 5.0, 5.0)
                    }
                    _ => {}
                }
            }
        }
    }
}
