#include "vtk_poly_data_algorithm.h"
#include "cxx.h"
#include "vtkDataObject.h"
#include "vtk_poly_data_algorithm.rs.h"

#include <vtkDataObject.h>
#include <vtkPolyDataAlgorithm.h>

vtkPolyDataAlgorithm* vtk_poly_data_algorithm_new() {
    return vtkPolyDataAlgorithm::New();
}

void vtk_poly_data_algorithm_delete(vtkPolyDataAlgorithm& poly_data_algorithm) {
    poly_data_algorithm.Delete();
}

void vtk_poly_data_algorithm_set_input_data(
    vtkPolyDataAlgorithm& poly_data_algorithm, int64_t port, const vtkDataObject& data_object
) {
    vtkDataObject* dobj = &const_cast<vtkDataObject&>(data_object);
    poly_data_algorithm.SetInputData(port, dobj);
}

void vtk_poly_data_algorithm_add_input_data(
    vtkPolyDataAlgorithm& poly_data_algorithm, int64_t port, const vtkDataObject& data_object
) {
    vtkDataObject* dobj = &const_cast<vtkDataObject&>(data_object);
    poly_data_algorithm.AddInputData(port, dobj);
}

const vtkDataObject&
vtk_poly_data_algorithm_get_input(const vtkPolyDataAlgorithm& poly_data_algorithm, int64_t port) {
    vtkDataObject* obj = const_cast<vtkPolyDataAlgorithm&>(poly_data_algorithm).GetInput(port);
    return *obj;
}

const vtkPolyData& vtk_poly_data_algorithm_get_poly_data_input(
    const vtkPolyDataAlgorithm& poly_data_algorithm, int64_t port
) {
    vtkPolyData* pd = const_cast<vtkPolyDataAlgorithm&>(poly_data_algorithm).GetPolyDataInput(port);
    return *pd;
}

const vtkPolyData&
vtk_poly_data_algorithm_get_output(const vtkPolyDataAlgorithm poly_data_algorithm, int64_t port) {
    vtkPolyData* pd = const_cast<vtkPolyDataAlgorithm&>(poly_data_algorithm).GetOutput(port);
    return *pd;
}
