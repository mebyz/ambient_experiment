use ambient_api::{
    components::core::{
        game_objects::player_camera,
        primitives::sphere_radius,
        transform::{lookat_center, translation},
        rendering::{cast_shadows, color}
    },
    concepts::{make_perspective_infinite_reverse_camera, make_sphere},
    prelude::*,
};

#[main]
pub async fn main() -> EventResult {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with_default(player_camera())
        .with(translation(), Vec3::ONE * 20.)
        .with(lookat_center(), vec3(0., 0., -20.))
        .spawn();

    for j in 0..10 {
        for i in 0..10 {
            let f = i as f32;
            let g = j as f32;
            Entity::new()
            .with_merge(make_sphere())
            .with_default(cast_shadows())
            .with(sphere_radius(), 1.)
            .with(translation(), vec3(2.0 * g, 2.0 * f, 1.))
            .with(color(), vec4(1., f*0.1, g*0.1, 1.))
            .spawn();
        }
    }


    println!("Hello, Ambient!");

    EventOk
}
