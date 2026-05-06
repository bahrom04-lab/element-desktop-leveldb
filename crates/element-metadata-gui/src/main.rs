// Don't show GTK 4.10 deprecations.
// We can't replace them without raising the GTK requirement to 4.10.
#![allow(deprecated)]

use std::convert::identity;
use std::path::PathBuf;
use std::process::Command;

use element_metadata::{ElementLevelDBParser, ElementMetadata};
use gtk::{glib, prelude::*};
use relm4::prelude::*;

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

struct Dialog {
    hidden: bool,
}

#[derive(Debug)]
enum DialogMsg {
    Show,
    Accept,
    Cancel,
}

struct DialogInit {
    text: String,
    secondary_text: Option<String>,
    accept_text: String,
    cancel_text: String,
}

#[relm4::component]
impl SimpleComponent for Dialog {
    type Init = DialogInit;
    type Input = DialogMsg;
    type Output = AppMsg;

    view! {
        dialog = gtk::MessageDialog {
            set_modal: true,
            set_text: Some(&init.text),
            set_secondary_text: init.secondary_text.as_deref(),
            add_button: (&init.accept_text, gtk::ResponseType::Accept),
            add_button: (&init.cancel_text, gtk::ResponseType::Cancel),

            #[watch]
            set_visible: !model.hidden,

            connect_response[sender] => move |_, resp| {
                sender.input(if resp == gtk::ResponseType::Accept {
                    DialogMsg::Accept
                } else {
                    DialogMsg::Cancel
                });
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Dialog { hidden: true };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            DialogMsg::Show => self.hidden = false,
            DialogMsg::Accept => {
                self.hidden = true;
                sender.output(AppMsg::Close).unwrap();
            }
            DialogMsg::Cancel => self.hidden = true,
        }
    }
}

#[derive(Debug)]
enum AppMsg {
    OpenLevelDB,
    ParseLevelDB(PathBuf),
    CloseRequest,
    Close,
    UpdateMetadata(ElementMetadata),
}

struct App {
    dialog: Controller<Dialog>,
    header: Controller<Header>,
    metadata: Option<ElementMetadata>,
    content: String,
    status: String,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        main_window = gtk::ApplicationWindow {
            set_default_size: (800, 600),
            set_titlebar: Some(model.header.widget()),

            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 10,
                set_margin_top: 10,
                set_margin_bottom: 10,
                set_margin_start: 10,
                set_margin_end: 10,

                append = &gtk::Label {
                    set_markup: "<b>Element Desktop Metadata Viewer</b>",
                    set_hexpand: true,
                    set_xalign: 0.0,
                },

                append = &gtk::ScrolledWindow {
                    set_vexpand: true,
                    set_hexpand: true,

                    #[wrap(Some)]
                    set_child = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 8,
                        set_margin_top: 10,
                        set_margin_bottom: 10,
                        set_margin_start: 10,
                        set_margin_end: 10,

                        append = &gtk::Label {
                            #[watch]
                            set_markup: &model.content,
                            set_wrap: true,
                            set_selectable: true,
                            set_use_markup: true,
                            set_hexpand: true,
                            set_xalign: 0.0,
                        }
                    }
                },

                append = &gtk::Separator {},

