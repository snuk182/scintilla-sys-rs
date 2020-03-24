#[allow(unused_parens)]

use qt_core::{Receiver, Signal, AsReceiver};
use qt_core::cpp_core::{CppDeletable, DynamicCast, StaticUpcast, StaticDowncast, CastInto, CppBox, Ref, Ptr};
use qt_core::q_meta_object;
use qt_core::{QString, QMetaObject, QObject, QBox};
use qt_widgets::{QAbstractScrollArea, QFrame, QWidget};

use super::SCNotification;

use std::os::raw::{c_uint, c_void, c_long, c_ulong};

extern "C" {
    pub fn qt_widgets_c_scintilla_ScintillaEditBase_new_no_args() -> *mut QScintillaEditBase;
    pub fn qt_widgets_c_scintilla_ScintillaEditBase_new(parent: *mut QWidget) -> *mut QScintillaEditBase;
    pub fn qt_widgets_c_scintilla_ScintillaEditBase_delete(this_ptr: *mut QScintillaEditBase);
    pub fn qt_widgets_c_scintilla_ScintillaEditBase_send(this_ptr: *mut QScintillaEditBase, message: c_uint, wparam: c_ulong, lparam: c_long) -> *mut c_void;

    pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QAbstractScrollArea(ptr: *mut QAbstractScrollArea) -> *mut QScintillaEditBase;
    pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QFrame(ptr: *mut QFrame) -> *mut QScintillaEditBase;
    pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QWidget(ptr: *mut QWidget) -> *mut QScintillaEditBase;
    pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QObject(ptr: *mut QObject) -> *mut QScintillaEditBase;

    pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QObject(ptr: *mut QObject) -> *mut QScintillaEditBase;
    pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QFrame(ptr: *mut QFrame) -> *mut QScintillaEditBase;
    pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QAbstractScrollArea(ptr: *mut QAbstractScrollArea) -> *mut QScintillaEditBase;
    pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QWidget(ptr: *mut QWidget) -> *mut QScintillaEditBase;

    pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(ptr: *mut QScintillaEditBase) -> *mut QWidget;
    pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QObject_ptr(ptr: *mut QScintillaEditBase) -> *mut QObject;
    pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QFrame_ptr(ptr: *mut QScintillaEditBase) -> *mut QFrame;
    pub fn qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QAbstractScrollArea_ptr(ptr: *mut QScintillaEditBase) -> *mut QAbstractScrollArea;

    pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_metaObject(
        this_ptr: *const SlotOfSCNotification,
    ) -> *const QMetaObject;
    pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr(
        parent: *mut QObject,
        callback: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void, *mut SCNotification)>,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    ) -> *mut SlotOfSCNotification;
    pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_qt_metacall(
        this_ptr: *mut SlotOfSCNotification,
        arg1: q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_qt_metacast(
        this_ptr: *mut SlotOfSCNotification,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void;
    pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_set(
        this_ptr: *mut SlotOfSCNotification,
        callback: ::std::option::Option<
            extern "C" fn(*mut ::std::ffi::c_void, *mut SCNotification),
        >,
        deleter: ::std::option::Option<extern "C" fn(*mut ::std::ffi::c_void)>,
        data: *mut ::std::ffi::c_void,
    );
    pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_slot_(
        this_ptr: *mut SlotOfSCNotification,
        arg0: *mut SCNotification,
    );
    pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_staticMetaObject(
    ) -> *const QMetaObject;
    pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut QString;
    pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_trUtf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut QString;
    pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_delete(
        this_ptr: *mut SlotOfSCNotification,
    );
    pub fn qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_static_cast_QObject_ptr(
        ptr: *mut SlotOfSCNotification
    ) -> *mut QObject;
}

/// C++ type: <span style='color: green;'>```QCustomEventFilter```</span>
#[repr(C)]
pub struct QScintillaEditBase(u8);

impl QScintillaEditBase {
    #[inline(always)]
    pub fn notify(&self) -> Signal<(*mut SCNotification,)> {
        unsafe {
            Signal::new(
                Ref::from_raw(self).expect("attempted to construct a null Ref"),
                ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"2notify(SCNotification*)\0"),
            )
        }
    }
}

impl QScintillaEditBase {
    pub fn new() -> QBox<QScintillaEditBase> {
        let ffi_result = unsafe { qt_widgets_c_scintilla_ScintillaEditBase_new_no_args() };
        unsafe { QBox::new(ffi_result) }
    }
    pub unsafe fn with_parent(parent: *mut QWidget) -> QBox<QScintillaEditBase> {
        let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_new(parent);
        QBox::new(ffi_result)
    }
    pub unsafe fn send(&self, message: u32, wparam: usize, lparam: isize) -> *mut c_void {
        qt_widgets_c_scintilla_ScintillaEditBase_send(self as *const _ as *mut QScintillaEditBase, message, wparam as c_ulong, lparam as c_long)
    }
}
impl CppDeletable for QScintillaEditBase {
    unsafe fn delete(&self) {
        qt_widgets_c_scintilla_ScintillaEditBase_delete(self as *const QScintillaEditBase as *mut QScintillaEditBase)
    }
}

