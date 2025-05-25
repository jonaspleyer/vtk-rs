#include "cxx.h"
#include "vtk_data_object.h"
#include "vtk_data_object.rs.h"

#include <vtkDataObject.h>


vtkDataObject* data_object_new() {
    return vtkDataObject::New();
}

void data_object_delete(vtkDataObject* data_object) {
    data_object->Delete();
}
