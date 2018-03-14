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

void qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_custom_slot(qt_widgets_c_SlotWrapper_SCNotification_ptr* ptr, SCNotification* arg0) {
	ptr->custom_slot(arg0);
}
void qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_delete(qt_widgets_c_SlotWrapper_SCNotification_ptr* ptr) {
	delete ptr;
}
qt_widgets_c_SlotWrapper_SCNotification_ptr* qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_new() {
	return new qt_widgets_c_SlotWrapper_SCNotification_ptr();
}
void qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_set(qt_widgets_c_SlotWrapper_SCNotification_ptr* this_ptr, void (*func)(void*, SCNotification*), void* data) {
  this_ptr->set(func, data);
}
QObject* qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_SCNotification_ptr(qt_widgets_c_SlotWrapper_SCNotification_ptr* ptr) {
	return static_cast<QObject*>(ptr);
}