impl ::std::ops::Deref for QScintillaEditBase {
    type Target = QWidget;
    /// Calls C++ function: <span style='color: green;'>```QWidget* static_cast<QWidget*>(QAbstractSpinBox* ptr)```</span>.
    #[inline(always)]
    fn deref(&self) -> &QWidget {
        let ffi_result = {
            unsafe {
                qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(self as *const _ as *mut QScintillaEditBase)
            }
        };
        unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
}

impl DynamicCast<QScintillaEditBase> for QAbstractScrollArea {
    unsafe fn dynamic_cast(ptr: Ptr<Self>) -> Ptr<QScintillaEditBase> {
        let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QAbstractScrollArea(ptr.as_raw_ptr() as *mut QAbstractScrollArea);
        Ptr::from_raw(ffi_result)
    }
}
impl DynamicCast<QScintillaEditBase> for QFrame {
    unsafe fn dynamic_cast(ptr: Ptr<Self>) -> Ptr<QScintillaEditBase> {
        let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QFrame(ptr.as_raw_ptr() as *mut QFrame);
        Ptr::from_raw(ffi_result)
    }
}
impl DynamicCast<QScintillaEditBase> for QWidget {
    unsafe fn dynamic_cast(ptr: Ptr<Self>) -> Ptr<QScintillaEditBase> {
        let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QWidget(ptr.as_raw_ptr() as *mut QWidget);
        Ptr::from_raw(ffi_result)
    }
}
impl DynamicCast<QScintillaEditBase> for QObject {
    unsafe fn dynamic_cast(ptr: Ptr<Self>) -> Ptr<QScintillaEditBase> {
        let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QObject(ptr.as_raw_ptr() as *mut QObject);
        Ptr::from_raw(ffi_result)
    }
}

impl StaticUpcast<QObject> for QScintillaEditBase {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QObject_ptr(ptr.as_raw_ptr() as *mut QScintillaEditBase);
        Ptr::from_raw(ffi_result)
    }
}
impl StaticUpcast<QWidget> for QScintillaEditBase {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QWidget> {
        let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(ptr.as_raw_ptr() as *mut QScintillaEditBase);
        Ptr::from_raw(ffi_result)
    }
}
impl StaticUpcast<QFrame> for QScintillaEditBase {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QFrame> {
        let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QFrame_ptr(ptr.as_raw_ptr() as *mut QScintillaEditBase);
        Ptr::from_raw(ffi_result)
    }
}
impl StaticUpcast<QAbstractScrollArea> for QScintillaEditBase {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QAbstractScrollArea> {
        let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QAbstractScrollArea_ptr(ptr.as_raw_ptr() as *mut QScintillaEditBase);
        Ptr::from_raw(ffi_result)
    }
}

impl StaticDowncast<QScintillaEditBase> for QObject {
    unsafe fn static_downcast(ptr: Ptr<Self>) -> Ptr<QScintillaEditBase> {
        let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QObject(ptr.as_raw_ptr() as *mut QObject);
        Ptr::from_raw(ffi_result)
    }
}

impl StaticDowncast<QScintillaEditBase> for QFrame {
    unsafe fn static_downcast(ptr: Ptr<Self>) -> Ptr<QScintillaEditBase> {
        let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QFrame(ptr.as_raw_ptr() as *mut QFrame);
        Ptr::from_raw(ffi_result)
    }
}

impl StaticDowncast<QScintillaEditBase> for QAbstractScrollArea {
    unsafe fn static_downcast(ptr: Ptr<Self>) -> Ptr<QScintillaEditBase> {
        let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QAbstractScrollArea(ptr.as_raw_ptr() as *mut QAbstractScrollArea);
        Ptr::from_raw(ffi_result)
    }
}

impl StaticDowncast<QScintillaEditBase> for QWidget {
    unsafe fn static_downcast(ptr: Ptr<Self>) -> Ptr<QScintillaEditBase> {
        let ffi_result = qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QWidget(ptr.as_raw_ptr() as *mut QWidget);
        Ptr::from_raw(ffi_result)
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

impl AsReceiver for SlotOfSCNotification {
    type Arguments = (*mut crate::SCNotification,);
    fn as_receiver(&self) -> Receiver<Self::Arguments> {
        unsafe {
            Receiver::new(
                Ref::from_raw(self).expect("attempted to construct a null Ref"),
                ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"slot_(SCNotification*)\0"),
            )
        }
    }
}

#[repr(C)]
pub struct SlotOfSCNotification {
    _unused: u8,
}
impl CppDeletable for SlotOfSCNotification {
    unsafe fn delete(&self) {
        qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_delete(self as *const SlotOfSCNotification as *mut SlotOfSCNotification);
    }
}
impl StaticUpcast<QObject> for SlotOfSCNotification {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        let ffi_result = qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_static_cast_QObject_ptr(ptr.as_raw_ptr() as *mut SlotOfSCNotification);
        Ptr::from_raw(ffi_result)
    }
}
impl SlotOfSCNotification {
    #[inline(always)]
    pub unsafe fn meta_object(&self) -> Ptr<QMetaObject> {
        let ffi_result = {
            qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_metaObject(
                self as *const SlotOfSCNotification,
            )
        };
        Ptr::from_raw(ffi_result as *mut QMetaObject)
    }

