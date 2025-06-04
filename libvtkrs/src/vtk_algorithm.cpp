#include "vtk_algorithm.h"
#include "vtk_algorithm.rs.h"

#include "cxx.h"
#include <array>
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

int64_t
vtk_algorithm_modify_request(const vtkAlgorithm& algorithm, vtkInformation& request, int64_t when) {
    return const_cast<vtkAlgorithm&>(algorithm).ModifyRequest(&request, when);
}

const vtkInformation&
vtk_algorithm_get_input_port_information(const vtkAlgorithm& algorithm, int64_t port) {
    vtkInformation* info = const_cast<vtkAlgorithm&>(algorithm).GetInputPortInformation(port);
    return *info;
}

const vtkInformation&
vtk_algorithm_get_output_port_information(const vtkAlgorithm& algorithm, int64_t port) {
    vtkInformation* info = const_cast<vtkAlgorithm&>(algorithm).GetOutputInformation(port);
    return *info;
}

int64_t vtk_algorithm_get_number_of_input_ports(const vtkAlgorithm& algorithm) {
    return const_cast<vtkAlgorithm&>(algorithm).GetNumberOfInputPorts();
}

int64_t vtk_algorithm_get_number_of_output_ports(const vtkAlgorithm& algorithm) {
    return const_cast<vtkAlgorithm&>(algorithm).GetNumberOfOutputPorts();
}

void vtk_algorithm_set_abort_execute_and_update_time(vtkAlgorithm& algorithm) {
#ifdef VTK094
    algorithm.SetAbortExecuteAndUpdateTime();
#endif
}

void vtk_algorithm_update_progress(vtkAlgorithm& algorithm, double amount) {
    algorithm.UpdateProgress(amount);
}

bool vtk_algorithm_check_abort(const vtkAlgorithm& algorithm) {
#ifdef VTK094
    return const_cast<vtkAlgorithm&>(algorithm).CheckAbort();
#else
    return true;
#endif
}

const vtkInformation&
vtk_algorithm_get_input_array_information(const vtkAlgorithm& algorithm, int64_t idx) {
    vtkInformation* info = const_cast<vtkAlgorithm&>(algorithm).GetInputArrayInformation(idx);
    return *info;
}

void vtk_algorithm_remove_all_inputs(vtkAlgorithm& algorithm) {
    algorithm.RemoveAllInputs();
}

const vtkDataObject&
vtk_algorithm_get_output_data_object(const vtkAlgorithm& algorithm, int64_t port) {
    vtkDataObject* obj = const_cast<vtkAlgorithm&>(algorithm).GetOutputDataObject(port);
    return *obj;
}

const vtkDataObject& vtk_algorithm_get_input_data_object(
    const vtkAlgorithm& algorithm, int64_t port, int64_t connection
) {
    vtkDataObject* obj = const_cast<vtkAlgorithm&>(algorithm).GetInputDataObject(port, connection);
    return *obj;
}

void vtk_algorithm_remove_input_connection(
    vtkAlgorithm& algorithm, int64_t port, const vtkAlgorithmOutput& input
) {
    algorithm.RemoveInputConnection(port, &const_cast<vtkAlgorithmOutput&>(input));
}

void vtk_algorithm_remove_input_connection_by_idx(
    vtkAlgorithm& algorithm, int64_t port, int64_t idx
) {
    algorithm.RemoveInputConnection(port, idx);
}

void vtk_algorithm_remove_all_input_connections(vtkAlgorithm& algorithm, int64_t port) {
    algorithm.RemoveAllInputConnections(port);
}

void vtk_algorithm_set_input_data_object(
    vtkAlgorithm& algorithm, int64_t port, const vtkDataObject& data
) {
    algorithm.SetInputDataObject(port, &const_cast<vtkDataObject&>(data));
}

void vtk_algorithm_add_input_data_object(
    vtkAlgorithm& algorithm, int64_t port, const vtkDataObject& data
) {
    algorithm.AddInputDataObject(port, &const_cast<vtkDataObject&>(data));
}

const vtkAlgorithmOutput&
vtk_algorithm_get_output_port(const vtkAlgorithm& algorithm, int64_t idx) {
    return *const_cast<vtkAlgorithm&>(algorithm).GetOutputPort(idx);
}

int64_t vtk_algorithm_get_number_of_input_connections(const vtkAlgorithm& algorithm, int64_t port) {
    return const_cast<vtkAlgorithm&>(algorithm).GetNumberOfInputConnections(port);
}

int64_t vtk_algorithm_get_total_number_of_input_connections(const vtkAlgorithm& algorithm) {
    return const_cast<vtkAlgorithm&>(algorithm).GetTotalNumberOfInputConnections();
}

const vtkAlgorithmOutput&
vtk_algorithm_get_input_connection(const vtkAlgorithm& algorithm, int64_t port, int64_t idx) {
    return *const_cast<vtkAlgorithm&>(algorithm).GetInputConnection(port, idx);
}

const vtkAlgorithm& vtk_algorithm_get_input_algorithm(
    const vtkAlgorithm& algorithm, int64_t port, int64_t idx, int64_t& alg_port
) {
    int aport;
    const vtkAlgorithm* algo =
        const_cast<vtkAlgorithm&>(algorithm).GetInputAlgorithm(port, idx, aport);
    alg_port = aport;
    return *algo;
}

const vtkExecutive&
vtk_algorithm_get_input_executive(const vtkAlgorithm& algorithm, int64_t port, int64_t idx) {
    return *const_cast<vtkAlgorithm&>(algorithm).GetInputExecutive(port, idx);
}

