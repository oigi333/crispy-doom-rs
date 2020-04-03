use enumflags2::BitFlags;

pub mod menu;

pub struct CrispySettings {
    automap_overlay: bool,
    automap_rotate: bool,
    automap_stats: bool,
    bob_factor: BOBFactor,
    bright_maps: BitFlags<BrightMaps>,
    center_weapon: CenterWeapon,
    colored_blood: bool,
    colored_hud: BitFlags<ColoredHud>,
    crosshair: Crosshair,
    crosshair_health: bool,
    crosshair_target: bool,
    crosshair_type: CrosshairType,
    demo_timer: BitFlags<DemoTimer>,
    demo_timer_dir: bool,
    demo_bar: bool,
    extended_auto_map: bool,
    extended_save_games: bool,
    flip_corpses: bool,
    free_aim: FreeAim,
    free_look: FreeLook,
    hires: bool,
    jump: Jump,
    level_time: Widgets,
    mouse_look: bool,
    negative_health: bool,
    /// "overunder"
    height_check: bool,
    // I do not know what is it does
    pitch: bool,
    player_coords: Widgets,
    recoil: bool,
    secret_message: SecretMessage,
    smooth_light: bool,
    smooth_scaling: bool,
    sound_fix: bool,
    sound_full: bool,
    sound_mono: bool,
    translucency: BitFlags<Translucency>,
    uncapped: bool,
    vsync: bool,
    weapon_squat: bool,
    wide_screen: bool,
    screenshot_message: bool,
    clean_screenshot: bool,
    demo_warp: bool,
    fps: bool,

    flashing_hom: bool,
    flip_levels: bool,
    flip_weapons: bool,
    // I have *NO* idea what those are
    haved1e5: bool,
    havee1m10: bool,
    havemap33: bool,
    havessg: bool,
    single_player: bool,
    stretch_sky: bool,

    sdl_version: String,
    platform: String,
    // TODO generic Fn trait
    post_rendering_hook: fn() -> ()
}

impl Default for CrispySettings {
    fn default() -> Self {
        Self {
            automap_overlay: false,
            automap_rotate: false,
            automap_stats: false,
            bob_factor: BOBFactor::Full,
            bright_maps: BitFlags::empty(),
            center_weapon: CenterWeapon::Off,
            colored_blood: true,
            colored_hud: BitFlags::empty(),
            crosshair: Crosshair::Off,
            crosshair_health: false,
            crosshair_target: false,
            crosshair_type: CrosshairType::Cross,
            demo_timer: BitFlags::empty(),
            demo_timer_dir: false,
            demo_bar: false,
            extended_auto_map: true,
            extended_save_games: true,
            flip_corpses: false,
            free_aim: FreeAim::Auto,
            free_look: FreeLook::Off,
            hires: true,
            jump: Jump::Off,
            level_time: Widgets::Off,
            mouse_look: false,
            negative_health: false,
            /// "overunder"
            height_check: false,
            /// I do not know what is it does
            pitch: false,
            player_coords: Widgets::Off,
            recoil: false,
            secret_message: SecretMessage::Off,
            smooth_light: false,
            smooth_scaling: true,
            sound_fix: true,
            sound_full: true,
            sound_mono: false,
            translucency: BitFlags::empty(),
            uncapped: false,
            vsync: true,
            weapon_squat: false,
            wide_screen: false,
            screenshot_message: false,
            clean_screenshot: false,
            demo_warp: false,
            fps: false,

            flashing_hom: false,
            flip_levels: false,
            flip_weapons: false,
            /// I have *NO* idea what those are
            haved1e5: false,
            havee1m10: false,
            havemap33: false,
            havessg: false,
            single_player: false,
            stretch_sky: false,
            sdl_version: "".into(),
            platform: "".into(),

            post_rendering_hook: || {}
        }
    }
}

/// I'll figure out what does it mean later
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BOBFactor {
    Off,
    _75,
    Full
}

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum BrightMaps {
    Textures = 1,
    Sprites = 2,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CenterWeapon {
    Off,
    Center,
    BOB
}

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum ColoredHud {
    Bar = 1,
    Text = 2,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Crosshair {
    Off,
    Static,
    Projected
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CrosshairType {
    None,
    Cross,
    Chevron,
    Dot
}

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum DemoTimer {
    Record = 1,
    Playback = 2
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FreeAim {
    Auto,
    Direct,
    Both
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FreeLook {
    Off,
    Spring,
    Lock
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Jump {
    Off,
    Low,
    High
}

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Translucency {
    Missile = 1,
    Item = 2
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SecretMessage {
    Off,
    On,
    Count
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Widgets {
    Off,
    Automap,
    Always
}
