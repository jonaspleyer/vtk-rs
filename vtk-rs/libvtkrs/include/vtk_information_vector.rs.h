// AUTO-GENERATED BY cxxbridge

#include "vtk_information_vector.h"

using vtkInformationVector = ::vtkInformationVector;

extern "C" {
::vtkInformationVector *cxxbridge1$vtk_information_vector_new() noexcept {
  ::vtkInformationVector *(*vtk_information_vector_new$)() = ::vtk_information_vector_new;
  return vtk_information_vector_new$();
}

void cxxbridge1$vtk_information_vector_delete(::vtkInformationVector &information_vector) noexcept {
  void (*vtk_information_vector_delete$)(::vtkInformationVector &) = ::vtk_information_vector_delete;
  vtk_information_vector_delete$(information_vector);
}
} // extern "C"
