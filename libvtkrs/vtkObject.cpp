#include <vtkObject.h>

extern "C" void *object_new() {
    vtkObject *object = vtkObject::New();
    return static_cast<void *>(object);
}

extern "C" void object_delete(void *object_ptr) {
    vtkObject *object;
    static_cast<vtkObject *>(object_ptr)->Delete();
}

extern "C" void object_debug_on(void *object_ptr) {
    vtkObject *object;
    static_cast<vtkObject *>(object_ptr)->DebugOn();
}

extern "C" void object_debug_off(void *object_ptr) {
    vtkObject *object;
    static_cast<vtkObject *>(object_ptr)->DebugOff();
}

extern "C" bool object_get_debug(void *object_ptr) {
    vtkObject *object;
    return static_cast<vtkObject *>(object_ptr)->GetDebug();
}

extern "C" void object_set_debug(void *object_ptr, bool flag) {
    vtkObject *object;
    static_cast<vtkObject *>(object_ptr)->SetDebug(flag);
}

