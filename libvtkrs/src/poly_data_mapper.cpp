#include "cxx.h"
#include "poly_data_mapper.h"
#include "poly_data_mapper.rs.h"

#include <vtkPolyDataMapper.h>

vtkPolyDataMapper* poly_data_mapper_new() {
    return vtkPolyDataMapper::New();
}

void poly_data_mapper_delete(vtkPolyDataMapper* pdm) {
    pdm->Delete();
}
