// Include header file
#include <vtk_common_misc.h>

// Default include in all modules
#include <vtkNew.h>
#include <vtkObjectBase.h>

// Include objects of this module
#include <vtkContourValues.h>
#include <vtkErrorCode.h>
#include <vtkExprTkFunctionParser.h>
#include <vtkFunctionParser.h>
#include <vtkHeap.h>
#include <vtkPolygonBuilder.h>
#include <vtkResourceFileLocator.h>

// Implement declared functions
extern "C" vtkNew<vtkContourValues> vtkContourValues_new() {
    return vtkNew<vtkContourValues>();
}
extern "C" void vtkContourValues_destructor(vtkNew<vtkContourValues> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkContourValues_get_ptr(vtkNew<vtkContourValues> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_contour_values_set_value(vtkNew<vtkContourValues> self, int i, double value) {
    return self->SetValue(i, value);
}
extern "C" double vtk_contour_values_get_value(vtkNew<vtkContourValues> self, int i) {
    return self->GetValue(i);
}
extern "C" void vtk_contour_values_set_number_of_contours(
    vtkNew<vtkContourValues> self, int number
) {
    return self->SetNumberOfContours(number);
}
extern "C" int vtk_contour_values_get_number_of_contours(vtkNew<vtkContourValues> self) {
    return self->GetNumberOfContours();
}
extern "C" void vtk_contour_values_generate_values_c_int_double_2(
    vtkNew<vtkContourValues> self, int numContours, double* range
) {
    return self->GenerateValues(numContours, range);
}
extern "C" void vtk_contour_values_generate_values_c_int_double_double(
    vtkNew<vtkContourValues> self, int numContours, double rangeStart, double rangeEnd
) {
    return self->GenerateValues(numContours, rangeStart, rangeEnd);
}
extern "C" vtkNew<vtkExprTkFunctionParser> vtkExprTkFunctionParser_new() {
    return vtkNew<vtkExprTkFunctionParser>();
}
extern "C" void vtkExprTkFunctionParser_destructor(vtkNew<vtkExprTkFunctionParser> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkExprTkFunctionParser_get_ptr(vtkNew<vtkExprTkFunctionParser> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long vtk_expr_tk_function_parser_get_m_time(
    vtkNew<vtkExprTkFunctionParser> self
) {
    return self->GetMTime();
}
extern "C" int vtk_expr_tk_function_parser_get_result_size(vtkNew<vtkExprTkFunctionParser> self) {
    return self->GetResultSize();
}
extern "C" int vtk_expr_tk_function_parser_is_scalar_result(vtkNew<vtkExprTkFunctionParser> self) {
    return self->IsScalarResult();
}
extern "C" int vtk_expr_tk_function_parser_is_vector_result(vtkNew<vtkExprTkFunctionParser> self) {
    return self->IsVectorResult();
}
extern "C" double vtk_expr_tk_function_parser_get_scalar_result(
    vtkNew<vtkExprTkFunctionParser> self
) {
    return self->GetScalarResult();
}
extern "C" void vtk_expr_tk_function_parser_set_scalar_variable_value_c_int_double(
    vtkNew<vtkExprTkFunctionParser> self, int i, double value
) {
    return self->SetScalarVariableValue(i, value);
}
extern "C" double vtk_expr_tk_function_parser_get_scalar_variable_value_c_int(
    vtkNew<vtkExprTkFunctionParser> self, int i
) {
    return self->GetScalarVariableValue(i);
}
extern "C" void vtk_expr_tk_function_parser_set_vector_variable_value_c_int_double_double_double(
    vtkNew<vtkExprTkFunctionParser> self, int i, double xValue, double yValue, double zValue
) {
    return self->SetVectorVariableValue(i, xValue, yValue, zValue);
}
extern "C" void vtk_expr_tk_function_parser_set_vector_variable_value_c_int_double_3_c_int(
    vtkNew<vtkExprTkFunctionParser> self, int i, double* values, int size
) {
    return self->SetVectorVariableValue(i, values, size);
}
extern "C" void vtk_expr_tk_function_parser_get_vector_variable_value_c_int_double_3(
    vtkNew<vtkExprTkFunctionParser> self, int i, double* value
) {
    return self->GetVectorVariableValue(i, value);
}
extern "C" int vtk_expr_tk_function_parser_get_number_of_scalar_variables(
    vtkNew<vtkExprTkFunctionParser> self
) {
    return self->GetNumberOfScalarVariables();
}
extern "C" int vtk_expr_tk_function_parser_get_number_of_vector_variables(
    vtkNew<vtkExprTkFunctionParser> self
) {
    return self->GetNumberOfVectorVariables();
}
extern "C" bool vtk_expr_tk_function_parser_get_scalar_variable_needed_c_int(
    vtkNew<vtkExprTkFunctionParser> self, int i
) {
    return self->GetScalarVariableNeeded(i);
}
extern "C" bool vtk_expr_tk_function_parser_get_vector_variable_needed_c_int(
    vtkNew<vtkExprTkFunctionParser> self, int i
) {
    return self->GetVectorVariableNeeded(i);
}
extern "C" void vtk_expr_tk_function_parser_remove_all_variables(
    vtkNew<vtkExprTkFunctionParser> self
) {
    return self->RemoveAllVariables();
}
extern "C" void vtk_expr_tk_function_parser_remove_scalar_variables(
    vtkNew<vtkExprTkFunctionParser> self
) {
    return self->RemoveScalarVariables();
}
extern "C" void vtk_expr_tk_function_parser_remove_vector_variables(
    vtkNew<vtkExprTkFunctionParser> self
) {
    return self->RemoveVectorVariables();
}
extern "C" void vtk_expr_tk_function_parser_set_replace_invalid_values(
    vtkNew<vtkExprTkFunctionParser> self, int _arg
) {
    return self->SetReplaceInvalidValues(_arg);
}
extern "C" int vtk_expr_tk_function_parser_get_replace_invalid_values(
    vtkNew<vtkExprTkFunctionParser> self
) {
    return self->GetReplaceInvalidValues();
}
extern "C" void vtk_expr_tk_function_parser_replace_invalid_values_on(
    vtkNew<vtkExprTkFunctionParser> self
) {
    return self->ReplaceInvalidValuesOn();
}
extern "C" void vtk_expr_tk_function_parser_replace_invalid_values_off(
    vtkNew<vtkExprTkFunctionParser> self
) {
    return self->ReplaceInvalidValuesOff();
}
extern "C" void vtk_expr_tk_function_parser_set_replacement_value(
    vtkNew<vtkExprTkFunctionParser> self, double _arg
) {
    return self->SetReplacementValue(_arg);
}
extern "C" double vtk_expr_tk_function_parser_get_replacement_value(
    vtkNew<vtkExprTkFunctionParser> self
) {
    return self->GetReplacementValue();
}
extern "C" void vtk_expr_tk_function_parser_invalidate_function(
    vtkNew<vtkExprTkFunctionParser> self
) {
    return self->InvalidateFunction();
}
extern "C" vtkNew<vtkFunctionParser> vtkFunctionParser_new() {
    return vtkNew<vtkFunctionParser>();
}
extern "C" void vtkFunctionParser_destructor(vtkNew<vtkFunctionParser> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkFunctionParser_get_ptr(vtkNew<vtkFunctionParser> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long vtk_function_parser_get_m_time(vtkNew<vtkFunctionParser> self) {
    return self->GetMTime();
}
extern "C" int vtk_function_parser_get_result_size(vtkNew<vtkFunctionParser> self) {
    return self->GetResultSize();
}
extern "C" int vtk_function_parser_is_scalar_result(vtkNew<vtkFunctionParser> self) {
    return self->IsScalarResult();
}
extern "C" int vtk_function_parser_is_vector_result(vtkNew<vtkFunctionParser> self) {
    return self->IsVectorResult();
}
extern "C" double vtk_function_parser_get_scalar_result(vtkNew<vtkFunctionParser> self) {
    return self->GetScalarResult();
}
extern "C" void vtk_function_parser_get_vector_result_double_3(
    vtkNew<vtkFunctionParser> self, double* result
) {
    return self->GetVectorResult(result);
}
extern "C" void vtk_function_parser_set_scalar_variable_value_c_int_double(
    vtkNew<vtkFunctionParser> self, int i, double value
) {
    return self->SetScalarVariableValue(i, value);
}
extern "C" double vtk_function_parser_get_scalar_variable_value_c_int(
    vtkNew<vtkFunctionParser> self, int i
) {
    return self->GetScalarVariableValue(i);
}
extern "C" void vtk_function_parser_set_vector_variable_value_c_int_double_double_double(
    vtkNew<vtkFunctionParser> self, int i, double xValue, double yValue, double zValue
) {
    return self->SetVectorVariableValue(i, xValue, yValue, zValue);
}
extern "C" void vtk_function_parser_get_vector_variable_value_c_int_double_3(
    vtkNew<vtkFunctionParser> self, int i, double* value
) {
    return self->GetVectorVariableValue(i, value);
}
extern "C" int vtk_function_parser_get_number_of_scalar_variables(vtkNew<vtkFunctionParser> self) {
    return self->GetNumberOfScalarVariables();
}
extern "C" int vtk_function_parser_get_number_of_vector_variables(vtkNew<vtkFunctionParser> self) {
    return self->GetNumberOfVectorVariables();
}
extern "C" bool vtk_function_parser_get_scalar_variable_needed_c_int(
    vtkNew<vtkFunctionParser> self, int i
) {
    return self->GetScalarVariableNeeded(i);
}
extern "C" bool vtk_function_parser_get_vector_variable_needed_c_int(
    vtkNew<vtkFunctionParser> self, int i
) {
    return self->GetVectorVariableNeeded(i);
}
extern "C" void vtk_function_parser_remove_all_variables(vtkNew<vtkFunctionParser> self) {
    return self->RemoveAllVariables();
}
extern "C" void vtk_function_parser_remove_scalar_variables(vtkNew<vtkFunctionParser> self) {
    return self->RemoveScalarVariables();
}
extern "C" void vtk_function_parser_remove_vector_variables(vtkNew<vtkFunctionParser> self) {
    return self->RemoveVectorVariables();
}
extern "C" void vtk_function_parser_set_replace_invalid_values(
    vtkNew<vtkFunctionParser> self, int _arg
) {
    return self->SetReplaceInvalidValues(_arg);
}
extern "C" int vtk_function_parser_get_replace_invalid_values(vtkNew<vtkFunctionParser> self) {
    return self->GetReplaceInvalidValues();
}
extern "C" void vtk_function_parser_replace_invalid_values_on(vtkNew<vtkFunctionParser> self) {
    return self->ReplaceInvalidValuesOn();
}
extern "C" void vtk_function_parser_replace_invalid_values_off(vtkNew<vtkFunctionParser> self) {
    return self->ReplaceInvalidValuesOff();
}
extern "C" void vtk_function_parser_set_replacement_value(
    vtkNew<vtkFunctionParser> self, double _arg
) {
    return self->SetReplacementValue(_arg);
}
extern "C" double vtk_function_parser_get_replacement_value(vtkNew<vtkFunctionParser> self) {
    return self->GetReplacementValue();
}
extern "C" void vtk_function_parser_invalidate_function(vtkNew<vtkFunctionParser> self) {
    return self->InvalidateFunction();
}
extern "C" vtkNew<vtkHeap> vtkHeap_new() {
    return vtkNew<vtkHeap>();
}
extern "C" void vtkHeap_destructor(vtkNew<vtkHeap> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkHeap_get_ptr(vtkNew<vtkHeap> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_heap_set_block_size(vtkNew<vtkHeap> self, size_t p0) {
    return self->SetBlockSize(p0);
}
extern "C" size_t vtk_heap_get_block_size(vtkNew<vtkHeap> self) {
    return self->GetBlockSize();
}
extern "C" int vtk_heap_get_number_of_blocks(vtkNew<vtkHeap> self) {
    return self->GetNumberOfBlocks();
}
extern "C" int vtk_heap_get_number_of_allocations(vtkNew<vtkHeap> self) {
    return self->GetNumberOfAllocations();
}
extern "C" void vtk_heap_reset(vtkNew<vtkHeap> self) {
    return self->Reset();
}
extern "C" vtkNew<vtkResourceFileLocator> vtkResourceFileLocator_new() {
    return vtkNew<vtkResourceFileLocator>();
}
extern "C" void vtkResourceFileLocator_destructor(vtkNew<vtkResourceFileLocator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkResourceFileLocator_get_ptr(vtkNew<vtkResourceFileLocator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_resource_file_locator_set_log_verbosity(
    vtkNew<vtkResourceFileLocator> self, int _arg
) {
    return self->SetLogVerbosity(_arg);
}
extern "C" int vtk_resource_file_locator_get_log_verbosity(vtkNew<vtkResourceFileLocator> self) {
    return self->GetLogVerbosity();
}
