#include "vtk_data_object.h"
#include "cxx.h"
#include "vtk_data_object.rs.h"

#include <vtkDataObject.h>

vtkDataObject* vtk_data_object_new() {
    return vtkDataObject::New();
}

void vtk_data_object_delete(vtkDataObject& data_object) {
    data_object.Delete();
}

void vtk_data_object_initialize(vtkDataObject& data_object) {
    data_object.Initialize();
}

void vtk_data_object_release_data(vtkDataObject& data_object) {
    data_object.ReleaseData();
}
