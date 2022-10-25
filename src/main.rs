use bevy::prelude::*;
use bevy::window::*;
use bevy_prototype_lyon::prelude::*;
use bevy::input::mouse::*;
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use heron::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Follow(Entity);

#[derive(Component)]
struct Attach(Entity);

#[derive(Component)]
struct Sphere {
    pub color: Color,
    pub radius: f32,
    pub border: f32,
}

#[derive(PhysicsLayer)]
enum Layer {
    Player,
    Enemy,
}

#[derive(Default)]
struct EditorState {
    pub entity_id: String,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Strange Quark".to_string(),
            mode: WindowMode::BorderlessFullscreen,
            // width: 800 as f32,
            // height: 600 as f32,
            position: WindowPosition::At(Vec2::new(0.0, 0.0)),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(EditorState { entity_id: "10".into() })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(ShapePlugin)
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        .add_startup_system_to_stage(StartupStage::PostStartup, setup_shape)
        .add_startup_system_to_stage(StartupStage::PostStartup, setup_physics)
        .add_system(gui)
        .add_system(zoom_camera)
        .add_system(keyboard_input_system)
        .add_system(handle_collisions)
        // .add_system_to_stage(CoreStage::Last, debug_position)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    let player = commands
        .spawn()
        .insert(Player)
        .insert(Sphere {
            radius: 50.0,
            border: 50.0 / 10.0,
            color: Color::GRAY,
        })
        .insert(Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Transform::default()
        })
        .insert(CollisionLayers::new(Layer::Player, Layer::Enemy))
        .id();

    commands.spawn_bundle(Camera2dBundle::default())
        .insert(Follow(player));

    let mut rnd = StdRng::seed_from_u64(1);

    for _i in 0..1000 {
        let hue: f32 = rnd.gen_range(0.0..=360.0);
        let color = Color::hsla(hue, 0.4, 0.3, 1.0);
        let radius = rnd.gen_range(5.0..200.0);
        let x: f32 = rnd.gen_range(-10000.0..=10000.0);
        let y: f32 = rnd.gen_range(-10000.0..=10000.0);
    
        commands
            .spawn()
            .insert(Sphere {
                radius: radius,
                border: radius / 10.0,
                color: color,
            })
            .insert(Transform {
                translation: Vec3::new(x, y, 1.0),
                ..Transform::default()
            })
            .insert(CollisionLayers::new(Layer::Enemy, Layer::Player).with_mask(Layer::Enemy))
            ;
    }
}

fn setup_shape(
    query: Query<(Entity, &Sphere, &Transform)>,
    mut commands: Commands,
) {
    for (entity, sphere, transform) in query.iter() {
        commands.entity(entity)
            .insert_bundle(GeometryBuilder::build_as(
                &shapes::Circle {
                    radius: sphere.radius,
                    center: Vec2::ZERO,
                },
                DrawMode::Outlined {
                    fill_mode: FillMode::color(sphere.color),
                    outline_mode: StrokeMode::new(sphere.color + Color::hsla(0.0, 0.0, 0.5, -0.1), sphere.border),
                },
                *transform,
            ));
    }
}

fn setup_physics(
    query: Query<(Entity, &Sphere)>,
    mut commands: Commands,
) {
    for (entity, sphere) in query.iter() {
        commands.entity(entity)
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Sphere { radius: sphere.radius })
            .insert(Velocity::default());
    }
}

fn gui(
    mut egui_context: ResMut<EguiContext>,
    mut editor_state: ResMut<EditorState>,
    query: Query<&GlobalTransform>
) {
    egui::Window::new("debug").show(egui_context.ctx_mut(), |ui| {
        let id = editor_state.entity_id.parse::<u32>()
            .or(Err(bevy::ecs::query::QueryEntityError::NoSuchEntity))
            .and_then(|id| Ok(Entity::from_raw(id)))
            .and_then(|entity| Ok(query.get(entity)));

        ui.label("entity");
        ui.text_edit_singleline(&mut editor_state.entity_id);

        ui.label("position");

        if let Ok(Ok(obj)) = id {
            ui.label(format!("{}", obj.translation()));
        } else {
            ui.label("no entity");
        }
    });
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let mut direction = Vec3::ZERO;
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

    for mut velocity in query.iter_mut() {
        velocity.linear = direction * 200.0;
        velocity.angular = AxisAngle::new(Vec3::new(0.0, 0.0, 1.0), 1.0);
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

#[allow(dead_code)]
fn debug_position(query: Query<&GlobalTransform>) {
    let e = Entity::from_raw(10);
    if let Ok(obj) = query.get(e) {
        println!("{}", obj.translation());
    } else {
        // println!("nope");
    }
}

fn handle_collisions(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut query: Query<&mut Transform>,
) {
    events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();

            if is_player(layers_1) && is_enemy(layers_2) {
                Some((entity_1, entity_2))
            } else if is_player(layers_2) && is_enemy(layers_1) {
                Some((entity_1, entity_2))
            } else {
                None
            }
        })
        .for_each(|(player, enemy)| {
            commands.entity(enemy)
                .remove::<RigidBody>()
                .insert(CollisionLayers::new(Layer::Player, Layer::Enemy))
                ;

            if let Ok([parent_t, mut child_t]) = query.get_many_mut([player, enemy]) {
                let relative_t = Transform::from_matrix(parent_t.compute_matrix().inverse() * child_t.compute_matrix());
                child_t.translation = relative_t.translation;
                child_t.rotation = relative_t.rotation;
                child_t.scale = relative_t.scale;
            }

            commands.entity(player)
                .push_children(&[enemy]);
        });
}

fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Player) && !layers.contains_group(Layer::Enemy)
}

fn is_enemy(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::Player) && layers.contains_group(Layer::Enemy)
}
