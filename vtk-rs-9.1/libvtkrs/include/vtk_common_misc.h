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

// Declare exported functions
extern "C" vtkContourValues * vtkContourValues_new () ;
extern "C" void vtkContourValues_destructor (vtkContourValues * sself) ;
extern "C" void vtk_contour_values_set_value(vtkContourValues* sself, int i, double value);
extern "C" double vtk_contour_values_get_value(vtkContourValues* sself, int i);
extern "C" void vtk_contour_values_set_number_of_contours(vtkContourValues* sself, const int number);
extern "C" int vtk_contour_values_get_number_of_contours(vtkContourValues* sself);
extern "C" void vtk_contour_values_generate_values(vtkContourValues* sself, int numContours, double rangeStart, double rangeEnd);
extern "C" vtkExprTkFunctionParser * vtkExprTkFunctionParser_new () ;
extern "C" void vtkExprTkFunctionParser_destructor (vtkExprTkFunctionParser * sself) ;
extern "C" unsigned long vtk_expr_tk_function_parser_get_m_time(vtkExprTkFunctionParser* sself);
extern "C" void vtk_expr_tk_function_parser_set_function(vtkExprTkFunctionParser* sself, const char* function);
extern "C" const char* vtk_expr_tk_function_parser_get_function(vtkExprTkFunctionParser* sself);
extern "C" int vtk_expr_tk_function_parser_is_scalar_result(vtkExprTkFunctionParser* sself);
extern "C" int vtk_expr_tk_function_parser_is_vector_result(vtkExprTkFunctionParser* sself);
extern "C" double vtk_expr_tk_function_parser_get_scalar_result(vtkExprTkFunctionParser* sself);
extern "C" void vtk_expr_tk_function_parser_set_scalar_variable_value(vtkExprTkFunctionParser* sself, const char*& variableName, double value);
extern "C" double vtk_expr_tk_function_parser_get_scalar_variable_value(vtkExprTkFunctionParser* sself, const char*& variableName);
extern "C" void vtk_expr_tk_function_parser_set_vector_variable_value(vtkExprTkFunctionParser* sself, const char*& variableName, double xValue, double yValue, double zValue);
extern "C" int vtk_expr_tk_function_parser_get_number_of_scalar_variables(vtkExprTkFunctionParser* sself);
extern "C" int vtk_expr_tk_function_parser_get_scalar_variable_index(vtkExprTkFunctionParser* sself, const char*& name);
extern "C" int vtk_expr_tk_function_parser_get_number_of_vector_variables(vtkExprTkFunctionParser* sself);
extern "C" int vtk_expr_tk_function_parser_get_vector_variable_index(vtkExprTkFunctionParser* sself, const char*& name);
extern "C" bool vtk_expr_tk_function_parser_get_scalar_variable_needed(vtkExprTkFunctionParser* sself, int i);
extern "C" bool vtk_expr_tk_function_parser_get_vector_variable_needed(vtkExprTkFunctionParser* sself, int i);
extern "C" void vtk_expr_tk_function_parser_remove_all_variables(vtkExprTkFunctionParser* sself);
extern "C" void vtk_expr_tk_function_parser_remove_scalar_variables(vtkExprTkFunctionParser* sself);
extern "C" void vtk_expr_tk_function_parser_remove_vector_variables(vtkExprTkFunctionParser* sself);
extern "C" void vtk_expr_tk_function_parser_set_replace_invalid_values(vtkExprTkFunctionParser* sself, int _arg);
extern "C" int vtk_expr_tk_function_parser_get_replace_invalid_values(vtkExprTkFunctionParser* sself);
extern "C" void vtk_expr_tk_function_parser_replace_invalid_values_on(vtkExprTkFunctionParser* sself);
extern "C" void vtk_expr_tk_function_parser_replace_invalid_values_off(vtkExprTkFunctionParser* sself);
extern "C" void vtk_expr_tk_function_parser_set_replacement_value(vtkExprTkFunctionParser* sself, double _arg);
extern "C" double vtk_expr_tk_function_parser_get_replacement_value(vtkExprTkFunctionParser* sself);
extern "C" void vtk_expr_tk_function_parser_invalidate_function(vtkExprTkFunctionParser* sself);
extern "C" vtkFunctionParser * vtkFunctionParser_new () ;
extern "C" void vtkFunctionParser_destructor (vtkFunctionParser * sself) ;
extern "C" unsigned long vtk_function_parser_get_m_time(vtkFunctionParser* sself);
extern "C" void vtk_function_parser_set_function(vtkFunctionParser* sself, const char* function);
extern "C" int vtk_function_parser_is_scalar_result(vtkFunctionParser* sself);
extern "C" int vtk_function_parser_is_vector_result(vtkFunctionParser* sself);
extern "C" double vtk_function_parser_get_scalar_result(vtkFunctionParser* sself);
extern "C" void vtk_function_parser_set_scalar_variable_value(vtkFunctionParser* sself, const char* variableName, double value);
extern "C" double vtk_function_parser_get_scalar_variable_value(vtkFunctionParser* sself, const char* variableName);
extern "C" void vtk_function_parser_set_vector_variable_value(vtkFunctionParser* sself, const char* variableName, double xValue, double yValue, double zValue);
extern "C" int vtk_function_parser_get_number_of_scalar_variables(vtkFunctionParser* sself);
extern "C" int vtk_function_parser_get_scalar_variable_index(vtkFunctionParser* sself, const char* name);
extern "C" int vtk_function_parser_get_number_of_vector_variables(vtkFunctionParser* sself);
extern "C" int vtk_function_parser_get_vector_variable_index(vtkFunctionParser* sself, const char* name);
extern "C" const char* vtk_function_parser_get_scalar_variable_name(vtkFunctionParser* sself, int i);
extern "C" const char* vtk_function_parser_get_vector_variable_name(vtkFunctionParser* sself, int i);
extern "C" bool vtk_function_parser_get_scalar_variable_needed(vtkFunctionParser* sself, int i);
extern "C" bool vtk_function_parser_get_vector_variable_needed(vtkFunctionParser* sself, int i);
extern "C" void vtk_function_parser_remove_all_variables(vtkFunctionParser* sself);
extern "C" void vtk_function_parser_remove_scalar_variables(vtkFunctionParser* sself);
extern "C" void vtk_function_parser_remove_vector_variables(vtkFunctionParser* sself);
extern "C" void vtk_function_parser_set_replace_invalid_values(vtkFunctionParser* sself, int _arg);
extern "C" int vtk_function_parser_get_replace_invalid_values(vtkFunctionParser* sself);
extern "C" void vtk_function_parser_replace_invalid_values_on(vtkFunctionParser* sself);
extern "C" void vtk_function_parser_replace_invalid_values_off(vtkFunctionParser* sself);
extern "C" void vtk_function_parser_set_replacement_value(vtkFunctionParser* sself, double _arg);
extern "C" double vtk_function_parser_get_replacement_value(vtkFunctionParser* sself);
extern "C" void vtk_function_parser_invalidate_function(vtkFunctionParser* sself);
extern "C" vtkHeap * vtkHeap_new () ;
extern "C" void vtkHeap_destructor (vtkHeap * sself) ;
extern "C" void* vtk_heap_allocate_memory(vtkHeap* sself, size_t n);
extern "C" void vtk_heap_set_block_size(vtkHeap* sself, size_t p0);
extern "C" size_t vtk_heap_get_block_size(vtkHeap* sself);
extern "C" int vtk_heap_get_number_of_blocks(vtkHeap* sself);
extern "C" int vtk_heap_get_number_of_allocations(vtkHeap* sself);
extern "C" void vtk_heap_reset(vtkHeap* sself);
extern "C" vtkResourceFileLocator * vtkResourceFileLocator_new () ;
extern "C" void vtkResourceFileLocator_destructor (vtkResourceFileLocator * sself) ;
extern "C" void vtk_resource_file_locator_set_print_debug_information(vtkResourceFileLocator* sself, bool p0);
extern "C" bool vtk_resource_file_locator_get_print_debug_information(vtkResourceFileLocator* sself);
extern "C" void vtk_resource_file_locator_print_debug_information_on(vtkResourceFileLocator* sself);
extern "C" void vtk_resource_file_locator_print_debug_information_off(vtkResourceFileLocator* sself);
extern "C" void vtk_resource_file_locator_set_log_verbosity(vtkResourceFileLocator* sself, int _arg);
extern "C" int vtk_resource_file_locator_get_log_verbosity(vtkResourceFileLocator* sself);
