use element_metadata_gui::window::App;
use relm4::RelmApp;

fn main() {
    let app = RelmApp::new("relm4.example.components");
    app.run::<App>(());
}
