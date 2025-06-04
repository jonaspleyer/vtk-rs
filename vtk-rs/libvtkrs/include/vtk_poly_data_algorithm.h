#include "cxx.h"
#include <vtkDataObject.h>
#include <vtkPolyDataAlgorithm.h>

vtkPolyDataAlgorithm* vtk_poly_data_algorithm_new();
void vtk_poly_data_algorithm_delete(vtkPolyDataAlgorithm& poly_data_algorithm);
void vtk_poly_data_algorithm_set_input_data(
    vtkPolyDataAlgorithm& poly_data_algorithm, int64_t port, const vtkDataObject& data_object
);
void vtk_poly_data_algorithm_add_input_data(
    vtkPolyDataAlgorithm& poly_data_algorithm, int64_t port, const vtkDataObject& data_object
);
const vtkDataObject&
vtk_poly_data_algorithm_get_input(const vtkPolyDataAlgorithm& poly_data_algorithm, int64_t port);
const vtkPolyData& vtk_poly_data_algorithm_get_poly_data_input(
    const vtkPolyDataAlgorithm& poly_data_algorithm, int64_t port
);
const vtkPolyData&
vtk_poly_data_algorithm_get_output(const vtkPolyDataAlgorithm& poly_data_algorithm, int64_t port);
void vtk_poly_data_algorithm_set_output(
    vtkPolyDataAlgorithm& poly_data_algorithm, const vtkDataObject& data_object
);
