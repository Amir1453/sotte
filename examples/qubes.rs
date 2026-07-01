use core::f64::consts::PI;

use sotte::{AabbBuilder, ImageRenderer, Material, QuadBuilder, SceneBuilder};
use sotte::{RenderConfig, SceneConfig, render_config, scene_config};

render_config!(RConf, width: 512, height: 512, samples: 100);
scene_config!(SConf);

fn main() {
    let mut scene = SceneBuilder::new()
        .camera_center(0., 0., 55.)
        .light_position(0., 30., 0.)
        .light_intensity(1E7)
        .fov(60. * PI / 180.)
        .gamma(2.2)
        .max_light_bounce(5)
        .build();

    let white = scene.add_material(Material::lambertian([0.8, 0.8, 0.8]));
    let green = scene.add_material(Material::lambertian([0.12, 0.45, 0.15]));
    let red = scene.add_material(Material::lambertian([0.65, 0.05, 0.05]));

    let gold = scene.add_material(Material::lambertian([0.85, 0.70, 0.20]));
    let blue = scene.add_material(Material::lambertian([0.20, 0.35, 0.80]));
    let purple = scene.add_material(Material::lambertian([0.55, 0.25, 0.75]));

    let floor = QuadBuilder::new()
        .corner(-200.0, -40.0, -200.0)
        .u(0.0, 0.0, 400.0)
        .v(400.0, 0.0, 0.0)
        .material(white)
        .build();

    let ceiling = QuadBuilder::new()
        .corner(-200.0, 40.0, -200.0)
        .u(400.0, 0.0, 0.0)
        .v(0.0, 0.0, 400.0)
        .material(white)
        .build();

    let left_wall = QuadBuilder::new()
        .corner(-40.0, -200.0, -200.0)
        .u(0.0, 400.0, 0.0)
        .v(0.0, 0.0, 400.0)
        .material(green)
        .build();

    let right_wall = QuadBuilder::new()
        .corner(40.0, -200.0, -200.0)
        .u(0.0, 0.0, 400.0)
        .v(0.0, 400.0, 0.0)
        .material(red)
        .build();

    let front_wall = QuadBuilder::new()
        .corner(-200.0, -200.0, -70.0)
        .u(400.0, 0.0, 0.0)
        .v(0.0, 400.0, 0.0)
        .material(white)
        .build();

    let back_wall = QuadBuilder::new()
        .corner(-200.0, -200.0, 70.0)
        .u(400.0, 0.0, 0.0)
        .v(0.0, 400.0, 0.0)
        .material(white)
        .build();

    scene.add_object(floor);
    scene.add_object(ceiling);
    scene.add_object(left_wall);
    scene.add_object(right_wall);
    scene.add_object(front_wall);
    scene.add_object(back_wall);

    // Generates a pyramid, by ChatGPT 5.5

    let box_size = 8.0;
    let gap = 0.01;
    let step = box_size + gap;

    let mut add_box = |cx: f64, cy: f64, cz: f64, material| {
        let h = box_size * 0.5;

        scene.add_object(
            AabbBuilder::new()
                .min(cx - h, cy - h, cz - h)
                .max(cx + h, cy + h, cz + h)
                .material(material)
                .build(),
        );
    };

    let base_y = -40.0 + box_size * 0.5;
    let center_z = -40.0;

    for ix in 0..4 {
        for iz in 0..4 {
            let x = (ix as f64 - 1.5) * step;
            let z = center_z + (iz as f64 - 1.5) * step;

            let material = match (ix + iz) % 3 {
                0 => gold,
                1 => blue,
                _ => purple,
            };

            add_box(x, base_y, z, material);
        }
    }

    for ix in 0..3 {
        for iz in 0..3 {
            let x = (ix as f64 - 1.0) * step;
            let z = center_z + (iz as f64 - 1.0) * step;

            let material = match (ix + iz + 1) % 3 {
                0 => gold,
                1 => blue,
                _ => purple,
            };

            add_box(x, base_y + box_size, z, material);
        }
    }

    for ix in 0..2 {
        for iz in 0..2 {
            let x = (ix as f64 - 0.5) * step;
            let z = center_z + (iz as f64 - 0.5) * step;

            let material = match (ix + iz + 2) % 3 {
                0 => gold,
                1 => blue,
                _ => purple,
            };

            add_box(x, base_y + 2.0 * box_size, z, material);
        }
    }

    add_box(0.0, base_y + 3.0 * box_size, center_z, gold);

    let img = ImageRenderer::render::<RConf, SConf, { RConf::IMAGE_SIZE }>(scene);

    image::save_buffer(
        "render.png",
        &img,
        RConf::WIDTH as u32,
        RConf::HEIGHT as u32,
        image::ExtendedColorType::Rgb8,
    )
    .unwrap();
}
