use librecipe::{Recipe, Unit, IngredientAmount, Ingredient};
use fraction::Fraction;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let r = Recipe {
        ingredients: vec![IngredientAmount {
            ingredient: Ingredient {
                name: String::from("Jaloviina")
            },
            unit: Unit::Dl,
            amount: Fraction::from(2)
        }],
        steps: vec![String::from("Ou jee, juo")]
    };
    println!("{:?}", r);

    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |button| {
        button.set_label("Hello world!");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Reseptisofta")
        .child(&button)
        .build();

    window.present();
}
