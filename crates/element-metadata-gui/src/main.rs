// Don't show GTK 4.10 deprecations.
// We can't replace them without raising the GTK requirement to 4.10.

use std::convert::identity;
use std::path::PathBuf;
use std::process::Command;

use element_metadata::{ElementLevelDBParser, ElementMetadata};
use relm4::{
    adw::{self, prelude::*},
    factory::{FactoryComponent, FactorySender, FactoryVecDeque},
    gtk::{self, prelude::*},
    prelude::*,
};

struct Header;

#[relm4::component]
impl SimpleComponent for Header {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        gtk::HeaderBar {
            pack_end = &gtk::Button {
                set_label: "Parse LevelDB",
                connect_clicked[sender] => move |_| {
                    sender.output(AppMsg::OpenLevelDB).unwrap();
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Header;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

#[derive(Debug)]
enum AppMsg {
    OpenLevelDB,
    ParseLevelDB(PathBuf),
    UpdateMetadata(ElementMetadata),
}

/* ------------------ FACTORY COMPONENT ------------------ */

struct Room {
    id: String,
    index: usize,
}

#[relm4::factory]
impl FactoryComponent for Room {
    type Init = (String, usize);
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = adw::PreferencesGroup;

    view! {
        adw::ExpanderRow {
            set_use_markup: true,
            #[watch]
            set_title: &format!("Room {}", self.index + 1),

            add_row = &adw::ActionRow {
                set_title: &self.id,
            },
        },
    }

    fn init_model(
        (id, index): Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self { id, index }
    }
}

/* ------------------ MAIN APP ------------------ */

struct App {
    header: Controller<Header>,
    metadata: Option<ElementMetadata>,
    content: String,
    status: String,
    rooms: FactoryVecDeque<Room>,
    encrypted_rooms: FactoryVecDeque<Room>,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        main_window = gtk::ApplicationWindow {
            set_default_size: (900, 700),
            set_titlebar: Some(model.header.widget()),

            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                append = &gtk::ScrolledWindow {
                    set_vexpand: true,
                    set_hexpand: true,

                    #[wrap(Some)]
                    set_child = &adw::PreferencesPage {

                        /* ---------- USER PROFILE ---------- */
                        add = &adw::PreferencesGroup {
                            set_title: "👤 User Profile",
                            set_description: Some("Element account information"),

                            add = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_margin_all: 12,

                                append = &gtk::Label {
                                    #[watch]
                                    set_markup: &model.content,
                                    set_wrap: true,
                                    set_selectable: true,
                                    set_use_markup: true,
                                    set_xalign: 0.0,
                                },
                            }
                        },

                        /* ---------- ROOMS ---------- */
                        add = &adw::PreferencesGroup {
                            set_title: "🏠 Rooms",
                            #[watch]
                            set_description: Some(&format!("total rooms: {:?}", model.rooms.len())),
                            add = model.rooms.widget(),
                        },

                        /* ---------- ENCRYPTED ROOMS ---------- */
                        add = &adw::PreferencesGroup {
                            set_title: "🔒 Encrypted Rooms",
                            #[watch]
                            set_description: Some(&format!("total rooms: {:?}", model.encrypted_rooms.len())),
                            add = model.rooms.widget(),
                        }
                    },
                },

                append = &gtk::Separator {},

                append = &gtk::Label {
                    #[watch]
                    set_markup: &format!("<small>{}</small>", model.status),
                    set_wrap: true,
                    set_selectable: true,
                    set_use_markup: true,
                    set_margin_all: 8,
                    set_xalign: 0.0,
                },
            },
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header = Header::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let rooms = FactoryVecDeque::builder().launch_default().detach();
        let encrypted_rooms = FactoryVecDeque::builder().launch_default().detach();

        let model = App {
            header,
            metadata: None,
            content: "<i>Click 'Parse LevelDB' to load database</i>".to_string(),
            status: "Ready".to_string(),
            rooms,
            encrypted_rooms,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::OpenLevelDB => {
                self.status = "Running parser...".to_string();

                if let Ok(current_exe) = std::env::current_exe() {
                    if let Some(project_dir) = current_exe
                        .parent()
                        .and_then(|p| p.parent())
                        .and_then(|p| p.parent())
                    {
                        let script_path = project_dir.join("copy-leveldb.sh");
                        let leveldb_path = project_dir.join("leveldb");

                        match Command::new("bash").arg(script_path).output() {
                            Ok(output) if output.status.success() => {
                                sender.input(AppMsg::ParseLevelDB(leveldb_path));
                            }
                            Ok(output) => {
                                let err = String::from_utf8_lossy(&output.stderr);
                                self.status = format!("Error: {}", err);
                                self.content = format!("<b>Error:</b> {}", err);
                            }
                            Err(e) => {
                                self.status = format!("Exec error: {}", e);
                                self.content = format!("<b>Error:</b> {}", e);
                            }
                        }
                    }
                }
            }

            AppMsg::ParseLevelDB(path) => {
                self.status = format!("Parsing: {}", path.display());

                match ElementLevelDBParser::open(path.to_str().unwrap_or("")) {
                    Ok(parser) => match parser.parse_metadata() {
                        Ok(metadata) => {
                            self.metadata = Some(metadata.clone());

                            /* ---------- UPDATE FACTORIES ---------- */
                            let mut rooms = self.rooms.guard();
                            rooms.clear();
                            for (i, id) in metadata.room_ids.iter().enumerate() {
                                rooms.push_back((id.clone(), i));
                            }

                            let mut enc = self.encrypted_rooms.guard();
                            enc.clear();
                            for (i, id) in metadata.encrypted_rooms.iter().enumerate() {
                                enc.push_back((id.clone(), i));
                            }

                            self.content = Self::format_metadata(&Some(metadata));
                            self.status = "Loaded successfully".to_string();
                        }
                        Err(e) => {
                            self.status = format!("Parse error: {}", e);
                        }
                    },
                    Err(e) => {
                        self.status = format!("Open error: {}", e);
                    }
                }
            }

            AppMsg::UpdateMetadata(metadata) => {
                self.metadata = Some(metadata.clone());
                self.content = Self::format_metadata(&self.metadata);
            }
        }
    }
}

/* ------------------ HELPERS ------------------ */

impl App {
    fn format_metadata(metadata: &Option<ElementMetadata>) -> String {
        match metadata {
            Some(meta) => {
                let mut s = String::new();

                if let Some(ref user) = meta.user_id {
                    s.push_str(&format!("<b>User:</b> <tt>{}</tt>\n", user));
                }

                if let Some(ref name) = meta.display_name {
                    s.push_str(&format!("Name: <b>{}</b>\n", name));
                }

                s
            }
            None => "<i>No metadata</i>".to_string(),
        }
    }
}

/* ------------------ MAIN ------------------ */

fn main() {
    let app = RelmApp::new("relm4.example.components");
    app.run::<App>(());
}
