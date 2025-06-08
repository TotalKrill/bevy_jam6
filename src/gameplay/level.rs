use super::*;
use crate::gameplay::apple::Apple;
use crate::gameplay::bullet::{Bullet, BulletSplitEvent};
use crate::gameplay::health::DamageEvent;
use crate::gameplay::seed::{SeedAssets, SeedSpawnEvent};
use crate::gameplay::tractor::{LeftWheel, LeftWheels, RightWheels, Tractor};
use crate::{PausableSystems, ReplaceOnHotreload};
use avian3d::prelude::{ColliderConstructor, Friction, RigidBody};
use bevy::color::palettes::tailwind::{AMBER_800, GREEN_400};
use bevy::math::Affine2;
use bevy::render::mesh::VertexAttributeValues;
use noise::{BasicMulti, NoiseFn, Perlin};
use std::collections::HashSet;

#[derive(Resource)]
pub struct LevelManager {
    pub noise: BasicMulti<Perlin>,
    pub grid: [[usize; LevelManager::MAX_GRID_SIZE]; LevelManager::MAX_GRID_SIZE],
}

impl LevelManager {
    const SEED: u32 = 1135;
    pub(crate) const TERRAIN_HEIGHT: f32 = 40.;
    const MAX_GRID_SIZE: usize = 128;
    const PLANE_X_SIZE: f32 = 50.;
    const PLANE_Z_SIZE: f32 = 50.;
    const PLANE_SUB_DIVISION_COUNT: u32 = 20;
    pub fn new() -> Self {
        Self {
            noise: BasicMulti::<Perlin>::new(LevelManager::SEED),
            grid: [[0; LevelManager::MAX_GRID_SIZE]; LevelManager::MAX_GRID_SIZE],
        }
    }

    fn create_plane() -> Mesh {
        Mesh::from(
            Plane3d::default()
                .mesh()
                .size(LevelManager::PLANE_X_SIZE, LevelManager::PLANE_Z_SIZE)
                .subdivisions(LevelManager::PLANE_SUB_DIVISION_COUNT),
        )
    }

