use bevy::prelude::*;

mod sprite;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "hyperbevy".to_string(),
            width: 1280,
            height: 720,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_default_plugins()
        .add_startup_system(sprite::setup.system())
        .add_system(sprite::animate.system())
        .run();
}
