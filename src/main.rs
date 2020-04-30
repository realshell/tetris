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

type Shape = Vec<Point>;
type Board = Vec<Point>;

#[derive(PartialEq)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn is_intersect(current_shape: &Shape, board: &Board) -> bool {
    for i in 0..current_shape.len() {
        for j in 0..board.len() {
            if current_shape[i].x == board[j].x && current_shape[i].y == board[j].y {
                return true;
            }
        }
    }
    false
}

fn rotate90_point(point: Point, center: Point) -> Point {
    Point {
        x: (point.y - center.y) + center.x,
        y: -(point.x - center.x) + center.y,
    }
}

fn next_move(dir: Dir, shape: &Shape) -> Option<Shape> {
    match dir {
        Dir::UP => {
            let first_element = shape[0];
            let mut x_min = first_element.x;
            let mut x_max = first_element.x;
            let mut y_min = first_element.y;
            let mut y_max = first_element.y;
            for i in 0..shape.len() {
                if shape[i].x < x_min {
                    x_min = shape[i].x;
                }
                if shape[i].x > x_max {
                    x_max = shape[i].x;
                }
                if shape[i].y < y_min {
                    y_min = shape[i].y;
                }
                if shape[i].y > y_max {
                    y_max = shape[i].y;
                }
            }
            let rotate_center_point = Point {
                x: (x_max + x_min) / 2,
                y: (y_max + y_min) / 2,
            };
            let mut new_shape = Vec::new();
            for i in 0..shape.len() {
                new_shape.push(rotate90_point(shape[i], rotate_center_point));
            }
            let left_limit = 1;
            let right_limt = 10;
            let down_limit = 17;
            for i in 0..new_shape.len() {
                if new_shape[i].x < left_limit
                    || new_shape[i].x > right_limt
                    || new_shape[i].y > down_limit
                {
                    return None;
                }
            }
            return Some(new_shape);
        }
        Dir::DOWN => {
            let mut new_shape = shape.clone();
            let down_limt = 17;
            for i in 0..new_shape.len() {
                if new_shape[i].y + 1 == down_limt {
                    return None;
                }
                new_shape[i].y += 1;
            }
            return Some(new_shape);
        }
        Dir::LEFT => {
            let mut new_shape = shape.clone();
            let left_limit = 0;
            for i in 0..new_shape.len() {
                if new_shape[i].x - 1 == left_limit {
                    return None;
                }
                new_shape[i].x -= 1;
            }
            return Some(new_shape);
        }
        Dir::RIGHT => {
            let mut new_shape = shape.clone();
            let right_limit = 11;
            for i in 0..new_shape.len() {
                if new_shape[i].x + 1 == right_limit {
                    return None;
                }
                new_shape[i].x += 1;
            }
            return Some(new_shape);
        }
    }
}

fn mv_add_str(y: i32, x: i32, s: String) {
    for i in 0..s.len() {
        mvaddch(y, x + (i as i32), s.as_bytes()[i] as u32);
    }
}

fn find_line(board: &Board, n: usize) -> bool {
    let mut count = 0;
    for i in 0..board.len() {
        if board[i].y == n as i32 {
            count += 1;
        }
    }
    return count == 10;
}







