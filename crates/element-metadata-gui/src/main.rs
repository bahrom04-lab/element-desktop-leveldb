use element_metadata_gui::window::App;
use relm4::RelmApp;

fn main() {
    let app = RelmApp::new("uz.bahrom04.element-desktop-leveldb");
    app.run::<App>(());
}
