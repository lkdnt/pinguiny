use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// this enum for dummy movement input of our character
/// for testing purposes
/// because later we dont control our character movement with keyboard input
/// they will move automatically based on other logic, but for now we will use keyboard input to test the movement and physics of our character
#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum DummyAction {
    // this movement actions are for testing purposes, it will be used to test the movement of our character
    Up,
    Down,
    Left,
    Right,
    // this action is for testing purposes, it will be used to test the attack of our character
    Attack,
    Defend,
}

impl DummyAction {
    const DIRECTIONS: [Self; 4] = [
        DummyAction::Up,
        DummyAction::Down,
        DummyAction::Left,
        DummyAction::Right,
    ];

    fn direction(self) -> Option<Dir2> {
        match self {
            DummyAction::Up => Some(Dir2::Y),
            DummyAction::Down => Some(Dir2::NEG_Y),
            DummyAction::Left => Some(Dir2::NEG_X),
            DummyAction::Right => Some(Dir2::X),
            _ => None,
        }
    }
}

/// this just a dummy player entities, cause it shouldnt be in game_input.rs later on
/// but for now we will use it to test the movement and physics of our character
#[derive(Component)]
pub struct Player;

impl Player {
    pub fn default_input_map() -> InputMap<DummyAction> {
        use DummyAction::*;
        let mut input_map = InputMap::default();

        // movement input
        input_map.insert(Up, KeyCode::KeyW);
        input_map.insert(Down, KeyCode::KeyS);
        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Right, KeyCode::KeyD);

        // attack input
        input_map.insert(Attack, KeyCode::Space);

        // defend input
        input_map.insert(Defend, KeyCode::ShiftLeft);

        input_map
    }
}

pub fn debug_player_walk(mut reader: MessageReader<PlayerWalk>) {
    for ev in reader.read() {
        info!("walk dir: {:?}", ev.direction);
    }
}

#[derive(Message)]
pub struct PlayerWalk {
    pub direction: Dir2,
}

pub fn player_walks(
    action_state: Single<&ActionState<DummyAction>, With<Player>>,
    mut message_writer: MessageWriter<PlayerWalk>,
) {
    let mut direction_vector = Vec2::ZERO;

    for input_direction in DummyAction::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            if let Some(direction) = input_direction.direction() {
                // sum the directions as 2D vectors
                direction_vector += *direction;
            }
        }
    }

    // then reconvert at the end, normalizing the magnitude
    let net_direction = Dir2::new(direction_vector);

    if let Ok(direction) = net_direction {
        message_writer.write(PlayerWalk { direction });
    }
}
