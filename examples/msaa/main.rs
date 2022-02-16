use dotrix::assets::Texture;
use dotrix::math::Vec3;
use dotrix::prelude::*;
use dotrix::primitives::Cube;
use dotrix::renderer::{Antialiasing, Pipeline};
use dotrix::{camera, egui, input, overlay, pbr};
use dotrix::{Assets, Camera, Color, Frame, Input, Renderer, Transform, World};

fn main() {
    Dotrix::application("Dotrix: MSAA")
        .with(System::from(startup))
        .with(System::from(camera::control))
        .with(System::from(ui))
        .with(pbr::extension)
        .with(overlay::extension)
        .with(egui::extension)
        .run();
}

fn startup(mut camera: Mut<Camera>, mut world: Mut<World>, mut assets: Mut<Assets>) {
    // setup camera
    camera.target = [0., 0., 0.].into();
    camera.distance = 3.0;
    camera.tilt = 0.0;

    // import material textures
    assets.import("assets/space-crate/space-crate1-albedo.png");
    assets.import("assets/space-crate/space-crate1-ao.png");
    assets.import("assets/space-crate/space-crate1-height.png");
    assets.import("assets/space-crate/space-crate1-metallic.png");
    assets.import("assets/space-crate/space-crate1-normal-ogl.png");
    assets.import("assets/space-crate/space-crate1-roughness.png");

    let albedo = assets.register::<Texture>("space-crate1-albedo");
    let ao = assets.register::<Texture>("space-crate1-ao");
    let heightmap = assets.register::<Texture>("space-crate1-height");
    let metallic = assets.register::<Texture>("space-crate1-metallic");
    let normal_ogl = assets.register::<Texture>("space-crate1-normal-ogl");
    let roughness = assets.register::<Texture>("space-crate1-roughness");

    // generate simple cube mesh
    let mesh = Cube::builder(1.0)
        .with_positions()
        .with_normals()
        .with_tangents_bitangents()
        .with_uvs(Cube::default_uvs())
        .mesh();

    // store the mesh
    let mesh = assets.store(mesh);

    // spawn the crate
    world.spawn([(
        pbr::Model {
            mesh,
            ..Default::default()
        },
        pbr::Material {
            texture: albedo,
            roughness_texture: roughness,
            metallic_texture: metallic,
            normal_texture: normal_ogl,
            ao_texture: ao,
            heightmap,
            ..Default::default()
        },
        Transform::default(),
        Pipeline::render(Id::default()),
    )]);

    // Spawn lights
    world.spawn([
        (pbr::Light::Directional {
            direction: Vec3::new(1.0, 1.0, 10.0),
            color: Color::rgb(1.0, 1.0, 1.0),
            intensity: 3.0,
            enabled: true,
        },),
        (pbr::Light::Directional {
            direction: Vec3::new(-1.0, -1.0, -10.0),
            color: Color::rgb(1.0, 1.0, 1.0),
            intensity: 3.0,
            enabled: true,
        },),
    ]);
}

fn ui(
    overlay: Const<overlay::Overlay>,
    frame: Const<Frame>,
    input: Const<Input>,
    mut renderer: Mut<Renderer>,
) {
    let egui_overlay = overlay
        .get::<egui::Egui>()
        .expect("Egui overlay must be added on startup");

    egui::Area::new("FPS counter")
        .fixed_pos(egui::pos2(16.0, 16.0))
        .show(&egui_overlay.ctx, |ui| {
            ui.colored_label(
                egui::Rgba::from_rgb(255.0, 255.0, 255.0),
                format!(
                    "These PBR material was created by FreePBR.com\n\
                    Press SPACE to toggle antialiasing\n\
                    FPS: {:.1}; ",
                    frame.fps()
                ),
            );
        });

    if input.button_state(input::Button::Key(input::KeyCode::Space))
        == Some(input::State::Activated)
    {
        renderer.antialiasing = if renderer.antialiasing == Antialiasing::Enabled {
            Antialiasing::Disabled
        } else {
            Antialiasing::Enabled
        };
    }
}