    /// Creates a new object.
    #[inline(always)]
    pub unsafe fn new<T: FnMut(Ref<SCNotification>) + 'static>(
        parent: impl CastInto<Ptr<QObject>>,
        callback: T,
    ) -> ::qt_core::QBox<SlotOfSCNotification> {
        let ffi_result = {
            extern "C" fn deleter<T>(data: *mut ::std::ffi::c_void) {
                unsafe {
                    let _ = Box::from_raw(data as *mut T);
                }
            }
            extern "C" fn ffi_callback<
                T: FnMut(Ref<SCNotification>) + 'static,
            >(
                data: *mut ::std::ffi::c_void,
                arg0: *mut SCNotification,
            ) {
                unsafe {
                    (*(data as *mut T))(
                        Ref::from_raw(arg0 as *mut SCNotification)
                            .expect("attempted to construct a null Ref"),
                    )
                }
            }
            let data = Box::into_raw(Box::new(callback)) as *mut ::std::ffi::c_void;
            qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr(CastInto::<Ptr<QObject>>::cast_into(parent).as_raw_ptr() as *mut QObject, Some(ffi_callback::<T>), Some(deleter::<T>), data)
        };
        ::qt_core::QBox::from_raw(ffi_result)
    }

    #[inline(always)]
    pub unsafe fn qt_metacall(
        &self,
        arg1: q_meta_object::Call,
        arg2: ::std::os::raw::c_int,
        arg3: *mut *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int {
        qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_qt_metacall(
            self as *const SlotOfSCNotification as *mut SlotOfSCNotification,
            arg1,
            arg2,
            arg3,
        )
    }

    #[inline(always)]
    pub unsafe fn qt_metacast(
        &self,
        arg1: *const ::std::os::raw::c_char,
    ) -> *mut ::std::ffi::c_void {
        qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_qt_metacast(
            self as *const SlotOfSCNotification as *mut SlotOfSCNotification,
            arg1,
        )
    }

    /// Assigns `callback` as the signal handler.
    ///
    /// `func` will be called each time a connected signal is emitted. Any previously assigned function will be deregistered. Passing `None` will deregister the handler without setting a new one.
    #[inline(always)]
    pub unsafe fn set<T: FnMut(Ref<SCNotification>) + 'static>(
        &self,
        callback: T,
    ) {
        extern "C" fn deleter<T>(data: *mut ::std::ffi::c_void) {
            unsafe {
                let _ = Box::from_raw(data as *mut T);
            }
        }
        extern "C" fn ffi_callback<T: FnMut(Ref<SCNotification>) + 'static>(
            data: *mut ::std::ffi::c_void,
            arg0: *mut SCNotification,
        ) {
            unsafe {
                (*(data as *mut T))(
                    Ref::from_raw(arg0 as *mut SCNotification)
                        .expect("attempted to construct a null Ref"),
                )
            }
        }
        let data = Box::into_raw(Box::new(callback)) as *mut ::std::ffi::c_void;
        qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_set(
            self as *const SlotOfSCNotification as *mut SlotOfSCNotification,
            Some(ffi_callback::<T>),
            Some(deleter::<T>),
            data,
        )
    }

    /// Calls the slot directly, invoking the assigned handler (if any).
    #[inline(always)]
    pub unsafe fn slot(&self, arg0: *mut SCNotification) {
        qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_slot_(
            self as *const SlotOfSCNotification as *mut SlotOfSCNotification,
            arg0,
        )
    }

    #[inline(always)]
    pub unsafe fn static_meta_object() -> Ref<QMetaObject> {
        let ffi_result = {
            qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_staticMetaObject()
        };
        Ref::from_raw(ffi_result as *mut QMetaObject)
            .expect("attempted to construct a null Ref")
    }

    #[inline(always)]
    pub unsafe fn tr(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> CppBox<QString> {
        let ffi_result = {
            qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_tr(s, c, n)
        };
        CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    #[inline(always)]
    pub unsafe fn tr_utf8(
        s: *const ::std::os::raw::c_char,
        c: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> CppBox<QString> {
        let ffi_result = {
            qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_trUtf8(
                s, c, n,
            )
        };
        CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }
}
