#pragma once
#include "cxx.h"

#include <vtkObject.h>

void debug_on(vtkObject &obj);
void debug_off(vtkObject &obj);
void set_debug(vtkObject &ptr, bool status);
bool get_debug(const vtkObject &object);
void modified(const vtkObject &ptr);
void remove_observer(vtkObject &ptr, unsigned long tag);
void remove_observers(vtkObject &ptr, unsigned long event);
void remove_all_observers(vtkObject &ptr);
int64_t has_observer(const vtkObject &ptr, unsigned long event);

void add_observer(vtkObject &obj);
