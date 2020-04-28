use libc::usleep;
use ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE;
use ncurses::*;
use rand::Rng;

// fn swap_diagonal(shape: &[[i32; 4]; 4]) -> [[i32; 4]; 4]{
//     let mut result: [[i32; 3]; 3] = [[0; 3]; 3];
//     for i in 0..4 {
//         for j in 0..4 {
//             result[i][j] = shape[j][i];
//         }
//     }
//     return result;
// }

// fn swap_left_right(shape: &[[i32; 3]; 3]) -> [[i32; 3]; 3]{
//     let mut result: [[i32; 3]; 3] = [[0; 3]; 3];
//     for i in 0..3 {
//         for j in 0..3 {
//             result[i][j] = shape[i][2-j];
//         }
//     }
//     return result;
// }

// fn rotate_shape(shape: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
//     return swap_diagonal(&mut swap_left_right(shape));
// }

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x: x, y: y }
    }
}

fn show_line(e: &[[i32; 3]; 3], n: i32) -> String {
    let mut result = String::new();
    for i in &e[n as usize] {
        if *i == 0 {
            result.push_str(" ");
        } else {
            result.push_str("0");
        }
    }
    return result;
}

fn mv_add_str(y: i32, x: i32, s: String) {
    for i in 0..s.len() {
        mvaddch(y, x + (i as i32), s.as_bytes()[i] as u32);
    }
}

fn show_count_and_next(count: &usize, next: &[Point; 4]) {
    let start_x = 12;
    let start_y = 0;
    let width = 5;
    let height = 7;
    for i in start_x + 1..start_x + width + 1 {
        mvaddch(start_y, i, ACS_HLINE());
        mvaddch(start_y + height + 1, i, ACS_HLINE());
    }
    for i in start_y + 1..start_y + height + 1 {
        mvaddch(i, start_x, ACS_VLINE());
        mvaddch(i, start_x + width + 1, ACS_VLINE());
    }
    mvaddch(start_y, 12, ACS_ULCORNER());
    mvaddch(start_y, 18, ACS_URCORNER());
    mvaddch(start_y + height + 1, start_x, ACS_LLCORNER());
    mvaddch(start_y + height + 1, start_x + width + 1, ACS_LRCORNER());
    mv_add_str(start_y + 1, start_x + 1, String::from("Count"));
    mv_add_str(
        start_y + 2,
        start_x + 1,
        format!("{:>5}", count.to_string()),
    );
    mv_add_str(start_y + 3, start_x + 1, String::from("Next"));
    for e in next {
        mvaddch(
            start_y + 4 + e.y as i32,
            start_x + 2 + e.x as i32,
            'O' as u32,
        );
    }
}

fn show_board(board: &Vec<Point>) {
    let start_x = 0;
    let start_y = 0;
    let width = 10;
    let height = 16;
    for e in board {
        if e.y > 4 {
            mvaddch(e.y as i32 - 4, e.x as i32, 'O' as u32);
        }
    }
    for i in start_x..start_x + width + 1 {
        mvaddch(start_y, i, ACS_HLINE());
        mvaddch(start_y + height + 1, i, ACS_HLINE());
    }
    for i in start_y + 1..start_y + height + 1 {
        mvaddch(i, start_x, ACS_VLINE());
        mvaddch(i, start_x+width+1, ACS_VLINE());
    }
    mvaddch(start_y, start_x, ACS_ULCORNER());
    mvaddch(start_y, start_x+width+1, ACS_URCORNER());
    mvaddch(start_y+height+1, start_x, ACS_LLCORNER());
    mvaddch(start_y+height+1, start_x+width+1, ACS_LRCORNER());
}


fn random_shape() ->  &'static [Point; 4] {
    let mut rng = rand::thread_rng();
    let all_shape = vec![
    &[Point {x: 0, y: 0}, Point {x: 0, y: 1}, Point {x: 0, y: 2}, Point {x: 0,y :3}], 
    &[Point {x: 0, y: 2}, Point {x: 1, y: 2}, Point {x: 0, y: 3}, Point {x: 1, y: 3}],
    &[Point {x: 1, y: 2}, Point {x: 0, y: 3}, Point {x: 1, y: 3}, Point {x: 2, y: 3}],
    &[Point {x: 2, y: 2}, Point {x: 0, y: 3}, Point {x: 1, y: 3}, Point {x: 2, y: 3}],
    &[Point {x: 0, y: 2}, Point {x: 0, y: 3}, Point {x: 1, y: 3}, Point {x: 2, y: 3}],
    &[Point {x: 0, y: 2}, Point {x: 1, y: 2}, Point {x: 1, y: 3}, Point {x: 2, y: 3}],
    &[Point {x: 1, y: 2}, Point {x: 2, y: 2}, Point {x: 0, y: 3}, Point {x: 1, y: 3}]
    ];
    all_shape[rng.gen_range(0,7)]
}

fn move_shape(shape: &mut Vec<Point>, board: &mut Vec<Point>) {
    
}

fn hit_key() {}

fn main() {
    initscr();
    noecho();
    keypad(stdscr(), true);
    curs_set(CURSOR_INVISIBLE);
    nodelay(stdscr(), true);
    leaveok(stdscr(), true);
    scrollok(stdscr(), false);
    cbreak();
    let mut count = 0;
    let mut next_shape = random_shape();
    let mut current_shape = random_shape();
    let mut board: Vec<Point> = vec![Point::new(5, 5)];
    loop {
        if COLS() < 13 || LINES() < 20 {
            endwin();
            println!("The screen does not have enough space");
            return;
        }

        hit_key();
        move_shape(&mut current_shape, &mut board);
        show_board(&board);
        show_count_and_next(&count, &next_shape);
        // swap_diagonal(&mut next);
        refresh();
        unsafe {
            usleep(40000);
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // #[test]
    // fn _rotate_shape() {
    //     let a: [[i32; 3]; 3] = [[0,0,1], [0, 1, 1], [0, 1, 0]];
    //     assert_eq!(rotate_shape(&a), [[1,1,0], [0,1,1], [0,0,0]]);
    // }
    // #[test]
    // fn _swap_diagonal() {
    //     let a: [[i32; 3]; 3] = [[0,0,1], [0, 1, 1], [0, 1, 0]];
    //     assert_eq!(swap_diagonal(&a), [[0,0,0],[0,1,1],[1,1,0]]);
    // }
    // #[test]
    // fn _swap_left_right() {
    //     let a: [[i32; 3]; 3] = [[0,0,1], [0, 1, 1], [0, 1, 0]];
    //     assert_eq!(swap_left_right(&a),  [[1,0,0],[1,1,0],[0,1,0]]);
    // }
    // #[test]
    // fn _test_666() {
    //     assert_eq!(3 % 2, 1);
    // }
}
