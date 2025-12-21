use crate::ColorScheme;
use crate::bar::font::{Font, FontDraw};
use crate::errors::X11Error;
use crate::layout::tabbed::TAB_BAR_HEIGHT;
use crate::x11::xlib_graphic_context::XLibGC;
use crate::x11::{X11, X11Display};
use x11rb::COPY_DEPTH_FROM_PARENT;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;

pub struct TabBar {
    window: Window,
    width: u16,
    height: u16,
    x_offset: i16,
    y_offset: i16,
    graphics_context: Gcontext,
    pixmap: x11::xlib::Pixmap,
    display: X11Display,
    font_draw: FontDraw,
    scheme_normal: ColorScheme,
    scheme_selected: ColorScheme,
}

impl TabBar {
    pub fn new(
        x11: &mut X11,
        screen_num: usize,
        x: i16,
        y: i16,
        width: u16,
        scheme_normal: ColorScheme,
        scheme_selected: ColorScheme,
    ) -> Result<Self, X11Error> {
        let window = x11.connection.generate_id()?;
        let graphics_context = x11.connection.generate_id()?;

        let height = TAB_BAR_HEIGHT as u16;

        x11.connection.create_window(
            COPY_DEPTH_FROM_PARENT,
            window,
            x11.screen.root,
            x,
            y,
            width,
            height,
            0,
            WindowClass::INPUT_OUTPUT,
            x11.screen.root_visual,
            &CreateWindowAux::new()
                .background_pixel(scheme_normal.background)
                .event_mask(EventMask::EXPOSURE | EventMask::BUTTON_PRESS)
                .override_redirect(1),
        )?;

        x11.connection.create_gc(
            graphics_context,
            window,
            &CreateGCAux::new()
                .foreground(scheme_normal.foreground)
                .background(scheme_normal.background),
        )?;

        x11.connection.map_window(window)?;
        x11.connection.flush()?;

        let depth = unsafe { x11::xlib::XDefaultDepth(x11.display.as_mut(), screen_num as i32) };

        let pixmap = unsafe {
            x11::xlib::XCreatePixmap(
                x11.display.as_mut(),
                window as x11::xlib::Drawable,
                width as u32,
                height as u32,
                depth as u32,
            )
        };

        let font_draw = x11.default_font_draw(pixmap, screen_num as i32)?;

        Ok(Self {
            window,
            width,
            height,
            x_offset: x,
            y_offset: y,
            graphics_context,
            pixmap,
            display: x11.display,
            font_draw,
            scheme_normal,
            scheme_selected,
        })
    }

    pub fn window(&self) -> Window {
        self.window
    }

    pub fn draw(
        &mut self,
        connection: &RustConnection,
        font: &mut Font,
        windows: &[(Window, String)],
        focused_window: Option<Window>,
    ) -> Result<(), X11Error> {
        connection.change_gc(
            self.graphics_context,
            &ChangeGCAux::new().foreground(self.scheme_normal.background),
        )?;
        connection.flush()?;

        let mut gc = XLibGC::new(self.display, self.pixmap, 0, None);
        unsafe {
            x11::xlib::XSetForeground(
                self.display.as_mut(),
                gc.ptr(),
                self.scheme_normal.background as u64,
            );
            x11::xlib::XFillRectangle(
                self.display.as_mut(),
                self.pixmap,
                gc.ptr(),
                0,
                0,
                self.width as u32,
                self.height as u32,
            );
        }

        if windows.is_empty() {
            self.copy_pixmap_to_window();
            return Ok(());
        }

        let tab_width = self.width / windows.len() as u16;
        let mut x_position: i16 = 0;

        for (index, &(window, ref title)) in windows.iter().enumerate() {
            let is_focused = Some(window) == focused_window;
            let scheme = if is_focused {
                &self.scheme_selected
            } else {
                &self.scheme_normal
            };

            let display_title = if title.is_empty() {
                format!("Window {}", index + 1)
            } else {
                title.clone()
            };

            let text_width = font.text_width(&display_title);
            let text_x = x_position + ((tab_width.saturating_sub(text_width)) / 2) as i16;

            let top_padding = 6;
            let text_y = top_padding + font.ascent();

            self.font_draw
                .draw_text(font, scheme.foreground, text_x, text_y, &display_title);

            if is_focused {
                let underline_height = 3;
                let underline_y = self.height as i16 - underline_height;

                let mut gc = XLibGC::new(self.display, self.pixmap, 0, None);
                unsafe {
                    x11::xlib::XSetForeground(
                        self.display.as_mut(),
                        gc.ptr(),
                        scheme.underline as u64,
                    );
                    x11::xlib::XFillRectangle(
                        self.display.as_mut(),
                        self.pixmap,
                        gc.ptr(),
                        x_position as i32,
                        underline_y as i32,
                        tab_width as u32,
                        underline_height as u32,
                    );
                }
            }

            x_position += tab_width as i16;
        }

        self.copy_pixmap_to_window();
        Ok(())
    }

    fn copy_pixmap_to_window(&mut self) {
        let mut gc = XLibGC::new(self.display, self.window as x11::xlib::Drawable, 0, None);
        unsafe {
            x11::xlib::XCopyArea(
                self.display.as_mut(),
                self.pixmap,
                self.window as u64,
                gc.ptr(),
                0,
                0,
                self.width as u32,
                self.height as u32,
                0,
                0,
            );
        }
    }

    pub fn get_clicked_window(&self, windows: &[(Window, String)], click_x: i16) -> Option<Window> {
        if windows.is_empty() {
            return None;
        }

        let tab_width = self.width / windows.len() as u16;
        let tab_index = (click_x as u16 / tab_width) as usize;

        windows.get(tab_index).map(|&(win, _)| win)
    }

    pub fn reposition(
        &mut self,
        x11: &mut X11,
        x: i16,
        y: i16,
        width: u16,
    ) -> Result<(), X11Error> {
        self.x_offset = x;
        self.y_offset = y;
        self.width = width;

        x11.connection.configure_window(
            self.window,
            &ConfigureWindowAux::new()
                .x(x as i32)
                .y(y as i32)
                .width(width as u32),
        )?;

        unsafe {
            x11::xlib::XFreePixmap(self.display.as_mut(), self.pixmap);
        }

        let depth = unsafe { x11::xlib::XDefaultDepth(self.display.as_mut(), 0) };
        self.pixmap = unsafe {
            x11::xlib::XCreatePixmap(
                self.display.as_mut(),
                self.window as x11::xlib::Drawable,
                width as u32,
                self.height as u32,
                depth as u32,
            )
        };

        self.font_draw = x11.default_font_draw(self.pixmap, 0)?;

        x11.connection.flush()?;
        Ok(())
    }

    pub fn hide(&self, connection: &RustConnection) -> Result<(), X11Error> {
        connection.unmap_window(self.window)?;
        connection.flush()?;
        Ok(())
    }

    pub fn show(&self, connection: &RustConnection) -> Result<(), X11Error> {
        connection.map_window(self.window)?;
        connection.flush()?;
        Ok(())
    }
}

impl Drop for TabBar {
    fn drop(&mut self) {
        unsafe {
            x11::xlib::XFreePixmap(self.display.as_mut(), self.pixmap);
        }
    }
}
