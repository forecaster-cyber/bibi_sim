use bevy::prelude::*;
use rand::prelude::*;
use bevy::window::PrimaryWindow;
use  bevy::window::{WindowMode, WindowResized};
pub const  ENEMY_SPEED: f32 = 200.0;
pub const PLAYER_SIZE: f32 = 70.0; 
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.
use rand::Rng;


fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Fullscreen Test".into(),
            mode: WindowMode::Windowed{},
            ..default()
        }),
        ..default()
    };
    App::new()
    .add_plugins(DefaultPlugins.set(window_plugin))
    .add_startup_system(spawnPlayer)
    .add_startup_system(spawnCam)
    .add_system(spawn_enemies)
    .add_system(player_movement)
    .add_system(enemy_hit_player)
    .add_system(enemy_movement)
    .init_resource::<StarSpawnTimer>()
    .init_resource::<Score>()
    .init_resource::<GameLoop>()
    .add_system(tick_star_spawn_timer)
    .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))

    
    .run();
}

#[derive(Component)]
pub struct Player{
    
}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}
#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}
impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer { timer: Timer::from_seconds(5.0, TimerMode::Repeating) }
    }
}
#[derive(Resource)]
pub struct Score {
    pub score: i32,
}
impl Default for Score {
    fn default() -> Score {
        Score { score: (0) }
    }
}
#[derive(Resource)]
pub struct GameLoop {
    pub game_loop: bool,
}
impl Default for GameLoop {
    fn default() -> GameLoop {
        GameLoop { game_loop: (true) } 
    }
}
pub fn spawnPlayer(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>,){
    let window = window_query.get_single().unwrap();
    commands.spawn((SpriteBundle{
        //transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
        texture: asset_server.load("sprites/bibi.png"),
        ..default()
    }, Player{},));
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                
                font_size: 60.0,
                color: Color::GOLD,
                ..Default::default()
            }),
        ]),
        
    ));
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/standing.ogg"),
        ..default()
    });
}
pub fn spawnCam(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>){
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle::default());
}
pub const player_velocity: f32 = 500.0;
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut cursor_ev: EventReader<CursorMoved>,
    
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut position = None;

        for ev in cursor_ev.iter() {
            
                position = Some((ev.position.x as f32 -600.0, ev.position.y as f32 -400.0));
            
        }

        if let Some((x, y)) = position {
            transform.translation.x = x;
            transform.translation.y = -y;
        }
        if transform.translation.y < -350.0 {
            //enemy.direction.y = -enemy.direction.y;
           
                
            transform.translation.y = -350.0;
            
        }
        if transform.translation.y > 350.0 {
            //enemy.direction.y = -enemy.direction.y;
           
            transform.translation.y = 350.0;
            
        }
        if transform.translation.x > 600.0 {
            //enemy.direction.y = -enemy.direction.y;
           
            transform.translation.x = 600.0;
            
        }
        if transform.translation.x < -600.0 {
            //enemy.direction.y = -enemy.direction.y;
           
            transform.translation.x = -600.0;
            
            
        }
    }
}
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
    mut score: ResMut<Score>,
    game_loop: Res<GameLoop>
) {
     
    if star_spawn_timer.timer.finished() && game_loop.game_loop == true {
        
    
    let window = window_query.get_single().unwrap();

    
        let random_x = random::<f32>() * window.width() - window.width() / 2.0;
        let random_y =  window.height() *0.0005;
        let cigar_or_champ = random::<f32>();
        print!("{}", window.height());
        let mut rng = rand::thread_rng();
        print!("{}", cigar_or_champ);
        if cigar_or_champ < 0.5 {
            
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y+250.0, 0.0),
                    texture: asset_server.load("sprites/cigar.png"),
                    ..default()
                },
                Enemy {direction: Vec2::new(random::<f32>(), -10.0).normalize()},
            ));
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/sound1.ogg"),
                ..default()
            });
        }
        else if cigar_or_champ > 0.5 {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y+250.0, 0.0),
                    texture: asset_server.load("sprites/champ.png"),
                    ..default()
                },
                Enemy {direction: Vec2::new(random::<f32>(), -10.0).normalize()},
            ));
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/sound2.ogg"),
                ..default()
            });
        }
score.score += 1;
    
    }
}
pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &mut Enemy)>, time: Res<Time>, mut query: Query<&mut Text>,score: Res<Score>) {
    
    
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let tilt_x = random::<f32>();
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
        if transform.translation.y < -350.0 {
            //enemy.direction.y = -enemy.direction.y;
           
                enemy.direction.y = -enemy.direction.y +0.005;
                enemy.direction.x = enemy.direction.x + tilt_x;
            
            
        }
        if transform.translation.y > 350.0 {
            //enemy.direction.y = -enemy.direction.y;
           
                enemy.direction.y = -enemy.direction.y -0.005;
                enemy.direction.x = enemy.direction.x - tilt_x;
            
            
        }
        if transform.translation.x > 600.0 {
            //enemy.direction.y = -enemy.direction.y;
           
                enemy.direction.x = -enemy.direction.x -0.005;
                enemy.direction.y = enemy.direction.y - tilt_x;
            
            
        }
        if transform.translation.x < -600.0 {
            //enemy.direction.y = -enemy.direction.y;
           
                enemy.direction.x = -enemy.direction.x +0.005;
                enemy.direction.y = enemy.direction.y + tilt_x;
            
            
        }
    }
    for mut text in &mut query {
        text.sections[1].value = score.score.to_string();
    }
}
pub fn collision(mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>, mut player_query: Query<( &Transform), With<Player>>){
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        for (mut enemy_transform, mut enemy) in enemy_query.iter_mut() {
            //print!("player x position: {}", player_transform.translation.x);
            //print!("player y position: {}", player_transform.translation.y);
            //print!("enemy x position: {}", enemy_transform.translation.x);
            //print!("enemy y position: {}", enemy_transform.translation.y);
            if player_transform.translation.x - enemy_transform.translation.x <= 0.0 && player_transform.translation.y - enemy_transform.translation.y <= 0.0{
                print!("sus");
                enemy.direction.y = -enemy.direction.y;
            }
        }
    }
}
pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut game_loop: ResMut<GameLoop>
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for (mut enemy_transform, mut enemy) in enemy_query.iter_mut() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over!");
                game_loop.game_loop = false;
                
                
                commands.entity(player_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/fart.ogg"),
                    ..default()
                });
            }
        }
    }
}
pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}