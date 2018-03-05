use std::os::raw::{c_void, c_int, c_uint};

extern "C" {
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_new(parent: *mut ::qt_widgets::widget::Widget) -> *mut ScintillaEditBase;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_delete(this_ptr: *mut ScintillaEditBase);
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_send(this_ptr: *mut ScintillaEditBase, message: c_uint, wparam: c_uint, lparam: c_int) -> *mut c_void;
  
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QAbstractScrollArea(ptr: *mut ::qt_widgets::abstract_scroll_area::AbstractScrollArea) -> *mut ScintillaEditBase;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QFrame(ptr: *mut ::qt_widgets::frame::Frame) -> *mut ScintillaEditBase;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QWidget(ptr: *mut ::qt_widgets::widget::Widget) -> *mut ScintillaEditBase;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QObject(ptr: *mut ::qt_core::object::Object) -> *mut ScintillaEditBase;

  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(ptr: *mut ScintillaEditBase) -> *mut ::qt_widgets::widget::Widget;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QObject_ptr(ptr: *mut ScintillaEditBase) -> *mut ::qt_core::object::Object;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QFrame_ptr(ptr: *mut ScintillaEditBase) -> *mut ::qt_widgets::frame::Frame;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QAbstractScrollArea_ptr(ptr: *mut ScintillaEditBase) -> *mut ::qt_widgets::abstract_scroll_area::AbstractScrollArea;
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
    let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_new(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }
  pub fn send(&mut self, message: u32, wparam: u32, lparam: i32) -> *mut c_void {
    unsafe {
      qt_widgets_c_scintilla_ScintillaEditBase_send(self as *mut ScintillaEditBase, message, wparam, lparam)
    }
  }
}
impl ::cpp_utils::CppDeletable for ScintillaEditBase {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
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
  use ::cpp_utils::StaticCast;
  pub struct Signals<'a>(&'a super::ScintillaEditBase);
  
  pub struct HorizontalScrolled<'a>(&'a super::ScintillaEditBase);
  impl<'a> ::qt_core::connection::Receiver for HorizontalScrolled<'a> {
    type Arguments = (i32);
    fn object(&self) -> &::qt_core::object::Object {
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
    fn object(&self) -> &::qt_core::object::Object {
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

impl ::cpp_utils::DynamicCast<ScintillaEditBase> for ::qt_widgets::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QAbstractScrollArea(self as *mut ::qt_widgets::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QAbstractScrollArea(self as *const ::qt_widgets::abstract_scroll_area::AbstractScrollArea as *mut ::qt_widgets::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}
impl ::cpp_utils::DynamicCast<ScintillaEditBase> for ::qt_widgets::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QFrame(self as *mut ::qt_widgets::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QFrame(self as *const ::qt_widgets::frame::Frame as *mut ::qt_widgets::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}
impl ::cpp_utils::DynamicCast<ScintillaEditBase> for ::qt_widgets::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QWidget(self as *mut ::qt_widgets::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QWidget(self as *const ::qt_widgets::widget::Widget as *mut ::qt_widgets::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}
impl ::cpp_utils::DynamicCast<ScintillaEditBase> for ::qt_core::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QObject(self as *mut ::qt_core::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&ScintillaEditBase> {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ScintillaEditBase {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QObject_ptr(self as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QObject_ptr(self as *const ScintillaEditBase as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}
impl ::cpp_utils::StaticCast<::qt_widgets::widget::Widget> for ScintillaEditBase {
  fn static_cast_mut(&mut self) -> &mut ::qt_widgets::widget::Widget {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(self as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_widgets::widget::Widget {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(self as *const ScintillaEditBase as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}
impl ::cpp_utils::StaticCast<::qt_widgets::frame::Frame> for ScintillaEditBase {
  fn static_cast_mut(&mut self) -> &mut ::qt_widgets::frame::Frame {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QFrame_ptr(self as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_widgets::frame::Frame {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QFrame_ptr(self as *const ScintillaEditBase as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}
impl ::cpp_utils::StaticCast<::qt_widgets::abstract_scroll_area::AbstractScrollArea> for ScintillaEditBase {
  fn static_cast_mut(&mut self) -> &mut ::qt_widgets::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QAbstractScrollArea_ptr(self as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_widgets::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QAbstractScrollArea_ptr(self as *const ScintillaEditBase as *mut ScintillaEditBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}
