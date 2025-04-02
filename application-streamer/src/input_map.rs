use std::collections::HashMap;

/// Maps JavaScript key names to xdotool key names.
/// Only contains mappings that are different.
const JS_TO_XDOTOOL_MAP: HashMap<&str, &str> = HashMap::from([
    ("Backspace", "BackSpace"),
    ("Enter", "Return"),
    ("ArrowLeft", "Left"),
    ("ArrowUp", "Up"),
    ("ArrowRight", "Right"),
    ("ArrowDown", "Down"),
    ("Control", "Control_L"),
    ("Shift", "Shift_L"),
    ("Alt", "Alt_L"),
    ("Meta", "Super_L"),
    ("CapsLock", "Caps_Lock"),
    ("PrintScreen", "Print"),
    ("ScrollLock", "Scroll_Lock"),
    ("PageUp", "Page_Up"),
    ("PageDown", "Page_Down"),
    ("NumLock", "Num_Lock"),
]);

pub fn js_to_xdotool(js: &str) -> String {
    match JS_TO_XDOTOOL_MAP.get(js) {
        Some(xdotool) => xdotool.to_string(),
        None => js.to_string()
    }
}