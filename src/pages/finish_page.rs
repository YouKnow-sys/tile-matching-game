use vizia::prelude::*;

use crate::data::{GameEvent, Page};

pub struct FinishedScreen;

impl FinishedScreen {
    pub fn new(cx: &mut Context, win: bool, time: Duration, flip_count: usize) -> Handle<Self> {
        cx.add_stylesheet(include_style!("src/pages/styles/finish_page.css"))
            .unwrap();

        Self.build(cx, |cx| {
            Label::new(cx, if win { "You WON!" } else { "You LOSE!" })
                .checked(win)
                .class("title");

            Label::new(cx, &format!("Time: {}s", time.as_secs()));
            Label::new(cx, &format!("Flip count: {flip_count}"));

            Button::new(
                cx,
                |cx| cx.emit(GameEvent::ChangePage(Page::StartScreen)),
                |cx| Label::new(cx, "Go back to MainMenu"),
            );
        })
        .layout_type(LayoutType::Column)
    }
}

impl View for FinishedScreen {
    fn element(&self) -> Option<&'static str> {
        Some("finishedpage")
    }
}
