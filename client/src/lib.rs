use cursive::{views::*, *};
use std::{io::*, net::*};

#[derive(Debug)]
pub enum DialogType {
    Error,
    Info,
}

pub fn dialog<T>(cursive: &mut Cursive, text: T, dialog_type: DialogType)
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

pub fn submit(writer: &mut BufWriter<TcpStream>, cursive: &mut Cursive) -> Result<()> {
    let content = match cursive.call_on_name("content", |view: &mut EditView| view.get_content()) {
        Some(content) => content,
        None => return Ok(()),
    };

    if let None = cursive.call_on_name("content", |view: &mut EditView| view.set_content(String::new())) {
        return Ok(());
    }

    if let None = cursive.call_on_name("chat", |view: &mut TextView| {
        if view.get_content().source().trim() == "Enter your name" {
            view.set_content(String::new());
        }
    }) {
        return Ok(());
    }

    writer.write(format!("{}\n", content.trim()).as_bytes())?;
    writer.flush()?;

    if let None = cursive.call_on_name("scroll", |view: &mut ScrollView<NamedView<TextView>>| view.scroll_to_bottom()) {
        return Ok(());
    }

    Ok(())
}
