extern crate pancurses;
extern crate noise;

use pancurses::{
    initscr,
    start_color,
    endwin,
    Input,
    noecho,
    resize_term,
    Window,
    init_pair,
    COLOR_PAIR,
};
use noise::Perlin;
use noise::NoiseFn;
use std::env;

use std::time::Instant;

fn draw(w: &Window, writer: &mut CellWriter, state: i32) {
    let (starty, startx) = w.get_beg_yx();
    let (maxy, maxx) = w.get_max_yx();

    //let (midy, midx) = (( maxy-starty )/2, ( maxx-startx )/2);

    let width = maxx - startx;
    let height = maxy - starty;

    let noise = Perlin::new();

    for x in 0..width {
        for y in 0..height {
            //let fac = ( (state as f64) / 10.0 ).sin() * 0.5 + 0.5;
            //let fac = fac * 10.0 + 10.0;
            let fac = 15.0;

            let n = noise.get([( ( x + state ) as f64 )/fac, ( y as f64 )/fac]);
            let n = ( n * 5.0 + 5.0 ) as i64;

            w.mv(starty + y, startx + x);
            writer.write_cell(w, n);
        }
    }

}

trait CellWriter {
    fn write_cell(&mut self, w: &Window, n: i64);
}

struct TerrainWriter;

impl TerrainWriter {
    pub fn color_map(&self, n: i64) -> u64 {
        match n {
            0 => 30, // water
            1 => 30,
            2 => 30,
            3 => 30,
            4 => 31, // beach
            5 => 32, // grass
            6 => 32,
            7 => 33, // stone
            8 => 33,
            9 => 34, // snow
            10 => 34,
            _ => 30,
        }
    }
}

impl CellWriter for TerrainWriter {
    fn write_cell(&mut self, w: &Window, n: i64) {
        w.attron(COLOR_PAIR(self.color_map(n)));
        w.printw(" ");
    }
}

struct CloudWriter;

impl CloudWriter {
    fn color_map(&self, n: i64) -> u64 {
        ( n + 10 ) as u64
    }

    fn cell_map(&self, n: i64) -> &'static str {
        match n {
            0 => " ",
            1 => " ",
            2 => ".",
            3 => ",",
            4 => "-",
            5 => "*",
            6 => "=",
            7 => "&",
            8 => "$",
            9 => "#",
            10 => "#",
            _ => " ",
        }
    }
}

impl CellWriter for CloudWriter {
    fn write_cell(&mut self, w: &Window, n: i64) {
        w.attron(COLOR_PAIR(self.color_map(n)));
        w.printw(self.cell_map(n));
    }
}



fn initialize_colors() {
    // Greyscale characters
    init_pair(10, 0, 0);
    init_pair(11, 235, 0);
    init_pair(12, 237, 0);
    init_pair(13, 239, 0);
    init_pair(14, 241, 0);
    init_pair(15, 243, 0);
    init_pair(16, 245, 0);
    init_pair(17, 247, 0);
    init_pair(18, 249, 0);
    init_pair(19, 251, 0);
    init_pair(20, 253, 0);

    // Terrain backgrounds
    init_pair(30, 7, 38); // water
    init_pair(31, 7, 228); // beach
    init_pair(32, 7, 70); // grass
    init_pair(33, 7, 247); // stone
    init_pair(34, 7, 7); // snow
}

fn parse_args() -> Box<CellWriter> {
    let args: Vec<String> = env::args().collect();
    let error = || {
        panic!("Wrong args: clouds [clouds|terrain]?")
    };

    match args.len() {
        1 => Box::new(CloudWriter {}),
        2 => {
            match args[1].as_ref() {
                "clouds" => Box::new(CloudWriter{}),
                "terrain" => Box::new(TerrainWriter{}),
                _ => error()
            }
        },
        _ => error()
    }
}

fn main() {
    let window = initscr();
    let mut writer = parse_args();
    let mut state: i32 = 0;

    window.keypad(true);
    window.nodelay(true);
    start_color();
    initialize_colors();
    noecho();

    let mut last_inc = Instant::now();
    loop {
        if last_inc.elapsed().as_secs() > 0 {
            state += 1;
            last_inc = Instant::now();
        }

        draw(&window, &mut *writer, state);

        match window.getch() {
            Some(Input::KeyResize) => {
                resize_term(0, 0);
                window.clear();
            },
            Some(Input::Character('q')) => { break; },
            Some(Input::KeyDC) => break,
            None => (),
            _ => (),
        }
        window.refresh();
    }
    endwin();
}
