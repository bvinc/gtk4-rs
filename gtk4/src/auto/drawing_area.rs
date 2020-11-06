// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use cairo;
use gdk;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Accessible;
use AccessibleRole;
use Align;
use Buildable;
use ConstraintTarget;
use LayoutManager;
use Overflow;
use Widget;

glib_wrapper! {
    pub struct DrawingArea(Object<gtk_sys::GtkDrawingArea, gtk_sys::GtkDrawingAreaClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        get_type => || gtk_sys::gtk_drawing_area_get_type(),
    }
}

impl DrawingArea {
    pub fn new() -> DrawingArea {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_drawing_area_new()).unsafe_cast() }
    }
}

impl Default for DrawingArea {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct DrawingAreaBuilder {
    content_height: Option<i32>,
    content_width: Option<i32>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<AccessibleRole>,
}

impl DrawingAreaBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> DrawingArea {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref content_height) = self.content_height {
            properties.push(("content-height", content_height));
        }
        if let Some(ref content_width) = self.content_width {
            properties.push(("content-width", content_width));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        let ret = glib::Object::new(DrawingArea::static_type(), &properties)
            .expect("object new")
            .downcast::<DrawingArea>()
            .expect("downcast");
        ret
    }

    pub fn content_height(mut self, content_height: i32) -> Self {
        self.content_height = Some(content_height);
        self
    }

    pub fn content_width(mut self, content_width: i32) -> Self {
        self.content_width = Some(content_width);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }
}

pub const NONE_DRAWING_AREA: Option<&DrawingArea> = None;

pub trait DrawingAreaExt: 'static {
    fn get_content_height(&self) -> i32;

    fn get_content_width(&self) -> i32;

    fn set_content_height(&self, height: i32);

    fn set_content_width(&self, width: i32);

    fn set_draw_func(
        &self,
        draw_func: Option<Box_<dyn Fn(&DrawingArea, &cairo::Context, i32, i32) + 'static>>,
    );

    fn connect_resize<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_content_height_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_content_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DrawingArea>> DrawingAreaExt for O {
    fn get_content_height(&self) -> i32 {
        unsafe { gtk_sys::gtk_drawing_area_get_content_height(self.as_ref().to_glib_none().0) }
    }

    fn get_content_width(&self) -> i32 {
        unsafe { gtk_sys::gtk_drawing_area_get_content_width(self.as_ref().to_glib_none().0) }
    }

    fn set_content_height(&self, height: i32) {
        unsafe {
            gtk_sys::gtk_drawing_area_set_content_height(self.as_ref().to_glib_none().0, height);
        }
    }

    fn set_content_width(&self, width: i32) {
        unsafe {
            gtk_sys::gtk_drawing_area_set_content_width(self.as_ref().to_glib_none().0, width);
        }
    }

    fn set_draw_func(
        &self,
        draw_func: Option<Box_<dyn Fn(&DrawingArea, &cairo::Context, i32, i32) + 'static>>,
    ) {
        let draw_func_data: Box_<
            Option<Box_<dyn Fn(&DrawingArea, &cairo::Context, i32, i32) + 'static>>,
        > = Box_::new(draw_func);
        unsafe extern "C" fn draw_func_func(
            drawing_area: *mut gtk_sys::GtkDrawingArea,
            cr: *mut cairo_sys::cairo_t,
            width: libc::c_int,
            height: libc::c_int,
            user_data: glib_sys::gpointer,
        ) {
            let drawing_area = from_glib_borrow(drawing_area);
            let cr = from_glib_borrow(cr);
            let callback: &Option<Box_<dyn Fn(&DrawingArea, &cairo::Context, i32, i32) + 'static>> =
                &*(user_data as *mut _);
            if let Some(ref callback) = *callback {
                callback(&drawing_area, &cr, width, height)
            } else {
                panic!("cannot get closure...")
            };
        }
        let draw_func = if draw_func_data.is_some() {
            Some(draw_func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn destroy_func(data: glib_sys::gpointer) {
            let _callback: Box_<
                Option<Box_<dyn Fn(&DrawingArea, &cairo::Context, i32, i32) + 'static>>,
            > = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<
            Option<Box_<dyn Fn(&DrawingArea, &cairo::Context, i32, i32) + 'static>>,
        > = draw_func_data;
        unsafe {
            gtk_sys::gtk_drawing_area_set_draw_func(
                self.as_ref().to_glib_none().0,
                draw_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn connect_resize<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn resize_trampoline<P, F: Fn(&P, i32, i32) + 'static>(
            this: *mut gtk_sys::GtkDrawingArea,
            width: libc::c_int,
            height: libc::c_int,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DrawingArea>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DrawingArea::from_glib_borrow(this).unsafe_cast_ref(),
                width,
                height,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"resize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    resize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_content_height_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_height_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkDrawingArea,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DrawingArea>,
        {
            let f: &F = &*(f as *const F);
            f(&DrawingArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::content-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_content_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_content_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_width_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkDrawingArea,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DrawingArea>,
        {
            let f: &F = &*(f as *const F);
            f(&DrawingArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::content-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_content_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DrawingArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DrawingArea")
    }
}
