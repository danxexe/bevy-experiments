use bevy::prelude::*;
use heron::prelude::*;

use super::core::*;

pub struct HeronPhysicsPlugin;

#[derive(PhysicsLayer)]
pub enum Layer {
    Player,
    Enemy,
}

impl HeronPhysicsPlugin {
    fn attach_layers(
        mut commands: Commands,
        player_q: Query<Entity, With<Player>>,
        gluable_q: Query<Entity, With<Gluable>>,
    ) {
        for entity in player_q.iter() {
            commands.entity(entity)
                .insert(CollisionLayers::new(Layer::Player, Layer::Enemy));
        }

        for entity in gluable_q.iter() {
            commands.entity(entity)
                .insert(CollisionLayers::new(Layer::Enemy, Layer::Player).with_mask(Layer::Enemy));
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
    
                if Self::is_player(layers_1) && Self::is_enemy(layers_2) {
                    Some((entity_1, entity_2))
                } else if Self::is_player(layers_2) && Self::is_enemy(layers_1) {
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
}

impl Plugin for HeronPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PhysicsPlugin::default())
            .add_startup_system_to_stage(StartupStage::PostStartup, Self::attach_layers)
            .add_startup_system_to_stage(StartupStage::PostStartup, Self::setup_physics)
            .add_system(Self::keyboard_input_system)
            .add_system(Self::handle_collisions)    
            ;
    } 
}
