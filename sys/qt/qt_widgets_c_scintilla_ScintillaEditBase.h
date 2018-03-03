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
	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT ScintillaEditBase * qt_widgets_c_scintilla_ScintillaEditBase_new(QWidget *parent);
	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT void qt_widgets_c_scintilla_ScintillaEditBase_delete(ScintillaEditBase * thisptr);
	QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_EXPORT sptr_t qt_widgets_c_scintilla_ScintillaEditBase_send(unsigned int iMessage, uptr_t wParam, sptr_t lParam);
}

#endif //QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_H
