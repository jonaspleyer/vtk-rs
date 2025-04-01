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
