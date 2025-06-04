#include "vtk_algorithm_output.h"
#include "vtk_algorithm_output.rs.h"

#include <vtkAlgorithm.h>
#include <vtkAlgorithmOutput.h>

vtkAlgorithmOutput* vtk_algorithm_output_new() {
    return vtkAlgorithmOutput::New();
}

void vtk_algorithm_output_delete(vtkAlgorithmOutput& algorithm) {
    algorithm.Delete();
}

int64_t vtk_algorithm_output_get_index(const vtkAlgorithmOutput& algorithm) {
    return const_cast<vtkAlgorithmOutput&>(algorithm).GetIndex();
}

void vtk_algorithm_output_set_index(vtkAlgorithmOutput& algorithm, int64_t index) {
    algorithm.SetIndex(index);
}

const vtkAlgorithm& vtk_algorithm_output_get_producer(const vtkAlgorithmOutput& algorithm) {
    vtkAlgorithm* algo = const_cast<vtkAlgorithmOutput&>(algorithm).GetProducer();
    return *algo;
}

void vtk_algorithm_output_set_producer(
    vtkAlgorithmOutput& algorithm, const vtkAlgorithm& producer
) {
    algorithm.SetProducer(&const_cast<vtkAlgorithm&>(producer));
}
