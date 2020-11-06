// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use ListItem;
use ListItemFactory;

glib_wrapper! {
    pub struct SignalListItemFactory(Object<gtk_sys::GtkSignalListItemFactory, gtk_sys::GtkSignalListItemFactoryClass>) @extends ListItemFactory;

    match fn {
        get_type => || gtk_sys::gtk_signal_list_item_factory_get_type(),
    }
}

impl SignalListItemFactory {
    pub fn new() -> SignalListItemFactory {
        assert_initialized_main_thread!();
        unsafe {
            ListItemFactory::from_glib_full(gtk_sys::gtk_signal_list_item_factory_new())
                .unsafe_cast()
        }
    }

    pub fn connect_bind<F: Fn(&SignalListItemFactory, &ListItem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn bind_trampoline<F: Fn(&SignalListItemFactory, &ListItem) + 'static>(
            this: *mut gtk_sys::GtkSignalListItemFactory,
            listitem: *mut gtk_sys::GtkListItem,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(listitem))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"bind\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    bind_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_setup<F: Fn(&SignalListItemFactory, &ListItem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn setup_trampoline<
            F: Fn(&SignalListItemFactory, &ListItem) + 'static,
        >(
            this: *mut gtk_sys::GtkSignalListItemFactory,
            listitem: *mut gtk_sys::GtkListItem,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(listitem))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"setup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    setup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_teardown<F: Fn(&SignalListItemFactory, &ListItem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn teardown_trampoline<
            F: Fn(&SignalListItemFactory, &ListItem) + 'static,
        >(
            this: *mut gtk_sys::GtkSignalListItemFactory,
            listitem: *mut gtk_sys::GtkListItem,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(listitem))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"teardown\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    teardown_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_unbind<F: Fn(&SignalListItemFactory, &ListItem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn unbind_trampoline<
            F: Fn(&SignalListItemFactory, &ListItem) + 'static,
        >(
            this: *mut gtk_sys::GtkSignalListItemFactory,
            listitem: *mut gtk_sys::GtkListItem,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(listitem))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"unbind\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    unbind_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SignalListItemFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SignalListItemFactory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SignalListItemFactory")
    }
}
