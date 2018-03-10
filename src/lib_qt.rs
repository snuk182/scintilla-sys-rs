use qt_core::cpp_utils::{CppBox, CppDeletable, StaticCast, DynamicCast, Deleter};
use qt_core::object::Object;
use qt_widgets::widget::Widget;
use qt_widgets::abstract_scroll_area::AbstractScrollArea;
use qt_widgets::frame::Frame;

use std::os::raw::{c_void, c_int, c_uint};
use std::ops::{Deref, DerefMut};

extern "C" {
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_new_no_args() -> *mut ScintillaEditBase;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_new(parent: *mut Widget) -> *mut ScintillaEditBase;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_delete(this_ptr: *mut ScintillaEditBase);
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_send(this_ptr: *mut ScintillaEditBase, message: c_uint, wparam: c_uint, lparam: c_int) -> *mut c_void;
  
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QAbstractScrollArea(ptr: *mut AbstractScrollArea) -> *mut ScintillaEditBase;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QFrame(ptr: *mut Frame) -> *mut ScintillaEditBase;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QWidget(ptr: *mut Widget) -> *mut ScintillaEditBase;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QObject(ptr: *mut Object) -> *mut ScintillaEditBase;

  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(ptr: *mut ScintillaEditBase) -> *mut Widget;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QObject_ptr(ptr: *mut ScintillaEditBase) -> *mut Object;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QFrame_ptr(ptr: *mut ScintillaEditBase) -> *mut Frame;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QAbstractScrollArea_ptr(ptr: *mut ScintillaEditBase) -> *mut AbstractScrollArea;
}

/// C++ type: <span style='color: green;'>```QCustomEventFilter```</span>
#[repr(C)]
pub struct ScintillaEditBase(u8);

impl ScintillaEditBase {
  pub fn new() -> CppBox<ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_new_no_args() };
    unsafe { CppBox::new(ffi_result) }
  }
  pub unsafe fn with_parent(parent: *mut Widget) -> CppBox<ScintillaEditBase> {
    let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_new(parent);
    CppBox::new(ffi_result)
  }
  pub fn send(&mut self, message: u32, wparam: u32, lparam: i32) -> *mut c_void {
    unsafe {
      qt_widgets_c_scintilla_ScintillaEditBase_send(self as *mut ScintillaEditBase, message, wparam, lparam)
    }
  }
}
impl CppDeletable for ScintillaEditBase {
  fn deleter() -> Deleter<Self> {
    qt_widgets_c_scintilla_ScintillaEditBase_delete
  }
}

/*
	void horizontalScrolled(int value);
	void verticalScrolled(int value);
	void horizontalRangeChanged(int max, int page);
	void verticalRangeChanged(int max, int page);
	void notifyChange();
	void linesAdded(int linesAdded);

	// Clients can use this hook to add additional
	// formats (e.g. rich text) to the MIME data.
	void aboutToCopy(QMimeData *data);

	// Scintilla Notifications
	void styleNeeded(int position);
	void charAdded(int ch);
	void savePointChanged(bool dirty);
	void modifyAttemptReadOnly();
	void key(int key);
	void doubleClick(int position, int line);
	void updateUi(int updated);
	void modified(int type, int position, int length, int linesAdded,
	              const QByteArray &text, int line, int foldNow, int foldPrev);
	void macroRecord(int message, uptr_t wParam, sptr_t lParam);
	void marginClicked(int position, int modifiers, int margin);
	void textAreaClicked(int line, int modifiers);
	void needShown(int position, int length);
	void painted();
	void userListSelection(); // Wants some args.
	void uriDropped(const QString &uri);
	void dwellStart(int x, int y);
	void dwellEnd(int x, int y);
	void zoom(int zoom);
	void hotSpotClick(int position, int modifiers);
	void hotSpotDoubleClick(int position, int modifiers);
	void callTipClick();
	void autoCompleteSelection(int position, const QString &text);
	void autoCompleteCancelled();
	void focusChanged(bool focused);

	// Base notifications for compatibility with other Scintilla implementations
	void notify(SCNotification *pscn);
	void command(uptr_t wParam, sptr_t lParam);

	// GUI event notifications needed under Qt
	void buttonPressed(QMouseEvent *event);
	void buttonReleased(QMouseEvent *event);
	void keyPressed(QKeyEvent *event);
	void resized();

*/

