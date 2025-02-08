use anathema::{backend, component::Component, prelude::{Document, TuiBackend}, runtime::{self, Runtime}, state::State};

#[derive(State)]
struct PetState {}

impl PetState {
    pub fn new() -> Self {
        Self {}
    }
}

struct Pet;

impl Component for Pet {
    type State = PetState;
    type Message = ();
}

impl Pet {
    pub fn new() -> Self {
        Self {}
    }
}

fn main() {
    let doc = Document::new("@main");

    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);
    
    let _pet_id = runtime
        .register_component(
            "main", 
            "pet_template.aml", 
            Pet::new(), 
            PetState::new()
        ).unwrap();

    runtime.finish().unwrap().run();
}
