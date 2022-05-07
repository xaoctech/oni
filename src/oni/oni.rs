use anyhow::anyhow;
use std::path::PathBuf;

pub struct CommandLine {
    pub allowPrivateData: bool,
    pub readConfigFile: bool,
    pub logCombos: bool,
    pub filmPlayback: bool,
    pub useSound: bool,
    pub haltOnError: bool,
    pub debugMaps: bool,
    pub resolutionSwitch: bool,
    pub debugFileEnable: bool,
    pub soundSearchOnDisk: bool,
    pub soundSearchBinariesOnDisk: bool,
    pub game_folder: Option<PathBuf>,
}

impl Default for CommandLine {
    fn default() -> Self {
        CommandLine {
            allowPrivateData: true,
            readConfigFile: true,
            logCombos: false,
            filmPlayback: false,
            useSound: true,
            haltOnError: false,
            debugMaps: false,
            resolutionSwitch: true,
            debugFileEnable: false,
            soundSearchOnDisk: false,
            soundSearchBinariesOnDisk: false,
            game_folder: None,
        }
    }
}

pub fn parse_command_line() -> anyhow::Result<CommandLine> {
    let mut cmd = CommandLine::default();
    let mut args = std::env::args_os().skip(1);
    while let Some(arg) = args.next() {
        match arg.to_str().ok_or(anyhow!("invalid option {:?}", arg))? {
            "-ignore_config" => cmd.readConfigFile = false,
            "-nosound" => cmd.useSound = false,
            "-ehalt" => cmd.haltOnError = true,
            "-combos" => cmd.logCombos = true,
            "-debug" => cmd.debugMaps = true,
            "-ignore_private_data" => cmd.allowPrivateData = false,
            "-noswitch" => cmd.resolutionSwitch = false,
            "-debugfiles" => cmd.debugFileEnable = true,
            "-findsounds" => cmd.soundSearchOnDisk = true,
            "-findsoundbinaries" => cmd.soundSearchBinariesOnDisk = true,
            "-game_folder" => {
                cmd.game_folder = Some(PathBuf::from(
                    args.next()
                        .ok_or(anyhow!("expected -game_folder parameter"))?,
                ))
            }
            _ => return Err(anyhow!("invalid option {:?}", arg)),
        }
    }
    Ok(cmd)
}
