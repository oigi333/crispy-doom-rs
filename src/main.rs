mod crispy;

use structopt::StructOpt;
use std::str::FromStr;

#[derive(Debug, StructOpt)]
enum Opt {
    /// Start a dedicated server, routing packages but not participating in the game itself.
    Dedicated,
    /// Query the Internet master server for global list of active servers.
    Search,
    /// Query the status of the server running on the given IP address.
    Query{#[structopt()] addr: String},
    /// Search the local LAN for running servers.
    LocalServers,
    Game(DoomOpt)
}

#[derive(Debug, StructOpt)]
struct DoomOpt {
    /// Disable monsters.
    #[structopt(long)]
    no_monsters: bool,
    /// Monsters respawn after being killed.
    #[structopt(long)]
    respawn: bool,
    /// Monsters move faster.
    #[structopt(long)]
    fast: bool,
    /// Developer mode. F1 saves a screenshot in current working directory.
    #[structopt(long)]
    dev: bool,
    /// Game mode.
    /// 1. singleplayer(or sp)
    /// Single player game
    /// 2. deathmatch1(or deathmatch, dm, dm1):
    /// Start a deathmatch game.
    /// 3. deathmatch2(or altdeath, dm2)
    /// Start a deathmatch 2.0 game.
    /// Wet stay in place and all items respawn after 30 seconds.
    /// 4. deathmatch3(or dm3)
    /// Start a deathmatch 3.0 game.
    /// Weapons stay in place and all items respawn after 30 seconds.
    #[structopt(long, default_value = "singleplayer", verbatim_doc_comment)]
    game_mode: GameMode,
    /// Turbo mode. The player's speed is multiplied by x%.
    #[structopt(long)]
    turbo: Option<u32>
}

#[derive(Debug)]
pub enum GameMode {
    SinglePlayer,
    DeathMatch1,
    DeathMatch2,
    DeathMatch3,
}

impl FromStr for GameMode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_in = |s: &str, options: &[&str]| options.iter().any(|&option|s == option);

        if is_in(s, &["singleplayer", "sp"]) {
            Ok(GameMode::SinglePlayer)
        } else if is_in(s, &["deathmatch", "deathmatch1", "dm", "dm1"]) {
            Ok(GameMode::DeathMatch1)
        } else if is_in(s, &["deathmatch2", "altdeath", "dm"]) {
            Ok(GameMode::DeathMatch2)
        } else if is_in(s, &["deathmatch3", "dm3"]) {
            Ok(GameMode::DeathMatch3)
        } else if s == "" {
            Err("Game mode is required.")
        } else {
            Err("There is no such game mode.")
        }
    }
}

fn main() {
    let mut _crispy = crispy::CrispySettings::default();
    let opt: Opt = Opt::from_args();

    // TODO implement subcommands

    match opt {
        Opt::Dedicated => unimplemented!(),
        Opt::Search => unimplemented!(),
        Opt::Query{ addr } => unimplemented!(),
        Opt::LocalServers => unimplemented!(),
        Opt::Game(doom) => {
            // TODO: Init subsystems.
            // TODO: Read cfg file.
            // TODO: Find IWAD file.
            let iwad_file = false;
            if !iwad_file {
                // TODO: Error out correctly.
                eprintln!("Game variant indeterminate!");
            }
            // TODO: Load IWAD.


        }
    }

    ///

    println!("Hello, world!");
}
