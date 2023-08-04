#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console in release mode in windows

use vizia::prelude::*;

use data::*;
use pages::*;

mod data;
mod pages;

fn run_game(cx: &mut Context) {
    GameData::new().build(cx);

    Binding::new(cx, GameData::page, |cx, lens| match lens.get(cx) {
        Page::StartScreen => {
            StartScreen::new(cx);
        }
        Page::GameScreen => {
            GameScreen::new(cx);
        }
        Page::Finished(win, time, flip_count) => {
            FinishedScreen::new(cx, win, time, flip_count);
        }
    })
}

fn main() {
    Application::new(run_game)
        .title("Tile Matching Game")
        .min_inner_size(Some((800, 616)))
        .run()
}
