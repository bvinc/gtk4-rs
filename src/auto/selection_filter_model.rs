// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use SelectionModel;

glib_wrapper! {
    pub struct SelectionFilterModel(Object<gtk_sys::GtkSelectionFilterModel, gtk_sys::GtkSelectionFilterModelClass, SelectionFilterModelClass>) @implements gio::ListModel;

    match fn {
        get_type => || gtk_sys::gtk_selection_filter_model_get_type(),
    }
}

impl SelectionFilterModel {
    pub fn new<P: IsA<SelectionModel>>(model: Option<&P>) -> SelectionFilterModel {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_selection_filter_model_new(
                model.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    pub fn new_for_type(item_type: glib::types::Type) -> SelectionFilterModel {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_selection_filter_model_new_for_type(
                item_type.to_glib(),
            ))
        }
    }
}

pub const NONE_SELECTION_FILTER_MODEL: Option<&SelectionFilterModel> = None;

pub trait SelectionFilterModelExt: 'static {
    fn get_model(&self) -> Option<SelectionModel>;

    fn set_model<P: IsA<SelectionModel>>(&self, model: Option<&P>);

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SelectionFilterModel>> SelectionFilterModelExt for O {
    fn get_model(&self) -> Option<SelectionModel> {
        unsafe {
            from_glib_none(gtk_sys::gtk_selection_filter_model_get_model(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_model<P: IsA<SelectionModel>>(&self, model: Option<&P>) {
        unsafe {
            gtk_sys::gtk_selection_filter_model_set_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkSelectionFilterModel,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SelectionFilterModel>,
        {
            let f: &F = &*(f as *const F);
            f(&SelectionFilterModel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SelectionFilterModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SelectionFilterModel")
    }
}
