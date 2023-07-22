use crate::Cindy;
use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};

const APP_ID: &str = "org.xfbs.Cindy";

pub fn main(cindy: Cindy) -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run_with_args::<String>(&[])
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Present window
    window.present();
}
