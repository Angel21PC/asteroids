use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement.label("movement"));
    }
}

fn spawn_player(mut commands: Commands, mut asset_server: Res<AssetServer>) {
    let player_sprite: Entity = set_up_sprite(&mut commands, &mut asset_server);

    commands
        .entity(player_sprite)
        .insert(Name::new("Player"))
        .insert(Player { speed: 222.0 });
}

fn set_up_sprite(commands: &mut Commands, asset_server: &mut Res<AssetServer>) -> Entity {
    let sprite: Handle<Image> = asset_server.load("spr_player_0.png");
    return commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Option::Some(Vec2::new(64.0, 64.0)),
                ..Default::default()
            },
            texture: sprite,
            transform: Transform {
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
}

fn player_movement(
    mut query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (player, mut transform) in query.iter_mut() {
        let mut y_delta = 0.0;
        if keyboard.pressed(KeyCode::W) {
            y_delta += player.speed * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::S) {
            y_delta -= player.speed * time.delta_seconds();
        }

        let mut x_delta = 0.0;
        if keyboard.pressed(KeyCode::A) {
            x_delta -= player.speed * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::D) {
            x_delta += player.speed * time.delta_seconds();
        }

        let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);

        transform.translation = target;

        let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);

        transform.translation = target;

        println!("{:?}", transform.translation);
    }
}
