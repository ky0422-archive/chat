mod ui;

use cursive::{align::*, view::*, views::*, *};
use std::{cell::RefCell, io::*, net::*, rc::Rc, thread};
use ui::*;

fn main() {
    let mut siv = cursive::default();

    siv.set_theme(theme());

    let stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let writer = Rc::new(RefCell::new(BufWriter::new(stream)));

    let layout = LinearLayout::vertical().child(Panel::new(scroll_view())).child(EditView::new().with_name("content"));

    let layer = Dialog::around(layout)
        .title("Chat")
        .button("Send", move |s| submit(&mut *writer.borrow_mut(), s))
        .button("Quit", |s| s.quit())
        .h_align(HAlign::Center);

    siv.add_fullscreen_layer(layer.full_screen());

    let cb_sink = siv.cb_sink().clone();

    thread::spawn(move || loop {
        let mut line = String::new();

        reader.read_line(&mut line).unwrap();

        if line.trim().len() != 0 {
            cb_sink
                .send(Box::new(move |siv| {
                    siv.call_on_name("chat", |view: &mut TextView| view.append(format!("{}\n", line.trim())));
                }))
                .unwrap();
        }
    });

    siv.run();
}

fn submit(writer: &mut BufWriter<TcpStream>, cursive: &mut Cursive) {
    let content = cursive.call_on_name("content", |view: &mut EditView| view.get_content()).unwrap();

    writer.write(format!("{}\n", content.trim()).as_bytes()).unwrap();
    writer.flush().unwrap();

    cursive.call_on_name("content", |view: &mut EditView| view.set_content(String::new())).unwrap();

    cursive
        .call_on_name("chat", |view: &mut TextView| {
            if view.get_content().source().trim() == "Enter your name" {
                view.set_content(String::new());
            }
        })
        .unwrap();
}
