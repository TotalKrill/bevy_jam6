use super::*;
use avian3d::prelude::{ColliderConstructor, Friction, RigidBody};
use bevy::color::palettes::tailwind::{AMBER_800, GREEN_400};
use bevy::math::Affine2;
use bevy::math::sampling::UniformMeshSampler;
use bevy::render::mesh::VertexAttributeValues;
use noise::{BasicMulti, NoiseFn, Perlin};
use rand::prelude::Distribution;
use rand_chacha::rand_core::SeedableRng;

#[derive(Component)]
pub struct Ground;

#[derive(Resource, Asset, Clone, Reflect)]
pub struct LevelAssets {
    tree: Handle<Scene>,
    rock: Handle<Scene>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets: &AssetServer = world.resource::<AssetServer>();
        let tree =
            assets.load(GltfAssetLabel::Scene(0).from_asset("models/tree/harmless_tree.glb"));
        let rock = assets.load(GltfAssetLabel::Scene(0).from_asset("models/tree/rock.glb"));
        Self { tree, rock }
    }
}

pub fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}

const TERRAIN_SEED: u32 = 1135;
pub const TERRAIN_HEIGHT: f32 = 40.;
pub const PLANE_X_SIZE: f32 = 400.;
pub const PLANE_Z_SIZE: f32 = 400.;
const PLANE_SUB_DIVISION_COUNT: u32 = 20;

fn create_plane() -> Mesh {
    Mesh::from(
        Plane3d::default()
            .mesh()
            .size(PLANE_X_SIZE, PLANE_Z_SIZE)
            .subdivisions(PLANE_SUB_DIVISION_COUNT),
    )
}

fn create_terrain(mut terrain: Mesh, seed: u32) -> Mesh {
    // TODO We can modify the noise type
    let noise = BasicMulti::<Perlin>::new(seed);

    if let Some(VertexAttributeValues::Float32x3(positions)) =
        terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for pos in positions.iter_mut() {
            pos[1] = noise.get([
                // TODO We can modify 300 the change the level
                pos[0] as f64 / 300.,
                pos[2] as f64 / 300.,
            ]) as f32
                * TERRAIN_HEIGHT;
        }

        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[_, g, _]| {
                let g = *g / TERRAIN_HEIGHT * 2.;

                if g > 0.8 {
                    (Color::LinearRgba(LinearRgba {
                        red: 20.,
                        green: 20.,
                        blue: 20.,
                        alpha: 1.,
                    }))
                    .to_linear()
                    .to_f32_array()
                } else if g > 0.3 {
                    Color::from(AMBER_800).to_linear().to_f32_array()
                } else if g < -0.8 {
                    Color::BLACK.to_linear().to_f32_array()
                } else {
                    Color::from(GREEN_400).to_linear().to_f32_array()
                }
            })
            .collect();
        terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

        terrain.compute_normals();
    }

    terrain
}

pub fn level(
    commands: &mut Commands,
    world_assets: Res<WorldAssets>,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    level_assets: &LevelAssets,
) {
    let plane = create_plane();
    let terrain = create_terrain(plane, TERRAIN_SEED);

    const LEVEL_OFFSET: f32 = -2.0;
    const EDGE_START: f32 = 140.;

    const DETAILS_SEED: u64 = 2;
    let mut seeded_rng = rand_chacha::ChaCha8Rng::seed_from_u64(DETAILS_SEED);
    let distribution = UniformMeshSampler::try_new(terrain.triangles().unwrap()).unwrap();
    // Add sample points as children of the sphere:

    const WALL_START: f32 = EDGE_START + 2.;
    let walls = [
        (WALL_START, 0.),
        (-WALL_START, 0.),
        (0., WALL_START),
        (0., -WALL_START),
    ];

    for (x, z) in walls {
        commands.spawn((
            RigidBody::Static,
            Collider::half_space(Vec3::new(-x, 0., -z)),
            Transform::from_translation(Vec3::new(x, 0., z)),
        ));
    }

    for position in distribution.sample_iter(&mut seeded_rng).take(7000) {
        if position.x.abs() > EDGE_START || position.z.abs() > EDGE_START {
            let mut position = position;
            position.y += LEVEL_OFFSET;

            commands.spawn((
                // ReplaceOnHotreload,
                Name::new("StaticTree"),
                SceneRoot(level_assets.tree.clone()),
                Transform {
                    translation: position,
                    scale: Vec3::splat(4.),
                    ..Default::default()
                },
            ));
        }
    }

    let distribution = UniformMeshSampler::try_new(terrain.triangles().unwrap()).unwrap();
    for position in distribution.sample_iter(&mut seeded_rng).take(25) {
        if position.x.abs() < EDGE_START || position.z.abs() < EDGE_START {
            let mut position = position;
            position.y += LEVEL_OFFSET;

            commands.spawn((
                // ReplaceOnHotreload,
                Name::new("Rock"),
                SceneRoot(level_assets.rock.clone()),
                Collider::sphere(0.8),
                RigidBody::Static,
                Transform {
                    translation: position,
                    scale: Vec3::splat(4.),
                    rotation: rand::random(),
                },
            ));
        }
    }

    let grass = world_assets.ground.clone();
    let material = StandardMaterial {
        base_color_texture: Some(grass.clone()),
        uv_transform: Affine2::from_scale(Vec2::new(2., 3.)),
        reflectance: 0.05,
        ..default()
    };

    commands.spawn((
        ColliderConstructor::TrimeshFromMesh,
        Mesh3d(meshes.add(terrain)),
        MeshMaterial3d(materials.add(material)),
        // TODO Where should we spawn the tractor?
        Transform::from_xyz(0., LEVEL_OFFSET, 0.),
        RigidBody::Static,
        Friction::new(1.0),
        Ground,
        Name::new("Ground"),
    ));
}
