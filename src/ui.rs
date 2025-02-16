use crate::board::Board;
use crate::consts::{COLS, GTK_NONE, HEIGHT, ROWS, WIDTH};
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
    let board = Rc::new(RefCell::new(Board::new()));
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
            // let cell_button = build_button(row, col, &Piece::Empty(Empty {}));
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
    if let Some(image) = get_image(&(board_ref_cell.borrow().board[row * COLS + col])) {
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
                let (r, c) = ui_board_state.borrow().pressed_piece.unwrap();
                let mut board = board_ref_cell.borrow_mut();
                ui_move_piece(r, c, row, col, &mut board, &grid);
                ui_board_state.borrow_mut().pressed_piece = None;
            } else {
                ui_board_state.borrow_mut().pressed_piece = Some((row, col));
            }
            let to_print = board_ref_cell.borrow().board[row * COLS + col].is_empty();
            println!("{to_print}");
        }
    ));

    let is_black = (row + col) % 2 == 1; // Alternating black & white pattern
                                         // Set button color
    let css = if is_black {
        "button { background-color: #769656; }" // Greenish black square
    } else {
        "button { background-color: #eeeed2; }" // Beige white square
    };
    let provider = gtk::CssProvider::new();
    provider.load_from_data(css);

    let style_context = cell_button.style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    cell_button
}
fn ui_move_piece(r1: usize, c1: usize, r2: usize, c2: usize, board: &mut Board, grid: &Grid) {
    board.move_piece((r1, c1), (r2, c2));

    // Get buttons at source and destination positions
    let button1 = grid
        .child_at(c1 as i32, r1 as i32)
        .and_downcast::<gtk::Button>()
        .expect("Failed to get button1");
    let button2 = grid
        .child_at(c2 as i32, r2 as i32)
        .and_downcast::<gtk::Button>()
        .expect("Failed to get button2");

    // Get new images based on board state
    let piece1 = board.get_piece(r1, c1);
    let piece2 = board.get_piece(r2, c2);

    let image1 = get_image(piece1);
    let image2 = get_image(piece2);
    let image1 = button1
        .child()
        .and_then(|child| child.downcast_ref::<gtk::Image>().cloned());
    let image2 = button2
        .child()
        .and_then(|child| child.downcast_ref::<gtk::Image>().cloned());
    println!("{:?} {:?}", image1, image2);
    // Update buttons with new images
    // if let Some(image) = image1 {
    //     button1.set_child(Some(&image));
    // } else {
    //     button1.set_child(GTK_NONE);
    // }
    button1.set_child(GTK_NONE);
    if let Some(image) = image1 {
        button2.set_child(Some(&image));
    } else {
        button2.set_child(GTK_NONE);
    }
}
