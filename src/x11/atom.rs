use x11rb::{protocol::xproto::*, rust_connection::RustConnection};

use crate::x11::X11Result;

pub struct AtomCache {
    pub net_current_desktop: Atom,
    pub net_client_info: Atom,
    pub wm_state: Atom,
    pub wm_protocols: Atom,
    pub wm_delete_window: Atom,
    pub net_wm_state: Atom,
    pub net_wm_state_fullscreen: Atom,
    pub net_wm_window_type: Atom,
    pub net_wm_window_type_dialog: Atom,
    pub wm_name: Atom,
    pub net_wm_name: Atom,
    pub utf8_string: Atom,
    pub net_active_window: Atom,
}

impl AtomCache {
    pub fn new(connection: &RustConnection) -> X11Result<Self> {
        let net_current_desktop = connection
            .intern_atom(false, b"_NET_CURRENT_DESKTOP")?
            .reply()?
            .atom;

        let net_client_info = connection
            .intern_atom(false, b"_NET_CLIENT_INFO")?
            .reply()?
            .atom;

        let wm_state = connection.intern_atom(false, b"WM_STATE")?.reply()?.atom;

        let wm_protocols = connection
            .intern_atom(false, b"WM_PROTOCOLS")?
            .reply()?
            .atom;

        let wm_delete_window = connection
            .intern_atom(false, b"WM_DELETE_WINDOW")?
            .reply()?
            .atom;

        let net_wm_state = connection
            .intern_atom(false, b"_NET_WM_STATE")?
            .reply()?
            .atom;

        let net_wm_state_fullscreen = connection
            .intern_atom(false, b"_NET_WM_STATE_FULLSCREEN")?
            .reply()?
            .atom;

        let net_wm_window_type = connection
            .intern_atom(false, b"_NET_WM_WINDOW_TYPE")?
            .reply()?
            .atom;

        let net_wm_window_type_dialog = connection
            .intern_atom(false, b"_NET_WM_WINDOW_TYPE_DIALOG")?
            .reply()?
            .atom;

        let wm_name = AtomEnum::WM_NAME.into();
        let net_wm_name = connection
            .intern_atom(false, b"_NET_WM_NAME")?
            .reply()?
            .atom;
        let utf8_string = connection.intern_atom(false, b"UTF8_STRING")?.reply()?.atom;
        let net_active_window = connection
            .intern_atom(false, b"_NET_ACTIVE_WINDOW")?
            .reply()?
            .atom;

        Ok(Self {
            net_current_desktop,
            net_client_info,
            wm_state,
            wm_protocols,
            wm_delete_window,
            net_wm_state,
            net_wm_state_fullscreen,
            net_wm_window_type,
            net_wm_window_type_dialog,
            wm_name,
            net_wm_name,
            utf8_string,
            net_active_window,
        })
    }
}