    fn create_terrain(&self, mut terrain: Mesh) -> Mesh {
        if let Some(VertexAttributeValues::Float32x3(positions)) =
            terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            for pos in positions.iter_mut() {
                pos[1] = self.noise.get([pos[0] as f64 / 300., pos[2] as f64 / 300.]) as f32
                    * LevelManager::TERRAIN_HEIGHT;
            }

            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[_, g, _]| {
                    let g = *g / LevelManager::TERRAIN_HEIGHT * 2.;

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

    pub fn create_grid(&mut self, x: usize, z: usize) -> Option<Mesh> {
        // let x = x + LevelManager::MAX_GRID_SIZE / 2;
        // let z = z + LevelManager::MAX_GRID_SIZE / 2;

        println!("x: {}, z: {}", x, z);
        println!("self.check_element(x, z) = {}", self.check_element(x, z));

        if self.check_element(x, z) {
            return None;
        }

        let plane = LevelManager::create_plane();
        let terrain = self.create_terrain(plane);

        self.set_element(x, z);

        println!("terrain: {}", self.check_element(x, z));

        Some(terrain)
    }

    fn check_element(&self, x: usize, z: usize) -> bool {
        self.grid[x][z] == 1
        // self.grid[x][z + LevelManager::MAX_GRID_SIZE / 2] == 1
    }
    fn set_element(&mut self, x: usize, z: usize) {
        self.grid[x][z] = 1
    }

    pub fn get_empty_grid_neighbours(&self, x: usize, z: usize) -> Vec<(usize, usize)> {

        println!("get_empty_grid_neighbours x: {}, z: {}", x, z);

        [
            (x - 1, z), // left
            (x + 1, z), // right
            (x, z - 1), // up
            (x, z + 1), // down
            // (x - 1, z - 1), // up
            // (x - 1, z + 1), // up
            // (x + 1, z + 1), // up
            // (x - 1, z - 1), // up
        ]
        .into_iter()
        .filter(|&(nx, nz)| !self.check_element(nx, nz))
        .collect()
    }
}

#[derive(Component)]
pub struct Ground {
    pub x: usize,
    pub z: usize,
}

impl Ground {

}

impl Ground {

    fn new_init(x: usize, z: usize) -> Self {
        Self {
            x: x + LevelManager::MAX_GRID_SIZE / 2,
            z: z + LevelManager::MAX_GRID_SIZE / 2
        }
    }
    pub fn new(x: usize, z: usize) -> Self {

        let g = Self {
            x,
            z
        };

        println!("Ground CREATED x: {}, z: {}", x, z);

        g
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(LevelManager::new());
    app.add_systems(SpawnScene, generate_new_grounds);
}

fn generate_new_grounds(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionStarted>,
    ground: Query<(Entity, &Ground)>,
    tractor: Single<(Entity, &LeftWheels, &RightWheels), With<Tractor>>,
    mut level_manager: ResMut<LevelManager>,
    world_assets: Res<WorldAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (tractor, left, right) = *tractor;

    let mut tractor_entities = HashSet::with_capacity(5);
    tractor_entities.insert(tractor);
    tractor_entities.extend(left.collection());
    tractor_entities.extend(right.collection());

    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        for (ground_candidate, tractor_candidate) in [(*entity1, *entity2), (*entity2, *entity1)] {
            if tractor_entities.contains(&tractor_candidate) {
                if let Ok((ground_entity, ground)) = ground.get(ground_candidate) {
                    println!("ground_entity found = {}", ground_candidate);
                    println!("tractor_candidate found = {}", tractor_candidate);

                    let grids = level_manager.get_empty_grid_neighbours(ground.x, ground.z);


                    for (x, z) in grids {

                        println!("neightbor grid x={}, z={}", x, z);

                        let grass = world_assets.ground.clone();
                        let material = StandardMaterial {
                            base_color_texture: Some(grass.clone()),
                            uv_transform: Affine2::from_scale(Vec2::new(2., 3.)),
                            reflectance: 0.05,
                            ..default()
                        };

                        if let Some(mesh) = level_manager.create_grid(x, z) {

                            // let x =  ;
                            // let z = ;

                            println!("create grid x={}, z={}",
                                     0. + (x as f32 - (LevelManager::MAX_GRID_SIZE / 2) as f32) * LevelManager::PLANE_X_SIZE,
                                     0. + (z as f32 - (LevelManager::MAX_GRID_SIZE / 2) as f32) as f32 * LevelManager::PLANE_Z_SIZE
                            );

                            commands.spawn((
                                ColliderConstructor::TrimeshFromMesh,
                                Mesh3d(meshes.add(mesh)),
                                MeshMaterial3d(materials.add(material)),
                                Transform::from_xyz(
                                    0. + (x as f32 - (LevelManager::MAX_GRID_SIZE / 2) as f32) * LevelManager::PLANE_X_SIZE,
                                    -2.,
                                    0. + (z as f32 - (LevelManager::MAX_GRID_SIZE / 2) as f32) as f32 * LevelManager::PLANE_Z_SIZE,
                                ),
                                RigidBody::Static,
                                Friction::new(1.0),
                                Ground::new(x, z),
                                Name::new("Ground"),
                            ));
                        }
                    }

                    break;
                }
            }
        }
    }
}

pub fn setup_level(
    mut commands: Commands,
    world_assets: Res<WorldAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut level_manager: ResMut<LevelManager>,
) {
    let grass = world_assets.ground.clone();
    let material = StandardMaterial {
        base_color_texture: Some(grass.clone()),
        uv_transform: Affine2::from_scale(Vec2::new(2., 3.)),
        reflectance: 0.05,
        ..default()
    };

    let (x, z) = (0, 0);

    if let Some(mesh) = level_manager.create_grid(x, z) {
        commands.spawn((
            ColliderConstructor::TrimeshFromMesh,
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(material)),
            Transform::from_xyz(0., -2., 0.),
            RigidBody::Static,
            Friction::new(1.0),
            Ground::new_init(x, z),
            Name::new("Ground"),
        ));
    }
}
