use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::input::mouse::*;
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Glue(Entity);

#[derive(Component)]
struct Follow(Entity);

#[derive(Component)]
struct Attach(Entity);

#[derive(Component)]
struct Sphere {
    pub radius: f32,
    pub border_radius: f32,
    pub fill_color: Color,
    pub border_color: Color,
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Msaa { samples: 4 })
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_startup_system(setup);
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Strange Quark".to_string(),
            width: 800 as f32,
            height: 600 as f32,
            position: Vec2::new(0.0, 0.0).into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(HelloPlugin)
        .add_system(zoom_camera)
        .add_system(keyboard_input_system)
        .add_system(handle_collisions)
        .add_system_to_stage(CoreStage::Last, debug_position)
        .add_system_to_stage(CoreStage::Update, on_attach)
        .run();
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vector2::zeros();

    let fill_color = Color::GRAY;
    let stroke_color = Color::WHITE;
    let radius = 50.0;
    let stroke = radius / 10.0;

    let player = commands
    .spawn()
    .insert(Sphere {
        radius: radius,
        border_radius: radius / 10.0,
        fill_color: fill_color,
        border_color: stroke_color,
    })
    .insert_bundle(GeometryBuilder::build_as(
        &shapes::Circle {
            radius: radius,
            center: Vec2::ZERO,
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(fill_color),
            outline_mode: StrokeMode::new(stroke_color, stroke),
        },
        Transform {
            scale: Vec3::new(1.0, 1.0, 1.0),
            translation: Vec3::new(0.0, 0.0, 1.1),
            ..Transform::default()
        },
    ))
    .insert_bundle(ColliderBundle {
        position: Vec2::new(0.0, 0.0).into(),
        shape: ColliderShape::ball(radius).into(),
        flags: (ActiveEvents::CONTACT_EVENTS).into(),
        ..Default::default()
    })
    .insert_bundle(RigidBodyBundle {
        position: Vec2::new(0.0, 0.0).into(),
        ..Default::default()
    })
    // .insert(ColliderPositionSync::Discrete)
    .insert(RigidBodyPositionSync::Discrete)
    // .insert(ColliderDebugRender::with_id(1))
    .insert(Player)
    // .push_children(&[child])
    .id();

    commands.entity(player)
        .insert(Glue(player));

    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Follow(player));

    let mut rnd = StdRng::seed_from_u64(2);

    for _i in 0..200 {
        let hue: f32 = rnd.gen_range(0.0..=360.0);
        let fill_color = Color::hsla(hue, 0.4, 0.3, 1.0);
        let stroke_color = Color::hsla(hue, 0.5, 0.4, 0.9);
        let radius = rnd.gen_range(5.0..200.0);
        let stroke = radius / 10.0;
        let x: f32 = rnd.gen_range(-4000.0..=4000.0);
        let y: f32 = rnd.gen_range(-3000.0..=4000.0);
    
        commands
        .spawn()
        .insert(Sphere {
            radius: radius,
            border_radius: stroke,
            fill_color: fill_color,
            border_color: stroke_color,
        })    
        .insert_bundle(GeometryBuilder::build_as(
            &shapes::Circle {
                radius: radius,
                center: Vec2::ZERO,
            },
            DrawMode::Outlined {
                fill_mode: FillMode::color(fill_color),
                outline_mode: StrokeMode::new(stroke_color, stroke),
            },
            Transform {
                scale: Vec3::new(1.0, 1.0, 1.0),
                translation: Vec3::new(x, y, 1.0),
                ..Transform::default()
            },
        ))
        .insert_bundle(ColliderBundle {
            position: Vec2::new(x, y).into(),
            shape: ColliderShape::ball(radius).into(),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec2::new(x, y).into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        // .insert(RigidBodyPositionSync::Discrete)
        // .insert(ColliderDebugRender::with_id(2))
        ;
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut RigidBodyForcesComponent, &RigidBodyMassPropsComponent)>,
) {
    let mut direction = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }

    for (_player, mut forces, mass) in query.iter_mut() {
        forces.force = (direction * 128.0 * mass.mass()).into();
    }
}

fn zoom_camera(
    mut ev_scroll: EventReader<MouseWheel>,
    mut query: Query<(&mut OrthographicProjection, &mut Transform, &Follow)>,
    player_query: Query<(&Player, &Transform), Without<OrthographicProjection>>,
) {
    let mut scroll = 0.0;

    for ev in ev_scroll.iter() {
        scroll += ev.y * 0.1;
    }

    for (mut projection, mut transform, follow) in query.iter_mut() {
        projection.scale = f32::max(projection.scale + scroll, 0.1);

        if let Ok((_player, player_transform)) = player_query.get(follow.0) {
            transform.translation = player_transform.translation;
        }
    }
}

