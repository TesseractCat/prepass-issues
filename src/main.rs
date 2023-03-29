use bevy::{
    core_pipeline::{
        prepass::{NormalPrepass, DepthPrepass}
    },
    pbr::{NotShadowCaster, PbrPlugin},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    reflect::TypeUuid,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(PbrPlugin {
            ..default()
        }))
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,

    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 3., 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // To enable the prepass you need to add the components associated with the ones you need
        // This will write the depth buffer to a texture that you can use in the main pass
        DepthPrepass,
        // This will generate a texture containing world normals (with normal maps applied)
        NormalPrepass,
    ));

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: std_materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(CustomMaterial {
            color: Color::WHITE,
            color_texture: Some(asset_server.load("test.png")),
        }),
        transform: Transform::from_xyz(1.0, 0.5, 0.0),
        ..default()
    });
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f695fdae-d598-45ab-8244-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "custom_material.wgsl".into()
    }
}
