use std::time::Duration;

use super::*;

pub const BARREL_LEN: f32 = 2.0;
pub const BARREL_RADIE: f32 = 0.2;
pub const BODY_RADIE: f32 = 0.5;

pub fn turret_plugin(app: &mut App) {
    app.add_systems(Update, tick_and_fire_turret);
}

#[derive(Component)]
pub struct Turret {
    pub rate_of_fire: Timer,
    pub firing: bool,
}

#[cfg_attr(feature = "dev_native", hot)]
fn tick_and_fire_turret(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut turrets: Query<(&mut Turret, &GlobalTransform)>,
) {
    for (mut turret, transform) in turrets.iter_mut() {
        turret.rate_of_fire.tick(time.delta());
        if turret.rate_of_fire.finished() && turret.firing {
            turret.rate_of_fire.reset();
            let forward = transform.forward();
            let bullet_spawnpoint = transform.translation() + (BARREL_LEN + 0.5) * forward;
            commands.spawn(bullet::bullet(
                &mut meshes,
                &mut materials,
                forward,
                50.,
                bullet_spawnpoint,
            ));
        }
    }
}

pub fn turret(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
) -> impl Bundle {
    let timer = Timer::new(Duration::from_millis(1000), TimerMode::Once);

    (
        Name::new("Turret Body"),
        Turret {
            rate_of_fire: timer,
            firing: false,
        },
        Mesh3d(meshes.add(Sphere::new(BODY_RADIE))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(BLACK))),
        Transform::from_translation(pos),
        children![(
            Transform::from_rotation(Quat::from_rotation_x(-90f32.to_radians())),
            children![(
                Name::new("Turret Barrel"),
                Mesh3d(meshes.add(Cylinder::new(BARREL_RADIE, BARREL_LEN))),
                MeshMaterial3d(materials.add(StandardMaterial::from_color(GRAY))),
                // Transform::from_translation(Vec3::ZERO),
                Transform::from_translation(Vec3::Y * BARREL_LEN / 2.)
            )]
        )],
    )
}
