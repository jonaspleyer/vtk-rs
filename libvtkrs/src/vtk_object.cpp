#include "vtk_object.h"
#include "vtk_object.rs.h"

#include <vtkObject.h>

using vtkObject = vtkObject;

extern "C" void debug_on(vtkObject* ptr) {
    static_cast<vtkObject*>(ptr)->DebugOn();
}

extern "C" void debug_off(vtkObject* ptr) {
    static_cast<vtkObject*>(ptr)->DebugOff();
}

extern "C" void set_debug(vtkObject* ptr, bool status) {
    static_cast<vtkObject*>(ptr)->SetDebug(status);
}

extern "C" bool get_debug(vtkObject* ptr) {
    return static_cast<vtkObject*>(ptr)->GetDebug();
}

extern "C" void modified(vtkObject* ptr) {
    static_cast<vtkObject*>(ptr)->Modified();
}

extern "C" void remove_observer(vtkObject* ptr, unsigned long tag) {
    static_cast<vtkObject*>(ptr)->RemoveObserver(tag);
}

extern "C" void remove_observers(vtkObject* ptr, unsigned long tag) {
    static_cast<vtkObject*>(ptr)->RemoveObservers(tag);
}

extern "C" void remove_all_observer(vtkObject* ptr) {
    static_cast<vtkObject*>(ptr)->RemoveAllObservers();
}

extern "C" int has_observer(vtkObject* ptr, unsigned long event) {
    return static_cast<vtkObject*>(ptr)->HasObserver(event);
}
