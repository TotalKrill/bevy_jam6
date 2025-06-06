use super::*;
use crate::ReplaceOnHotreload;
use avian3d::prelude::{ColliderConstructor, Friction, RigidBody};
use bevy::color::palettes::tailwind::{AMBER_800, GREEN_400};
use bevy::math::Affine2;
use bevy::render::mesh::VertexAttributeValues;
use noise::{BasicMulti, NoiseFn, Perlin};

#[derive(Component)]
pub struct Ground;

const SEED: u32 = 1134;
const TERRAIN_HEIGHT: f32 = 20.;

const PLANE_X_SIZE: f32 = 300.;
const PLANE_Z_SIZE: f32 = 300.;
const PLANE_SUB_DIVISION_COUNT: u32 = 200;

fn create_plane() -> Mesh {
    Mesh::from(
        Plane3d::default()
            .mesh()
            .size(PLANE_X_SIZE, PLANE_Z_SIZE)
            .subdivisions(PLANE_SUB_DIVISION_COUNT),
    )
}

fn create_terrain(mut terrain: Mesh) -> Mesh {
    // TODO We can modify the noise type
    let noise = BasicMulti::<Perlin>::new(SEED);

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
    world_assets: Res<WorldAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> impl Bundle {
    let plane = create_plane();
    let terrain = create_terrain(plane);

    let grass = world_assets.ground.clone();

    (
        ReplaceOnHotreload,
        ColliderConstructor::TrimeshFromMesh,
        Mesh3d(meshes.add(terrain)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(grass.clone()),
            uv_transform: Affine2::from_scale(Vec2::new(2., 3.)),
            ..default()
        })),
        // TODO Where should we spawn the tractor?
        Transform::from_xyz(0., -2., 0.),
        RigidBody::Static,
        Friction::new(1.0),
        Ground,
        Name::new("Ground"),
    )
}
