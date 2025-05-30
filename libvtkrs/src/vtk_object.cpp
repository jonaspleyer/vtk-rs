#include "vtk_object.h"
#include "vtk_object.rs.h"

#include <cstdint>
#include <vtkObject.h>

using vtkObject = vtkObject;

void debug_on(vtkObject& obj) {
    obj.DebugOn();
}

void debug_off(vtkObject& obj) {
    obj.DebugOff();
}

void set_debug(vtkObject& obj, bool status) {
    obj.SetDebug(status);
}

bool get_debug(const vtkObject& obj) {
    vtkObject& obj2 = const_cast<vtkObject&>(obj);
    return obj2.GetDebug();
}

void modified(const vtkObject& obj) {
    vtkObject& obj2 = const_cast<vtkObject&>(obj);
    obj2.Modified();
}

void remove_observer(vtkObject& obj, uint64_t tag) {
    obj.RemoveObserver(tag);
}

void remove_observers(vtkObject& obj, uint64_t tag) {
    obj.RemoveObservers(tag);
}

void remove_all_observer(vtkObject& obj) {
    obj.RemoveAllObservers();
}

uint64_t has_observer(const vtkObject& obj, uint64_t event) {
    vtkObject& obj2 = const_cast<vtkObject&>(obj);
    return obj2.HasObserver(event);
}