                append = &gtk::Label {
                    #[watch]
                    set_markup: &format!("<small>{}</small>", model.status),
                    set_wrap: true,
                    set_selectable: true,
                    set_use_markup: true,
                    set_hexpand: true,
                    set_xalign: 0.0,
                },
            },

            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::CloseRequest);
                glib::Propagation::Stop
            }
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
        let dialog = Dialog::builder()
            .transient_for(&root)
            .launch(DialogInit {
                text: "Do you want to close before saving?".to_string(),
                secondary_text: Some("All unsaved changes will be lost".to_string()),
                accept_text: "Close".to_string(),
                cancel_text: "Cancel".to_string(),
            })
            .forward(sender.input_sender(), identity);

        let model = App {
            header,
            dialog,
            metadata: None,
            content: "<i>Click 'Parse LevelDB' to automatically load and parse the database</i>".to_string(),
            status: "Ready".to_string(),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::OpenLevelDB => {
                self.status = "Running LevelDB parser...".to_string();
                
                // Get the project directory and run the copy-leveldb.sh script
                if let Ok(current_exe) = std::env::current_exe() {
                    if let Some(project_dir) = current_exe.parent().and_then(|p| p.parent()).and_then(|p| p.parent()) {
                        let script_path = project_dir.join("copy-leveldb.sh");
                        let leveldb_path = project_dir.join("leveldb");
                        
                        eprintln!("Running script: {}", script_path.display());
                        eprintln!("LevelDB path: {}", leveldb_path.display());
                        
                        match Command::new("bash")
                            .arg(script_path)
                            .output()
                        {
                            Ok(output) => {
                                if output.status.success() {
                                    eprintln!("Script executed successfully");
                                    // Now parse the leveldb
                                    sender.input(AppMsg::ParseLevelDB(leveldb_path));
                                } else {
                                    let stderr = String::from_utf8_lossy(&output.stderr);
                                    eprintln!("Script failed: {}", stderr);
                                    self.status = format!("Error running parser: {}", stderr);
                                    self.content = format!("<b>Error:</b> {}", stderr);
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to execute script: {}", e);
                                self.status = format!("Error executing parser: {}", e);
                                self.content = format!("<b>Error:</b> {}", e);
                            }
                        }
                    } else {
                        self.status = "Error: Could not determine project directory".to_string();
                        self.content = "<b>Error:</b> Could not determine project directory".to_string();
                    }
                } else {
                    self.status = "Error: Could not get executable path".to_string();
                    self.content = "<b>Error:</b> Could not get executable path".to_string();
                }
            }
            AppMsg::ParseLevelDB(path) => {
                eprintln!("Parsing: {}", path.display());
                self.status = format!("Parsing: {}", path.display());

                match ElementLevelDBParser::open(path.to_str().unwrap_or("")) {
                    Ok(parser) => match parser.parse_metadata() {
                        Ok(metadata) => {
                            eprintln!("Success! Found user_id: {:?}", metadata.user_id);
                            self.metadata = Some(metadata.clone());
                            self.content = Self::format_metadata(&Some(metadata));
                            self.status = format!("Successfully loaded: {}", path.display());
                        }
                        Err(e) => {
                            eprintln!("Parse error: {}", e);
                            self.status = format!("Error parsing: {}", e);
                            self.content = format!("<b>Error:</b> {}", e);
                        }
                    },
                    Err(e) => {
                        eprintln!("Open error: {}", e);
                        self.status = format!("Error opening database: {}", e);
                        self.content = format!("<b>Error:</b> {}", e);
                    }
                }
            }
            AppMsg::CloseRequest => {
                self.dialog.emit(DialogMsg::Show);
            }
            AppMsg::Close => {
                relm4::main_application().quit();
            }
            AppMsg::UpdateMetadata(metadata) => {
                self.metadata = Some(metadata.clone());
                self.content = Self::format_metadata(&self.metadata);
                self.status = "Metadata updated".to_string();
            }
        }
    }
}

impl App {
    fn format_metadata(metadata: &Option<ElementMetadata>) -> String {
        match metadata {
            Some(meta) => {
                let mut content = String::new();

                if let Some(ref user) = meta.user_id {
                    content.push_str(&format!("<b>User ID:</b> {}\n", user));
                }
                if let Some(ref name) = meta.display_name {
                    content.push_str(&format!("<b>Display Name:</b> {}\n", name));
                }
                if let Some(ref avatar) = meta.avatar_url {
                    content.push_str(&format!("<b>Avatar:</b> {}\n", avatar));
                }
                if let Some(ref theme) = meta.theme {
                    content.push_str(&format!("<b>Theme:</b> {}\n", theme));
                }
                if let Some(ref lang) = meta.language {
                    content.push_str(&format!("<b>Language:</b> {}\n", lang));
                }
                if let Some(notif) = meta.notifications_enabled {
                    content.push_str(&format!("<b>Notifications:</b> {}\n", notif));
                }
                if let Some(ref device_id) = meta.device_id {
                    content.push_str(&format!("<b>Device ID:</b> {}\n", device_id));
                }
                if let Some(ref device_name) = meta.device_name {
                    content.push_str(&format!("<b>Device Name:</b> {}\n", device_name));
                }
                if !meta.room_ids.is_empty() {
                    content.push_str(&format!(
                        "<b>Rooms ({}):</b> {}\n",
                        meta.room_ids.len(),
                        meta.room_ids.join(", ")
                    ));
                }
                if !meta.encrypted_rooms.is_empty() {
                    content.push_str(&format!(
                        "<b>Encrypted Rooms ({}):</b> {}\n",
                        meta.encrypted_rooms.len(),
                        meta.encrypted_rooms.join(", ")
                    ));
                }

                if content.is_empty() {
                    "<i>No metadata available</i>".to_string()
                } else {
                    content
                }
            }
            None => "<i>No metadata loaded</i>".to_string(),
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.components");
    app.run::<App>(());
}
