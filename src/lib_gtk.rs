use glib::object::{IsA, Object, ObjectExt};
use glib::signal;
use glib::signal::{SignalHandlerId};
use glib::translate::*;
use glib::subclass::SignalId;
use glib::Cast;
use gtk::{Buildable, Container, Widget};

use glib_sys as glib_ffi;
use gobject_sys as gobject_ffi;
use gtk_sys as gtk_ffi;

use std::os::raw::{c_int, c_long, c_uint, c_ulong, c_void};
use std::{mem, fmt, boxed::Box as Box_};

pub type Ptr = c_long;
pub type Uptr = c_ulong;

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

wrapper! {
    pub struct Scintilla(Object<ffi::GtkScintilla, ffi::GtkScintillaClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        type_ => || ffi::scintilla_object_get_type(),
    }
}

impl Scintilla {
    pub fn new() -> Scintilla {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { Widget::from_glib_none(ffi::scintilla_object_new()).unsafe_cast() }
    }
}

impl Default for Scintilla {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<O: super::IsA<super::Scintilla>> Sealed for O {}
}             

pub trait ScintillaExt: IsA<Scintilla> + sealed::Sealed + 'static {
    fn send_message(&self, message: u32, wparam: Uptr, lparam: Ptr) -> Ptr {
        unsafe { ffi::scintilla_object_send_message(self.as_ref().to_glib_none().0, message, wparam, lparam) }
    }
    fn connect_notify<F: Fn(&Self, i32, Ptr, Ptr) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_trampoline<
            P: IsA<Scintilla>,
            F: Fn(&P, i32, Ptr, Ptr) + 'static,
        >(
            this: *mut ffi::GtkScintilla, 
            msg: c_int, 
            notification: glib_ffi::gpointer, 
            data: glib_ffi::gpointer,
        ) {
            let f: &F = &*(data as *const F);
            f(
                Scintilla::from_glib_borrow(this).unsafe_cast_ref(),
                msg, notification as Ptr, data as Ptr,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            signal::connect_raw(
                self.as_ptr() as *mut _,
                b"sci-notify\0".as_ptr() as *const _,
                Some(mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
    fn emit_notify(&self, msg: i32, notification: Ptr, data: Ptr) {
        let _ = self.emit_by_name::<()>("sci-notify", &[&msg, &notification, &data]);
    }
}

impl<O: IsA<Scintilla>> ScintillaExt for O {}

impl fmt::Display for Scintilla {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Scintilla")
    }
}
