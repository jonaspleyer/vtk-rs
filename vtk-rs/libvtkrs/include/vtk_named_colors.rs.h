// AUTO-GENERATED BY cxxbridge

#include "vtk_named_colors.h"

using vtkNamedColors = ::vtkNamedColors;

extern "C" {
::vtkNamedColors *cxxbridge1$named_colors_new() noexcept {
  ::vtkNamedColors *(*named_colors_new$)() = ::named_colors_new;
  return named_colors_new$();
}

void cxxbridge1$named_colors_delete(::vtkNamedColors &named_colors) noexcept {
  void (*named_colors_delete$)(::vtkNamedColors &) = ::named_colors_delete;
  named_colors_delete$(named_colors);
}
} // extern "C"
