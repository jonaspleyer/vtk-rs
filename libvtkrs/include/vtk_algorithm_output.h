#include <vtkAlgorithm.h>
#include <vtkAlgorithmOutput.h>
#include <vtkObject.h>

vtkAlgorithmOutput* vtk_algorithm_output_new();
void vtk_algorithm_output_delete(vtkAlgorithmOutput& algorithm);

int64_t vtk_algorithm_output_get_index(const vtkAlgorithmOutput& algorithm);
void vtk_algorithm_output_set_index(vtkAlgorithmOutput& algorithm, int64_t index);
const vtkAlgorithm& vtk_algorithm_output_get_producer(const vtkAlgorithmOutput& algorithm);
void vtk_algorithm_output_set_producer(vtkAlgorithmOutput& algorithm, const vtkAlgorithm& producer);
