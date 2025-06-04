#include "cxx.h"
#include <vtkDataObject.h>

vtkDataObject* vtk_data_object_new();
void vtk_data_object_delete(vtkDataObject& data_object);
void vtk_data_object_initialize(vtkDataObject& data_object);
void vtk_data_object_release_data(vtkDataObject& data_object);
