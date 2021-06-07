/*

Planning to have a system (not ecs system) where all
things with sprites have an up, down, left, right, hurt, dead, etc
sprite to simplify changing sprites

*/

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

 // Weird things
// Used to tell Bevy what mode we're in, allowing us to switch between gameplay and menus
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,         // Used for the titlescreen
    StartGame,   // Used once when transitioning from Menu to the Game this ensures that unpausing does fuck up the game
    InGame,     // Used to run the game loop, includes Player movement and Enemy AI
    Paused,    // Simply a pause screen
    GameOver, // I think you're not stupid.
}

// Used for general collision
enum Collider {
    Player,         // Assigned only to the player
    Enemy,         // Assigned to enemies the player can collide with
    Bullet,       // Assigned to bullets, used in conjunction with Faction to determine what they should hit.
    Environment, // Assigned to the environment. May not be neccesary idk I don't use this engine
}

// Used to tell things with no inherit Player/Enemy alliance what to not hit.
enum FactionEnum {
    Players,
    Enemies,
}
/* These two both have Enum in their name 
      to differentiate them from their 
            associated Components          */
#[derive(Copy, Clone)]            
enum DirectionEnum { // Self explanitory
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
enum EnemyAI {
    Chaser,
    Gunner,
}

// Entities
struct Player;
struct Enemy;
struct Bullet;

// Components
struct Controllable;        // Entities that can be moved with the movement function
struct Health { hp : i16 } // Health, this is the quintessential ECS Component, the obvious one.
struct Speed { speed : f32 }    // Speed, determines how quickly moving entities can move
struct Damage { damage : i16 } // Damage determines how much Health you reduce when attacking
struct Direction { direction : DirectionEnum }
struct Faction { faction : FactionEnum }
struct Shooter {
    bullet_sprite : Handle<ColorMaterial>,
    time_out : f32,
    max_time_out : f32,
}

struct Expire { time : f32 } // time in seconds to wait before despawning the associated entity
/* Give this to an Entity and fill it with another entity (with the Direction
 component) to spawn it whenever an entity "shoots" something */
// used for camera scrolling, to differentiate from other entities with Transform.
struct Scrolling;
  // Thanks Bit for the ideas!
 // This component holds every frame for an entity, pretty self explanitory from there
// (Fun Fact, Handle<T> means that these sprites are only loaded once! they're simply reused anywhere they're needed.)
struct SpriteFrames {
    up : Handle<ColorMaterial>,
    down : Handle<ColorMaterial>,
    left : Handle<ColorMaterial>,
    right : Handle<ColorMaterial>,
    dead : Handle<ColorMaterial>,
}

// Resources
#[derive(Default)]
struct ResizeStopper(bool);

 // God help you
// I mean, Systems.
fn setup_game(
    mut state : ResMut<State<AppState>>,
    mut commands : Commands,
    asset_server : Res<AssetServer>,
    mut materials : ResMut<Assets<ColorMaterial>>
) {
    let player_texture = asset_server.load("player/up.png"); // Load the player's sprite
    let enemy_texture = asset_server.load("enemies/chaser/up.png"); // Load the Chaser's sprite
    // Spawn the camera and give it the scrolling component so it moves up slowly
    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Scrolling);
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // Spawn the player with a Sprite
    commands.spawn_bundle(SpriteBundle {
        material : materials.add(player_texture.into()),
        transform : Transform::from_xyz(1.0, 1.0, 0.0),
        sprite : Sprite::new(Vec2::new(48.0, 48.0)),
        ..Default::default()
    })
        .insert(Player)
        // 100 Hit points
        .insert(Health {
            hp : 100
        })
        // 300 base speed
        .insert(Speed {
            speed : 300.0
        })
        // Give them frames for their different directions
        .insert(SpriteFrames {
            up : materials.add(asset_server.load("player/up.png").into()),
            down : materials.add(asset_server.load("player/down.png").into()),
            left : materials.add(asset_server.load("player/left.png").into()),
            right : materials.add(asset_server.load("player/right.png").into()),
            dead : materials.add(asset_server.load("player/dead.png").into()),
        })
        // Give the player a direction
        .insert(Direction {
            direction : DirectionEnum::Up
        })
        // Tells the game this entity can shoot
        .insert(Shooter {
            bullet_sprite : materials.add(asset_server.load("boolet.png").into()),
            max_time_out : 0.1,
            time_out : 0.0,
        })
        .insert(Collider::Player)
        /* 
        I *HAD* a fucking thing here with a bullet component so entities could all have unique bullets, but NOOOOOOOOOOOOOOOOO
        Bevy is fucking stupid and doesn't have a clear way to create an entity without spawning it
        I wanted to have a stored entity that could be spawned over n over but nah, fuck you.
        */

