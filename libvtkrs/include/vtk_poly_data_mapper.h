#pragma once
#include "cxx.h"
#include <memory>

#include <vtkPolyDataMapper.h>

vtkPolyDataMapper* poly_data_mapper_new();
void poly_data_mapper_delete(vtkPolyDataMapper* pdm);
