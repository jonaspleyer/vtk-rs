#pragma once
#include "cxx.h"
#include <memory>

#include <vtkNamedColors.h>

vtkNamedColors* named_colors_new();
void named_colors_delete(vtkNamedColors* named_colors);
