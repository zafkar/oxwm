use crate::bar::font::FontDraw;
use crate::errors::X11Error;
use crate::x11::atom::AtomCache;
use std::ptr::NonNull;
use x11::xlib::{Display, Drawable, Visual};
use x11rb::connection::Connection;
use x11rb::cursor::Handle as CursorHandle;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;

pub mod atom;
pub mod xlib_graphic_context;

#[derive(Debug, Copy, Clone)]
pub struct X11Display(NonNull<Display>);

impl X11Display {
    pub unsafe fn from_raw(display: *mut Display) -> Option<Self> {
        NonNull::new(display).map(Self)
    }
}

impl AsRef<Display> for X11Display {
    fn as_ref(&self) -> &Display {
        unsafe { self.0.as_ref() }
    }
}

impl AsMut<Display> for X11Display {
    fn as_mut(&mut self) -> &mut Display {
        unsafe { self.0.as_mut() }
    }
}

pub type X11Result<T> = Result<T, X11Error>;

pub struct X11 {
    pub atoms: AtomCache,
    pub display: X11Display,
    pub font: crate::bar::font::Font,
    pub connection: RustConnection,
    pub screen_number: usize,
    pub root: Window,
    pub screen: Screen,
    pub windows: Vec<Window>,
}

impl X11 {
    pub fn new(font: &str) -> X11Result<X11> {
        let (connection, screen_number) = x11rb::connect(None)?;
        let root = connection.setup().roots[screen_number].root;
        let screen = connection.setup().roots[screen_number].clone();

        let normal_cursor = CursorHandle::new(
            &connection,
            screen_number,
            &x11rb::resource_manager::new_from_default(&connection)?,
        )?
        .reply()?
        .load_cursor(&connection, "left_ptr")?;

        connection
            .change_window_attributes(
                root,
                &ChangeWindowAttributesAux::new()
                    .cursor(normal_cursor)
                    .event_mask(
                        EventMask::SUBSTRUCTURE_REDIRECT
                            | EventMask::SUBSTRUCTURE_NOTIFY
                            | EventMask::PROPERTY_CHANGE
                            | EventMask::KEY_PRESS
                            | EventMask::BUTTON_PRESS
                            | EventMask::POINTER_MOTION,
                    ),
            )?
            .check()?;

        let display_ptr = unsafe { x11::xlib::XOpenDisplay(std::ptr::null()) };
        if display_ptr.is_null() {
            return Err(crate::errors::X11Error::DisplayOpenFailed);
        }
        let display = unsafe { X11Display::from_raw(display_ptr) }
            .ok_or(crate::errors::X11Error::DisplayOpenFailed)?;

        let font = crate::bar::font::Font::new(display, screen_number as i32, font)?;

        let atoms = AtomCache::new(&connection)?;

        Ok(Self {
            atoms,
            display,
            font,
            connection,
            screen_number,
            root,
            screen,
            windows: vec![],
        })
    }

    pub fn grab_button(
        &self,
        owner_events: bool,
        event_mask: EventMask,
        pointer_mode: GrabMode,
        keyboard_mode: GrabMode,
        confine_to: u32,
        cursor: u32,
        button: ButtonIndex,
        modifiers: u16,
    ) -> X11Result<x11rb::cookie::VoidCookie<'_, RustConnection>> {
        Ok(self.connection.grab_button(
            owner_events,
            self.root,
            event_mask,
            pointer_mode,
            keyboard_mode,
            confine_to,
            cursor,
            button,
            modifiers.into(),
        )?)
    }

    pub fn default_visual(&mut self, screen_num: i32) -> Option<XVisual> {
        XVisual::from_raw(unsafe { x11::xlib::XDefaultVisual(self.display.as_mut(), screen_num) })
    }

    pub fn default_color_map(&mut self, screen_num: i32) -> u64 {
        unsafe { x11::xlib::XDefaultColormap(self.display.as_mut(), screen_num) }
    }

    pub fn default_font_draw(
        &mut self,
        drawable: Drawable,
        screen_num: i32,
    ) -> X11Result<FontDraw> {
        let visual = self
            .default_visual(screen_num)
            .ok_or(X11Error::FontLoadFailed(
                "Couldn't get default Visual".to_string(),
            ))?;
        let colormap = self.default_color_map(screen_num);
        FontDraw::new(self.display, drawable, visual, colormap)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct XVisual(NonNull<Visual>);

impl XVisual {
    fn from_raw(ptr: *mut Visual) -> Option<XVisual> {
        NonNull::new(ptr).map(XVisual)
    }
}

impl AsMut<Visual> for XVisual {
    fn as_mut(&mut self) -> &mut Visual {
        unsafe { self.0.as_mut() }
    }
}
