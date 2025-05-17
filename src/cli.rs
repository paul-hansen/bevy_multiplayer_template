use crate::game::spawn_player;
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon_renet2::{
    netcode::{
        ClientAuthentication, NativeSocket, NetcodeClientTransport, NetcodeServerTransport,
        ServerAuthentication, ServerSetupConfig,
    },
    renet2::{ConnectionConfig, RenetClient, RenetServer},
    RenetChannelsExt, RepliconRenetPlugins,
};
use clap::{Parser, Subcommand};
use std::net::IpAddr;
use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Option<LaunchCommand>,
}

#[derive(Subcommand, Clone, Debug)]
enum LaunchCommand {
    Join {
        #[arg(long, default_value = "127.0.0.1")]
        ip: IpAddr,
        #[arg(long, default_value = "{{default_port}}")]
        port: u16,
    },
    Host {
        #[arg(long, default_value = "127.0.0.1")]
        ip: IpAddr,
        #[arg(long, default_value = "{{default_port}}")]
        port: u16,
    },
}

pub struct CliConfigPlugin;

impl Plugin for CliConfigPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(RepliconRenetPlugins);
        app.add_systems(Startup, setup.map(Result::unwrap));
    }
}

fn setup(
    mut commands: Commands,
    channels: Res<RepliconChannels>,
) -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();
    match args.command {
        None => {
            spawn_player(&mut commands, SERVER);
        }
        Some(LaunchCommand::Host { ip, port }) => {
            info!("Hosting on {ip}:{port}");
            let server = RenetServer::new(ConnectionConfig::from_channels(
                channels.server_configs(),
                channels.client_configs(),
            ));

            let current_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default();
            let public_addr = SocketAddr::new(ip, port);
            let socket = UdpSocket::bind(public_addr)?;
            let server_config = ServerSetupConfig {
                current_time,
                max_clients: 10,
                protocol_id: gen_protocol_id_from_crate_version(),
                authentication: ServerAuthentication::Unsecure,
                socket_addresses: vec![vec![public_addr]],
            };
            let transport =
                NetcodeServerTransport::new(server_config, NativeSocket::new(socket).unwrap())?;

            commands.insert_resource(server);
            commands.insert_resource(transport);

            commands.spawn((
                Text::new("Server"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            spawn_player(&mut commands, SERVER);
        }
        Some(LaunchCommand::Join { ip, port }) => {
            info!("connecting to {ip}:{port}");
            let client = RenetClient::new(
                ConnectionConfig::from_channels(
                    channels.server_configs(),
                    channels.client_configs(),
                ),
                false,
            );

            let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
            let client_id = current_time.as_millis() as u64;
            let server_addr = SocketAddr::new(ip, port);
            let socket = UdpSocket::bind((ip, 0))?;
            let authentication = ClientAuthentication::Unsecure {
                client_id,
                protocol_id: gen_protocol_id_from_crate_version(),
                socket_id: 0,
                server_addr,
                user_data: None,
            };
            let transport = NetcodeClientTransport::new(
                current_time,
                authentication,
                NativeSocket::new(socket).unwrap(),
            )?;

            commands.insert_resource(client);
            commands.insert_resource(transport);

            commands.spawn((
                Text(format!("Client: {client_id}")),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor::WHITE,
            ));
        }
    }
    Ok(())
}

/// Generates a protocol id by hashing the crate name and version number.
fn gen_protocol_id_from_crate_version() -> u64 {
    use bevy::platform::hash::*;
    use std::hash::{BuildHasher, Hasher};
    let s = FixedState::with_seed(65431287124274784);
    let mut hasher = s.build_hasher();
    hasher.write(env!("CARGO_PKG_NAME").as_bytes());
    hasher.write(env!("CARGO_PKG_VERSION").as_bytes());
    hasher.finish()
}
