// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use cairo;
use gdk_sys;
use glib::translate::*;
use std::fmt;
use DrawContext;

glib_wrapper! {
    pub struct CairoContext(Object<gdk_sys::GdkCairoContext>) @extends DrawContext;

    match fn {
        get_type => || gdk_sys::gdk_cairo_context_get_type(),
    }
}

impl CairoContext {
    pub fn cairo_create(&self) -> Option<cairo::Context> {
        unsafe {
            from_glib_full(gdk_sys::gdk_cairo_context_cairo_create(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for CairoContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CairoContext")
    }
}
