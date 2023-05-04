use ambient_api::{
    components::core::{
        app::main_scene,
        game_objects::player_camera,
        primitives::sphere_radius,
        transform::{lookat_center, rotation, translation, scale},
        rendering::{cast_shadows, color, fog_density, light_diffuse, sun, sky, water}
    },
    concepts::{make_perspective_infinite_reverse_camera, make_sphere, make_transformable},
    prelude::*,
};

// This code creates a field of spheres, and makes them move up and down, with a sun that rotates around them.
#[main]
pub async fn main() -> EventResult {
    
    // Create a new Entity
    Entity::new()
    // Add a perspective camera with infinite reverse z
    .with_merge(make_perspective_infinite_reverse_camera())
    // Add a player camera component
    .with_default(player_camera())
    // Add a translation component
    .with(translation(), Vec3::ONE * 30.)
    // Add a lookat_center component
    .with(lookat_center(), vec3(0., 0., -20.))
    // Spawn the entity
    .spawn();

    let mut entity_ids_array = Vec::new();
               
    for j in 0..30 {
        for i in 0..30 {
            let f = i as f32;
            let g = j as f32;   

            // create a new entity
            let id = Entity::new()
            // add the merge component and grab the sphere
            .with_merge(make_sphere())
            // add the cast shadows component
            .with_default(cast_shadows())
            // add the radius component
            .with(sphere_radius(), 1.)
            // add the translation component
            .with(translation(), vec3(1.5 * g, 1.5 * f, 1.))
            // add the color component
            .with(color(), vec4(1., f*0.1, g*0.1, 1.))
            // spawn the entity
            .spawn();
            
            entity_ids_array.push(id);
            
        }
    }

    // Create a new entity
    let sun = Entity::new()
    // Add a transform component to the entity
    .with_merge(make_transformable())
    // Add a default component with the Sun component
    .with_default(sun())
    // Add a default component with the Rotation component
    .with_default(rotation())
    // Add a default component with the MainScene component
    .with_default(main_scene())
    // Add a light component to the entity
    .with(light_diffuse(), Vec3::ONE)
    // Add a fog component to the entity
    .with(fog_density(), 0.)
    // Spawn the entity
    .spawn();

    // Create a new entity.
    Entity::new()
    // Make it transformable.
    .with_merge(make_transformable())
    // Set it to be a sky.
    .with_default(sky())
    // Spawn the entity.
    .spawn();

    // Create a new entity
    Entity::new()
    // Give the entity a transform
    .with_merge(make_transformable())
    // Give the entity a water component
    .with_default(water())
    // Scale the entity by 2000 in all directions
    .with(scale(), Vec3::ONE * 2000.)
    // Spawn the entity
    .spawn();

    on(event::FRAME, move |_| {
        // Get the rotation component of the sun entity
        let rot = entity::get_component(sun, rotation()).unwrap();
        // Set the rotation component of the sun entity
        entity::set_component(sun, rotation(), rot * Quat::from_rotation_y(0.01));
        
         
        for j in 0..30 {
            for i in 0..30 {
                // Get the position of the sphere
                let pos = entity::get_component(entity_ids_array[j*30 + i], translation()).unwrap();
                // Set the position of the sphere
                entity::set_component(entity_ids_array[j*30 + i], translation(), Vec3 { x: pos.x, y: pos.y, z: f32::cos(((j as f32*30.0 + i as f32)/5.0) * rot.y) });
            }
        }
        EventOk
    });

    EventOk
}
