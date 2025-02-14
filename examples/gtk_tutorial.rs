use gtk::prelude::*;
use gtk::{glib::ExitCode, Application, ApplicationWindow, Button, Grid, Image};
use xp_chess::consts::{HEIGHT, WIDTH};

fn building_window(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("First GTK Program")
        .default_width(HEIGHT)
        .default_height(WIDTH)
        .build();
    let grid = Grid::new();
    grid.set_hexpand(true);
    grid.set_vexpand(true);
    for row in 1..=8 {
        for col in 1..=8 {
            let button = Button::new();
            button.set_hexpand(true);
            button.set_vexpand(true);
            button.set_size_request(0, 0); // Allows dynamic resizing
            button.connect_clicked(move |_| {
                println!("Button clicked: ({}, {})", row, col);
            });
            let image = Image::from_file("./resources/Chess_bdt60.png"); // Replace with your image path
            button.set_child(Some(&image));
            let is_black = (row + col) % 2 == 1; // Alternating black & white pattern

            // Set button color
            let css = if is_black {
                "button { background-color: #769656; }" // Greenish black square
            } else {
                "button { background-color: #eeeed2; }" // Beige white square
            };
            let provider = gtk::CssProvider::new();
            provider.load_from_data(css);

            let style_context = button.style_context();
            style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

            // Attach button to the grid
            grid.attach(&button, col, row, 1, 1);
        }
    }
    window.set_child(Some(&grid));
    window.present();
}
fn main() -> ExitCode {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(building_window);
    let exit_code = application.run();
    println!("closed the application");
    return exit_code;
}
