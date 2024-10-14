extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::thread::current;
use std::time::Duration;

use rand::prelude::*;

#[derive(Clone)]
#[derive(PartialEq)]
enum Player {
    Tick,
    Tack
}

impl Player {
    fn other(&self) -> Player {
        match self {
            Self::Tick => Self::Tack,
            Self::Tack => Self::Tick,
        }
    }
    fn same(&self) -> Player {
        return self.clone()
    }
}

#[derive(Clone)]
#[derive(PartialEq)]
enum Field {
    Filled(Player),
    Empty
}

enum State {
    Won(Player),
    Tie,
    Running,
}

struct Board {
    n0: usize,
    n1: usize,
    board: Vec<Vec<Field>>,
}

impl Board {
    fn new(n0: usize, n1: usize) -> Self {
        Board {n0, n1, board: vec![vec![Field::Empty; n1]; n0]}
    }
    fn set(&mut self, x: usize, y: usize, player: &Player) -> Player {
        if x < self.n0 && y < self.n1 {
            match self.board[x][y] {
                Field::Empty => self.board[x][y] = Field::Filled(player.clone()),
                Field::Filled(_) => return player.same()
            }
            return player.other();
        }
        return player.same();
    }
    fn is_diag_won(&self, x: usize, y: usize) -> bool {
        if self.board[x][y] == Field::Empty {return false}
        if x + 4 >= self.n0 || y + 4 >= self.n1 {return false}
        for i in 0usize..4 {
            if self.board[x][y] != self.board[x+i][y+i] {return false}
        }
        return true;
    }
    fn is_diag2_won(&self, x: usize, y: usize) -> bool {
        if self.board[x][y] == Field::Empty {return false}
        if x + 4 >= self.n0 || y < 4 {return false}
        for i in 0usize..4 {
            if self.board[x][y] != self.board[x+i][y-i] {return false}
        }
        return true;
    }
    // fn state(&self) -> State {
    //     for x in 0usize..board.n0 {
    //         for y in 0usize..board.n1 {
    //             if self.is_diag2_won(x, y) || self.is_diag_won(x, y) {
    //                 match 
    //             }
    //         }
    //     }
    // }

}

struct BoardArtist {
    tile_width: u32,
    tile_height: u32,
    board_offset_width: u32,
    board_offset_height: u32,
    n0: u32,
    n1: u32,
    border_size: u32,
}

impl BoardArtist {
    fn draw(&self, x: u32, y: u32, canvas: &mut Canvas<Window>, c: Color) -> () {
        let old_c = canvas.draw_color();
        canvas.set_draw_color(c);
        canvas.fill_rect(Rect::new(
            (x*self.tile_width + self.board_offset_width).try_into().unwrap(),
            (y*self.tile_height + self.board_offset_height).try_into().unwrap(),
            self.tile_width,
            self.tile_height
        )).unwrap();
        canvas.set_draw_color(old_c);
    }
    fn draw_border(&self, canvas: &mut Canvas<Window>) {
        let old_c = canvas.draw_color();
        canvas.set_draw_color(Color::RGB(123, 100, 23));
        canvas.fill_rect(Rect::new(
            (self.board_offset_width - self.border_size).try_into().unwrap(),
            (self.board_offset_height - self.border_size).try_into().unwrap(),
            self.tile_width * self.n0 + self.border_size * 2,
            self.tile_height * self.n1 + self.border_size * 2
        )).unwrap();
        canvas.set_draw_color(old_c);

    }
    fn get_coords(&self, x: i32, y: i32) -> (u32, u32) {
        let ux = u32::try_from(x).unwrap();
        let uy = u32::try_from(y).unwrap();
        ((ux / self.tile_width).try_into().unwrap(), (uy / self.tile_height).try_into().unwrap())
    }
    fn draw_tick(&self, x: u32, y: u32, canvas: &mut Canvas<Window>) -> () {
        self.draw(x, y, canvas, Color::RGB(255, 0, 0));
    }
    fn draw_tack(&self, x: u32, y: u32, canvas: &mut Canvas<Window>) -> () {
        self.draw(x, y, canvas, Color::RGB(0, 255, 0));
    }
    fn draw_board(&self, board: &Board, canvas: &mut Canvas<Window>) -> () {
        self.draw_border(canvas);
        for i in 0usize..board.n0 {
            for j in 0usize..board.n1 {
                match board.board[i][j] {
                    Field::Filled(Player::Tick) => {self.draw_tick(i as u32, j as u32, canvas);}
                    Field::Filled(Player::Tack) => {self.draw_tack(i as u32, j as u32, canvas);}
                    _ => {}
                }
            }
        }
    }
}

type CallbackT = dyn Fn(usize, usize) -> ();
struct CallbackInfo {
    callback: Box<CallbackT>,
    rect: Rect,
}

struct ClickDispatcher {
    vec: Vec<CallbackInfo>,
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = thread_rng();
    let mut i = 0;

    // Set the initial background color before drawing
    // canvas.set_draw_color(Color::RGB(0, 128, 128));
    // canvas.clear();
    // canvas.present();
    let mut board: Board = Board::new(10, 10);
    let mut current_player = Player::Tick;
    let board_artist = BoardArtist {
        tile_width: 30, tile_height: 30, board_offset_width: 60,
         board_offset_height: 60, border_size: 10,
        n0: 10, n1: 10
    };

    'running: loop {

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // match turn {
        //     State::Turn(player) => {

        //     }
        // }

        // Handle events like quitting the application
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                | Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    let (x, y) = board_artist.get_coords(x, y);
                    current_player = board.set(x as usize, y as usize, &current_player);
                }
                _ => {}
            }
        }
        board_artist.draw_board(&board, &mut canvas);


        // Present the canvas with the updated frame
        canvas.present();

        // Delay to keep the loop running at ~60 FPS
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}