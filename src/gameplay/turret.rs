use super::*;

const BARREL_LEN: f32 = 2.0;
const BARREL_RADIE: f32 = 0.2;
pub const BODY_RADIE: f32 = 0.5;

#[derive(Component)]
pub struct Turret;

#[derive(Event)]
pub struct FireEvent;

pub fn turret(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
) -> impl Bundle {
    (
        Name::new("Turret Body"),
        Turret,
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
        Observer::new(fire_bullet),
    )
}

fn fire_bullet(
    trigger: Trigger<FireEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    transforms: Query<&GlobalTransform>,
) {
    if let Ok(t) = transforms.get(trigger.target()) {
        let forward = t.forward();
        let bullet_spawnpoint = t.translation() + (BARREL_LEN + 0.5) * forward;
        commands.spawn(bullet::bullet(
            &mut meshes,
            &mut materials,
            forward,
            50.,
            bullet_spawnpoint,
        ));
    }
}
