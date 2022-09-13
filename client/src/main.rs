mod ui;

use cursive::{align::*, event::*, view::*, views::*, *};
use std::{io::*, net::*, sync::*, thread};
use ui::*;

enum DialogType {
    Error,
    Info,
}

fn dialog<T>(cursive: &mut Cursive, text: T, dialog_type: DialogType)
where
    T: Into<String>,
{
    cursive.add_layer(Dialog::text(text).button("Ok", move |s| match dialog_type {
        DialogType::Error => s.quit(),
        DialogType::Info => {
            s.pop_layer();
        }
    }));
}

fn submit(writer: &mut BufWriter<TcpStream>, cursive: &mut Cursive) -> Result<()> {
    let content = match cursive.call_on_name("content", |view: &mut EditView| view.get_content()) {
        Some(content) => content,
        None => return Ok(()),
    };

    writer.write(format!("{}\n", content.trim()).as_bytes())?;
    writer.flush()?;

    if let None = cursive.call_on_name("content", |view: &mut EditView| view.set_content(String::new())) {
        return Ok(());
    }

    if let None = cursive.call_on_name("chat", |view: &mut TextView| {
        if view.get_content().source().trim() == "Enter your name" {
            view.set_content(String::new());

            view.append(format!("\n[Client] Welcome, {}!\n\n", content.trim()));
        }
    }) {
        return Ok(());
    }

    Ok(())
}

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

    let layout = LinearLayout::vertical()
        .child(Panel::new(scroll_view()))
        .child(OnEventView::new(EditView::new().with_name("content")).on_event(Key::Enter, move |s| {
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
        }));

    siv.add_fullscreen_layer(Dialog::around(layout).h_align(HAlign::Center).full_screen());

    let cb_sink = siv.cb_sink().clone();

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

    siv.run();
}
