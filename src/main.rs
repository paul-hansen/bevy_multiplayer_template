mod cli;
mod game;
use bevy::prelude::*;
use bevy::winit::{UpdateMode::Continuous, WinitSettings};
{% if egui_inspector -%}
#[cfg(feature = "dev")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
#[cfg(feature = "dev")]
use bevy_egui::EguiPlugin;
{%- endif %}
use bevy_replicon::RepliconPlugins;
use cli::CliConfigPlugin;
use game::GamePlugin;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
{% if color_eyre %}    color_eyre::install()?;{% endif -%}
    App::new()
        .insert_resource(WinitSettings {
            focused_mode: Continuous,
            unfocused_mode: Continuous,
        })
        .add_plugins((
            DefaultPlugins,
            CliConfigPlugin,
            RepliconPlugins,
            GamePlugin,
            {% if egui_inspector -%}
            #[cfg(feature = "dev")]
            EguiPlugin {
                enable_multipass_for_primary_context: true
            },
            #[cfg(feature = "dev")]
            WorldInspectorPlugin::new(),
            {%- endif %}
        ))
        .run();
    Ok(())
}
