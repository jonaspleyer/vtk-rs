#include <stdio.h>
#include <vtkObjectBase.h>

extern "C" void object_delete(void *object_ptr) {
    vtkObjectBase *object;
    static_cast<vtkObjectBase *>(object_ptr)->Delete();
}

extern "C" void object_fast_delete(void *object_ptr) {
    vtkObjectBase *object;
    static_cast<vtkObjectBase *>(object_ptr)->FastDelete();
}

extern "C" void *object_new() {
    vtkObjectBase *object = vtkObjectBase::New();
    return static_cast<void *>(object);
}

extern "C" const char *object_get_class_name(void *object_ptr) {
    vtkObjectBase *object;
    return static_cast<vtkObjectBase *>(object_ptr)->GetClassName();
}

extern "C" const char *object_get_object_description(void *object_ptr) {
    vtkObjectBase *object;
    std::string descr = static_cast<vtkObjectBase *>(object_ptr)->GetObjectDescription();
    char *desc        = new char[descr.length() + 1];
    strcpy(desc, descr.c_str());
    return desc;
}
