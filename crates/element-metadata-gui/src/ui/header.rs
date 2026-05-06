use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};

use crate::window::AppMsg;

pub(crate) struct Header;

#[relm4::component(pub(crate))]
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
