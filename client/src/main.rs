mod lib;
mod ui;

use cursive::{align::*, event::*, view::*, views::*};
use lib::*;
use std::{io::*, net::*, sync::*, thread};
use ui::*;

fn main() {
    let mut siv = cursive::default();

    siv.set_theme(theme());

    let stream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(stream) => stream,
        Err(e) => {
            dialog(&mut siv, format!("Error: Failed to connect to server:\n{e}"), DialogType::Error);
            siv.run();

            return;
        }
    };

    let mut reader = BufReader::new(match stream.try_clone() {
        Ok(stream) => stream,
        Err(e) => {
            dialog(&mut siv, format!("Error: Failed to clone stream:\n{e}"), DialogType::Error);
            siv.run();

            return;
        }
    });
    let writer = Arc::new(Mutex::new(BufWriter::new(stream)));
    let writer_clone = writer.clone();

    let layout = LinearLayout::vertical().child(Panel::new(scroll_view())).child(
        OnEventView::new(EditView::new().with_name("content"))
            .on_event(Key::Enter, move |s| {
                if let Err(e) = submit(
                    match &mut (writer.lock()) {
                        Ok(writer) => writer,
                        Err(e) => {
                            dialog(s, format!("Error: Failed to lock writer:\n{e}"), DialogType::Error);
                            return;
                        }
                    },
                    s,
                ) {
                    dialog(s, format!("Error: Failed to send message:\n{e}"), DialogType::Info);
                }
            })
            .on_event(EventTrigger::any(), |s| {
                if let None = s.call_on_name("scroll", |view: &mut ScrollView<NamedView<TextView>>| view.scroll_to_bottom()) {
                    return;
                }
            }),
    );

    siv.add_fullscreen_layer(Dialog::around(layout).h_align(HAlign::Center).full_screen());

    let cb_sink = siv.cb_sink().clone();
    let cb_sink_clone = cb_sink.clone();

    thread::spawn(move || loop {
        let mut line = String::new();

        if let Err(e) = reader.read_line(&mut line) {
            _ = cb_sink.send(Box::new(move |s| dialog(s, format!("Error: Failed to read message:\n{e}"), DialogType::Info)));

            break;
        };

        if line.trim().len() != 0 {
            _ = cb_sink.send(Box::new(move |siv| {
                _ = siv.call_on_name("chat", |view: &mut TextView| view.append(format!("{}\n", line.trim())));
            }));
        }
    });

    siv.add_layer(Dialog::around(
        LinearLayout::vertical().child(Panel::new(TextView::new("Enter your name").with_name("chat"))).child(
            OnEventView::new(EditView::new().with_name("client_name")).on_event(Key::Enter, move |s| {
                submit_name(writer_clone.clone(), cb_sink_clone.clone(), s);
            }),
        ),
    ));

    siv.run();
}
