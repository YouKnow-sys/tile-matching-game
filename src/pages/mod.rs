/// The page that use see in end of each play time, this page can show user that they won or lose
pub use finish_page::FinishedScreen;
/// Game page
pub use game_page::GameScreen;
/// Start menu screen that let user change games modes and start playing
pub use start_menu::StartScreen;

mod finish_page;
mod game_page;
mod start_menu;
