use gtk::prelude::*;
use gtk::{glib::ExitCode, Application};
use xp_chess::ui::build_ui;
fn main() -> ExitCode {
    // let mut board = Board::new();
    let application = Application::builder().application_id("Chess").build();
    application.connect_startup(move |app| {
        build_ui(app);
    });
    // application.connect_activate(|_| {});
    let exit_code = application.run();
    println!("closed the application");
    return exit_code;
}
