use crate::board::piece::Piece;
use crate::board::{Board, MoveType};
use crate::consts::{BLACK_CSS, COLS, GTK_NONE, HEIGHT, ROWS, WHITE_CSS, WIDTH};
use glib::clone;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Grid};
use std::cell::RefCell;
use std::rc::Rc;
mod image;
use image::get_image;
pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Chess game")
        .default_width(HEIGHT)
        .default_height(WIDTH)
        .build();
    let board = Rc::new(RefCell::new(Board::default()));
    let grid = build_grid(board);
    window.set_child(Some(&grid));
    window.present();
}
struct UIBoardState {
    pressed_piece: Option<(usize, usize)>,
}
impl UIBoardState {
    pub fn new() -> UIBoardState {
        UIBoardState {
            pressed_piece: None,
        }
    }
}
fn build_grid(board: Rc<RefCell<Board>>) -> Grid {
    let grid = Grid::new();
    let ui_board_state = Rc::new(RefCell::new(UIBoardState::new()));
    grid.set_hexpand(true);
    grid.set_vexpand(true);
    for row in 0..ROWS {
        for col in 0..COLS {
            let cell_button = build_button(row, col, &board, &ui_board_state, &grid);
            grid.attach(&cell_button, col as i32, row as i32, 1, 1);
        }
    }
    grid
}
fn build_button(
    row: usize,
    col: usize,
    board_ref_cell: &Rc<RefCell<Board>>,
    ui_board_state: &Rc<RefCell<UIBoardState>>,
    grid: &Grid,
) -> Button {
    let cell_button = Button::new();
    cell_button.set_hexpand(true);
    cell_button.set_vexpand(true);
    cell_button.set_size_request(0, 0); // Allows dynamic resizing
    if let Some(image) = get_image(board_ref_cell.borrow().get_piece(row, col)) {
        cell_button.set_child(Some(&image));
    }
    cell_button.connect_clicked(clone!(
        #[strong]
        board_ref_cell,
        #[strong]
        ui_board_state,
        #[strong]
        grid,
        move |_| {
            println!("Button clicked: ({}, {})", row, col);
            if ui_board_state.borrow().pressed_piece.is_some() {
                ui_reset_grid_color(&grid);
                let (r, c) = ui_board_state.borrow().pressed_piece.unwrap();
                ui_board_state.borrow_mut().pressed_piece = None;
                let mut board = board_ref_cell.borrow_mut();
                if let Ok(move_type) = board.try_move_piece((r, c), (row, col)) {
                    match move_type {
                        MoveType::Promotion(piece_promoted) => {
                            ui_move_piece(r, c, row, col, &grid);
                            ui_promote_pawn(&grid, row, col, &piece_promoted);
                        }
                        _ => {
                            ui_move_piece(r, c, row, col, &grid);
                        }
                    }
                    if board.is_checkmate() {
                        println!("Checkmate!");
                    }
                } else if board.get_piece(row, col).color() == Some(board.turn_player) {
                    ui_board_state.borrow_mut().pressed_piece = Some((row, col));
                    let legal_moves = board.get_legal_moves((row, col));
                    higlight_legal_moves(&grid, legal_moves, (row, col));
                }
            } else {
                ui_board_state.borrow_mut().pressed_piece = Some((row, col));
                let legal_moves = board_ref_cell.borrow().get_legal_moves((row, col));
                higlight_legal_moves(&grid, legal_moves, (row, col));
            }
        }
    ));

    let is_black = (row + col) % 2 == 1;
    let css = if is_black { BLACK_CSS } else { WHITE_CSS };
    let provider = gtk::CssProvider::new();
    provider.load_from_data(css);

    cell_button
        .style_context()
        .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    cell_button
}
fn ui_move_piece(r1: usize, c1: usize, r2: usize, c2: usize, grid: &Grid) {
    // Get buttons at source and destination positions
    let button1 = grid
        .child_at(c1 as i32, r1 as i32)
        .and_downcast::<gtk::Button>()
        .expect("Failed to get button1");
    let button2 = grid
        .child_at(c2 as i32, r2 as i32)
        .and_downcast::<gtk::Button>()
        .expect("Failed to get button2");

    let image1 = button1
        .child()
        .and_then(|child| child.downcast_ref::<gtk::Image>().cloned());
    button1.set_child(GTK_NONE);
    if let Some(image) = image1 {
        button2.set_child(Some(&image));
    } else {
        button2.set_child(GTK_NONE);
    }
}
fn ui_reset_grid_color(grid: &Grid) {
    for r in 0..ROWS {
        for c in 0..COLS {
            let button = grid
                .child_at(c as i32, r as i32)
                .and_downcast::<gtk::Button>()
                .expect("Failed to get button");
            let is_black = (r + c) % 2 == 1;
            let css = if is_black { BLACK_CSS } else { WHITE_CSS };
            let provider = gtk::CssProvider::new();
            provider.load_from_data(css);
            button
                .style_context()
                .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        }
    }
}
fn higlight_legal_moves(grid: &Grid, legal_moves: Vec<(usize, usize)>, from: (usize, usize)) {
    highlight_chosen_square(grid, from.0, from.1);
    highlight_squares_to_go_to(grid, legal_moves);
}
fn highlight_chosen_square(grid: &Grid, row: usize, col: usize) {
    let button = grid
        .child_at(col as i32, row as i32)
        .and_downcast::<gtk::Button>()
        .expect("Failed to get button");
    let css = "button { background-color:rgb(47, 0, 255); }";
    let provider = gtk::CssProvider::new();
    provider.load_from_data(css);
    button
        .style_context()
        .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}

fn highlight_squares_to_go_to(grid: &Grid, legal_moves: Vec<(usize, usize)>) {
    for (r, c) in legal_moves {
        let button = grid
            .child_at(c as i32, r as i32)
            .and_downcast::<gtk::Button>()
            .expect("Failed to get button");
        let css = "button { background-color: #ff0000; }"; // Red square
        let provider = gtk::CssProvider::new();
        provider.load_from_data(css);
        button
            .style_context()
            .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}
fn ui_promote_pawn(grid: &Grid, row: usize, col: usize, piece_promoted: &Piece) {
    let button = grid
        .child_at(col as i32, row as i32)
        .and_downcast::<gtk::Button>()
        .expect("Failed to get button");
    let image = get_image(piece_promoted).expect("Failed to get image of a promoted peice");
    button.set_child(Some(&image));
}
