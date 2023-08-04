use std::fmt::Display;

use vizia::prelude::*;

pub enum GameEvent {
    /// Change the current page of the game
    ChangePage(Page),
    /// Change game mode
    ChangeGameMode,
    /// Change the defficulty of the game, if true add to it
    /// and if false sub from it
    ChangeDifficulty(bool),
    /// Exit from the game (save settings if serde is enabled)
    Exit,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Data)]
pub enum Page {
    #[default]
    StartScreen,
    GameScreen,
    Finished(bool, Duration, usize),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Data)]
pub enum GameMode {
    /// FreePlay mode, player can take as long as they want to finish
    /// the game in this mode
    #[default]
    FreePlay,
    /// You will have a limited amount of time to finish the game
    /// if you dont finish the game in time you'll lose!
    Timed,
}

impl GameMode {
    fn next(&mut self) {
        *self = match self {
            GameMode::FreePlay => GameMode::Timed,
            GameMode::Timed => GameMode::FreePlay,
        }
    }
}

impl Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(default)
)]
#[derive(Lens)]
pub struct GameData {
    #[serde(skip)]
    pub page: Page,
    pub game_mode: GameMode,
    pub difficulty: usize,
}

impl GameData {
    pub fn new() -> Self {
        #[cfg(feature = "serde")]
        {
            // try to load settings
            match std::env::current_dir().and_then(|p| std::fs::File::open(p.join("settings.json")))
            {
                Ok(reader) => match serde_json::from_reader(reader) {
                    Ok(game_data) => return game_data,
                    Err(e) => eprintln!(
                        "Failed to load settings json because of: {e:#?}, using default instead."
                    ),
                },
                Err(e) => {
                    eprintln!(
                        "Failed to open settings file because of: {e:#?}, using default instead."
                    )
                }
            }
        }
        Self::default()
    }
}

impl Model for GameData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, meta| {
            match event {
                GameEvent::ChangePage(page) => self.page = *page,
                GameEvent::ChangeGameMode => self.game_mode.next(),
                GameEvent::ChangeDifficulty(add) => {
                    self.difficulty = match add {
                        true => self.difficulty.saturating_add(2).clamp(2, 10),
                        false => self.difficulty.saturating_sub(2).clamp(2, 10),
                    }
                }
                GameEvent::Exit => cx.emit(WindowEvent::WindowClose),
            }
            meta.consume();
        });

        #[cfg(feature = "serde")]
        event.map(|event, _| {
            if let WindowEvent::WindowClose = event {
                // save the settings before exit
                match std::env::current_dir()
                    .and_then(|p| std::fs::File::create(p.join("settings.json")))
                {
                    Ok(writer) => {
                        if let Err(e) = serde_json::to_writer_pretty(writer, self) {
                            eprintln!("Failed to write settings because of: {e:#?}")
                        }
                    }
                    Err(e) => eprintln!("Failed to write settings to disk because of: {e:#?}"),
                }
            }
        });
    }
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            page: Default::default(),
            game_mode: Default::default(),
            difficulty: 4,
        }
    }
}
