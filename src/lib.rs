pub mod board;
pub mod consts {
    pub const GTK_NONE: Option<&gtk::Widget> = None;
    pub const HEIGHT: i32 = 700;
    pub const WIDTH: i32 = 700;
    pub const ROWS: usize = 8;
    pub const COLS: usize = 8;
    pub const WHITE_CSS: &str = "button { background-color: #eeeed2; }";
    pub const BLACK_CSS: &str = "button { background-color: #769656; }";
}
pub mod ui;
