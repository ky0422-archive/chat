pub mod ui;

use cursive::{views::*, *};
use std::{io::*, net::*, sync::*};

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

    writer.write(format!("{}\n", content.trim()).as_bytes())?;
    writer.flush()?;

    if let None = cursive.call_on_name("scroll", |view: &mut ScrollView<NamedView<TextView>>| view.scroll_to_bottom()) {
        return Ok(());
    }

    Ok(())
}

pub fn submit_name(writer: Arc<Mutex<BufWriter<TcpStream>>>, cb_sink: CbSink, cursive: &mut Cursive) {
    _ = cursive.call_on_name("client_name", |view: &mut EditView| match writer.lock() {
        Ok(ref mut writer) => {
            if let Err(e) = (|| -> Result<()> {
                writer.write(format!("{}\n", view.get_content().trim()).as_bytes())?;
                writer.flush()?;

                Ok(())
            })() {
                _ = cb_sink.send(Box::new(move |s| dialog(s, format!("Error: Failed to send message:\n{e}"), DialogType::Error)));
            }
        }
        _ => _ = cb_sink.send(Box::new(move |s| dialog(s, format!("Error: Failed to lock writer"), DialogType::Error))),
    });

    cursive.pop_layer();
}
