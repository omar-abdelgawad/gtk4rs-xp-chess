use glib::clone;
use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "org.gtk_rs.MutableStateExample";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // ✅ Use Rc<RefCell<i32>> for mutable state
    let number = Rc::new(RefCell::new(0));

    button_increase.connect_clicked(clone!(
        #[strong]
        number,
        #[strong]
        button_decrease,
        move |_| {
            let mut num = number.borrow_mut(); // 🔥 Mutable access
            *num += 1;
            button_decrease.set_label(&num.to_string());
        }
    ));

    button_decrease.connect_clicked(clone!(
        #[strong]
        number,
        #[strong]
        button_increase,
        move |_| {
            let mut num = number.borrow_mut(); // 🔥 Mutable access
            *num -= 1;
            button_increase.set_label(&num.to_string());
        }
    ));

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Mutable GTK State")
        .child(&gtk_box)
        .build();

    window.present();
}
