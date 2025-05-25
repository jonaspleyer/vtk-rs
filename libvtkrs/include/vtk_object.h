#pragma once
#include "cxx.h"

#include <vtkObject.h>

extern "C" void debug_on(vtkObject* ptr);
extern "C" void debug_off(vtkObject* ptr);
extern "C" void set_debug(vtkObject* ptr, bool status);
extern "C" bool get_debug(vtkObject* ptr);
extern "C" void modified(vtkObject* ptr);
extern "C" void remove_observer(vtkObject* ptr, unsigned long tag);
extern "C" void remove_observers(vtkObject* ptr, unsigned long event);
extern "C" void remove_all_observers(vtkObject* ptr);
extern "C" int has_observer(vtkObject* ptr, unsigned long event);

rust::String print_self(vtkObject* ptr, size_t indent);
rust::String get_object_description(vtkObject* ptr);
