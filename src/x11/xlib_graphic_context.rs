use crate::x11::X11Display;
use x11::xlib::{Drawable, GC, XCreateGC, XFreeGC, XGCValues};

#[derive(Debug, Clone)]
pub struct XLibGC {
    gc: GC,
    display: X11Display,
}

impl XLibGC {
    pub fn new(
        mut display: X11Display,
        drawable: Drawable,
        valuemask: u64,
        values: Option<&mut XGCValues>,
    ) -> XLibGC {
        let values = match values {
            Some(xgcvalues) => xgcvalues as *mut XGCValues,
            None => std::ptr::null_mut(),
        };
        XLibGC {
            gc: unsafe { XCreateGC(display.as_mut(), drawable, valuemask, values) },
            display,
        }
    }

    pub fn ptr(&mut self) -> GC {
        self.gc
    }
}

impl AsMut<GC> for XLibGC {
    fn as_mut(&mut self) -> &mut GC {
        &mut self.gc
    }
}

impl Drop for XLibGC {
    fn drop(&mut self) {
        unsafe { XFreeGC(self.display.as_mut(), self.gc) };
    }
}
