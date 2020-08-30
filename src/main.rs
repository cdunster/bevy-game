use bevy::prelude::*;

struct Person;
struct Name(String);
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Laura Colbeck".to_string())))
        .spawn((Person, Name("Callum Dunster".to_string())))
        .spawn((Person, Name("Bailey van Colster".to_string())));
}

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut query: Query<(&Person, &Name)>,
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        for (_person, name) in &mut query.iter() {
            println!("Hello, {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(HelloPlugin)
        .run();
}
