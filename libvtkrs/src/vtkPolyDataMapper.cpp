#include "vtkPolyDataMapper.h"
#include "vtkPolyDataMapper.rs.h"

#include "cxx.h"
// #include <vtkPolyDataMapper.h>
// #include <memory>

rust::String speak() {return "";}

/* vtkPolyDataMapper* poly_data_mapper_new() {
    // auto pdm = std::unique_ptr<vtkPolyDataMapper>(new vtkPolyDataMapper);
    // return pdm;
    return vtkPolyDataMapper::New();
}*/

// void poly_data_mapper_delete(std::unique_ptr<vtkPolyDataMapper> pdm) {
    // pdm->Delete();
// }

// extern "C" void *poly_data_mapper_new() { return vtkPolyDataMapper::New(); }
// extern "C" void poly_data_mapper_delete(void *poly_data_mapper_ptr) {
//     static_cast<vtkPolyDataMapper *>(poly_data_mapper_ptr)->Delete();
// }
// extern "C" void poly_data_mapper_set_input_connection(void *poly_data_mapper_ptr, int port) {
//     static_cast<vtkPolyDataMapper *>(poly_data_mapper_ptr)->SetInputConnection(port);
// }
