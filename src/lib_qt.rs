use std::os::raw::{c_void, c_int, c_uint};

extern "C" {
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_new(parent: *mut ::qt_widgets::widget::Widget) -> *mut ScintillaEditBase;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_delete(this_ptr: *mut ScintillaEditBase);
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_send(this_ptr: *mut ScintillaEditBase, message: c_uint, wparam: c_uint, lparam: c_int) -> *mut c_void;
}

/// C++ type: <span style='color: green;'>```QCustomEventFilter```</span>
#[repr(C)]
pub struct ScintillaEditBase(u8);

impl ScintillaEditBase {
  pub fn new() -> ::qt_core::cpp_utils::CppBox<ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_new(::std::ptr::null_mut()) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
  pub unsafe fn with_parent(parent: *mut ::qt_widgets::widget::Widget) -> ::qt_core::cpp_utils::CppBox<ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_new(parent) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
  pub fn send_mut(&mut self, message: usize, wparam: usize, lparam: usize) -> usize {
    unsafe {
      qt_widgets_c_scintilla_ScintillaEditBase_send(self as *mut ScintillaEditBase, message, wparam, lparam)
    }
  }
}
/*impl Drop for ScintillaEditBase {
  fn drop(&mut self) {
  	unsafe {
      qt_widgets_c_scintilla_ScintillaEditBase_delete(self as *mut ScintillaEditBase)
    }
  }
}*/
impl ::cpp_utils::CppDeletable for ScintillaEditBase {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    qt_widgets_c_scintilla_ScintillaEditBase_delete
  }
}
/*impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::custom_event_filter::CustomEventFilter {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QCustomEventFilter_G_static_cast_QObject_ptr(self as *mut ::custom_event_filter::CustomEventFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QCustomEventFilter_G_static_cast_QObject_ptr(self as *const ::custom_event_filter::CustomEventFilter as *mut ::custom_event_filter::CustomEventFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}
extern "C" fn handler(filter_ptr: *mut ::libc::c_void, target: *mut ::qt_core::object::Object, event: *const ::qt_core::event::Event) -> bool {
    let filter: &mut Box<FnMut(&mut ::qt_core::object::Object,&::qt_core::event::Event) -> bool> = unsafe { ::std::mem::transmute(filter_ptr) };
    unsafe { filter(&mut *target, &*event) as bool }
}*/
