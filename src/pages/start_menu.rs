use vizia::prelude::*;

use crate::data::{GameData, GameEvent, GameMode, Page};

pub struct StartScreen;

impl StartScreen {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        cx.add_stylesheet(include_style!("src/pages/styles/start_menu.css"))
            .unwrap();

        Self.build(cx, |cx| {
            Label::new(cx, "Tile Matching Game \u{eb10}") // because we are using the icon class we'll only see T M G because Icon font dont have rest of the characters
                .id("title")
                .class("icon");

            Button::new(
                cx,
                |cx| cx.emit(GameEvent::ChangePage(Page::GameScreen)),
                |cx| Label::new(cx, "Start the game"),
            );

            HStack::new(cx, |cx| {
                Label::new(cx, "Game mode : ");
                Button::new(
                    cx, |cx| cx.emit(GameEvent::ChangeGameMode),
                    |cx| Label::new(cx, GameData::game_mode),
                )
                .tooltip(|cx| {
                    Binding::new(cx, GameData::game_mode, |cx, lens| match lens.get(cx) {
                        GameMode::FreePlay => {
                            Label::new(cx, "FreePlay mode, player can take as long as they want to finish\nthe game in this mode");
                        },
                        GameMode::Timed => {
                            Label::new(cx, "You will have a limited amount of time to finish the game\nif you dont finish the game in time you'll lose!");
                        },
                    })
                });
            })
            .class("game-mode");

            HStack::new(cx, |cx| {
                Label::new(cx, "Difficulty :");
                Spinbox::new(
                    cx,
                    GameData::difficulty,
                    SpinboxKind::Horizontal,
                    SpinboxIcons::PlusMinus,
                )
                .on_increment(|cx| cx.emit(GameEvent::ChangeDifficulty(true)))
                .on_decrement(|cx| cx.emit(GameEvent::ChangeDifficulty(false)));
            })
            .class("difficulty");

            Button::new(
                cx,
                |cx| cx.emit(GameEvent::Exit),
                |cx| Label::new(cx, "Close the game"),
            );
        })
        .layout_type(LayoutType::Column)
    }
}

impl View for StartScreen {
    fn element(&self) -> Option<&'static str> {
        Some("startscreen")
    }
}