const vtkInformation&
vtk_algorithm_get_input_information(const vtkAlgorithm& algorithm, int64_t port, int64_t idx) {
    return *const_cast<vtkAlgorithm&>(algorithm).GetInputInformation(port, idx);
}

const vtkInformation&
vtk_algorithm_get_output_information(const vtkAlgorithm& algorithm, int64_t port) {
    return *const_cast<vtkAlgorithm&>(algorithm).GetOutputInformation(port);
}

bool vtk_algorithm_update(
    vtkAlgorithm& algorithm, int64_t port, const vtkInformationVector& requests
) {
    return algorithm.Update(port, &const_cast<vtkInformationVector&>(requests));
}

void vtk_algorithm_update_information(vtkAlgorithm& algorithm) {
    algorithm.UpdateInformation();
}

void vtk_algorithm_update_data_object(vtkAlgorithm& algorithm) {
    algorithm.UpdateDataObject();
}

void vtk_algorithm_propagate_update_extent(vtkAlgorithm& algorithm) {
    algorithm.PropagateUpdateExtent();
}

void vtk_algorithm_update_whole_extent(vtkAlgorithm& algorithm) {
    algorithm.UpdateWholeExtent();
}

std::array<int64_t, 2>
vtk_algorithm_convert_total_input_to_port_connection(const vtkAlgorithm& algorithm, int64_t ind) {
    int port;
    int conn;
    const_cast<vtkAlgorithm&>(algorithm).ConvertTotalInputToPortConnection(ind, port, conn);
    std::array<int64_t, 2> arr = {port, conn};
    return arr;
}

void vtk_algorithm_remove_no_prior_temporal_access_information_key(vtkAlgorithm& algorithm) {
#ifdef VTK094
    algorithm.RemoveNoPriorTemporalAccessInformationKey();
#endif
}

const vtkInformation& vtk_algorithm_get_information(const vtkAlgorithm& algorithm) {
    return *const_cast<vtkAlgorithm&>(algorithm).GetInformation();
}

void vtk_algorithm_set_information(vtkAlgorithm& algorithm, const vtkInformation& information) {
    algorithm.SetInformation(&const_cast<vtkInformation&>(information));
}

bool vtk_algorithm_get_abort_execute(const vtkAlgorithm& algorithm) {
    return const_cast<vtkAlgorithm&>(algorithm).GetAbortExecute();
}

void vtk_algorithm_abort_execute_on(vtkAlgorithm& algorithm) {
    algorithm.AbortExecuteOn();
}

void vtk_algorithm_abort_execute_off(vtkAlgorithm& algorithm) {
    algorithm.AbortExecuteOff();
}

double vtk_algorithm_get_progress(const vtkAlgorithm& algorithm) {
    return const_cast<vtkAlgorithm&>(algorithm).GetProgress();
}

void vtk_algorithm_set_container_algorithm(
    vtkAlgorithm& algorithm, const vtkAlgorithm& container_algorithm
) {
#ifdef VTK094
    algorithm.SetContainerAlgorithm(&const_cast<vtkAlgorithm&>(container_algorithm));
#endif
}

const vtkAlgorithm& vtk_algorithm_get_container_algorithm(const vtkAlgorithm& algorithm) {
#ifdef VTK094
    return *const_cast<vtkAlgorithm&>(algorithm).GetContainerAlgorithm();
#else
    return algorithm;
#endif
}

void vtk_algorithm_set_abort_output(vtkAlgorithm& algorithm, bool flag) {
#ifdef VTK094
    algorithm.SetAbortOutput(flag);
#endif
}

bool vtk_algorithm_get_abort_output(const vtkAlgorithm& algorithm) {
#ifdef VTK094
    return const_cast<vtkAlgorithm&>(algorithm).GetAbortOutput();
#else
    return true;
#endif
}

void vtk_algorithm_set_progress_shift_scale(vtkAlgorithm& algorithm, double shift, double scale) {
    algorithm.SetProgressShiftScale(shift, scale);
}

double vtk_algorithm_get_progress_shift(const vtkAlgorithm& algorithm) {
    return const_cast<vtkAlgorithm&>(algorithm).GetProgressShift();
}

double vtk_algorithm_get_progress_scale(const vtkAlgorithm& algorithm) {
    return const_cast<vtkAlgorithm&>(algorithm).GetProgressScale();
}

void vtk_algorithm_set_progress_text(vtkAlgorithm& algorithm, rust::Str ptext) {
    algorithm.SetProgressText(ptext.data());
}

rust::String vtk_algorithm_get_progress_text(const vtkAlgorithm& algorithm) {
    rust::String txt = const_cast<vtkAlgorithm&>(algorithm).GetProgressText();
    return txt;
}

uint64_t vtk_algorithm_get_error_code(const vtkAlgorithm& algorithm) {
    return const_cast<vtkAlgorithm&>(algorithm).GetErrorCode();
}

void vtk_algorithm_set_input_array_to_process(
    vtkAlgorithm& algorithm, int64_t idx, int64_t port, int64_t connection,
    rust::Str field_association, rust::Str attribute_type_or_name
) {
    algorithm.SetInputArrayToProcess(
        idx, port, connection, field_association.data(), attribute_type_or_name.data()
    );
}

void vtk_algorithm_add_input_connection(
    vtkAlgorithm& algorithm, int64_t port, const vtkAlgorithmOutput& input
) {
    algorithm.AddInputConnection(port, &const_cast<vtkAlgorithmOutput&>(input));
}
