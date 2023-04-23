use ambient_api::{
    components::core::{
        app::main_scene,
        game_objects::player_camera,
        primitives::sphere_radius,
        transform::{lookat_center, rotation, translation},
        rendering::{cast_shadows, color, fog_density, light_diffuse, sun}
    },
    concepts::{make_perspective_infinite_reverse_camera, make_sphere, make_transformable},
    prelude::*,
};
use arrayvec::ArrayVec; // 0.5.1

#[main]
pub async fn main() -> EventResult {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with_default(player_camera())
        .with(translation(), Vec3::ONE * 20.)
        .with(lookat_center(), vec3(0., 0., -20.))
        .spawn();

                let mut EntityIdsArray = ArrayVec::<[EntityId; 100]>::new();
        
//    let EntityIdsArray : [EntityId; 100] = [new EntityId(); 100];

    for j in 0..10 {
        for i in 0..10 {
            let f = i as f32;
            let g = j as f32;   
            
            EntityIdsArray.push(Entity::new()
            .with_merge(make_sphere())
            .with_default(cast_shadows())
            .with(sphere_radius(), 1.)
            .with(translation(), vec3(2.0 * g, 2.0 * f, 1.))
            .with(color(), vec4(1., f*0.1, g*0.1, 1.))
            .spawn());
            
        }
    }
    //EntityIdsArray.into_inner();


    println!("Hello, Ambient!");

    let sun = Entity::new()
        .with_merge(make_transformable())
        .with_default(sun())
        .with_default(rotation())
        .with_default(main_scene())
        .with(light_diffuse(), Vec3::ONE)
        .with(fog_density(), 0.)
        .spawn();

    on(event::FRAME, move |_| {
        let rot = entity::get_component(sun, rotation()).unwrap();
        entity::set_component(sun, rotation(), rot * Quat::from_rotation_y(0.01));
        
         
        for j in 0..10 {
            for i in 0..10 {
                let pos = entity::get_component(EntityIdsArray[j*10 + i], translation()).unwrap();
       
                entity::set_component(EntityIdsArray[j*10 + i], translation(), Vec3 { x: pos.x, y: pos.y, z: f32::cos((j*10 + i) as f32 * rot.y) });
            }
        }
        EventOk
    });

    EventOk
}
