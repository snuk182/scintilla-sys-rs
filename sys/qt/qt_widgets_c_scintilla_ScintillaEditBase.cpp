#include "qt_widgets_c_scintilla_ScintillaEditBase.h"

ScintillaEditBase * qt_widgets_c_scintilla_ScintillaEditBase_new_no_args() {
	return new ScintillaEditBase();
}
ScintillaEditBase * qt_widgets_c_scintilla_ScintillaEditBase_new(QWidget *parent) {
	return new ScintillaEditBase(parent);
}
void qt_widgets_c_scintilla_ScintillaEditBase_delete(ScintillaEditBase * thisptr) {
	delete thisptr;
}
sptr_t qt_widgets_c_scintilla_ScintillaEditBase_send(ScintillaEditBase * thisptr, unsigned int iMessage, uptr_t wParam, sptr_t lParam) {
	return thisptr->send(iMessage, wParam, lParam);
}

ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
  return dynamic_cast<ScintillaEditBase*>(ptr);
}
ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QFrame(QFrame* ptr) {
  return dynamic_cast<ScintillaEditBase*>(ptr);
}
ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<ScintillaEditBase*>(ptr);
}
ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QObject(QObject* ptr) {
  return dynamic_cast<ScintillaEditBase*>(ptr);
}

QWidget* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(ScintillaEditBase* ptr) {
  return static_cast<QWidget*>(ptr);
}
QObject* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QObject_ptr(ScintillaEditBase* ptr) {
  return static_cast<QObject*>(ptr);
}
QFrame* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QFrame_ptr(ScintillaEditBase* ptr) {
  return static_cast<QFrame*>(ptr);
}
QAbstractScrollArea* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QAbstractScrollArea_ptr(ScintillaEditBase* ptr) {
  return static_cast<QAbstractScrollArea*>(ptr);
}

ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QObject(QObject* ptr) {
	return static_cast<ScintillaEditBase*>(ptr);
}
ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QFrame(QFrame* ptr) {
	return static_cast<ScintillaEditBase*>(ptr);
}
ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr) {
	return static_cast<ScintillaEditBase*>(ptr);
}
ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QWidget(QWidget* ptr) {
	return static_cast<ScintillaEditBase*>(ptr);
}

QMetaObject const * qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_staticMetaObject() {
  return &qt_widgets_c_SlotWrapper_SCNotification_ptr::staticMetaObject;
}
QMetaObject const * qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_metaObject(qt_widgets_c_SlotWrapper_SCNotification_ptr const * this_ptr) {
  return this_ptr->metaObject();
}
void * qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_qt_metacast(qt_widgets_c_SlotWrapper_SCNotification_ptr * this_ptr, char const * arg1) {
  return this_ptr->qt_metacast(arg1);
}
int qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_qt_metacall(qt_widgets_c_SlotWrapper_SCNotification_ptr * this_ptr, QMetaObject::Call arg1, int arg2, void * * arg3) {
  return this_ptr->qt_metacall(arg1, arg2, arg3);
}
QString * qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_tr(char const * s, char const * c, int n) {
  return new QString(qt_widgets_c_SlotWrapper_SCNotification_ptr::tr(s, c, n));
}
QString * qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_trUtf8(char const * s, char const * c, int n) {
  return new QString(qt_widgets_c_SlotWrapper_SCNotification_ptr::trUtf8(s, c, n));
}
qt_widgets_c_SlotWrapper_SCNotification_ptr * qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr(QObject * parent, void (*callback)(void *, SCNotification *), void (*deleter)(void *), void * data) {
  return new qt_widgets_c_SlotWrapper_SCNotification_ptr(parent, callback, deleter, data);
}
void qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_set(qt_widgets_c_SlotWrapper_SCNotification_ptr * this_ptr, void (*callback)(void *, SCNotification *), void (*deleter)(void *), void * data) {
  this_ptr->set(callback, deleter, data);
}
void qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_slot_(qt_widgets_c_SlotWrapper_SCNotification_ptr * this_ptr, SCNotification * arg0) {
  this_ptr->slot_(arg0);
}
void qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_delete(qt_widgets_c_SlotWrapper_SCNotification_ptr * thisptr) {
	delete thisptr;
}
QObject* qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_static_cast_QObject_ptr(qt_widgets_c_SlotWrapper_SCNotification_ptr* ptr) {
  return static_cast<QObject*>(ptr);
}