fn show_count_and_next(count: &usize, next_shape: &Shape) {
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

fn show_shape(shape: &Shape) {
    for e in shape {
        mvaddch(e.y as i32, e.x as i32, 'O' as u32);
    }
}

fn show_board(board: &Board) {
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

fn check_line(board: &mut Board, count: &mut usize) {
    if board.len() < 10 {
        return;
    }
    let mut del_lines: Vec<usize> = Vec::new();
    let first_element = board[0];
    let mut y_min = first_element.y as usize;
    let mut y_max = first_element.y as usize;
    for i in 0..board.len() {
        if board[i].y < y_min as i32 {
            y_min = board[i].y as usize;
        }
        if board[i].y > y_max as i32 {
            y_max = board[i].y as usize;
        }
    }
    for i in y_min..y_max + 1 {
        if find_line(&board, i) {
            del_lines.push(i);
        }
    }
    if del_lines.len() == 0 {
        return; 
    }
    for i in 0..del_lines.len() {
        let mut new_board = Vec::new();
        for j in 0..board.len() {
            if board[j].y != del_lines[i] as i32 {
                new_board.push(board[j]);
            }
        }
        for k in 0..new_board.len() {
            if new_board[k].y < del_lines[i] as i32 {
                new_board[k].y += 1;
            }
        }
        *board = new_board;
    }
    *count += del_lines.len();
    clear();
    return;
}

fn del_next_shape(next_shape: &Shape) {
    let start_x = 12;
    let start_y = 0;
    for e in next_shape {
        mvaddch(start_y + 4 + e.y as i32, start_x + e.x as i32, ' ' as u32);
    }
}

fn del_shape(shape: &Shape) {
    for e in shape {
        mvaddch(e.y as i32, e.x as i32, ' ' as u32);
    }
}

fn board_add(current_shape: &Shape, board: &mut Vec<Point>) {
    for i in 0..current_shape.len() {
        if !board.contains(&current_shape[i]) {
            board.push(current_shape[i])
        }
    }
}

fn move_shape(
    dir: Dir,
    current_shape: &mut Shape,
    next_shape: &mut Shape,
    board: &mut Board,
) -> bool {
    match dir {
        Dir::UP => match next_move(Dir::UP, current_shape) {
            Some(new_shape) => {
                if is_intersect(&new_shape, board) {
                    return move_shape(Dir::DOWN, current_shape, next_shape, board);
                } else {
                    del_shape(current_shape);
                    *current_shape = new_shape;
                    return true;
                }
            }
            None => {
                return move_shape(Dir::DOWN, current_shape, next_shape, board);
            }
        },
        Dir::DOWN => match next_move(Dir::DOWN, current_shape) {
            Some(new_shape) => {
                if is_intersect(&new_shape, board) {
                    del_shape(current_shape);
                    del_next_shape(&next_shape);
                    board_add(current_shape, board);
                    *current_shape = next_shape.clone();
                    *next_shape = random_shape(board);
                    return false;
                } else {
                    del_shape(current_shape);
                    *current_shape = new_shape;
                    return true;
                }
            }
            None => {
                del_shape(current_shape);
                del_next_shape(&next_shape);
                board_add(current_shape, board);
                *current_shape = next_shape.clone();
                *next_shape = random_shape(board);
                return false;
            }
        },
        Dir::LEFT => match next_move(Dir::LEFT, current_shape) {
            Some(new_shape) => {
                if is_intersect(&new_shape, &board) {
                    return move_shape(Dir::DOWN, current_shape, next_shape, board);
                } else {
                    del_shape(&current_shape);
                    *current_shape = new_shape;
                    return true;
                }
            }
            None => {
                return move_shape(Dir::DOWN, current_shape, next_shape, board);
            }
        },
        Dir::RIGHT => match next_move(Dir::RIGHT, current_shape) {
            Some(new_shape) => {
                if is_intersect(&new_shape, &board) {
                    return move_shape(Dir::DOWN, current_shape, next_shape, board);
                } else {
                    del_shape(&current_shape);
                    *current_shape = new_shape;
                    return true;
                }
            }
            None => {
                return move_shape(Dir::DOWN, current_shape, next_shape, board);
            }
        },
    }
}

fn hit_key(current_shape: &mut Shape, next_shape: &mut Shape, board: &mut Board) {
    match getch() {
        KEY_UP => {
            move_shape(Dir::UP, current_shape, next_shape, board);
            move_shape(Dir::DOWN, current_shape, next_shape, board);
            return;
        }
        KEY_DOWN => {
            while move_shape(Dir::DOWN, current_shape, next_shape, board) {}
            return;
        }
        KEY_LEFT => {
            move_shape(Dir::LEFT, current_shape, next_shape, board);
            return;
        }
        KEY_RIGHT => {
            move_shape(Dir::RIGHT, current_shape, next_shape, board);
            return;
        }
        _ => {
            move_shape(Dir::DOWN, current_shape, next_shape, board);
        }
    }
}

fn random_shape(board: &mut Board) -> Shape {
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

    for i in 0..board.len() {
        for j in 0..shape.len() {
            if board[i] == shape[j] {
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
    let mut board: Board = Vec::new();
    let mut current_shape = random_shape(&mut board);
    let mut next_shape = random_shape(&mut board);
    loop {
        if COLS() < 20 || LINES() < 20 {
            endwin();
            println!("The screen does not have enough space.");
            return;
        }
        hit_key(&mut current_shape, &mut next_shape, &mut board);
        check_line(&mut board, &mut count);
        show_board(&board);
        show_shape(&current_shape);
        show_count_and_next(&count, &next_shape);
        refresh();
        while getch() != -1 {}
        unsafe {
            usleep(500000);
        }
    }
}
