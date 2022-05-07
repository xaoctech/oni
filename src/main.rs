#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use crate::bungie::template_manager::game::Game;
use crate::oni::oni::parse_command_line;
use anyhow::anyhow;
use std::path::{Path, PathBuf};

pub mod bungie;
pub mod oni;

const GameDataFolder1: &'static str = "GameDataFolder";
const GameDataFolder2: &'static str = "OniEngine/GameDataFolder";

fn find_path(base: &Path, name: &Path) -> anyhow::Result<PathBuf> {
    let mut base = base.canonicalize()?;
    loop {
        let path = base.join(name);
        if path.exists() {
            return Ok(path);
        }
        if !base.pop() {
            return Err(anyhow!("file {:?} not found", name));
        }
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let cmd = parse_command_line()?;
    log::info!("begin initializing oni");
    log::info!("looking for the game data folder");
    let base = if let Some(path) = cmd.game_folder {
        path
    } else {
        std::env::current_dir()?
    };

    let game_data_folder = if let Ok(path) = find_path(&base, Path::new(GameDataFolder1)) {
        path
    } else {
        log::warn!("unable to find game data folder at {}", GameDataFolder1);
        if let Ok(path) = find_path(&base, Path::new(GameDataFolder1)) {
            path
        } else {
            log::warn!("unable to find game data folder at {}", GameDataFolder2);
            return Err(anyhow!("could not find game data folder"));
        }
    };
    log::info!("data folder: {:?}", game_data_folder);

    let mut game = Game::new(game_data_folder);
    game.initialize()?;

    Ok(())
}
