use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Box as GtkBox, Orientation, CssProvider};
use gtk::gdk::Display;
use std::env;



const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    

    app.run()
}

fn build_ui(app: &Application) {


    let button_image = Button::builder()
        .build();
    button_image.set_size_request(300, 300);
    button_image.set_widget_name("btimage");

    let button_play = Button::builder()
        .label("  ")
        .build();
    button_play.set_size_request(100, 100);
    button_play.set_widget_name("bt1");

    let button_back = Button::builder()
        .label(" 󰼥 ")
        .build();
    button_back.set_size_request(100, 100);
    button_back.set_widget_name("bt2"); 

    let button_next = Button::builder()
        .label(" 󰼧 ")
        .build();
    button_next.set_size_request(100, 100); 
    button_next.set_widget_name("bt3"); 

    let music_name = Button::builder()
        .label("Musicaaa")
        .build();
    music_name.set_widget_name("music_name");

    let music_bar = Button::builder()
        .build();

    music_bar.set_widget_name("music_bar");


    let boximage = GtkBox::new(Orientation::Vertical, 0);
    boximage.append(&button_image);
    boximage.append(&music_name);
    boximage.append(&music_bar);

    let box1 = GtkBox::new(Orientation::Horizontal, 0);
    box1.append(&button_back);
    box1.append(&button_play);
    box1.append(&button_next);



    let container1 = GtkBox::new(Orientation::Vertical, 0);
    container1.append(&boximage);
    container1.append(&box1);

    let container_main = GtkBox::new(Orientation::Horizontal,0);

    container_main.append(&container1);
    container_main.set_widget_name("container");

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Player")
        .child(&container_main)
        .build();


    window.present();
}

fn load_css() {

    let provider = CssProvider::new();

    provider.load_from_string(include_str!("styles/style.css"));


    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Não foi possível conectar a um display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

