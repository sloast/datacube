use datacube::cube::Facelets;
use godot::classes::Node;
use godot::prelude::*;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

mod datacube;
pub use datacube::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct VisualCube {
    cube_state_sender: Sender<Facelets>,
    cube_state_receiver: Receiver<Facelets>,
    state: Facelets,
    base: Base<Node>,
}

#[godot_api]
impl INode for VisualCube {
    fn init(base: Base<Node>) -> Self {
        let (cube_state_sender, cube_state_receiver) = mpsc::channel::<Facelets>();

        Self {
            cube_state_sender,
            cube_state_receiver,
            state: Facelets::new(),
            base,
        }
    }
}

#[godot_api]
impl VisualCube {
    #[func]
    fn get_state(&self) -> Array<i64> {
        match self.cube_state_receiver.try_recv() {
            Ok(state) => return Array::from(&state.to_facelet_array()),
            Err(_) => return Array::from(&self.state.to_facelet_array()),
        };
    }

    #[func]
    fn rotate(&mut self, face: i64, n: i64) {
        self.state
            .rotate(unsafe { std::mem::transmute(face as u8) }, n as u32);
    }
}
