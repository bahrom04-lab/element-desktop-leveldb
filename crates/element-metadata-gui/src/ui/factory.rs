/* ------------------ FACTORY COMPONENT ------------------ */
use relm4::{
    adw::{self, prelude::*},
    factory::{FactoryComponent, FactorySender},
    prelude::*,
};

pub(crate) struct Room {
    id: String,
    index: usize,
}

#[relm4::factory(pub(crate))]
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
