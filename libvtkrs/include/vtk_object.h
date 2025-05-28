#pragma once
#include "cxx.h"

#include <cstdint>
#include <vtkObject.h>

void debug_on(vtkObject &obj);
void debug_off(vtkObject &obj);
void set_debug(vtkObject &ptr, bool status);
bool get_debug(const vtkObject &object);
void modified(const vtkObject &ptr);
void remove_observer(vtkObject &ptr, uint64_t tag);
void remove_observers(vtkObject &ptr, uint64_t event);
void remove_all_observers(vtkObject &ptr);
uint64_t has_observer(const vtkObject &ptr, uint64_t event);

void add_observer(vtkObject &obj);
