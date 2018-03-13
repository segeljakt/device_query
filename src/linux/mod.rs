extern crate x11;

use linux::x11::xlib;
use linux::x11::keysym;
use std::ptr;
use keymap::Keycode;
use mouse_state::MouseState;
use std::slice;

pub struct DeviceState {
    display: *mut xlib::Display,
}

impl DeviceState {
    pub fn new() -> DeviceState {
        unsafe {
            xlib::XInitThreads();
            let disp = xlib::XOpenDisplay(ptr::null());
            DeviceState { display: disp }
        }
    }

    pub fn query_pointer(&self) -> MouseState {
        unsafe {
            let root;
            root = xlib::XDefaultRootWindow(self.display);
            let mut root_x = 0;
            let mut root_y = 0;
            let mut win_x = 0;
            let mut win_y = 0;
            let mut root_return = 0;
            let mut child_return = 0;
            let mut mask_return = 0;
            xlib::XQueryPointer(
                self.display,
                root,
                &mut root_return,
                &mut child_return,
                &mut root_x,
                &mut root_y,
                &mut win_x,
                &mut win_y,
                &mut mask_return,
            );
            let button1pressed = mask_return & xlib::Button1Mask > 0;
            let button2pressed = mask_return & xlib::Button2Mask > 0;
            let button3pressed = mask_return & xlib::Button3Mask > 0;
            let button4pressed = mask_return & xlib::Button4Mask > 0;
            let button5pressed = mask_return & xlib::Button5Mask > 0;

            // Use 1-based indexing here so people can just query the button
            // number they're interested in directly.
            let button_pressed = vec![
                false,
                button1pressed,
                button2pressed,
                button3pressed,
                button4pressed,
                button5pressed,
            ];
            MouseState {
                coords: (win_x, win_y),
                button_pressed: button_pressed,
            }
        }
    }

    pub fn query_keymap(&self) -> Vec<Keycode> {
        unsafe {
            let keymap: *mut i8 = [0; 32].as_mut_ptr();
            let mut keycodes = vec![];
            xlib::XQueryKeymap(self.display, keymap);
            for (ix, byte) in
                slice::from_raw_parts(keymap, 32).iter().enumerate()
            {
                for bit in 0_u8..8_u8 {
                    let bitmask = 1 << bit;
                    if byte & bitmask != 0 {
                        let keycode = ix as u8 * 8 + bit;
                        let mut keysyms_per_keycode_return = 0_i32;
                        let keysym = xlib::XGetKeyboardMapping(
                            self.display,
                            keycode,
                            1,
                            &mut keysyms_per_keycode_return,
                        );

                        for ks in slice::from_raw_parts(
                            keysym,
                            keysyms_per_keycode_return as usize,
                        ).iter()
                        {
                            let key = self.keysym_to_key(*ks as u32);
                            match key {
                                Some(k) => keycodes.push(k),
                                None => (),
                            }
                        }
                    }
                }
            }
            keycodes.dedup();
            keycodes
        }
    }

    fn keysym_to_key(&self, keysym: u32) -> Option<Keycode> {
        match keysym {
            keysym::XK_A => Some(Keycode::A),
            keysym::XK_B => Some(Keycode::B),
            keysym::XK_C => Some(Keycode::C),
            keysym::XK_D => Some(Keycode::D),
            keysym::XK_E => Some(Keycode::E),
            keysym::XK_F => Some(Keycode::F),
            keysym::XK_G => Some(Keycode::G),
            keysym::XK_H => Some(Keycode::H),
            keysym::XK_I => Some(Keycode::I),
            keysym::XK_J => Some(Keycode::J),
            keysym::XK_K => Some(Keycode::K),
            keysym::XK_L => Some(Keycode::L),
            keysym::XK_M => Some(Keycode::M),
            keysym::XK_N => Some(Keycode::N),
            keysym::XK_O => Some(Keycode::O),
            keysym::XK_P => Some(Keycode::P),
            keysym::XK_Q => Some(Keycode::Q),
            keysym::XK_R => Some(Keycode::R),
            keysym::XK_S => Some(Keycode::S),
            keysym::XK_T => Some(Keycode::T),
            keysym::XK_U => Some(Keycode::U),
            keysym::XK_V => Some(Keycode::V),
            keysym::XK_W => Some(Keycode::W),
            keysym::XK_X => Some(Keycode::X),
            keysym::XK_Y => Some(Keycode::Y),
            keysym::XK_Z => Some(Keycode::Z),
            keysym::XK_space => Some(Keycode::Space),
            keysym::XK_Return => Some(Keycode::Enter),
            _ => None,
        }
    }
}
