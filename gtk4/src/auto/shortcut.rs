// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use ShortcutAction;
use ShortcutTrigger;

glib_wrapper! {
    pub struct Shortcut(Object<gtk_sys::GtkShortcut, gtk_sys::GtkShortcutClass>);

    match fn {
        get_type => || gtk_sys::gtk_shortcut_get_type(),
    }
}

impl Shortcut {
    pub fn new<P: IsA<ShortcutTrigger>, Q: IsA<ShortcutAction>>(
        trigger: Option<&P>,
        action: Option<&Q>,
    ) -> Shortcut {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_shortcut_new(
                trigger.map(|p| p.as_ref()).to_glib_full(),
                action.map(|p| p.as_ref()).to_glib_full(),
            ))
        }
    }

    //pub fn with_arguments<P: IsA<ShortcutTrigger>, Q: IsA<ShortcutAction>>(trigger: Option<&P>, action: Option<&Q>, format_string: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Shortcut {
    //    unsafe { TODO: call gtk_sys:gtk_shortcut_new_with_arguments() }
    //}
}

#[derive(Clone, Default)]
pub struct ShortcutBuilder {
    action: Option<ShortcutAction>,
    arguments: Option<glib::Variant>,
    trigger: Option<ShortcutTrigger>,
}

impl ShortcutBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Shortcut {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref action) = self.action {
            properties.push(("action", action));
        }
        if let Some(ref arguments) = self.arguments {
            properties.push(("arguments", arguments));
        }
        if let Some(ref trigger) = self.trigger {
            properties.push(("trigger", trigger));
        }
        let ret = glib::Object::new(Shortcut::static_type(), &properties)
            .expect("object new")
            .downcast::<Shortcut>()
            .expect("downcast");
        ret
    }

    pub fn action<P: IsA<ShortcutAction>>(mut self, action: &P) -> Self {
        self.action = Some(action.clone().upcast());
        self
    }

    pub fn arguments(mut self, arguments: &glib::Variant) -> Self {
        self.arguments = Some(arguments.clone());
        self
    }

    pub fn trigger<P: IsA<ShortcutTrigger>>(mut self, trigger: &P) -> Self {
        self.trigger = Some(trigger.clone().upcast());
        self
    }
}

pub const NONE_SHORTCUT: Option<&Shortcut> = None;

pub trait ShortcutExt: 'static {
    fn get_action(&self) -> Option<ShortcutAction>;

    fn get_arguments(&self) -> Option<glib::Variant>;

    fn get_trigger(&self) -> Option<ShortcutTrigger>;

    fn set_action<P: IsA<ShortcutAction>>(&self, action: Option<&P>);

    fn set_arguments(&self, args: Option<&glib::Variant>);

    fn set_trigger<P: IsA<ShortcutTrigger>>(&self, trigger: Option<&P>);

    fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_arguments_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_trigger_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Shortcut>> ShortcutExt for O {
    fn get_action(&self) -> Option<ShortcutAction> {
        unsafe {
            from_glib_none(gtk_sys::gtk_shortcut_get_action(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_arguments(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(gtk_sys::gtk_shortcut_get_arguments(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_trigger(&self) -> Option<ShortcutTrigger> {
        unsafe {
            from_glib_none(gtk_sys::gtk_shortcut_get_trigger(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_action<P: IsA<ShortcutAction>>(&self, action: Option<&P>) {
        unsafe {
            gtk_sys::gtk_shortcut_set_action(
                self.as_ref().to_glib_none().0,
                action.map(|p| p.as_ref()).to_glib_full(),
            );
        }
    }

    fn set_arguments(&self, args: Option<&glib::Variant>) {
        unsafe {
            gtk_sys::gtk_shortcut_set_arguments(
                self.as_ref().to_glib_none().0,
                args.to_glib_none().0,
            );
        }
    }

    fn set_trigger<P: IsA<ShortcutTrigger>>(&self, trigger: Option<&P>) {
        unsafe {
            gtk_sys::gtk_shortcut_set_trigger(
                self.as_ref().to_glib_none().0,
                trigger.map(|p| p.as_ref()).to_glib_full(),
            );
        }
    }

    fn connect_property_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_action_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkShortcut,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Shortcut>,
        {
            let f: &F = &*(f as *const F);
            f(&Shortcut::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::action\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_action_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_arguments_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_arguments_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkShortcut,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Shortcut>,
        {
            let f: &F = &*(f as *const F);
            f(&Shortcut::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::arguments\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_arguments_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_trigger_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_trigger_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkShortcut,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Shortcut>,
        {
            let f: &F = &*(f as *const F);
            f(&Shortcut::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::trigger\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_trigger_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Shortcut")
    }
}
