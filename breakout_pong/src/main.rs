use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

/// Stage for systems
pub const GAME_STATE_STAGE: &str = "game_state_stage";

/// Game States
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Menu,
    Playing,
    Pause,
}

/// A mash-up implementation of the classic games "Breakout" and "Pong"
const TIME_STEP: f32 = 1.0 / 60.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Menu)

        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu))
        .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu))
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))


        .insert_resource(Scoreboard {
            score1: 0,
            score2: 0,
        })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup)
        // systems to run only while Playing
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(paddle1_movement_system)
                .with_system(paddle2_movement_system)
                .with_system(ball_collision_system)
                .with_system(ball_movement_system),
        )
        .add_system(scoreboard_system)
        .add_system(space_to_pause)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

struct MenuData {
    button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}


fn menu(
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                state.set(GameState::Playing).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}


#[derive(Component)]
struct Paddle1 {
    speed: f32,
}

#[derive(Component)]
struct Paddle2 {
    speed: f32,
}

#[derive(Component)]
struct Ball {
    velocity: Vec3,
}

#[derive(Component)]
enum Collider {
    Solid,
    Scorable1,
    Scorable2,
    Paddle,
}

struct Scoreboard {
    score1: usize,
    score2: usize,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // paddle 1
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(400.0, 200.0, 0.0),
                scale: Vec3::new(20.0, 120.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(190.0 / 255.0, 124.0 / 255.0, 230.0 / 255.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paddle1 { speed: 500.0 })
        .insert(Collider::Paddle);
    // paddle 2
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-400.0, -200.0, 0.0),
                scale: Vec3::new(20.0, 120.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(130.0 / 255.0, 90.0 / 255.0, 195.0 / 255.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paddle2 { speed: 500.0 })
        .insert(Collider::Paddle);
    // .insert(Collider::Paddle2);
    // ball
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(20.0, 20.0, 0.0),
                translation: Vec3::new(0.0, -50.0, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(150.0 / 255.0, 150.0 / 255.0, 150.0 / 255.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
    // scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Player 1: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 30.0,
                        color: Color::rgb(0.5, 0.5, 0.5),
                    },
                },
                TextSection {
                    value: "\nPlayer 2: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::rgb(190.0 / 255.0, 124.0 / 255.0, 230.0 / 255.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 30.0,
                        color: Color::rgb(0.5, 0.5, 0.5),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    // Add walls
    let wall_color = Color::rgb(0.8, 0.8, 0.8);
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    // left
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-bounds.x / 2.0, 0.0, 0.0),
                scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid);
    // right
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(bounds.x / 2.0, 0.0, 0.0),
                scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid);
    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -bounds.y / 2.0, 0.0),
                scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid);
    // top
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, bounds.y / 2.0, 0.0),
                scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Solid);

    // Add bricks
    let num_bricks = 7;
    let space = 84 as f32;
    for row in 0..num_bricks {
        // right
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        (bounds.x / 2.0) - 20.0,
                        (-bounds.y / 2.0) + 48.0 + (row as f32 * space),
                        0.0,
                    ),
                    scale: Vec3::new(25.0, 80.0, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(230.0 / 255.0, 184.0 / 255.0, 255.0 / 255.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Collider::Scorable1);
    }

    // Add bricks
    let num_bricks = 7;
    let space = 84 as f32;
    for row in 0..num_bricks {
        // left
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        (-bounds.x / 2.0) + 20.0,
                        (-bounds.y / 2.0) + 48.0 + (row as f32 * space),
                        0.0,
                    ),
                    scale: Vec3::new(25.0, 80.0, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(207.0 / 255.0, 194.0 / 255.0, 255.0 / 255.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Collider::Scorable2);
    }
    // let brick_rows = 1;
    // let brick_columns = 1;
    // let brick_spacing = 20.0;
    // let brick_size = Vec3::new(10.0, 150.0, 1.0);
    // let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;
    // // center the bricks and move them up a bit
    // let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 100.0, 0.0);
    // let brick_color = Color::rgb(0.5, 0.5, 1.0);
    // for row in 0..brick_rows {
    //     let y_position = row as f32 * (brick_size.y + brick_spacing);
    //     for column in 0..brick_columns {
    //         let brick_position = Vec3::new(
    //             column as f32 * (brick_size.x + brick_spacing),
    //             y_position,
    //             0.0,
    //         ) + bricks_offset;
    //         // brick
    //         commands
    //             .spawn_bundle(SpriteBundle {
    //                 sprite: Sprite {
    //                     color: brick_color,
    //                     ..Default::default()
    //                 },
    //                 transform: Transform {
    //                     translation: brick_position,
    //                     scale: brick_size,
    //                     ..Default::default()
    //                 },
    //                 ..Default::default()
    //             })
    //             .insert(Collider::Scorable);
    //     }
    // }
}

fn paddle1_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle1, &mut Transform)>,
) {
    let (paddle, mut transform) = query.single_mut();
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction += 1.0;
    }

    let translation = &mut transform.translation;
    // move the paddle vertically
    translation.y += direction * paddle.speed * TIME_STEP;
    // bound the paddle within the walls
    translation.y = translation.y.min(220.0).max(-220.0);
}

fn paddle2_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle2, &mut Transform)>,
) {
    let (paddle, mut transform) = query.single_mut();
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::S) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction += 1.0;
    }

    let translation = &mut transform.translation;
    // move the paddle vertically
    translation.y += direction * paddle.speed * TIME_STEP;
    // bound the paddle within the walls
    translation.y = translation.y.min(220.0).max(-220.0);
}

fn ball_movement_system(mut ball_query: Query<(&Ball, &mut Transform)>, game_state: Res<State<GameState>>,) {
    match game_state.current() {
        GameState::Playing => {
            let (ball, mut transform) = ball_query.single_mut();
            transform.translation += ball.velocity * TIME_STEP;
        }
        GameState::Pause => {

        }
        GameState::Menu => {

        }
    }
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{}", scoreboard.score1);
    text.sections[3].value = format!("{}", scoreboard.score2);
}

fn space_to_pause(
    mut keys: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match game_state.current() {
            GameState::Playing => {
                game_state.push(GameState::Pause).unwrap();
            }
            GameState::Pause => {
                game_state.pop().unwrap();
            }
            GameState::Menu => {
            
            }
        }
        println!("{:?}", *game_state.current());
        keys.reset(KeyCode::Space);
    }
}

fn ball_collision_system(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Ball, &Transform)>,
    collider_query: Query<(Entity, &Collider, &Transform)>,
) {
    let (mut ball, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();
    let velocity = &mut ball.velocity;

    // check collision with walls
    for (collider_entity, collider, transform) in collider_query.iter() {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            // scorable colliders should be despawned and increment the scoreboard on collision
            if let Collider::Scorable1 = *collider {
                scoreboard.score1 += 1;
                commands.entity(collider_entity).despawn();
            }
            if let Collider::Scorable2 = *collider {
                scoreboard.score2 += 1;
                commands.entity(collider_entity).despawn();
            }

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = velocity.x > 0.0,
                Collision::Right => reflect_x = velocity.x < 0.0,
                Collision::Top => reflect_y = velocity.y < 0.0,
                Collision::Bottom => reflect_y = velocity.y > 0.0,
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                velocity.x = -velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                velocity.y = -velocity.y;
            }

            // break if this collide is on a solid, otherwise continue check whether a solid is
            // also in collision
            if let Collider::Solid = *collider {
                break;
            }
        }
    }
}
