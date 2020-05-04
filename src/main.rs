use libc::usleep;
use ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE;
use ncurses::*;
use rand::Rng;
use std::process;

#[derive(PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn is_intersect(current_shape: &[Point], board: &[Point]) -> bool {
    for i in current_shape {
        for j in board {
            if i.x == j.x && i.y == j.y {
                return true;
            }
        }
    }
    false
}

fn rotate_point_90degree_counter_clockwise(point: Point, center: Point) -> Point {
    Point {
        x: (point.y - center.y) + center.x,
        y: -(point.x - center.x) + center.y,
    }
}

fn next_move(dir: Dir, shape: &[Point]) -> Option<Vec<Point>> {
    match dir {
        Dir::UP => {
            let first_element = shape[0];
            let mut x_min = first_element.x;
            let mut x_max = first_element.x;
            let mut y_min = first_element.y;
            let mut y_max = first_element.y;
            for e in shape {
                if e.x < x_min {
                    x_min = e.x;
                }
                if e.x > x_max {
                    x_max = e.x;
                }
                if e.y < y_min {
                    y_min = e.y;
                }
                if e.y > y_max {
                    y_max = e.y;
                }
            }
            let rotate_center_point = Point {
                x: (x_max + x_min) / 2,
                y: (y_max + y_min) / 2,
            };
            let mut new_shape = Vec::new();
            for e in shape {
                new_shape.push(rotate_point_90degree_counter_clockwise(
                    *e,
                    rotate_center_point,
                ));
            }
            let left_limit = 1;
            let right_limt = 10;
            let down_limit = 17;
            for e in &new_shape {
                if e.x < left_limit || e.x > right_limt || e.y > down_limit {
                    return None;
                }
            }
            Some(new_shape)
        }
        Dir::DOWN => {
            let mut new_shape = shape.to_owned();
            let down_limt = 17;
            for mut e in &mut new_shape {
                if e.y + 1 == down_limt {
                    return None;
                }
                e.y += 1;
            }
            Some(new_shape)
        }
        Dir::LEFT => {
            let mut new_shape = shape.to_owned();
            let left_limit = 0;
            for mut e in &mut new_shape {
                if e.x - 1 == left_limit {
                    return None;
                }
                e.x -= 1;
            }
            Some(new_shape)
        }
        Dir::RIGHT => {
            let mut new_shape = shape.to_owned();
            let right_limit = 11;
            for e in &mut new_shape {
                if e.x + 1 == right_limit {
                    return None;
                }
                e.x += 1;
            }
            Some(new_shape)
        }
    }
}

fn mv_add_str(y: i32, x: i32, s: String) {
    for i in 0..s.len() {
        mvaddch(y, x + (i as i32), s.as_bytes()[i] as u32);
    }
}

fn find_line(board: &[Point], n: usize) -> bool {
    let mut count = 0;
    for e in board {
        if e.y == n as i32 {
            count += 1;
        }
    }
    count == 10
}

fn check(board: &mut Vec<Point>, count: &mut usize) {
    for n in 1..17 {
        let mut new_board: Vec<Point> = Vec::new();
        if find_line(&board, n) {
            for mut e in &mut *board {
                if e.y != n as i32 {
                    if e.y < n as i32 {
                        e.y += 1;
                        new_board.push(*e);
                    } else {
                        new_board.push(*e);
                    }
                }
            }
            *count += 1;
            *board = new_board;
        }
    }
    clear();
}

fn show_count_and_next(count: usize, next_shape: &[Point]) {
    let start_x = 12;
    let start_y = 0;
    let width = 10;
    let height = 8;
    for i in start_x + 1..start_x + width + 1 {
        mvaddch(start_y, i, ACS_HLINE());
        mvaddch(start_y + height + 1, i, ACS_HLINE());
    }
    for i in start_y + 1..start_y + height + 1 {
        mvaddch(i, start_x, ACS_VLINE());
        mvaddch(i, start_x + width + 1, ACS_VLINE());
    }
    mvaddch(start_y, start_x, ACS_ULCORNER());
    mvaddch(start_y, start_x + width + 1, ACS_URCORNER());
    mvaddch(start_y + height + 1, start_x, ACS_LLCORNER());
    mvaddch(start_y + height + 1, start_x + width + 1, ACS_LRCORNER());
    mv_add_str(start_y + 1, start_x + 1, String::from("Count:"));
    mv_add_str(
        start_y + 2,
        start_x + 1,
        format!("{:>10}", count.to_string()),
    );
    mv_add_str(start_y + 3, start_x + 1, String::from("Next:"));
    for e in next_shape {
        mvaddch(start_y + 4 + e.y as i32, start_x + e.x as i32, 'O' as u32);
    }
}

fn show_shape(shape: &[Point]) {
    for e in shape {
        mvaddch(e.y as i32, e.x as i32, 'O' as u32);
    }
}

fn show_board(board: &[Point]) {
    for e in board {
        mvaddch(e.y as i32, e.x as i32, 'O' as u32);
    }
    let start_x = 0;
    let start_y = 0;
    let width = 10;
    let height = 16;
    for i in start_x + 1..start_x + width + 1 {
        mvaddch(start_y, i, ACS_HLINE());
        mvaddch(start_y + height + 1, i, ACS_HLINE());
    }
    for i in start_y + 1..start_y + height + 1 {
        mvaddch(i, start_x, ACS_VLINE());
        mvaddch(i, start_x + width + 1, ACS_VLINE());
    }
    mvaddch(start_y, start_x, ACS_ULCORNER());
    mvaddch(start_y, start_x + width + 1, ACS_URCORNER());
    mvaddch(start_y + height + 1, start_x, ACS_LLCORNER());
    mvaddch(start_y + height + 1, start_x + width + 1, ACS_LRCORNER());
}

