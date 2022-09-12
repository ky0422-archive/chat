use cursive::{
    align::*,
    event::*,
    theme::{PaletteColor::*, *},
    view::{scroll::*, *},
    views::*,
    View, *,
};

use std::{cell::RefCell, io::*, net::*, rc::Rc, thread};

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
        .child(Panel::new(
            TextView::new(String::new())
                .with_name("chat")
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
                }),
        ))
        .child(EditView::new().with_name("content"));

    let layer = Dialog::around(layout)
        .title("Chat")
        .h_align(HAlign::Center)
        .button("Submit", move |s| submit(&mut *writer.borrow_mut(), s));

    siv.add_fullscreen_layer(layer.full_screen());

    let cb_sink = siv.cb_sink().clone();

    thread::spawn(move || {
        loop {
            let mut line = String::new();

            reader.read_line(&mut line).unwrap();

            if line.trim().len() != 0 {
                // siv.call_on_name("chat", |view: &mut TextView| {
                //     view.set_content(format!("{}{}", view.get_content().source(), line.trim()))
                // }); // <- this line is the problem; goto A

                cb_sink
                    .send(Box::new(move |siv| {
                        siv.call_on_name("chat", |view: &mut TextView| view.append(format!("{}\n", line.trim())));
                    }))
                    .unwrap();
            }
        }
    });

    siv.run();
}

fn submit(writer: &mut BufWriter<TcpStream>, cursive: &mut Cursive) {
    let content = cursive.call_on_name("content", |view: &mut EditView| view.get_content()).unwrap();

    writer.write(format!("{}\n", content.trim()).as_bytes()).unwrap();
    writer.flush().unwrap();

    cursive.call_on_name("content", |view: &mut EditView| view.set_content(String::new())).unwrap();

    // chat: focus last line

    cursive.call_on_name("chat", |view: &mut ScrollView<TextView>| {
        let scroller = view.get_scroller_mut();

        scroller.scroll_down(scroller.last_outer_size().y.saturating_sub(1));
    });
}
