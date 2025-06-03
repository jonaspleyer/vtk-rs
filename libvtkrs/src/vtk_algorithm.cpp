#include "vtk_algorithm.h"
#include "vtk_algorithm.rs.h"

#include "cxx.h"
#include <vtkAlgorithm.h>
#include <vtkExecutive.h>
#include <vtkInformation.h>
#include <vtkInformationVector.h>

vtkAlgorithm* vtk_algorithm_new() {
    return vtkAlgorithm::New();
}

void vtk_algorithm_delete(vtkAlgorithm& algorithm) {
    algorithm.Delete();
}

bool vtk_algorithm_has_executive(const vtkAlgorithm& algorithm) {
    return const_cast<vtkAlgorithm&>(algorithm).HasExecutive();
}

const vtkExecutive& vtk_algorithm_get_executive(const vtkAlgorithm& algorithm) {
    vtkExecutive* exec = const_cast<vtkAlgorithm&>(algorithm).GetExecutive();
    return *exec;
}

void vtk_algorithm_set_executive(vtkAlgorithm& algorithm, vtkExecutive& executive) {
    algorithm.SetExecutive(&executive);
}

void vtk_algorithm_set_input_connection(
    vtkAlgorithm& algorithm, int64_t port, const vtkAlgorithmOutput& input
) {
    algorithm.SetInputConnection(port, &const_cast<vtkAlgorithmOutput&>(input));
}

bool vtk_algorithm_process_request(
    const vtkAlgorithm& algorithm, const vtkInformation& request,
    rust::Slice<vtkInformationVector*> in_info, vtkInformationVector& out_info
) {
    vtkInformation* rq          = &const_cast<vtkInformation&>(request);
    vtkInformationVector* ovec  = &out_info;
    vtkInformationVector** ivec = in_info.data();
    return const_cast<vtkAlgorithm&>(algorithm).ProcessRequest(rq, ivec, ovec);
}
