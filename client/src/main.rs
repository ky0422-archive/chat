use cursive::{
    align::*,
    event::*,
    theme::{PaletteColor::*, *},
    view::{scroll::*, *},
    views::*,
    *,
};

use std::{cell::RefCell, io::*, net::*, rc::Rc, sync::mpsc, thread};

fn main() {
    let mut siv = cursive::default();

    siv.set_theme(cursive::theme::Theme {
        borders: BorderStyle::Simple,
        palette: Palette::default().with(|palette| {
            palette[Background] = Color::Rgb(30, 30, 30);
        }),
        ..cursive::theme::Theme::default()
    });

    let stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let writer = Rc::new(RefCell::new(BufWriter::new(stream)));

    let layout = LinearLayout::vertical()
        .child(TextView::new("Enter Name").with_name("title"))
        .child(Panel::new(
            TextView::new(String::new())
                .scrollable()
                .wrap_with(OnEventView::new)
                .on_pre_event_inner(Key::PageUp, |v, _| {
                    let scroller = v.get_scroller_mut();

                    if scroller.can_scroll_up() {
                        scroller.scroll_up(scroller.last_outer_size().y.saturating_sub(1));
                    }

                    Some(EventResult::Consumed(None))
                })
                .on_pre_event_inner(Key::PageDown, |v, _| {
                    let scroller = v.get_scroller_mut();

                    if scroller.can_scroll_down() {
                        scroller.scroll_down(scroller.last_outer_size().y.saturating_sub(1));
                    }

                    Some(EventResult::Consumed(None))
                })
                .with_name("chat"),
        ))
        .child(EditView::new().with_name("content"));

    let layer = Dialog::around(layout).title("Chat").h_align(HAlign::Center).button("Submit", move |s| {
        let (title, content) = (
            s.call_on_name("title", |view: &mut TextView| format!("{}", view.get_content().source())).unwrap(),
            s.call_on_name("content", |view: &mut EditView| view.get_content()).unwrap(),
        );

        if title == "Enter Name" {
            write_content(&mut *writer.borrow_mut(), format!("{content:}"));

            s.call_on_name("title", |view: &mut TextView| view.set_content(format!("Welcome, {content:}!")))
                .unwrap();

            s.call_on_name("content", |view: &mut EditView| view.set_content(String::new())).unwrap();
        } else {
            write_content(&mut *writer.borrow_mut(), format!("{content:}"));
        }
    });

    siv.add_fullscreen_layer(layer.full_screen());

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || { // A, TODO
        let mut line = String::new();

        loop {
            reader.read_line(&mut line).unwrap();

            if line.trim().len() != 0 {
                siv.call_on_name("chat", |view: &mut TextView| {
                    view.set_content(format!("{}{}", view.get_content().source(), line.trim()))
                }); // <- this line is the problem; goto A
            }

            line.clear();
        }
    });

    siv.run();
}

fn write_content(writer: &mut BufWriter<TcpStream>, content: String) {
    writer.write(format!("{}\n", content.trim()).as_bytes()).unwrap();
    writer.flush().unwrap();
}
