use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
};

fn main() {
    App::build()
        .add_default_plugins()
        .init_resource::<State>()
        .add_system(print_mouse_events_system.system())
        .run();
}

#[derive(Default)]
struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

fn print_mouse_events_system(
    mut state: ResMut<State>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    cursor_move_events: Res<Events<CursorMoved>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
) {
    for event in state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    {
        println!("{:?}", event);
    }

    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        println!("{:?}", event);
    }

    for event in state.cursor_moved_event_reader.iter(&cursor_move_events) {
        println!("{:?}", event);
    }

    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        println!("{:?}", event);
    }
}
