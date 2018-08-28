use qt_core::cpp_utils::{CppBox, CppDeletable, StaticCast, DynamicCast, UnsafeStaticCast, Deleter};
use qt_core::connection::{Signal, Receiver};
use qt_core::object::Object;
use qt_widgets::widget::Widget;
use qt_widgets::abstract_scroll_area::AbstractScrollArea;
use qt_widgets::frame::Frame;

use super::SCNotification;

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
  
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QObject(ptr: *mut Object) -> *mut ScintillaEditBase;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QFrame(ptr: *mut Frame) -> *mut ScintillaEditBase;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QAbstractScrollArea(ptr: *mut AbstractScrollArea) -> *mut ScintillaEditBase;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QWidget(ptr: *mut Widget) -> *mut ScintillaEditBase;

  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(ptr: *mut ScintillaEditBase) -> *mut Widget;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QObject_ptr(ptr: *mut ScintillaEditBase) -> *mut Object;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QFrame_ptr(ptr: *mut ScintillaEditBase) -> *mut Frame;
  pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QAbstractScrollArea_ptr(ptr: *mut ScintillaEditBase) -> *mut AbstractScrollArea;
  
  pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_custom_slot(ptr: *mut slots::RawSlotSCNotificationPtr, arg0: *const SCNotification);
  pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_new() -> *mut slots::RawSlotSCNotificationPtr;
  pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_set(ptr: *mut slots::RawSlotSCNotificationPtr, func: extern "C" fn(*mut c_void, *const SCNotification), data: *mut c_void);
  pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_delete(ptr: *mut slots::RawSlotSCNotificationPtr);
  pub fn qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_SCNotification_ptr(ptr: *mut slots::RawSlotSCNotificationPtr) -> *mut Object;
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
  pub unsafe fn send(&self, message: u32, wparam: u32, lparam: i32) -> *mut c_void {
    qt_widgets_c_scintilla_ScintillaEditBase_send(self as *const _ as *mut ScintillaEditBase, message, wparam, lparam)
  }
}
impl CppDeletable for ScintillaEditBase {
  fn deleter() -> Deleter<Self> {
    qt_widgets_c_scintilla_ScintillaEditBase_delete
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

impl UnsafeStaticCast<ScintillaEditBase> for Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ScintillaEditBase {
    let ffi_result =
      qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QObject(self as *mut Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &ScintillaEditBase {
    let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QObject(self as *const Object as *mut Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl UnsafeStaticCast<ScintillaEditBase> for Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ScintillaEditBase {
    let ffi_result =
      qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QFrame(self as *mut Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &ScintillaEditBase {
    let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QFrame(self as *const Frame as *mut Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl UnsafeStaticCast<ScintillaEditBase> for AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ScintillaEditBase {
    let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QAbstractScrollArea(self as *mut AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &ScintillaEditBase {
    let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QAbstractScrollArea(self as *const AbstractScrollArea as *mut AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl UnsafeStaticCast<ScintillaEditBase> for Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ScintillaEditBase {
    let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QWidget(self as *mut Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &ScintillaEditBase {
    let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QWidget(self as *const Widget as *mut Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
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
  use super::*;
  pub struct Signals<'a>(&'a ScintillaEditBase);
  
  pub struct HorizontalScrolled<'a>(&'a ScintillaEditBase);
  impl<'a> Receiver for HorizontalScrolled<'a> {
    type Arguments = (i32);
    fn object(&self) -> &Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2horizontalScrolled(i32)\0"
    }
  }
  impl<'a> Signal for HorizontalScrolled<'a> {}
  
  pub struct Notify<'a>(&'a ScintillaEditBase);
  impl<'a> Receiver for Notify<'a> {
    type Arguments = (&'static SCNotification,);
    fn object(&self) -> &Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2notify(SCNotification*)\0"
    }
  }
  impl<'a> Signal for Notify<'a> {}
  
  impl<'a> Signals<'a> {
    pub fn horizontal_scrolled(&self) -> HorizontalScrolled {
      HorizontalScrolled(self.0)
    }
    pub fn notify(&self) -> Notify {
      Notify(self.0)
    }
  }
  
  pub struct Slots<'a>(&'a ScintillaEditBase);
  
  pub struct EventCommand<'a>(&'a ScintillaEditBase);
  impl<'a> Receiver for EventCommand<'a> {
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
  impl ScintillaEditBase {
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }
}


pub mod slots {
  use super::*;
  
  #[repr(C)]
  pub struct RawSlotSCNotificationPtr(u8);
  
  impl StaticCast<Object> for RawSlotSCNotificationPtr {
    fn static_cast_mut(&mut self) -> &mut Object {
      let ffi_result = unsafe { qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_SCNotification_ptr(self as *mut RawSlotSCNotificationPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &Object {
      let ffi_result = unsafe { qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_SCNotification_ptr(self as *const RawSlotSCNotificationPtr as *mut RawSlotSCNotificationPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
}

  impl super::Receiver for RawSlotSCNotificationPtr {
    type Arguments = (&'static SCNotification,);
    fn object(&self) -> &Object {
      StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(SCNotification*)\0"
    }
  }
  impl RawSlotSCNotificationPtr {
    pub fn custom_slot(&mut self, arg0: &'static SCNotification) {
      unsafe { qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_custom_slot(self as *mut RawSlotSCNotificationPtr, arg0) }
    }

    pub fn new() -> CppBox<RawSlotSCNotificationPtr> {
      let ffi_result = unsafe { qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_new() };
      unsafe { CppBox::new(ffi_result) }
    }

    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut c_void,
                                          *const SCNotification),
                      data: *mut c_void) {
      qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_set(self as *mut RawSlotSCNotificationPtr, func, data)
    }
  }

  impl CppDeletable for RawSlotSCNotificationPtr {
    fn deleter() -> Deleter<Self> {
      qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_delete
    }
  }
}

pub struct SlotSCNotificationPtr<'a> {
  wrapper: CppBox<slots::RawSlotSCNotificationPtr>,
  func: Option<Box<Box<FnMut(&'static SCNotification) + 'a>>>,
}

impl<'a> SlotSCNotificationPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static SCNotification) + 'a>(f: F) -> SlotSCNotificationPtr<'a> {
    let mut obj = SlotSCNotificationPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static SCNotification) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static SCNotification) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_scnotification_ptr_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotSCNotificationPtr<'a> {
  fn default() -> Self {
    SlotSCNotificationPtr {
      wrapper: slots::RawSlotSCNotificationPtr::new(),
      func: None,
    }
  }
}

impl<'a> Receiver for SlotSCNotificationPtr<'a> {
  type Arguments = (&'static SCNotification,);
  fn object(&self) -> &Object {
    Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <slots::RawSlotSCNotificationPtr as Receiver>::receiver_id()
  }
}

//impl ArgumentsCompatible<(&'static SCNotification,)> for (&'static SCNotification,) {}

extern "C" fn slot_scnotification_ptr_callback(data: *mut c_void,
                                                       arg0: *const SCNotification) {
  let func: &mut Box<FnMut(&'static SCNotification)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { &*arg0 });
}

