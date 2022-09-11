use cursive::{
    theme::{PaletteColor::*, *},
    view::*,
    views::*,
    *,
};

use std::{cell::RefCell, io::*, net::*, rc::Rc, thread};

fn main() {
    let mut siv = cursive::default();

    siv.set_theme(cursive::theme::Theme {
        palette: Palette::default().with(|palette| {
            palette[Background] = Color::Rgb(30, 30, 30);
        }),
        ..cursive::theme::Theme::default()
    });

    let stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    // let mut reader = BufReader::new(stream.try_clone().unwrap());
    let writer = Rc::new(RefCell::new(BufWriter::new(stream)));

    // thread::spawn(move || {
    //     let mut line = String::new();

    //     loop {
    //         reader.read_line(&mut line).unwrap();

    //         println!("{}", line.trim());

    //         line.clear();
    //     }
    // });

    // loop {
    //     let mut reads = String::new();
    //     stdin().read_line(&mut reads).unwrap();

    //     writer
    //         .write(format!("{}\n", reads.trim()).as_bytes())
    //         .unwrap();
    //     writer.flush().unwrap();
    // }

    let layout = LinearLayout::vertical()
        .child(TextView::new("Hello, world!"))
        .child(
            EditView::new()
                .content("Hello, world!")
                .with_name("content"),
        );

    siv.add_layer(Dialog::around(layout).button("Submit", move |s| {
        let content = s
            .call_on_name("content", |view: &mut EditView| view.get_content())
            .unwrap();

        write_content(&mut *writer.borrow_mut(), format!("{content:}"));
    }));

    siv.run();
}

fn write_content(writer: &mut BufWriter<TcpStream>, content: String) {
    writer
        .write(format!("{}\n", content.trim()).as_bytes())
        .unwrap();
    writer.flush().unwrap();
}
