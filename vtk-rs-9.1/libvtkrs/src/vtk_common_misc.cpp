// Include header file
#include<vtk_common_misc.h>

// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkContourValues.h>
#include<vtkErrorCode.h>
#include<vtkExprTkFunctionParser.h>
#include<vtkFunctionParser.h>
#include<vtkHeap.h>
#include<vtkPolygonBuilder.h>
#include<vtkResourceFileLocator.h>

// Implement declared functions
extern "C" vtkContourValues * vtkContourValues_new () {return vtkContourValues :: New () ;}
extern "C" void vtkContourValues_destructor (vtkContourValues * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_contour_values_set_value(vtkContourValues* sself, int i, double value) { sself->SetValue(i, value); }
extern "C" double vtk_contour_values_get_value(vtkContourValues* sself, int i) { return sself->GetValue(i); }
extern "C" void vtk_contour_values_set_number_of_contours(vtkContourValues* sself, const int number) { sself->SetNumberOfContours(number); }
extern "C" int vtk_contour_values_get_number_of_contours(vtkContourValues* sself) { return sself->GetNumberOfContours(); }
extern "C" void vtk_contour_values_generate_values(vtkContourValues* sself, int numContours, double rangeStart, double rangeEnd) { sself->GenerateValues(numContours, rangeStart, rangeEnd); }
extern "C" vtkExprTkFunctionParser * vtkExprTkFunctionParser_new () {return vtkExprTkFunctionParser :: New () ;}
extern "C" void vtkExprTkFunctionParser_destructor (vtkExprTkFunctionParser * sself) {sself -> Delete () ; return ;}
extern "C" unsigned long vtk_expr_tk_function_parser_get_m_time(vtkExprTkFunctionParser* sself) { return sself->GetMTime(); }
extern "C" void vtk_expr_tk_function_parser_set_function(vtkExprTkFunctionParser* sself, const char* function) { sself->SetFunction(function); }
extern "C" const char* vtk_expr_tk_function_parser_get_function(vtkExprTkFunctionParser* sself) { return sself->GetFunction(); }
extern "C" int vtk_expr_tk_function_parser_is_scalar_result(vtkExprTkFunctionParser* sself) { return sself->IsScalarResult(); }
extern "C" int vtk_expr_tk_function_parser_is_vector_result(vtkExprTkFunctionParser* sself) { return sself->IsVectorResult(); }
extern "C" double vtk_expr_tk_function_parser_get_scalar_result(vtkExprTkFunctionParser* sself) { return sself->GetScalarResult(); }
extern "C" void vtk_expr_tk_function_parser_set_scalar_variable_value(vtkExprTkFunctionParser* sself, const char*& variableName, double value) { sself->SetScalarVariableValue(variableName, value); }
extern "C" double vtk_expr_tk_function_parser_get_scalar_variable_value(vtkExprTkFunctionParser* sself, const char*& variableName) { return sself->GetScalarVariableValue(variableName); }
extern "C" void vtk_expr_tk_function_parser_set_vector_variable_value(vtkExprTkFunctionParser* sself, const char*& variableName, double xValue, double yValue, double zValue) { sself->SetVectorVariableValue(variableName, xValue, yValue, zValue); }
extern "C" int vtk_expr_tk_function_parser_get_number_of_scalar_variables(vtkExprTkFunctionParser* sself) { return sself->GetNumberOfScalarVariables(); }
extern "C" int vtk_expr_tk_function_parser_get_scalar_variable_index(vtkExprTkFunctionParser* sself, const char*& name) { return sself->GetScalarVariableIndex(name); }
extern "C" int vtk_expr_tk_function_parser_get_number_of_vector_variables(vtkExprTkFunctionParser* sself) { return sself->GetNumberOfVectorVariables(); }
extern "C" int vtk_expr_tk_function_parser_get_vector_variable_index(vtkExprTkFunctionParser* sself, const char*& name) { return sself->GetVectorVariableIndex(name); }
extern "C" bool vtk_expr_tk_function_parser_get_scalar_variable_needed(vtkExprTkFunctionParser* sself, int i) { return sself->GetScalarVariableNeeded(i); }
extern "C" bool vtk_expr_tk_function_parser_get_vector_variable_needed(vtkExprTkFunctionParser* sself, int i) { return sself->GetVectorVariableNeeded(i); }
extern "C" void vtk_expr_tk_function_parser_remove_all_variables(vtkExprTkFunctionParser* sself) { sself->RemoveAllVariables(); }
extern "C" void vtk_expr_tk_function_parser_remove_scalar_variables(vtkExprTkFunctionParser* sself) { sself->RemoveScalarVariables(); }
extern "C" void vtk_expr_tk_function_parser_remove_vector_variables(vtkExprTkFunctionParser* sself) { sself->RemoveVectorVariables(); }
extern "C" void vtk_expr_tk_function_parser_set_replace_invalid_values(vtkExprTkFunctionParser* sself, int _arg) { sself->SetReplaceInvalidValues(_arg); }
extern "C" int vtk_expr_tk_function_parser_get_replace_invalid_values(vtkExprTkFunctionParser* sself) { return sself->GetReplaceInvalidValues(); }
extern "C" void vtk_expr_tk_function_parser_replace_invalid_values_on(vtkExprTkFunctionParser* sself) { sself->ReplaceInvalidValuesOn(); }
extern "C" void vtk_expr_tk_function_parser_replace_invalid_values_off(vtkExprTkFunctionParser* sself) { sself->ReplaceInvalidValuesOff(); }
extern "C" void vtk_expr_tk_function_parser_set_replacement_value(vtkExprTkFunctionParser* sself, double _arg) { sself->SetReplacementValue(_arg); }
extern "C" double vtk_expr_tk_function_parser_get_replacement_value(vtkExprTkFunctionParser* sself) { return sself->GetReplacementValue(); }
extern "C" void vtk_expr_tk_function_parser_invalidate_function(vtkExprTkFunctionParser* sself) { sself->InvalidateFunction(); }
extern "C" vtkFunctionParser * vtkFunctionParser_new () {return vtkFunctionParser :: New () ;}
extern "C" void vtkFunctionParser_destructor (vtkFunctionParser * sself) {sself -> Delete () ; return ;}
extern "C" unsigned long vtk_function_parser_get_m_time(vtkFunctionParser* sself) { return sself->GetMTime(); }
extern "C" void vtk_function_parser_set_function(vtkFunctionParser* sself, const char* function) { sself->SetFunction(function); }
extern "C" int vtk_function_parser_is_scalar_result(vtkFunctionParser* sself) { return sself->IsScalarResult(); }
extern "C" int vtk_function_parser_is_vector_result(vtkFunctionParser* sself) { return sself->IsVectorResult(); }
extern "C" double vtk_function_parser_get_scalar_result(vtkFunctionParser* sself) { return sself->GetScalarResult(); }
extern "C" void vtk_function_parser_set_scalar_variable_value(vtkFunctionParser* sself, const char* variableName, double value) { sself->SetScalarVariableValue(variableName, value); }
extern "C" double vtk_function_parser_get_scalar_variable_value(vtkFunctionParser* sself, const char* variableName) { return sself->GetScalarVariableValue(variableName); }
extern "C" void vtk_function_parser_set_vector_variable_value(vtkFunctionParser* sself, const char* variableName, double xValue, double yValue, double zValue) { sself->SetVectorVariableValue(variableName, xValue, yValue, zValue); }
extern "C" int vtk_function_parser_get_number_of_scalar_variables(vtkFunctionParser* sself) { return sself->GetNumberOfScalarVariables(); }
extern "C" int vtk_function_parser_get_scalar_variable_index(vtkFunctionParser* sself, const char* name) { return sself->GetScalarVariableIndex(name); }
extern "C" int vtk_function_parser_get_number_of_vector_variables(vtkFunctionParser* sself) { return sself->GetNumberOfVectorVariables(); }
extern "C" int vtk_function_parser_get_vector_variable_index(vtkFunctionParser* sself, const char* name) { return sself->GetVectorVariableIndex(name); }
extern "C" const char* vtk_function_parser_get_scalar_variable_name(vtkFunctionParser* sself, int i) { return sself->GetScalarVariableName(i); }
extern "C" const char* vtk_function_parser_get_vector_variable_name(vtkFunctionParser* sself, int i) { return sself->GetVectorVariableName(i); }
extern "C" bool vtk_function_parser_get_scalar_variable_needed(vtkFunctionParser* sself, int i) { return sself->GetScalarVariableNeeded(i); }
extern "C" bool vtk_function_parser_get_vector_variable_needed(vtkFunctionParser* sself, int i) { return sself->GetVectorVariableNeeded(i); }
extern "C" void vtk_function_parser_remove_all_variables(vtkFunctionParser* sself) { sself->RemoveAllVariables(); }
extern "C" void vtk_function_parser_remove_scalar_variables(vtkFunctionParser* sself) { sself->RemoveScalarVariables(); }
extern "C" void vtk_function_parser_remove_vector_variables(vtkFunctionParser* sself) { sself->RemoveVectorVariables(); }
extern "C" void vtk_function_parser_set_replace_invalid_values(vtkFunctionParser* sself, int _arg) { sself->SetReplaceInvalidValues(_arg); }
extern "C" int vtk_function_parser_get_replace_invalid_values(vtkFunctionParser* sself) { return sself->GetReplaceInvalidValues(); }
extern "C" void vtk_function_parser_replace_invalid_values_on(vtkFunctionParser* sself) { sself->ReplaceInvalidValuesOn(); }
extern "C" void vtk_function_parser_replace_invalid_values_off(vtkFunctionParser* sself) { sself->ReplaceInvalidValuesOff(); }
extern "C" void vtk_function_parser_set_replacement_value(vtkFunctionParser* sself, double _arg) { sself->SetReplacementValue(_arg); }
extern "C" double vtk_function_parser_get_replacement_value(vtkFunctionParser* sself) { return sself->GetReplacementValue(); }
extern "C" void vtk_function_parser_invalidate_function(vtkFunctionParser* sself) { sself->InvalidateFunction(); }
extern "C" vtkHeap * vtkHeap_new () {return vtkHeap :: New () ;}
extern "C" void vtkHeap_destructor (vtkHeap * sself) {sself -> Delete () ; return ;}
extern "C" void* vtk_heap_allocate_memory(vtkHeap* sself, size_t n) { return sself->AllocateMemory(n); }
extern "C" void vtk_heap_set_block_size(vtkHeap* sself, size_t p0) { sself->SetBlockSize(p0); }
extern "C" size_t vtk_heap_get_block_size(vtkHeap* sself) { return sself->GetBlockSize(); }
extern "C" int vtk_heap_get_number_of_blocks(vtkHeap* sself) { return sself->GetNumberOfBlocks(); }
extern "C" int vtk_heap_get_number_of_allocations(vtkHeap* sself) { return sself->GetNumberOfAllocations(); }
extern "C" void vtk_heap_reset(vtkHeap* sself) { sself->Reset(); }
extern "C" vtkResourceFileLocator * vtkResourceFileLocator_new () {return vtkResourceFileLocator :: New () ;}
extern "C" void vtkResourceFileLocator_destructor (vtkResourceFileLocator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_resource_file_locator_set_print_debug_information(vtkResourceFileLocator* sself, bool p0) { sself->SetPrintDebugInformation(p0); }
extern "C" bool vtk_resource_file_locator_get_print_debug_information(vtkResourceFileLocator* sself) { return sself->GetPrintDebugInformation(); }
extern "C" void vtk_resource_file_locator_print_debug_information_on(vtkResourceFileLocator* sself) { sself->PrintDebugInformationOn(); }
extern "C" void vtk_resource_file_locator_print_debug_information_off(vtkResourceFileLocator* sself) { sself->PrintDebugInformationOff(); }
extern "C" void vtk_resource_file_locator_set_log_verbosity(vtkResourceFileLocator* sself, int _arg) { sself->SetLogVerbosity(_arg); }
extern "C" int vtk_resource_file_locator_get_log_verbosity(vtkResourceFileLocator* sself) { return sself->GetLogVerbosity(); }
