use apimock::core::config::ConfigUrlPaths;
use fltk::{
    button::Button,
    enums::{Align, Color},
    group::{Flex, FlexType},
    prelude::{GroupExt, WidgetBase, WidgetExt},
};

use crate::app::consts::{BUTTON_HEIGHT, BUTTON_WIDTH, FLEX_SPACING};

/// entry point
pub fn handle(config_url_paths: Option<ConfigUrlPaths>) -> Flex {
    let tab = Flex::default_fill().with_label("URL Paths\t\t").row();

    let mut vflex = Flex::default_fill();
    vflex.set_spacing(FLEX_SPACING);
    vflex.set_type(FlexType::Column);

    let config_url_paths = config_url_paths.unwrap_or_default();
    let mut keys = config_url_paths
        .clone()
        .keys()
        .into_iter()
        .map(|x| x.to_owned())
        .collect::<Vec<String>>();
    keys.sort();
    keys.iter().for_each(|key| {
        let x = config_url_paths.get_key_value(key);
        match x {
            Some((k, v)) => {
                let mut row = Flex::default_fill();
                row.set_spacing(FLEX_SPACING);

                let mut col = Flex::default();
                col.set_align(Align::Inside | Align::Left);
                col.set_label(k.as_str());
                col.end();

                let mut link = Button::default().with_size(BUTTON_WIDTH, BUTTON_HEIGHT);
                let data_src = v.clone().data_src.unwrap_or_default();
                if !data_src.is_empty() {
                    link.set_label(data_src.as_str());
                    // todo: open file
                    link.set_callback(|_| println!("hejhej"));
                } else {
                    let data_text = v.clone().data_text.unwrap_or_default();
                    link.set_label(data_text.as_str());
                    link.set_color(Color::Free);
                }

                row.fixed(&link, BUTTON_WIDTH * 2);
                row.end();
            }
            _ => (),
        }
    });

    // vflex.fixed(&flex, BUTTON_HEIGHT);
    vflex.end();
    tab.end();

    tab
}