pub mod connection {
  use super::{Object, StaticCast};
  pub struct Signals<'a>(&'a super::ScintillaEditBase);
  
  pub struct HorizontalScrolled<'a>(&'a super::ScintillaEditBase);
  impl<'a> ::qt_core::connection::Receiver for HorizontalScrolled<'a> {
    type Arguments = (i32);
    fn object(&self) -> &Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2horizontalScrolled(i32)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HorizontalScrolled<'a> {}
  
  impl<'a> Signals<'a> {
    pub fn horizontal_scrolled(&self) -> HorizontalScrolled {
      HorizontalScrolled(self.0)
    }
  }
  
  pub struct Slots<'a>(&'a super::ScintillaEditBase);
  
  pub struct EventCommand<'a>(&'a super::ScintillaEditBase);
  impl<'a> ::qt_core::connection::Receiver for EventCommand<'a> {
    type Arguments = (u32, i32);
    fn object(&self) -> &Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1event_command(uptr_t, sptr_t)\0"
    }
  }
  impl<'a> Slots<'a> {
    pub fn event_command(&self) -> EventCommand {
      EventCommand(self.0)
    }
  }
  impl super::ScintillaEditBase {
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }
}

impl DynamicCast<ScintillaEditBase> for AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> Option<&mut ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QAbstractScrollArea(self as *mut AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> Option<&ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QAbstractScrollArea(self as *const AbstractScrollArea as *mut AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}
impl DynamicCast<ScintillaEditBase> for Frame {
  fn dynamic_cast_mut(&mut self) -> Option<&mut ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QFrame(self as *mut Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> Option<&ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QFrame(self as *const Frame as *mut Frame) };
    unsafe { ffi_result.as_ref() }
  }
}
impl DynamicCast<ScintillaEditBase> for Widget {
  fn dynamic_cast_mut(&mut self) -> Option<&mut ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QWidget(self as *mut Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> Option<&ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QWidget(self as *const Widget as *mut Widget) };
    unsafe { ffi_result.as_ref() }
  }
}
impl DynamicCast<ScintillaEditBase> for Object {
  fn dynamic_cast_mut(&mut self) -> Option<&mut ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QObject(self as *mut Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> Option<&ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QObject(self as *const Object as *mut Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl StaticCast<Object> for ScintillaEditBase {
  fn static_cast_mut(&mut self) -> &mut Object {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QObject_ptr(self as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &Object {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QObject_ptr(self as *const ScintillaEditBase as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}
impl StaticCast<Widget> for ScintillaEditBase {
  fn static_cast_mut(&mut self) -> &mut Widget {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(self as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &Widget {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(self as *const ScintillaEditBase as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}
impl StaticCast<Frame> for ScintillaEditBase {
  fn static_cast_mut(&mut self) -> &mut Frame {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QFrame_ptr(self as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &Frame {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QFrame_ptr(self as *const ScintillaEditBase as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}
impl StaticCast<AbstractScrollArea> for ScintillaEditBase {
  fn static_cast_mut(&mut self) -> &mut AbstractScrollArea {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QAbstractScrollArea_ptr(self as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &AbstractScrollArea {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QAbstractScrollArea_ptr(self as *const ScintillaEditBase as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl Deref for ScintillaEditBase {
  type Target = Widget;
  fn deref(&self) -> &Widget {
    let ffi_result =
      unsafe {
        qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(self as *const ScintillaEditBase as *mut ScintillaEditBase)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl DerefMut for ScintillaEditBase {
  fn deref_mut(&mut self) -> &mut Widget {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(self as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
