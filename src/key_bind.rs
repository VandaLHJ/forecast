use std::collections::HashMap;
use cosmic::widget::menu::key_bind::KeyBind;
use cosmic::widget::menu::key_bind::Modifier;
use cosmic::iced::keyboard::Key;

use crate::app::Action;

pub fn key_binds() -> HashMap<KeyBind, Action> {
    let mut key_binds = HashMap::new();
    
    macro_rules! bind {
        ([$($modifier:ident),* $(,)?], $key:expr, $action:ident) => {{
            key_binds.insert(
                KeyBind {
                    modifiers: vec![$(Modifier::$modifier),*],
                    key: $key,
                },
                Action::$action,
            );
        }};
    }
    
    bind!([Ctrl], Key::Character("a".into()), AddCity);
    bind!([Ctrl], Key::Character("r".into()), RemoveCity);
    bind!([Ctrl], Key::Character("q".into()), Quit);
    
    key_binds
}
