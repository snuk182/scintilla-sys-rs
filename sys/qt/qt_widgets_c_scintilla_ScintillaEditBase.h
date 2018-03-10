#ifndef QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_H
#define QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_H

#ifdef _WIN32
    #ifdef QT_CORE_C_LIBRARY
        #define QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT __declspec(dllexport)
    #else
        #define QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT __declspec(dllimport)
    #endif
#else
    #define QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT
#endif

#include "../scintilla/qt/ScintillaEditBase/ScintillaEditBase.h"

extern "C" {
	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT ScintillaEditBase * qt_widgets_c_scintilla_ScintillaEditBase_new_no_args();
	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT ScintillaEditBase * qt_widgets_c_scintilla_ScintillaEditBase_new(QWidget *parent);
	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT void qt_widgets_c_scintilla_ScintillaEditBase_delete(ScintillaEditBase * thisptr);
	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT sptr_t qt_widgets_c_scintilla_ScintillaEditBase_send(ScintillaEditBase * thisptr, unsigned int iMessage, uptr_t wParam, sptr_t lParam);

	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QFrame(QFrame* ptr);
	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QWidget(QWidget* ptr);
	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QObject(QObject* ptr);

	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT QWidget* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(ScintillaEditBase* ptr);
	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT QObject* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QObject_ptr(ScintillaEditBase* ptr);
	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT QFrame* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QFrame_ptr(ScintillaEditBase* ptr);
	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT QAbstractScrollArea* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QAbstractScrollArea_ptr(ScintillaEditBase* ptr);
}

#endif //QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_H