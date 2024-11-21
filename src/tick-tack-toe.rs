extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use std::cell::RefCell;
use std::rc::Rc;

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
    n0: u32,
    n1: u32,
    border_size: u32,
    canvas: Canvas<Surface<'static>>,
}

impl BoardArtist {
    fn new(tile_len: u32, n0: u32, n1: u32, border_size: u32) -> Self {
        return Self {
            tile_width: tile_len,
            tile_height: tile_len,
            n0,
            n1,
            border_size,
            canvas: Surface::new(tile_len*n0 + 2*border_size, tile_len * n1 + 2*border_size, PixelFormatEnum::RGB24).unwrap().into_canvas().unwrap(),
        }
    }

    fn target_rect(&self, x: u32, y: u32) -> Rect {
        Rect::new (
            x as i32, y as i32,
            self.tile_width * self.n0 + self.border_size * 2,
            self.tile_height * self.n1 + self.border_size * 2
        )
    }

    fn draw(&mut self, x: u32, y: u32, c: Color) -> () {
        let old_c = self.canvas.draw_color();
        self.canvas.set_draw_color(c);
        self.canvas.fill_rect(Rect::new(
            (x*self.tile_width + self.border_size).try_into().unwrap(),
            (y*self.tile_height + self.border_size).try_into().unwrap(),
            self.tile_width,
            self.tile_height
        )).unwrap();
        self.canvas.set_draw_color(old_c);
    }
    fn draw_border(&mut self) {
        let old_c = self.canvas.draw_color();
        self.canvas.set_draw_color(Color::RGB(123, 100, 23));
        self.canvas.fill_rect(Rect::new(
            0, 0,
            self.tile_width*self.n0 + 2*self.border_size,
             self.tile_height * self.n1 + 2*self.border_size)).unwrap();
        self.canvas.set_draw_color(old_c);

    }
    fn get_coords(&self, x: i32, y: i32) -> Option<(u32, u32)> {
        println!("{} {}", x, y);
        let ux = (x as u32).checked_sub(self.border_size)?;
        let uy = (y as u32).checked_sub(self.border_size)?;
        let result = ((ux / (self.tile_width)).try_into().unwrap(), (uy / (self.tile_height)).try_into().unwrap());
        println!("{:?}", result);
        return Some(result);
    }
    fn draw_tick(&mut self, x: u32, y: u32) -> () {
        self.draw(x, y, Color::RGB(255, 0, 0));
    }
    fn draw_tack(&mut self, x: u32, y: u32) -> () {
        self.draw(x, y, Color::RGB(0, 255, 0));
    }
    fn draw_board(&mut self, board: &Board) -> &Canvas<Surface<'static>> {
        self.draw_border();
        for i in 0usize..board.n0 {
            for j in 0usize..board.n1 {
                match board.board[i][j] {
                    Field::Filled(Player::Tick) => {self.draw_tick(i as u32, j as u32);}
                    Field::Filled(Player::Tack) => {self.draw_tack(i as u32, j as u32);}
                    _ => {}
                }
            }
        }
        return &self.canvas
    }
}

fn blit(source: &Canvas<Surface<'static>>, target: &mut Canvas<Window>, place: Rect) -> () {
    let texture_creator = target.texture_creator();
    let texture = texture_creator.create_texture_from_surface(source.surface()).unwrap();
    target.copy(&texture, None, place).unwrap();
}

type CallbackT<'a> = dyn FnMut(i32, i32) -> () + 'a;
struct CallbackInfo<'a> {
    callback: Box<CallbackT<'a>>,
    rect: Rect,
}

struct ClickDispatcher<'a> {
    vec: Vec<CallbackInfo<'a>>,
}

impl ClickDispatcher<'_> {
    fn new() -> Self {
        ClickDispatcher {vec: vec![]}
    }

    fn dispatch(&mut self, x: i32, y: i32) {
        for callback_info in self.vec.iter_mut().rev() {
            if callback_info.rect.contains_point((x, y)) {
                (callback_info.callback)(x - callback_info.rect.x, y - callback_info.rect.y);
                break;
            }
        }
    }
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

    let board = Rc::new(RefCell::new(Board::new(10, 10)));
    let mut current_player = Player::Tick;
    let board_artist: Rc<RefCell<BoardArtist>> = Rc::new(RefCell::new(BoardArtist::new (
        30,  10,
        10, 10,
    )));

    let mut click_dispatcher = ClickDispatcher::new();
    click_dispatcher.vec.push(
        CallbackInfo {
            callback: Box::new(|x: i32, y|  {
                board_artist.borrow().get_coords(x, y).and_then(
                    |(x, y)| { current_player = board.borrow_mut().set(x as usize, y as usize, &current_player); Option::<()>::None}
                );
            }), 
            rect: board_artist.borrow().target_rect(100, 100),

    });

    'running: loop {

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                | Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                    click_dispatcher.dispatch(x, y);
                }
                _ => {}
            }
        }
        // board_artist.borrow_mut().draw_board(&board.borrow(), &mut canvas);
        let target_rect = board_artist.borrow().target_rect(100, 100);
        blit(board_artist.borrow_mut().draw_board(&board.borrow()), &mut canvas, target_rect);

        // Present the canvas with the updated frame
        canvas.present();

        // Delay to keep the loop running at ~60 FPS
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}