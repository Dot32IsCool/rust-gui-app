use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(HelloPlugin)
        .add_startup_system(setup)
        .insert_resource(RectTimer(Timer::from_seconds(0.001, true)))
        .add_system(spawn_rects)
        .run();
}

// #[derive(Component)]
// struct Person;

// #[derive(Component)]
// struct Name(String);

// fn add_people(mut commands: Commands) {
//     commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
//     commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
//     commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
// }

// struct GreetTimer(Timer);

// fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in query.iter() {
//             println!("hello {}!", name.0);
//         }
//     }
// }

// pub struct HelloPlugin;
// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut App) {
//         app
//             .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
//             .add_startup_system(add_people)
//             .add_system(greet_people);
//     }
// }

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            // color: Color::rgb(0.25, 0.25, 0.75),
            color: Color::hsl(220.0, 0.5, 0.1),
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}

struct RectTimer(Timer);

fn spawn_rects(time: Res<Time>, mut timer: ResMut<RectTimer>, mut commands: Commands) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                // color: Color::rgb(0.25, 0.25, 0.75),
                color: Color::hsl(rand::thread_rng().gen_range(0, 360) as f32, 0.5, 0.5),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(rand::thread_rng().gen_range(-640, 640) as f32, rand::thread_rng().gen_range(-360, 360) as f32, 0.0),
                ..Default::default()    
            },
            ..Default::default()
        });
    }
}
