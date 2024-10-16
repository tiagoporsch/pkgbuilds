use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use webkit6::prelude::{WebViewExt};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("io.github.tiagoporsch.WhatsApp")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(1366)
            .default_height(768)
            .title("WhatsApp")
            .build();

        let browser = webkit6::WebView::new();
        browser.load_uri("https://web.whatsapp.com");
        window.set_child(Some(&browser));

        window.present();
    });

    app.run()
}
