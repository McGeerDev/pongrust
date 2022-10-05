use bevy::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;

// Colors
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PLAYER_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

const PADDLE_SIZE: Vec3 = Vec3::new(20.0, 120.0, 0.0);

const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0,-50.0, 1.);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}


#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Ball;

fn setup (mut commands: Commands){
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());
    
    commands
        .spawn()
        .insert(Ball)
        .insert_bundle(SpriteBundle{
            transform: Transform{
                scale: BALL_SIZE,
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            ..default()
        })
    .insert(Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED));
    
}
