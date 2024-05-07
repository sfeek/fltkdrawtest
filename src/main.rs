use fltk::{app::*, button::*, dialog::*, draw::*, enums::*, input::*, prelude::*, window::Window};

// Message enums
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Green,
    Red,
}

// Return a value from an integer text box safely
fn get_value(r: &IntInput) -> i32 {
    let v: i32 = match r.value().parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            alert(368, 265, "Radius Error!");
            return 0;
        }
    };
    v
}

fn main() {
    // Main app
    let app = App::default();

    // Main form
    let mut wind = Window::default().with_size(400, 300);

    // Two Buttons
    let mut but_green = Button::new(100, 250, 80, 40, "Green");
    let mut but_red = Button::new(200, 250, 80, 40, "Red");

    // Radius input box
    let radius = IntInput::new(150, 220, 54, 22, "Radius");

    // Setup a message handler
    let (s, r) = channel::<Message>();

    // Attach the buttons to messages
    but_green.emit(s, Message::Green);
    but_red.emit(s, Message::Red);

    // End the Main Window definitions and show the form
    wind.end();
    wind.show();

    // Main app wait loop
    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Green => {
                    // Draw Green Circle
                    let value = get_value(&radius);
                    wind.draw(move |_| {
                        set_draw_color(Color::Green);
                        draw_circle_fill(200, 150, value,Color::Green );
                    });

                    // Force form refresh
                    wind.redraw();
                }
                Message::Red => {
                    // Draw Red Circle
                    let value = get_value(&radius);
                    wind.draw(move |_| {
                        set_draw_color(Color::Red);
                        draw_circle_fill(200, 150, value,Color::Red);
                    });

                    // Force form refresh
                    wind.redraw();
                }
            }
        }
    }
}