fn del_next_shape(next_shape: &[Point]) {
    let start_x = 12;
    let start_y = 0;
    for e in next_shape {
        mvaddch(start_y + 4 + e.y as i32, start_x + e.x as i32, ' ' as u32);
    }
}

fn del_shape(shape: &[Point]) {
    for e in shape {
        mvaddch(e.y as i32, e.x as i32, ' ' as u32);
    }
}

fn board_add(current_shape: &[Point], board: &mut Vec<Point>) {
    for e in current_shape {
        if !board.contains(e) {
            board.push(*e)
        }
    }
}

fn move_shape(
    dir: Dir,
    current_shape: &mut Vec<Point>,
    next_shape: &mut Vec<Point>,
    board: &mut Vec<Point>,
) -> bool {
    match dir {
        Dir::UP => match next_move(Dir::UP, current_shape) {
            Some(new_shape) => {
                if is_intersect(&new_shape, board) {
                    move_shape(Dir::DOWN, current_shape, next_shape, board)
                } else {
                    del_shape(current_shape);
                    *current_shape = new_shape;
                    true
                }
            }
            None => move_shape(Dir::DOWN, current_shape, next_shape, board),
        },
        Dir::DOWN => match next_move(Dir::DOWN, current_shape) {
            Some(new_shape) => {
                if is_intersect(&new_shape, board) {
                    del_shape(current_shape);
                    del_next_shape(&next_shape);
                    board_add(current_shape, board);
                    *current_shape = next_shape.clone();
                    *next_shape = random_shape(board);
                    false
                } else {
                    del_shape(current_shape);
                    *current_shape = new_shape;
                    true
                }
            }
            None => {
                del_shape(current_shape);
                del_next_shape(&next_shape);
                board_add(current_shape, board);
                *current_shape = next_shape.clone();
                *next_shape = random_shape(board);
                false
            }
        },
        Dir::LEFT => match next_move(Dir::LEFT, current_shape) {
            Some(new_shape) => {
                if is_intersect(&new_shape, &board) {
                    move_shape(Dir::DOWN, current_shape, next_shape, board)
                } else {
                    del_shape(&current_shape);
                    *current_shape = new_shape;
                    true
                }
            }
            None => move_shape(Dir::DOWN, current_shape, next_shape, board),
        },
        Dir::RIGHT => match next_move(Dir::RIGHT, current_shape) {
            Some(new_shape) => {
                if is_intersect(&new_shape, &board) {
                    move_shape(Dir::DOWN, current_shape, next_shape, board)
                } else {
                    del_shape(&current_shape);
                    *current_shape = new_shape;
                    true
                }
            }
            None => move_shape(Dir::DOWN, current_shape, next_shape, board),
        },
    }
}

fn hit_key(current_shape: &mut Vec<Point>, next_shape: &mut Vec<Point>, board: &mut Vec<Point>) {
    match getch() {
        KEY_UP => {
            move_shape(Dir::UP, current_shape, next_shape, board);
            move_shape(Dir::DOWN, current_shape, next_shape, board);
        }
        KEY_DOWN => while move_shape(Dir::DOWN, current_shape, next_shape, board) {},
        KEY_LEFT => {
            move_shape(Dir::LEFT, current_shape, next_shape, board);
        }
        KEY_RIGHT => {
            move_shape(Dir::RIGHT, current_shape, next_shape, board);
        }
        _ => {
            move_shape(Dir::DOWN, current_shape, next_shape, board);
        }
    }
}

fn random_shape(board: &mut Vec<Point>) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let all_shape: [[Point; 4]; 7] = [
        [
            Point { x: 6, y: 1 },
            Point { x: 6, y: 2 },
            Point { x: 6, y: 3 },
            Point { x: 6, y: 4 },
        ],
        [
            Point { x: 5, y: 1 },
            Point { x: 6, y: 1 },
            Point { x: 5, y: 2 },
            Point { x: 6, y: 2 },
        ],
        [
            Point { x: 6, y: 1 },
            Point { x: 5, y: 2 },
            Point { x: 6, y: 2 },
            Point { x: 7, y: 2 },
        ],
        [
            Point { x: 7, y: 1 },
            Point { x: 5, y: 2 },
            Point { x: 6, y: 2 },
            Point { x: 7, y: 2 },
        ],
        [
            Point { x: 5, y: 1 },
            Point { x: 5, y: 2 },
            Point { x: 6, y: 2 },
            Point { x: 7, y: 2 },
        ],
        [
            Point { x: 5, y: 1 },
            Point { x: 6, y: 1 },
            Point { x: 6, y: 2 },
            Point { x: 7, y: 2 },
        ],
        [
            Point { x: 6, y: 1 },
            Point { x: 7, y: 1 },
            Point { x: 5, y: 2 },
            Point { x: 6, y: 2 },
        ],
    ];
    let shape = all_shape[rng.gen_range(0, 7)].to_vec();

    for i in board {
        for j in &shape {
            if i == j {
                endwin();
                println!("The screen does not have enough space \n Game Over.");
                process::exit(1);
            }
        }
    }
    shape
}

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
    let mut board: Vec<Point> = Vec::new();
    let mut current_shape = random_shape(&mut board);
    let mut next_shape = random_shape(&mut board);
    loop {
        if COLS() < 20 || LINES() < 20 {
            endwin();
            println!("The screen does not have enough space.");
            return;
        }
        hit_key(&mut current_shape, &mut next_shape, &mut board);
        check(&mut board, &mut count);
        show_board(&board);
        show_shape(&current_shape);
        show_count_and_next(count, &next_shape);
        refresh();
        while getch() != -1 {}
        unsafe {
            usleep(500_000);
        }
    }
}
