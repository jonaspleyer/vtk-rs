#include "vtk_object_base.h"
#include "cxx.h"
#include "vtkIndent.h"
#include "vtk_object_base.rs.h"
#include <sstream>

#include <cstdint>
#include <vtkObjectBase.h>

rust::String vtk_object_base_get_class_name(const vtkObjectBase& obj) {
    return obj.GetClassName();
}

bool vtk_object_base_is_a(const vtkObjectBase& obj, rust::Str class_name) {
    return const_cast<vtkObjectBase&>(obj).IsA(class_name.begin());
}

std::int64_t vtk_object_base_get_number_of_generations_from_base(
    const vtkObjectBase& obj, rust::Str base_class
) {
    return obj.GetNumberOfGenerationsFromBaseType(base_class.begin());
}

void vtk_object_base_fast_delete(vtkObjectBase& obj) {
    obj.FastDelete();
}

std::int64_t vtk_object_base_get_reference_count(const vtkObjectBase& obj) {
    return const_cast<vtkObjectBase&>(obj).GetReferenceCount();
}

void vtk_object_base_set_reference_count(vtkObjectBase& obj, std::int64_t count) {
    obj.SetReferenceCount(count);
}

bool vtk_object_base_get_is_in_memkind(const vtkObjectBase& obj) {
    return obj.GetIsInMemkind();
}

rust::String vtk_object_base_print_self(const vtkObjectBase& obj, std::uint64_t indent) {
    std::stringstream oss;
    const_cast<vtkObjectBase&>(obj).PrintSelf(oss, vtkIndent(indent));
    return oss.str();
}

rust::String vtk_object_base_print_header(const vtkObjectBase& obj, std::uint64_t indent) {
    std::stringstream oss;
    const_cast<vtkObjectBase&>(obj).PrintHeader(oss, vtkIndent(indent));
    return oss.str();
}

rust::String vtk_object_base_print_trailer(const vtkObjectBase& obj, std::uint64_t indent) {
    std::stringstream oss;
    const_cast<vtkObjectBase&>(obj).PrintTrailer(oss, vtkIndent(indent));
    return oss.str();
}

rust::String vtk_object_base_get_object_description(const vtkObjectBase& obj) {
#ifdef VTK094
    return obj.GetObjectDescription();
#else
    return "";
#endif
}

bool vtk_object_base_uses_garbage_collector(const vtkObjectBase& obj) {
#ifdef VTK094
    return const_cast<vtkObjectBase&>(obj).UsesGarbageCollector();
#else
    return "";
#endif
}
