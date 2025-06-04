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
void vtk_algorithm_set_input_connection(vtkAlgorithm& algorithm, int64_t, const vtkAlgorithmOutput&);

bool vtk_algorithm_process_request(
    const vtkAlgorithm& algorithm, const vtkInformation& request,
    rust::Slice<vtkInformationVector*> in_info, vtkInformationVector& out_info
);
int64_t vtk_algorithm_modify_request(const vtkAlgorithm& algorithm, vtkInformation&, int64_t);
const vtkInformation&
vtk_algorithm_get_input_port_information(const vtkAlgorithm& algorithm, int64_t port);
const vtkInformation&
vtk_algorithm_get_output_port_information(const vtkAlgorithm& algorithm, int64_t port);
int64_t vtk_algorithm_get_number_of_input_ports(const vtkAlgorithm& algorithm);
int64_t vtk_algorithm_get_number_of_output_ports(const vtkAlgorithm& algorithm);

void vtk_algorithm_set_abort_execute_and_update_time(vtkAlgorithm& algorithm);
void vtk_algorithm_update_progress(vtkAlgorithm& algorithm, double amount);
bool vtk_algorithm_check_abort(const vtkAlgorithm& algorithm);
const vtkInformation&
vtk_algorithm_get_input_array_information(const vtkAlgorithm& algorithm, int64_t idx);
void vtk_algorithm_remove_all_inputs(vtkAlgorithm& algorithm);
const vtkDataObject&
vtk_algorithm_get_output_data_object(const vtkAlgorithm& algorithm, int64_t port);
const vtkDataObject& vtk_algorithm_get_input_data_object(
    const vtkAlgorithm& algorithm, int64_t port, int64_t connection
);

void vtk_algorithm_remove_input_connection(
    vtkAlgorithm& algorithm, int64_t port, const vtkAlgorithmOutput& input
);
void vtk_algorithm_remove_input_connection_by_idx(
    vtkAlgorithm& algorithm, int64_t port, int64_t idx
);
void vtk_algorithm_remove_all_input_connections(vtkAlgorithm& algorithm, int64_t port);
void vtk_algorithm_set_input_data_object(
    vtkAlgorithm& algorithm, int64_t port, const vtkDataObject& data
);
void vtk_algorithm_add_input_data_object(
    vtkAlgorithm& algorithm, int64_t port, const vtkDataObject& data
);
const vtkAlgorithmOutput& vtk_algorithm_get_output_port(const vtkAlgorithm& algorithm, int64_t idx);
int64_t vtk_algorithm_get_number_of_input_connections(const vtkAlgorithm& algorithm, int64_t port);
int64_t vtk_algorithm_get_total_number_of_input_connections(const vtkAlgorithm& algorithm);
const vtkAlgorithmOutput&
vtk_algorithm_get_input_connection(const vtkAlgorithm& algorithm, int64_t port, int64_t idx);
const vtkAlgorithm& vtk_algorithm_get_input_algorithm(
    const vtkAlgorithm& algorithm, int64_t port, int64_t idx, int64_t& alg_port
);
const vtkExecutive&
vtk_algorithm_get_input_executive(const vtkAlgorithm& algorithm, int64_t port, int64_t idx);
const vtkInformation&
vtk_algorithm_get_input_information(const vtkAlgorithm& algorithm, int64_t port, int64_t idx);
const vtkInformation&
vtk_algorithm_get_output_information(const vtkAlgorithm& algorithm, int64_t port);
bool vtk_algorithm_update(
    vtkAlgorithm& algorithm, int64_t port, const vtkInformationVector& requests
);
void vtk_algorithm_update_information(vtkAlgorithm& algorithm);
void vtk_algorithm_update_data_object(vtkAlgorithm& algorithm);
void vtk_algorithm_propagate_update_extent(vtkAlgorithm& algorithm);
void vtk_algorithm_update_whole_extent(vtkAlgorithm& algorithm);
std::array<int64_t, 2>
vtk_algorithm_convert_total_input_to_port_connection(const vtkAlgorithm& algorithm, int64_t ind);
void vtk_algorithm_remove_no_prior_temporal_access_information_key(vtkAlgorithm& algorithm);
const vtkInformation& vtk_algorithm_get_information(const vtkAlgorithm& algorithm);
void vtk_algorithm_set_information(vtkAlgorithm& algorithm, const vtkInformation& information);
bool vtk_algorithm_get_abort_execute(const vtkAlgorithm& algorithm);
void vtk_algorithm_abort_execute_on(vtkAlgorithm& algorithm);
void vtk_algorithm_abort_execute_off(vtkAlgorithm& algorithm);
double vtk_algorithm_get_progress(const vtkAlgorithm& algorithm);
void vtk_algorithm_set_container_algorithm(
    vtkAlgorithm& algorithm, const vtkAlgorithm& container_algorithm
);
const vtkAlgorithm& vtk_algorithm_get_container_algorithm(const vtkAlgorithm& algorithm);
void vtk_algorithm_set_abort_output(vtkAlgorithm& algorithm, bool flag);
bool vtk_algorithm_get_abort_output(const vtkAlgorithm& algorithm);
void vtk_algorithm_set_progress_shift_scale(vtkAlgorithm& algorithm, double shift, double scale);
double vtk_algorithm_get_progress_shift(const vtkAlgorithm& algorithm);
double vtk_algorithm_get_progress_scale(const vtkAlgorithm& algorithm);
void vtk_algorithm_set_progress_text(vtkAlgorithm& algorithm, rust::Str ptext);
rust::String vtk_algorithm_get_progress_text(const vtkAlgorithm& algorithm);
uint64_t vtk_algorithm_get_error_code(const vtkAlgorithm& algorithm);
void vtk_algorithm_set_input_array_to_process(
    vtkAlgorithm& algorithm, int64_t idx, int64_t port, int64_t connection,
    rust::Str field_association, rust::Str attribute_type_or_name
);
void vtk_algorithm_set_input_connection(
    vtkAlgorithm& algorithm, int64_t port, const vtkAlgorithmOutput& input
);
void vtk_algorithm_add_input_connection(
    vtkAlgorithm& algorithm, int64_t port, const vtkAlgorithmOutput& input
);
