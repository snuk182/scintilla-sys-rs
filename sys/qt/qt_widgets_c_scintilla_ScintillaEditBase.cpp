#include "qt_widgets_c_scintilla_ScintillaEditBase.h"

ScintillaEditBase * qt_widgets_c_scintilla_ScintillaEditBase_new(QWidget *parent) {
	return new ScintillaEditBase(parent);
}
void qt_widgets_c_scintilla_ScintillaEditBase_delete(ScintillaEditBase * thisptr) {
	delete thisptr;
}
sptr_t qt_widgets_c_scintilla_ScintillaEditBase_send(ScintillaEditBase * thisptr, unsigned int iMessage, uptr_t wParam, sptr_t lParam) {
	return thisptr->send(iMessage, wParam, lParam);
}
