use rand::{seq::SliceRandom, thread_rng};
use vizia::prelude::*;

use crate::data::{GameData, GameEvent, GameMode, Page};

enum GameInnerEvent {
    TickTimer,
    FlipTile(usize),
    TimeFinished,
    GoBackToMainMenu,
}

#[derive(Lens)]
struct GameInnerData {
    tiles: Vec<(bool, bool, &'static str)>,
    game_mode: GameMode,
    timer: Timer,
    time: Duration,
    flip_count: usize,
    finish: bool,
}

impl GameInnerData {
    fn new(difficulty: usize, game_mode: GameMode, timer: Timer, starting_time: Duration) -> Self {
        let mut tiles: Vec<(bool, bool, &str)> = emojis::iter()
            .skip(84)
            .flat_map(|n| std::iter::repeat(n).take(2))
            .map(|e| (false, false, e.as_str()))
            .take(difficulty * difficulty)
            .collect();

        tiles.shuffle(&mut thread_rng());

        Self {
            tiles,
            game_mode,
            timer,
            time: starting_time,
            flip_count: 0,
            finish: false,
        }
    }
}

impl Model for GameInnerData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        if let Some(event) = event.take() {
            match event {
                GameInnerEvent::TickTimer => {
                    self.time = match self.game_mode {
                        GameMode::FreePlay => self.time.saturating_add(Duration::from_secs(1)),
                        GameMode::Timed => self.time.saturating_sub(Duration::from_secs(1)),
                    }
                }
                GameInnerEvent::FlipTile(idx) => {
                    if !self.tiles[idx].1 {
                        self.flip_count += 1;
                    }

                    let n_emoji = self.tiles[idx].2;
                    self.tiles[idx].0 = true; // make the new clicked tile visible

                    match self
                        .tiles
                        .iter_mut()
                        .enumerate()
                        .find(|(c_idx, (visible, solved, _))| *c_idx != idx && *visible && !*solved)
                    {
                        Some((_, (visible, solved, emoji))) => {
                            if *emoji == n_emoji {
                                *solved = true; // mark the tile as solved
                                self.tiles[idx].1 = true; // mark the tile as solved
                            } else {
                                *solved = false;
                                *visible = false;
                            }
                        }
                        None => self.tiles[idx].0 = true,
                    }

                    if self.tiles.iter().all(|(_, s, _)| *s) {
                        self.finish = true;
                        cx.stop_timer(self.timer);
                        cx.emit(GameEvent::ChangePage(Page::Finished(
                            true,
                            self.time,
                            self.flip_count,
                        )));
                    }
                }
                GameInnerEvent::TimeFinished => {
                    self.time = Duration::ZERO;
                    // time finished, player lost
                    if !self.finish {
                        cx.emit(GameEvent::ChangePage(Page::Finished(
                            false,
                            self.time,
                            self.flip_count,
                        )));
                    }
                }
                GameInnerEvent::GoBackToMainMenu => {
                    self.finish = true;
                    cx.stop_timer(self.timer);
                    cx.emit(GameEvent::ChangePage(Page::StartScreen))
                }
            }
        }
    }
}

pub struct GameScreen;

impl GameScreen {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        cx.add_stylesheet(include_style!("src/pages/styles/game_page.css"))
            .unwrap();

        let game_mode = GameData::game_mode.get(cx);
        let difficulty = GameData::difficulty.get(cx);

        let start_time = match game_mode {
            GameMode::FreePlay => None,
            GameMode::Timed => Some(Duration::from_secs((difficulty * 20) as u64)),
        };

        let timer = cx.add_timer(
            Duration::from_secs(1),
            start_time,
            |cx, action| match action {
                TimerAction::Start => (),
                TimerAction::Tick(_) => cx.emit(GameInnerEvent::TickTimer),
                TimerAction::Stop => cx.emit(GameInnerEvent::TimeFinished),
            },
        );

        Self.build(cx, |cx| {
            GameInnerData::new(difficulty, game_mode, timer, start_time.unwrap_or_default())
                .build(cx);

            cx.start_timer(timer);

            HStack::new(cx, |cx| {
                Label::new(
                    cx,
                    GameInnerData::time.map(move |t| {
                        format!(
                            "{} Time: {:?}s |",
                            match game_mode {
                                GameMode::FreePlay => "Elapsed",
                                GameMode::Timed => "Remaining",
                            },
                            t.as_secs(),
                        )
                    }),
                );

                Label::new(
                    cx,
                    GameInnerData::flip_count.map(|c| format!("Flip Count: {c}")),
                );

                Button::new(
                    cx,
                    |cx| cx.emit(GameInnerEvent::GoBackToMainMenu),
                    |cx| Label::new(cx, "Go back to MainScreen"),
                )
                .left(Stretch(1.0));
            })
            .class("topbar");

            VStack::new(cx, |cx| {
                for y in 0..difficulty {
                    HStack::new(cx, |cx| {
                        for x in 0..difficulty {
                            let idx = y * difficulty + x;
                            let tile = GameInnerData::tiles.index(idx);
                            Tile::new(
                                cx,
                                tile.map(|(c, _, _)| *c),
                                tile.map(|(_, _, s)| *s).get(cx),
                            )
                            .checked(tile.map(|(_, c, _)| *c))
                            .on_press(move |cx| cx.emit(GameInnerEvent::FlipTile(idx)));
                        }
                    });
                }
            })
            .class("tiles-area");
        })
        .layout_type(LayoutType::Column)
    }
}

impl View for GameScreen {
    fn element(&self) -> Option<&'static str> {
        Some("gamescreen")
    }
}

pub struct Tile;

impl Tile {
    pub fn new<'a, L: Lens<Target = bool>>(
        cx: &'a mut Context,
        checked: L,
        emoji: &str,
    ) -> Handle<'a, Self> {
        Self.build(cx, |cx| {
            Label::new(cx, emoji)
                .space(Stretch(1.0))
                .hoverable(false)
                .checked(checked)
                .class("inner");
        })
        .overflow(Overflow::Hidden)
    }
}

impl View for Tile {
    fn element(&self) -> Option<&'static str> {
        Some("tile")
    }
}
