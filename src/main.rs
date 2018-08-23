extern crate pancurses;
extern crate noise;

use pancurses::{initscr, endwin, Input, noecho, resize_term, Window};
use noise::Perlin;
use noise::NoiseFn;

fn draw(w: &Window) {
    let (starty, startx) = w.get_beg_yx();
    let (maxy, maxx) = w.get_max_yx();

    for i in starty..maxy {
        //w.mvprintw(i, startx, "#");
    }

    let (midy, midx) = (( maxy-starty )/2, ( maxx-startx )/2);

    let width = maxx - startx;
    let height = maxy - starty;

    //w.mvprintw(midy, midx, format!("{:?}", (midy, midx, width, height )));

    let perlin = Perlin::new();

    for x in 0..width {
        for y in 0..height {
            let n = perlin.get([( x as f64 )/10.0, ( y as f64 )/10.0]);
            let n = ( n * 5.0 + 5.0 ) as i64;

            w.mvprintw(starty + y, startx + x, format!("{:}", cell_map(n)));
        }
    }

}

fn cell_map(n: i64) -> &'static str {
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
        _ => panic!(""),
    }
}

fn main() {
    let window = initscr();
    window.printw("Type things, press delete to quit\n");
    window.keypad(true);
    noecho();
    loop {
        draw(&window);
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
