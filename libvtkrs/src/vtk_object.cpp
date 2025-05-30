#include "vtk_object.h"
#include "vtk_object.rs.h"

#include <cstdint>
#include <vtkObject.h>

using vtkObject = vtkObject;

void vtk_object_debug_on(vtkObject& obj) {
    obj.DebugOn();
}

void vtk_object_debug_off(vtkObject& obj) {
    obj.DebugOff();
}

void vtk_object_set_debug(vtkObject& obj, bool status) {
    obj.SetDebug(status);
}

bool vtk_object_get_debug(const vtkObject& obj) {
    vtkObject& obj2 = const_cast<vtkObject&>(obj);
    return obj2.GetDebug();
}

void vtk_object_modified(const vtkObject& obj) {
    vtkObject& obj2 = const_cast<vtkObject&>(obj);
    obj2.Modified();
}

void vtk_object_remove_observer(vtkObject& obj, uint64_t tag) {
    obj.RemoveObserver(tag);
}

void vtk_object_remove_observers(vtkObject& obj, uint64_t tag) {
    obj.RemoveObservers(tag);
}

void vtk_object_remove_all_observer(vtkObject& obj) {
    obj.RemoveAllObservers();
}

uint64_t vtk_object_has_observer(const vtkObject& obj, uint64_t event) {
    vtkObject& obj2 = const_cast<vtkObject&>(obj);
    return obj2.HasObserver(event);
}