        // And let them be controlled.
        .insert(Controllable);

    
    commands.spawn_bundle(SpriteBundle {
        material : materials.add(enemy_texture.into()),
        transform : Transform::from_xyz(1.0, 1.0, 0.0),
        sprite : Sprite::new(Vec2::new(48.0, 48.0)),
        ..Default::default()
    })
        .insert(Enemy)
        .insert(EnemyAI::Chaser)
        .insert(Health {
            hp : 20
        })
        .insert(Speed {
            speed : 200.0
        })
        .insert(SpriteFrames {
            up : materials.add(asset_server.load("enemies/chaser/up.png").into()),
            down : materials.add(asset_server.load("enemies/chaser/down.png").into()),
            left : materials.add(asset_server.load("enemies/chaser/left.png").into()),
            right : materials.add(asset_server.load("enemies/chaser/right.png").into()),
            dead : materials.add(asset_server.load("enemies/chaser/dead.png").into()),
        })
        .insert(Direction {
            direction : DirectionEnum::Up
        })
        .insert(Collider::Enemy);
    
    // After the Game is prepared switch to in game mode. This runs the actual game loop.
    state.set(AppState::InGame).unwrap();
}

// TODO add cleaning function here to remove Game components
fn clean_game(

) {

}

// TODO add pausing
fn setup_pause(

) {

}

// TODO add more features to pause menu (I.E. buttons to return to menu)
fn pause(
    mut state : ResMut<State<AppState>>,
    input : Res<Input<KeyCode>>
) {
    if input.pressed(KeyCode::Escape) {
        state.set(AppState::InGame).unwrap();
    }
}

// TODO add cleaning function here to remove Pause components
fn clean_pause(

) {

}

// TODO add actual Stuff here
fn setup_menu(
    mut commands : Commands,
    asset_server : Res<AssetServer>
) {
    commands.spawn_bundle(UiCameraBundle::default());
}

// TODO add menu loop i.e. buttons
fn menu(
    mut state : ResMut<State<AppState>>,
) {
    
}

// TODO add cleaning function here to remove Menu components
fn clean_menu( 
) {

}

// Movement of Player controlled entities
fn movement(
    time : Res<Time>, // Time used for delta time (how many milliseconds are between frames)
    input : Res<Input<KeyCode>>, // Make a guess. used for input.
    // TIL there's QuerySets in bevy. https://bevy-cheatbook.github.io/cheatsheet.html#query-sets
    mut set : QuerySet<(
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ i GUESS everything that can shoot needs Direction, now!
        Query<(&Controllable, Option<&Health>, &mut Transform, &Speed, &mut Handle<ColorMaterial>, &SpriteFrames, &mut Direction)>,  /* Gets Controllable Entities 
        with optional Health, gets mutable Transform to make changes to position when moving and
        the Speed component to move at a speed above a blazing fast 1 pixel*/
        Query<&Transform, With<Scrolling>> /* Gets the Camera (The only thing with the scrolling component),
            used to make sure the Player is clamped to the bottom of the screen rather than the bottom of 
            the initial position.*/
    )>,
) {
    let camera_translate = set.q1_mut().single().unwrap().translation; // Only reason to get the camera
    for (_, health, mut transform, speed, mut sprite, sprite_frames, mut facing_direction) in set.q0_mut().iter_mut() {
        // Shamelessly stolen from an example :p
        // https://github.com/bevyengine/bevy/blob/cf221f9659127427c99d621b76c8085c4860e2ef/examples/ecs/state.rs
        let mut direction = Vec3::ZERO;
        if let Some(health) = health {
            // Dead things can't move.
            if health.hp <= 0 {
                continue;
            }
        }

        if input.pressed(KeyCode::Left) {
            facing_direction.direction = DirectionEnum::Left;
        } if input.pressed(KeyCode::Right) {
            facing_direction.direction = DirectionEnum::Right;
        } if input.pressed(KeyCode::Up) {
            facing_direction.direction = DirectionEnum::Up;
        } if input.pressed(KeyCode::Down) {
            facing_direction.direction = DirectionEnum::Down;
        }

        // Well maybe it's not dead. Or maybe it can't ever BE dead.
        if input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
            *sprite = sprite_frames.left.clone();
        }
        if input.pressed(KeyCode::Right) {
            direction.x += 1.0;
            *sprite = sprite_frames.right.clone();
        }
        // +Y = Up in bevy (for some reason)
        if input.pressed(KeyCode::Up) {
            direction.y += 1.0;
            *sprite = sprite_frames.up.clone();
        }
        // inversely, -Y = Down (shocker, I know!)
        if input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
            *sprite = sprite_frames.down.clone();
        }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * speed.speed * time.delta_seconds();
            transform.translation.x = transform.translation.x.min(276.0).max(-276.0);
        }
        // Even if the player isn't moving, clamp the y position properly (you can skip x since the camera never moves left/right)
        transform.translation.y = transform.translation.y.min(200.0 + camera_translate.y).max(-276.0 + camera_translate.y);
    }
}

