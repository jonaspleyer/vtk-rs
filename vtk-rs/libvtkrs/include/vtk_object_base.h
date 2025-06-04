#pragma once
#include "cxx.h"
#include <cstdint>
#include <vtkObjectBase.h>

rust::String vtk_object_base_get_class_name(const vtkObjectBase& obj);
bool vtk_object_base_is_a(const vtkObjectBase& obj, rust::Str class_name);
std::int64_t
vtk_object_base_get_number_of_generations_from_base(const vtkObjectBase& obj, rust::Str base_class);
void vtk_object_base_fast_delete(vtkObjectBase& obj);
std::int64_t vtk_object_base_get_reference_count(const vtkObjectBase& obj);
void vtk_object_base_set_reference_count(vtkObjectBase& obj, std::int64_t count);
bool vtk_object_base_get_is_in_memkind(const vtkObjectBase& obj);
rust::String vtk_object_base_print_self(const vtkObjectBase& obj, std::uint64_t indent);
rust::String vtk_object_base_print_header(const vtkObjectBase& obj, std::uint64_t indent);
rust::String vtk_object_base_print_trailer(const vtkObjectBase& obj, std::uint64_t indent);
rust::String vtk_object_base_get_object_description(const vtkObjectBase& obj);
bool vtk_object_base_uses_garbage_collector(const vtkObjectBase& obj);