// fn debug_position(query: Query<(&Player, &ColliderPositionComponent)>) {
fn debug_position(query: Query<&ColliderPositionComponent>) {
    // let e = Entity::from_raw(92);
    // if let Ok(obj) = query.get(e) {
    //     println!("{}", obj.translation);
    // } else {
    //     println!("nope");
    // }
}

fn on_attach(
    query: Query<(Entity, &Attach, &Sphere)>,
    t_query: Query<&Transform>,
    mut commands: Commands,
) {
    for (e, &Attach(player), sphere) in query.iter() {
        println!("Attach! {}", e.id());

        let parent_t = t_query.get(player).unwrap();
        let child_t = t_query.get(e).unwrap();
        let relative_t = Transform::from_matrix(parent_t.compute_matrix().inverse() * child_t.compute_matrix());

        // let relative_t = parent_t.rotation * (child_t.translation.truncate() - parent_t.translation.truncate()).extend(0.0);
        // let relative_t = (child_t.translation.truncate() - parent_t.translation.truncate()).extend(0.0);
        println!("{:?}", relative_t);

        commands.entity(e)
            .despawn();

        let new_e = commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shapes::Circle {
                    radius: sphere.radius,
                    center: Vec2::ZERO,
                },
                DrawMode::Outlined {
                    fill_mode: FillMode::color(sphere.fill_color),
                    outline_mode: StrokeMode::new(sphere.border_color, sphere.border_radius),
                },
                relative_t,
            ))
            // .remove::<Attach>()
            .insert_bundle(ColliderBundle {
                position: relative_t.translation.into(),
                shape: ColliderShape::ball(sphere.radius).into(),
                flags: (ActiveEvents::CONTACT_EVENTS).into(),
                ..Default::default()
            })
            .insert(ColliderPositionSync::Discrete)
            // .insert(ColliderDebugRender::with_id(2))
            .insert(Glue(player))
            .id()
            ;

        commands.entity(player).push_children(&[new_e]);
    }
}

fn handle_collisions(
    mut contact_events: EventReader<ContactEvent>,
    mut commands: Commands,
    query: Query<&Glue>,
) {
    for event in contact_events.iter() {
        match event {
            ContactEvent::Started(a, b) => {
                let (a, b) = (a.entity(), b.entity());
                let (player, other) = match (query.get(a), query.get(b)) {
                    (Ok(&Glue(player)), _) => (player, b),
                    (_, Ok(&Glue(player))) => (player, a),
                    _ => return,
                };

                println!("collision: {}, {}", a.id(), b.id());
                println!("player, other: {}, {}", player.id(), other.id());

                commands.entity(other)
                    // .despawn()
                    .remove_bundle::<RigidBodyBundle>()
                    .remove_bundle::<ColliderBundle>()
                    // .remove::<ColliderPositionSync>()
                    .insert(Attach(player))
                    // .remove::<RigidBodyTypeComponent>()
                    // .remove::<RigidBodyPositionComponent>()
                    // .remove::<RigidBodyVelocityComponent>()
                    // .remove::<RigidBodyMassPropsComponent>()
                    // .remove::<RigidBodyForcesComponent>()
                    // .remove::<RigidBodyActivationComponent>()
                    // .remove::<RigidBodyDampingComponent>()
                    // .remove::<RigidBodyDominanceComponent>()
                    // .remove::<RigidBodyCcdComponent>()
                    // .remove::<RigidBodyChangesComponent>()
                    // .remove::<RigidBodyIdsComponent>()
                    // .insert_bundle(ColliderBundle {
                    //     position: ColliderPositionComponent(Vec2::new(0.0, 0.0).into()),
                    //     shape: ColliderShape::ball(100.0).into(),
                    //     ..Default::default()
                    // })
                    // .remove::<RigidBodyPositionSync>()
                    // .insert(ColliderDebugRender::with_id(1))
                    // .insert(ColliderPositionSync::Discrete)
                    // .insert(ColliderParentComponent(ColliderParent {
                    //     handle: player.handle(),
                    //     pos_wrt_parent: Vec2::ZERO.into(),
                    //     // pos_wrt_parent: Vec2::new(x, y).into(),
                    // }))
                    ;
                // commands.entity(b.entity()).push_children(&[a.entity()]);

                // println!("{}", other.id());
            },
            ContactEvent::Stopped(_a, _b) => (),
        }
    }
}
