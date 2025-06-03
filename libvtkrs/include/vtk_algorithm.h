#include "cxx.h"
#include <vtkAlgorithm.h>
#include <vtkExecutive.h>
#include <vtkInformation.h>
#include <vtkInformationVector.h>

vtkAlgorithm* vtk_algorithm_new();
void vtk_algorithm_delete(vtkAlgorithm& algorithm);

bool vtk_algorithm_has_executive(const vtkAlgorithm& algorithm);
const vtkExecutive& vtk_algorithm_get_executive(const vtkAlgorithm& algorithm);
void vtk_algorithm_set_executive(vtkAlgorithm& algorithm, vtkExecutive& executive);
void vtk_algorithm_set_input_connection(vtkAlgorithm&, int64_t, const vtkAlgorithmOutput&);

bool vtk_algorithm_process_request(
    const vtkAlgorithm& algorithm, const vtkInformation& request,
    rust::Slice<vtkInformationVector*> in_info, vtkInformationVector& out_info
);