fn controllable_shooting(
    time : Res<Time>,
    input : Res<Input<KeyCode>>,
    mut commands : Commands,
    mut query : Query<(&Transform, &Direction, &mut Shooter), With<Controllable>>,
) {
    // println!("{}", input.pressed(KeyCode::Z));
    if input.pressed(KeyCode::Z) {
        for (shooter_transform, direction, mut shooter) in query.iter_mut() {
            if shooter.time_out < shooter.max_time_out && time.delta_seconds() < shooter.time_out {
                shooter.time_out -= time.delta_seconds();
                continue;
            } else if time.delta_seconds() > shooter.time_out {
                shooter.time_out = shooter.max_time_out;
                continue;
            } 
            // Collapse this and never touch it
            let bullet = commands.spawn_bundle(SpriteBundle {
                material : shooter.bullet_sprite.clone(),
                sprite : Sprite::new(Vec2::new(16.0, 16.0)),
                ..Default::default()
            })
                .insert(Bullet)
                .insert(Faction {
                    faction : FactionEnum::Players
                })
                .insert(Damage {
                    damage : 10
                })
                .insert(Speed {
                    speed : 500.0
                })
                .insert(Expire {
                    time : 5.0
                })
                .insert(Direction {
                    direction : direction.direction
                })
                .insert(Collider::Bullet)
                .insert(Transform::from_matrix(
                    Mat4::from_scale_rotation_translation(
                        //Scale
                        Vec3::ONE,
                        //Rotation
                        match direction.direction {
                            DirectionEnum::Right => Quat::from_rotation_z(-1.57),
                            DirectionEnum::Left => Quat::from_rotation_z(1.57),
                            DirectionEnum::Up => Quat::from_rotation_z(0.0),
                            DirectionEnum::Down => Quat::from_rotation_z(3.1),
                        },
                        //Translation
                        Vec3::new(match direction.direction {
                            DirectionEnum::Left => -60.0 + shooter_transform.translation.x,
                            DirectionEnum::Right => 60.0 + shooter_transform.translation.x,
                            _ => shooter_transform.translation.x
                        }, match direction.direction {
                            DirectionEnum::Up => 60.0 + shooter_transform.translation.y,
                            DirectionEnum::Down => -60.0 + shooter_transform.translation.y,
                            _ => shooter_transform.translation.y
                        }, 0.0))));
            shooter.time_out -= time.delta_seconds();
        }
    }
}

// Move all bullets
fn bullet_mover(
    time : Res<Time>,
    mut query : Query<(&Bullet, &Speed, &mut Transform, &Direction)>
) {
    for (bullet, speed, mut transform, direction) in query.iter_mut() {
        match direction.direction {
            DirectionEnum::Up => {
                transform.translation.y += speed.speed * time.delta_seconds();
            },
            DirectionEnum::Down => {
                transform.translation.y -= speed.speed * time.delta_seconds();
            },
            DirectionEnum::Left => {
                transform.translation.x -= speed.speed * time.delta_seconds();
            },
            DirectionEnum::Right => {
                transform.translation.x += speed.speed * time.delta_seconds();
            },
        }
    }
}

// Remove all expiring entities
fn expire(
    time : Res<Time>,
    mut commands : Commands,
    mut query : Query<(Entity, &mut Expire)>
) {
    for (entity, mut expire) in query.iter_mut() {
        if time.delta_seconds() > expire.time {
            commands.entity(entity).despawn();
        } else {
            expire.time -= time.delta_seconds();
        }
    }
}

// Simple system to move into the Paused state when pressing Escape
fn pause_handler(
    mut state : ResMut<State<AppState>>,
    input : Res<Input<KeyCode>>
) {
    if input.pressed(KeyCode::Escape) {
        state.set(AppState::Paused).unwrap();
    }
}

// TODO add setup function here to initialize Game Over.
fn setup_game_over(

) {

}

// TODO add function body here to bring up Game Over menu.
fn game_over(

) {

}

// TODO add cleaning function here to remove Game Over components
fn clean_game_over(

) {
    
}

// Simple system to move the camera up a little bit
fn scroll_camera(time : Res<Time>, mut query : Query<&mut Transform, With<Scrolling>>) {
    let mut transform = query.single_mut().unwrap();
    transform.translation.y += 30.0 * time.delta_seconds();
}

