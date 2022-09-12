use cursive::{
    event::*,
    view::{scroll::*, *},
    views::*,
    With,
};

pub fn scroll_view() -> impl View {
    ScrollView::new(TextView::new(String::new()).with_name("chat"))
        .scroll_strategy(ScrollStrategy::StickToBottom)
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
}
