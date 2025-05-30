use phf::phf_map;

/// Maps JavaScript key names to xdotool key names.
/// Only contains mappings that are different.
static JS_TO_XDOTOOL_MAP: phf::Map<&'static str, &'static str> = phf_map! {
	"Backspace" => "BackSpace",
	"Enter" => "Return",
	"ArrowLeft" => "Left",
	"ArrowUp" => "Up",
	"ArrowRight" => "Right",
	"ArrowDown" => "Down",
	"Control" => "Control_L",
	"Shift" => "Shift_L",
	"Alt" => "Alt_L",
	"Meta" => "Super_L",
	"CapsLock" => "Caps_Lock",
	"PrintScreen" => "Print",
	"ScrollLock" => "Scroll_Lock",
	"PageUp" => "Page_Up",
	"PageDown" => "Page_Down",
	"NumLock" => "Num_Lock",
	"Period" => "KP_Decimal",
	"Comma" => "KP_Separator",
	"Slash" => "KP_Slash",
	"Semicolon" => "KP_Colon",
	"Quote" => "KP_Quote",
	" " => "space",
	"." => "KP_Decimal",
	"," => "KP_Separator",
	"/" => "KP_Slash",
	";" => "KP_Colon",
	"'" => "KP_Quote",
	"0" => "KP_0",
	"1" => "KP_1",
	"2" => "KP_2",
	"3" => "KP_3",
	"4" => "KP_4",
	"5" => "KP_5",
	"6" => "KP_6",
	"7" => "KP_7",
	"8" => "KP_8",
	"9" => "KP_9",
};

pub fn js_to_xdotool(js: &str) -> String {
	match JS_TO_XDOTOOL_MAP.get(js) {
		Some(xdotool) => xdotool.to_string(),
		None => js.to_string(),
	}
}

pub fn js_mouse_to_xdotool(button: i32) -> i32 {
	button + 1
}
