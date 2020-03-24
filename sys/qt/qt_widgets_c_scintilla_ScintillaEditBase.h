#ifndef QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_H
#define QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_H

#include "qt_core_c_global.h"
#include "../scintilla/qt/ScintillaEditBase/ScintillaEditBase.h"

class qt_widgets_c_SlotWrapper_SCNotification_ptr : public QObject {
    Q_OBJECT
public:
	qt_widgets_c_SlotWrapper_SCNotification_ptr(QObject* parent, void (*callback)(void *, SCNotification *), void (*deleter)(void*), void* data)
    : QObject(parent)
    {
        set(callback, deleter, data);
    }

    void set(void (*callback)(void *, SCNotification *), void (*deleter)(void*), void* data) {
        m_callback.set(callback, deleter, data);
    }

public Q_SLOTS:
    void slot_(SCNotification * arg0) {
        auto callback = m_callback.get();
        if (callback) {
            callback(m_callback.data(), arg0);
        }
    }

private:
    ritual::Callback<void (*)(void *, SCNotification *)> m_callback;
};

extern "C" {
	RITUAL_EXPORT ScintillaEditBase * qt_widgets_c_scintilla_ScintillaEditBase_new_no_args();
	RITUAL_EXPORT ScintillaEditBase * qt_widgets_c_scintilla_ScintillaEditBase_new(QWidget *parent);
	RITUAL_EXPORT void qt_widgets_c_scintilla_ScintillaEditBase_delete(ScintillaEditBase * thisptr);
	RITUAL_EXPORT sptr_t qt_widgets_c_scintilla_ScintillaEditBase_send(ScintillaEditBase * thisptr, unsigned int iMessage, uptr_t wParam, sptr_t lParam);

	RITUAL_EXPORT ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
	RITUAL_EXPORT ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QFrame(QFrame* ptr);
	RITUAL_EXPORT ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QWidget(QWidget* ptr);
	RITUAL_EXPORT ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_dynamic_cast_ScintillaEditBase_ptr_QObject(QObject* ptr);

	RITUAL_EXPORT QWidget* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QWidget_ptr(ScintillaEditBase* ptr);
	RITUAL_EXPORT QObject* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QObject_ptr(ScintillaEditBase* ptr);
	RITUAL_EXPORT QFrame* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QFrame_ptr(ScintillaEditBase* ptr);
	RITUAL_EXPORT QAbstractScrollArea* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_QAbstractScrollArea_ptr(ScintillaEditBase* ptr);

	RITUAL_EXPORT ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QObject(QObject* ptr);
	RITUAL_EXPORT ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QFrame(QFrame* ptr);
	RITUAL_EXPORT ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QAbstractScrollArea(QAbstractScrollArea* ptr);
	RITUAL_EXPORT ScintillaEditBase* qt_widgets_c_scintilla_ScintillaEditBase_G_static_cast_ScintillaEditBase_ptr_QWidget(QWidget* ptr);

	RITUAL_EXPORT QMetaObject const * qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_staticMetaObject();
	RITUAL_EXPORT QMetaObject const * qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_metaObject(qt_widgets_c_SlotWrapper_SCNotification_ptr const * this_ptr);
	RITUAL_EXPORT void * qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_qt_metacast(qt_widgets_c_SlotWrapper_SCNotification_ptr * this_ptr, char const * arg1);
	RITUAL_EXPORT int qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_qt_metacall(qt_widgets_c_SlotWrapper_SCNotification_ptr * this_ptr, QMetaObject::Call arg1, int arg2, void * * arg3);
	RITUAL_EXPORT QString * qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_tr(char const * s, char const * c, int n);
	RITUAL_EXPORT QString * qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_trUtf8(char const * s, char const * c, int n);
	RITUAL_EXPORT qt_widgets_c_SlotWrapper_SCNotification_ptr * qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr(QObject * parent, void (*callback)(void *, SCNotification *), void (*deleter)(void *), void * data);
	RITUAL_EXPORT void qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_set(qt_widgets_c_SlotWrapper_SCNotification_ptr * this_ptr, void (*callback)(void *, SCNotification *), void (*deleter)(void *), void * data);
	RITUAL_EXPORT void qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_slot_(qt_widgets_c_SlotWrapper_SCNotification_ptr * this_ptr, SCNotification * arg0);
	RITUAL_EXPORT void qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_delete(qt_widgets_c_SlotWrapper_SCNotification_ptr * thisptr);
	RITUAL_EXPORT QObject* qt_core_c_qt_core_c_SlotWrapper_SCNotification_ptr_static_cast_QObject_ptr(qt_widgets_c_SlotWrapper_SCNotification_ptr* ptr);
}

#endif //QT_WIDGETS_C_SCINTILLA_SCINTILLA_EDIT_BASE_H