// my initial solution to making the custom game window was to make a function that simply:
/*
 1) changed the title
 2) requested a resize
 3) disabled resizing
*/
/* Unfortunately the last bit then cancelled out the second bit. (Note: It really shouldn't do that. If MY code resizes something
when I don't want it to that's MY GOD DAMN FAULT and *I* should fix it. Disabling resizes should only remove the fullscreen button
and disable dragging the sides.)
*/
//... had to make a whole ass resource for this shit. fuckin hell.
fn stop_fucking_resizing(mut windows : ResMut<Windows>, mut not_running : ResMut<ResizeStopper>) {
    if not_running.0 {
        return;
    }
    let window = windows.get_primary_mut().unwrap();
    if window.width() == 600.0 && window.height() == 600.0 {
        window.set_resizable(false);
        window.set_title("Un-Divey".to_string());
        not_running.0 = true;
    } else {
        window.set_resolution(600.0, 600.0);
    }
}

 // Sub-section, ENEMY AI!!!!! :vomitting_face:
// Eugh

fn chaser_ai(
    time : Res<Time>,
    mut set : QuerySet<(
        Query<(Entity, &mut Transform, &Speed, &Health, &EnemyAI)>,
        Query<(&Transform), (With<Player>)>,
    )>,
) {
    let player_pos = set.q1().single().unwrap().translation;
    let delta = time.delta_seconds();
    for (_, mut transform, speed, health, ai_type) in set.q0_mut().iter_mut() {
        if health.hp <= 0 || match ai_type { EnemyAI::Chaser => false, _ => true } {
            continue;
        }
        //transform.rotation.lerp(Quat::from_rotation_z(transform.translation.angle_between(player_pos)), 0.55);
        let angle = (Quat::from_rotation_z(transform.translation.angle_between(player_pos)) - transform.rotation);//.max(-0.35).min(0.35));
        transform.rotation *= angle;
        //let direction : Vec3 = transform.rotation.mul_vec3(Vec3::new(0.0, speed.speed * delta, 0.0));
        transform.translation.y += 30.0 * delta;
    }
}

// Plugins
/*
pub struct Thingy;

impl Plugin for Thingy {
    fn build(&self, app : &mut AppBuilder) {
        app.enter

        ;
    }
}
*/

 // Simple plugin for the Game loop.
// Scratch that, quite complex.

pub struct Game;

impl Plugin for Game {
    fn build(&self, app : &mut AppBuilder) {
        app
            // Startup game
            .add_system_set(SystemSet::on_enter(AppState::StartGame)
                .with_system(setup_game.system())
            )
            // Run game
            .add_system_set(SystemSet::on_update(AppState::InGame)
                .with_system(movement.system())
                .with_system(controllable_shooting.system())
                .with_system(pause_handler.system())
                .with_system(scroll_camera.system())
                .with_system(bullet_mover.system())
                .with_system(expire.system())
            )
            // Pause screen
            .add_system_set(SystemSet::on_enter(AppState::Paused).with_system(setup_pause.system()))
            .add_system_set(SystemSet::on_update(AppState::Paused).with_system(pause.system()))
            .add_system_set(SystemSet::on_exit(AppState::Paused).with_system(clean_pause.system()))
            // Enemy AI
            .add_system_set(SystemSet::on_update(AppState::InGame)
                .with_system(chaser_ai.system())
            )
            // Game Over
            .add_system_set(SystemSet::on_enter(AppState::GameOver)
                .with_system(setup_game_over.system())
                // Called here rather than on_exit(AppState::InGame) because that would clean up the game when paused.
                .with_system(clean_game.system())
            )
            .add_system_set(SystemSet::on_exit(AppState::GameOver)
                .with_system(clean_game_over.system())
            )
            .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(game_over.system()))
        .run();
    }
}

// Simple plugin for the Menu

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app : &mut AppBuilder) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Menu)
                .with_system(setup_menu.system())
            )
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu.system()))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(clean_menu.system()))
        .run();
    }
}

fn main() {
    App::build()
          // Set clear color to darkest color I could pick from an image of the ocean
         //TODO add simple animation to simulate light refraction? Since the game is starting rather deep below the ocean
        // the animation should play after you make some progress, perhaps an indication of nearing the end?
        .insert_resource(ClearColor(Color::rgb_u8(4, 31, 59)))
        
         // Sets the game's state.
        //TODO when testing is over switch this to AppState::Menu
        .add_state(AppState::StartGame)
        // Add's bevy's vast list of default plugins.
        .add_plugins(DefaultPlugins)
        // see stop_fucking_resizing.
        .insert_resource(ResizeStopper(false))
        .add_system(stop_fucking_resizing.system())
        // Add my plugins. Modularity, yo.
        .add_plugin(Game)
        //.add_plugin(Menu)
    .run();
}
