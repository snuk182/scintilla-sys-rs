use glib::object::{Downcast, IsA, Object, ObjectExt};
use glib::signal::connect;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use gtk::{Buildable, Container, Widget};

use glib_sys as glib_ffi;
use gobject_sys as gobject_ffi;
use gtk_sys as gtk_ffi;

use std::os::raw::{c_int, c_long, c_uint, c_ulong, c_void};
use std::{mem, ptr};

const SIGNAL_NOTIFY: &str = "sci-notify";

pub type Ptr = c_long;
pub type Uptr = c_ulong;

macro_rules! callback_guard {
    () => {
        let _guard = ::glib::CallbackGuard::new();
        if cfg!(debug_assertions) {
            assert_initialized_main_thread!();
        }
    };
}
macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    };
}

pub mod ffi {
    extern "C" {
        pub fn scintilla_object_get_type() -> ::glib_sys::GType;
        pub fn scintilla_object_new() -> *mut ::gtk_sys::GtkWidget;
        pub fn scintilla_object_send_message(sci: *mut GtkScintilla, message: super::c_uint, wparam: super::Uptr, lparam: super::Ptr) -> super::Ptr;
    }

    #[repr(C)]
    pub struct GtkScintillaClass {
        _truncated_record_marker: super::c_void,
        // /*Ignored*/field parent_class has incomplete type
    }
    impl ::std::fmt::Debug for GtkScintillaClass {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkScintillaClass @ {:?}", self as *const _)).finish()
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GtkScintilla {
        pub container: ::gtk_sys::GtkContainer,
        _private: *mut super::c_void,
    }
    impl ::std::fmt::Debug for GtkScintilla {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkScintilla @ {:?}", self as *const _)).field("container", &self.container).field("_private", &self._private).finish()
        }
    }
}

glib_wrapper! {
    pub struct Scintilla(Object<ffi::GtkScintilla, ffi::GtkScintillaClass>): [
         Container => gtk_ffi::GtkContainer,
         Widget => gtk_ffi::GtkWidget,
         Buildable => gtk_ffi::GtkBuildable,
    ];

    match fn {
        get_type => || ffi::scintilla_object_get_type(),
    }
}

impl Scintilla {
    pub fn new() -> Scintilla {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::scintilla_object_new()).downcast_unchecked() }
    }
}
impl Default for Scintilla {
    fn default() -> Self {
        Self::new()
    }
}
pub trait ScintillaExt {
    fn send_message(&self, message: u32, wparam: Uptr, lparam: Ptr) -> Ptr;
    fn connect_notify<F: Fn(&Self, i32, Ptr, Ptr) + 'static>(&self, f: F) -> SignalHandlerId;
    fn emit_notify(&self, msg: i32, notification: Ptr, data: Ptr);
}

impl<O: IsA<Scintilla> + IsA<Object> + ObjectExt> ScintillaExt for O {
    fn send_message(&self, message: u32, wparam: Uptr, lparam: Ptr) -> Ptr {
        unsafe { ffi::scintilla_object_send_message(self.to_glib_none().0, message, wparam, lparam) }
    }
    fn connect_notify<F: Fn(&Self, i32, Ptr, Ptr) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box<Box<Fn(&Self, i32, Ptr, Ptr) + 'static>> = Box::new(Box::new(f));
            connect(self.to_glib_none().0, SIGNAL_NOTIFY, mem::transmute(notify_trampoline::<Self> as usize), Box::into_raw(f) as *mut _)
        }
    }
    fn emit_notify(&self, msg: i32, notification: Ptr, data: Ptr) {
        let _ = self.emit(SIGNAL_NOTIFY, &[&msg, &notification, &data]).unwrap();
    }
}
unsafe extern "C" fn notify_trampoline<P>(this: *mut ffi::GtkScintilla, msg: c_int, notification: glib_ffi::gpointer, data: glib_ffi::gpointer)
where
    P: IsA<Scintilla>,
{
    callback_guard!();
    let f: &&(Fn(&P, i32, Ptr, Ptr) + 'static) = mem::transmute(data);
    f(&Scintilla::from_glib_borrow(this).downcast_unchecked(), msg, notification as Ptr, data as Ptr)
}
