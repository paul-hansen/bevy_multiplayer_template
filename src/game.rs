use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

pub struct GamePlugin;

/// When this Component is inserted into an entity, a sprite of Ferris the crab will be attached.
#[derive(Component, Serialize, Deserialize)]
#[require(Replicated)]
pub struct Ferris;

/// Identifies which client/player controls the box.
///
/// Points to client entity. Used to apply movement to the correct box.
///
/// It's not replicated and present only on server or singleplayer.
///
/// Anything with this component will be despawned when the related client is disconnected.
#[derive(Component, Clone, Copy, Deref)]
pub struct NetOwner(Entity);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .replicate::<Ferris>()
            .replicate_group::<(Ferris, Transform)>()
            .add_client_trigger::<MoveBox>(Channel::Ordered)
            .add_observer(insert_ferris)
            .add_observer(spawn_clients)
            .add_observer(despawn_clients)
            .add_observer(apply_movement)
            .add_systems(Update, read_input);
    }
}

#[derive(Deserialize, Deref, Event, Serialize)]
struct MoveBox(Vec2);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Mutates [`Transform`] based on [`MoveBox`] events.
///
/// Fast-paced games usually you don't want to wait until server send a position back because of the latency.
/// But this example just demonstrates simple replication concept.
fn apply_movement(
    trigger: Trigger<FromClient<MoveBox>>,
    time: Res<Time>,
    mut boxes: Query<(&NetOwner, &mut Transform)>,
) {
    const MOVE_SPEED: f32 = 300.0;

    // Find the sender entity. We don't include the entity as a trigger target to save traffic, since the server knows
    // which entity to apply the input to. We could have a resource that maps connected clients to controlled entities,
    // but we didn't implement it for the sake of simplicity.
    let (_, mut transform) = boxes
        .iter_mut()
        .find(|&(&owner, _)| *owner == trigger.client_entity)
        .unwrap_or_else(|| panic!("`{}` should be connected", trigger.client_entity));

    transform.translation += (*trigger.event * time.delta_secs() * MOVE_SPEED).extend(0.0);
}

/// Reads player inputs and sends [`MoveDirection`] events.
fn read_input(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        commands.client_trigger(MoveBox(direction.normalize_or_zero()));
    }
}

/// Inserts the player visuals
fn insert_ferris(
    trigger: Trigger<OnInsert, Ferris>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("spawning ferris sprite for `{}`", trigger.target());
    commands.entity(trigger.target()).insert(Sprite {
        image: asset_server.load("rustacean-orig-noshadow.png"),
        ..default()
    });
}

/// Runs on the server when a new client is added.
fn spawn_clients(trigger: Trigger<OnAdd, ConnectedClient>, mut commands: Commands) {
    info!("spawning player for `{}`", trigger.target());
    spawn_player(&mut commands, trigger.target());
}

fn despawn_clients(
    trigger: Trigger<OnRemove, ConnectedClient>,
    mut commands: Commands,
    boxes: Query<(Entity, &NetOwner)>,
) {
    let (entity, _) = boxes
        .iter()
        .find(|&(_, &owner)| *owner == trigger.target())
        .expect("all clients should have entities");
    commands.entity(entity).despawn();
}

/// called by the cli module when the server spawn's it's player or by the game module when a
/// client is connected.
pub fn spawn_player(commands: &mut Commands, owner: Entity) {
    commands.spawn((Ferris, NetOwner(owner)));
}
