// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib::object::Cast;
use glib::translate::*;
use gtk_sys;
use std::fmt;
use Buildable;
use Filter;
use MultiFilter;

glib_wrapper! {
    pub struct EveryFilter(Object<gtk_sys::GtkEveryFilter, gtk_sys::GtkEveryFilterClass, EveryFilterClass>) @extends MultiFilter, Filter, @implements gio::ListModel, Buildable;

    match fn {
        get_type => || gtk_sys::gtk_every_filter_get_type(),
    }
}

impl EveryFilter {
    pub fn new() -> EveryFilter {
        assert_initialized_main_thread!();
        unsafe { Filter::from_glib_full(gtk_sys::gtk_every_filter_new()).unsafe_cast() }
    }
}

impl Default for EveryFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EveryFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EveryFilter")
    }
}
