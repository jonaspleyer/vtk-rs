#pragma once
#include "cxx.h"

#include <cstdint>
#include <vtkObject.h>

void vtk_object_debug_on(vtkObject& obj);
void vtk_object_debug_off(vtkObject& obj);
void vtk_object_set_debug(vtkObject& ptr, bool status);
bool vtk_object_get_debug(const vtkObject& object);
void vtk_object_modified(const vtkObject& ptr);
void vtk_object_remove_observer(vtkObject& ptr, uint64_t tag);
void vtk_object_remove_observers(vtkObject& ptr, uint64_t event);
void vtk_object_remove_all_observers(vtkObject& ptr);
uint64_t vtk_object_has_observer(const vtkObject& ptr, uint64_t event);
void vtk_object_add_observer(vtkObject& obj);
