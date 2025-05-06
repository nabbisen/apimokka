use apimock::core::config::ConfigUrlPaths;
use fltk::{
    button::Button,
    enums::{Align, Color},
    group::{Flex, FlexType},
    output::Output,
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt},
};

use crate::app::{
    consts::{BUTTON_HEIGHT, BUTTON_WIDTH, FLEX_SPACING},
    utils::open_with_default_app,
};

/// entry point
pub fn handle(config_url_paths: Option<ConfigUrlPaths>) -> Flex {
    let tab = Flex::default_fill().with_label("URL Paths\t\t").row();

    let mut vflex = Flex::default_fill();
    vflex.set_spacing(FLEX_SPACING);
    vflex.set_type(FlexType::Column);

    // let mut selected_content = Output::default().with_size(500, 200);
    // let selected_content_ref = Rc::from(RefCell::from(selected_content.clone()));
    // selected_content.set_value("test");

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
                row.set_align(Align::Left | Align::Inside);
                row.set_spacing(FLEX_SPACING);

                let url_path_key_col = Flex::default();
                let mut url_path = Output::default();
                url_path.set_value(k.as_str());
                url_path_key_col.end();

                let mut url_path_value_col = Flex::default();
                let mut link = Output::default().with_size(BUTTON_WIDTH, BUTTON_HEIGHT);
                link.set_color(Color::BackGround);
                let data_src = v.clone().data_src.unwrap_or_default();
                if !data_src.is_empty() {
                    link.set_value(format!("[file] {}", data_src).as_str());
                    link.set_text_color(Color::Blue);
                    let mut button = Button::default().with_size(10, BUTTON_HEIGHT / 2);
                    // todo: open file
                    // let selected_content_ref = Rc::clone(&selected_content_ref);
                    // button.set_callback(move |_| {
                    //     let mut selected_content_ref = selected_content_ref.borrow_mut();
                    //     selected_content_ref.set_value(data_src.as_str());
                    // });
                    button.set_color(Color::Blue);
                    button.set_callback(move |_| {
                        let _ = open_with_default_app(data_src.as_str());
                    });
                    url_path_value_col.fixed(&button, 10);
                    url_path_value_col.fixed(&link, BUTTON_WIDTH * 2);
                } else {
                    let data_text = v.clone().data_text;
                    if let Some(data_text) = data_text {
                        link.set_value(format!("[data] {}", data_text).as_str());
                    } else {
                        link.set_value(format!("[http status] {}", v.clone().code).as_str());
                    }
                    url_path_value_col.fixed(&link, BUTTON_WIDTH * 2);
                }
                url_path_value_col.end();

                row.fixed(&url_path_value_col, BUTTON_WIDTH * 2 + 20);
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
