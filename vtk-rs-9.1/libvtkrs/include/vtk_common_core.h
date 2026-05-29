// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkAOSDataArrayTemplate.h>
#include<vtkAbstractArray.h>
#include<vtkAnimationCue.h>
#include<vtkArchiver.h>
#include<vtkArray.h>
#include<vtkArrayCoordinates.h>
#include<vtkArrayExtents.h>
#include<vtkArrayExtentsList.h>
#include<vtkArrayIterator.h>
#include<vtkArrayIteratorTemplate.h>
#include<vtkArrayRange.h>
#include<vtkArraySort.h>
#include<vtkArrayWeights.h>
#include<vtkBitArray.h>
#include<vtkBitArrayIterator.h>
#include<vtkBoxMuellerRandomSequence.h>
#include<vtkBreakPoint.h>
#include<vtkBuffer.h>
#include<vtkByteSwap.h>
#include<vtkCallbackCommand.h>
#include<vtkCharArray.h>
#include<vtkCollection.h>
#include<vtkCollection.h>
#include<vtkCollectionIterator.h>
#include<vtkCommand.h>
#include<vtkCommonInformationKeyManager.h>
#include<vtkConditionVariable.h>
#include<vtkCriticalSection.h>
#include<vtkDataArray.h>
#include<vtkDataArrayCollection.h>
#include<vtkDataArrayCollectionIterator.h>
#include<vtkDataArraySelection.h>
#include<vtkDebugLeaks.h>
#include<vtkDebugLeaksManager.h>
#include<vtkDebugLeaks.h>
#include<vtkDenseArray.h>
#include<vtkDoubleArray.h>
#include<vtkDynamicLoader.h>
#include<vtkEventData.h>
#include<vtkEventData.h>
#include<vtkEventData.h>
#include<vtkEventForwarderCommand.h>
#include<vtkFileOutputWindow.h>
#include<vtkFloatArray.h>
#include<vtkFloatingPointExceptions.h>
#include<vtkGarbageCollector.h>
#include<vtkGarbageCollectorManager.h>
#include<vtkGaussianRandomSequence.h>
#include<vtkGenericDataArray.h>
#include<vtkGenericDataArrayLookupHelper.h>
#include<vtkIdList.h>
#include<vtkIdListCollection.h>
#include<vtkIdTypeArray.h>
#include<vtkIndent.h>
#include<vtkInformation.h>
#include<vtkInformationDataObjectKey.h>
#include<vtkInformationDoubleKey.h>
#include<vtkInformationDoubleVectorKey.h>
#include<vtkInformationIdTypeKey.h>
#include<vtkInformationInformationKey.h>
#include<vtkInformationInformationVectorKey.h>
#include<vtkInformationIntegerKey.h>
#include<vtkInformationIntegerPointerKey.h>
#include<vtkInformationIntegerVectorKey.h>
#include<vtkInformationInternals.h>
#include<vtkInformationIterator.h>
#include<vtkInformationKey.h>
#include<vtkInformationKeyLookup.h>
#include<vtkInformationKeyVectorKey.h>
#include<vtkInformationObjectBaseKey.h>
#include<vtkInformationObjectBaseVectorKey.h>
#include<vtkInformationRequestKey.h>
#include<vtkInformationStringKey.h>
#include<vtkInformationStringVectorKey.h>
#include<vtkInformationUnsignedLongKey.h>
#include<vtkInformationVariantKey.h>
#include<vtkInformationVariantVectorKey.h>
#include<vtkInformationVector.h>
#include<vtkIntArray.h>
#include<vtkLargeInteger.h>
#include<vtkLogger.h>
#include<vtkLongArray.h>
#include<vtkLongLongArray.h>
#include<vtkLookupTable.h>
#include<vtkMappedDataArray.h>
#include<vtkMath.h>
#include<vtkMersenneTwister.h>
#include<vtkMinimalStandardRandomSequence.h>
#include<vtkMultiThreader.h>
#include<vtkMutexLock.h>
#include<vtkNew.h>
#include<vtkOStrStreamWrapper.h>
#include<vtkObject.h>
#include<vtkObjectBase.h>
#include<vtkObjectFactory.h>
#include<vtkObjectFactoryCollection.h>
#include<vtkObjectFactory.h>
#include<vtkOldStyleCallbackCommand.h>
#include<vtkOutputWindow.h>
#include<vtkOutputWindow.h>
#include<vtkOverrideInformation.h>
#include<vtkOverrideInformationCollection.h>
#include<vtkPoints.h>
#include<vtkPoints2D.h>
#include<vtkPriorityQueue.h>
#include<vtkRandomPool.h>
#include<vtkRandomSequence.h>
#include<vtkReferenceCount.h>
#include<vtkSMPThreadLocal.h>
#include<vtkSMPThreadLocalObject.h>
#include<vtkSMPTools.h>
#include<vtkSOADataArrayTemplate.h>
#include<vtkScalarsToColors.h>
#include<vtkShortArray.h>
#include<vtkSignedCharArray.h>
#include<vtkConditionVariable.h>
#include<vtkSimpleCriticalSection.h>
#include<vtkMutexLock.h>
#include<vtkSmartPointer.h>
#include<vtkSmartPointerBase.h>
#include<vtkSortDataArray.h>
#include<vtkSparseArray.h>
#include<vtkStdString.h>
#include<vtkStringArray.h>
#include<vtkStringOutputWindow.h>
#include<vtkTestDataArray.h>
#include<vtkTimePointUtility.h>
#include<vtkTimeStamp.h>
#include<vtkTypeFloat32Array.h>
#include<vtkTypeFloat64Array.h>
#include<vtkTypeInt16Array.h>
#include<vtkTypeInt32Array.h>
#include<vtkTypeInt64Array.h>
#include<vtkTypeInt8Array.h>
#include<vtkTypeUInt16Array.h>
#include<vtkTypeUInt32Array.h>
#include<vtkTypeUInt64Array.h>
#include<vtkTypeUInt8Array.h>
#include<vtkTypedArray.h>
#include<vtkTypedDataArray.h>
#include<vtkTypedDataArrayIterator.h>
#include<vtkUnicodeString.h>
#include<vtkUnicodeStringArray.h>
#include<vtkUnsignedCharArray.h>
#include<vtkUnsignedIntArray.h>
#include<vtkUnsignedLongArray.h>
#include<vtkUnsignedLongLongArray.h>
#include<vtkUnsignedShortArray.h>
#include<vtkVariant.h>
#include<vtkVariantArray.h>
#include<vtkVersion.h>
#include<vtkVoidArray.h>
#include<vtkWeakPointer.h>
#include<vtkWeakPointerBase.h>
#include<vtkWeakReference.h>
#include<vtkWindow.h>
#include<vtkXMLFileOutputWindow.h>

// Declare exported functions
extern "C" vtkAnimationCue * vtkAnimationCue_new () ;
extern "C" void vtkAnimationCue_destructor (vtkAnimationCue * sself) ;
extern "C" void vtk_animation_cue_set_time_mode(vtkAnimationCue* sself, int mode);
extern "C" int vtk_animation_cue_get_time_mode(vtkAnimationCue* sself);
extern "C" void vtk_animation_cue_set_time_mode_to_relative(vtkAnimationCue* sself);
extern "C" void vtk_animation_cue_set_time_mode_to_normalized(vtkAnimationCue* sself);
extern "C" void vtk_animation_cue_set_start_time(vtkAnimationCue* sself, double _arg);
extern "C" double vtk_animation_cue_get_start_time(vtkAnimationCue* sself);
extern "C" void vtk_animation_cue_set_end_time(vtkAnimationCue* sself, double _arg);
extern "C" double vtk_animation_cue_get_end_time(vtkAnimationCue* sself);
extern "C" void vtk_animation_cue_tick(vtkAnimationCue* sself, double currenttime, double deltatime, double clocktime);
extern "C" void vtk_animation_cue_initialize(vtkAnimationCue* sself);
extern "C" void vtk_animation_cue_finalize(vtkAnimationCue* sself);
extern "C" double vtk_animation_cue_get_animation_time(vtkAnimationCue* sself);
extern "C" double vtk_animation_cue_get_delta_time(vtkAnimationCue* sself);
extern "C" double vtk_animation_cue_get_clock_time(vtkAnimationCue* sself);
extern "C" vtkArchiver * vtkArchiver_new () ;
extern "C" void vtkArchiver_destructor (vtkArchiver * sself) ;
extern "C" void vtk_archiver_set_archive_name(vtkArchiver* sself, const char* _arg);
extern "C" void vtk_archiver_open_archive(vtkArchiver* sself);
extern "C" void vtk_archiver_close_archive(vtkArchiver* sself);
extern "C" void vtk_archiver_insert_into_archive(vtkArchiver* sself, const char*& relativePath, const char* data, size_t size);
extern "C" bool vtk_archiver_contains(vtkArchiver* sself, const char*& relativePath);
extern "C" vtkBitArray * vtkBitArray_new () ;
extern "C" void vtkBitArray_destructor (vtkBitArray * sself) ;
extern "C" int vtk_bit_array_allocate(vtkBitArray* sself, long long sz, long long ext);
extern "C" void vtk_bit_array_initialize(vtkBitArray* sself);
extern "C" int vtk_bit_array_get_data_type(vtkBitArray* sself);
extern "C" int vtk_bit_array_get_data_type_size(vtkBitArray* sself);
extern "C" void vtk_bit_array_set_number_of_tuples(vtkBitArray* sself, long long number);
extern "C" bool vtk_bit_array_set_number_of_values(vtkBitArray* sself, long long number);
extern "C" void vtk_bit_array_remove_tuple(vtkBitArray* sself, long long id);
extern "C" void vtk_bit_array_set_component(vtkBitArray* sself, long long i, int j, double c);
extern "C" void vtk_bit_array_squeeze(vtkBitArray* sself);
extern "C" int vtk_bit_array_resize(vtkBitArray* sself, long long numTuples);
extern "C" int vtk_bit_array_get_value(vtkBitArray* sself, long long id);
extern "C" void vtk_bit_array_set_value(vtkBitArray* sself, long long id, int value);
extern "C" void vtk_bit_array_insert_value(vtkBitArray* sself, long long id, int i);
extern "C" long long vtk_bit_array_insert_next_value(vtkBitArray* sself, int i);
extern "C" void vtk_bit_array_insert_component(vtkBitArray* sself, long long i, int j, double c);
extern "C" void* vtk_bit_array_write_void_pointer(vtkBitArray* sself, long long id, long long number);
extern "C" void* vtk_bit_array_get_void_pointer(vtkBitArray* sself, long long id);
extern "C" void vtk_bit_array_set_void_array(vtkBitArray* sself, void* array, long long size, int save);
extern "C" void vtk_bit_array_data_changed(vtkBitArray* sself);
extern "C" void vtk_bit_array_clear_lookup(vtkBitArray* sself);
extern "C" vtkBitArrayIterator * vtkBitArrayIterator_new () ;
extern "C" void vtkBitArrayIterator_destructor (vtkBitArrayIterator * sself) ;
extern "C" int vtk_bit_array_iterator_get_value(vtkBitArrayIterator* sself, long long id);
extern "C" long long vtk_bit_array_iterator_get_number_of_tuples(vtkBitArrayIterator* sself);
extern "C" long long vtk_bit_array_iterator_get_number_of_values(vtkBitArrayIterator* sself);
extern "C" int vtk_bit_array_iterator_get_number_of_components(vtkBitArrayIterator* sself);
extern "C" int vtk_bit_array_iterator_get_data_type(vtkBitArrayIterator* sself);
extern "C" int vtk_bit_array_iterator_get_data_type_size(vtkBitArrayIterator* sself);
extern "C" void vtk_bit_array_iterator_set_value(vtkBitArrayIterator* sself, long long id, int value);
extern "C" vtkBoxMuellerRandomSequence * vtkBoxMuellerRandomSequence_new () ;
extern "C" void vtkBoxMuellerRandomSequence_destructor (vtkBoxMuellerRandomSequence * sself) ;
extern "C" void vtk_box_mueller_random_sequence_initialize(vtkBoxMuellerRandomSequence* sself, unsigned int seed);
extern "C" double vtk_box_mueller_random_sequence_get_value(vtkBoxMuellerRandomSequence* sself);
extern "C" void vtk_box_mueller_random_sequence_next(vtkBoxMuellerRandomSequence* sself);
extern "C" vtkByteSwap * vtkByteSwap_new () ;
extern "C" void vtkByteSwap_destructor (vtkByteSwap * sself) ;
extern "C" void vtk_byte_swap_swap_2_le(vtkByteSwap* sself, void* p);
extern "C" void vtk_byte_swap_swap_4_le(vtkByteSwap* sself, void* p);
extern "C" void vtk_byte_swap_swap_8_le(vtkByteSwap* sself, void* p);
extern "C" void vtk_byte_swap_swap_2_le_range(vtkByteSwap* sself, void* p, size_t num);
extern "C" void vtk_byte_swap_swap_4_le_range(vtkByteSwap* sself, void* p, size_t num);
extern "C" void vtk_byte_swap_swap_8_le_range(vtkByteSwap* sself, void* p, size_t num);
extern "C" void vtk_byte_swap_swap_2_be(vtkByteSwap* sself, void* p);
extern "C" void vtk_byte_swap_swap_4_be(vtkByteSwap* sself, void* p);
extern "C" void vtk_byte_swap_swap_8_be(vtkByteSwap* sself, void* p);
extern "C" void vtk_byte_swap_swap_2_be_range(vtkByteSwap* sself, void* p, size_t num);
extern "C" void vtk_byte_swap_swap_4_be_range(vtkByteSwap* sself, void* p, size_t num);
extern "C" void vtk_byte_swap_swap_8_be_range(vtkByteSwap* sself, void* p, size_t num);
extern "C" void vtk_byte_swap_swap_void_range(vtkByteSwap* sself, void* buffer, size_t numWords, size_t wordSize);
extern "C" vtkCallbackCommand * vtkCallbackCommand_new () ;
extern "C" void vtkCallbackCommand_destructor (vtkCallbackCommand * sself) ;
extern "C" void vtk_callback_command_set_client_data(vtkCallbackCommand* sself, void* cd);
extern "C" void* vtk_callback_command_get_client_data(vtkCallbackCommand* sself);
extern "C" void vtk_callback_command_set_abort_flag_on_execute(vtkCallbackCommand* sself, int f);
extern "C" int vtk_callback_command_get_abort_flag_on_execute(vtkCallbackCommand* sself);
extern "C" void vtk_callback_command_abort_flag_on_execute_on(vtkCallbackCommand* sself);
extern "C" void vtk_callback_command_abort_flag_on_execute_off(vtkCallbackCommand* sself);
extern "C" vtkCharArray * vtkCharArray_new () ;
extern "C" void vtkCharArray_destructor (vtkCharArray * sself) ;
extern "C" int vtk_char_array_get_data_type(vtkCharArray* sself);
extern "C" void vtk_char_array_set_typed_tuple(vtkCharArray* sself, long long i, const char* tuple);
extern "C" void vtk_char_array_insert_typed_tuple(vtkCharArray* sself, long long i, const char* tuple);
extern "C" long long vtk_char_array_insert_next_typed_tuple(vtkCharArray* sself, const char* tuple);
extern "C" char vtk_char_array_get_value(vtkCharArray* sself, long long id);
extern "C" void vtk_char_array_set_value(vtkCharArray* sself, long long id, char value);
extern "C" bool vtk_char_array_set_number_of_values(vtkCharArray* sself, long long number);
extern "C" void vtk_char_array_insert_value(vtkCharArray* sself, long long id, char f);
extern "C" long long vtk_char_array_insert_next_value(vtkCharArray* sself, char f);
extern "C" char vtk_char_array_get_data_type_value_min(vtkCharArray* sself);
extern "C" char vtk_char_array_get_data_type_value_max(vtkCharArray* sself);
extern "C" vtkCollection * vtkCollection_new () ;
extern "C" void vtkCollection_destructor (vtkCollection * sself) ;
extern "C" void vtk_collection_remove_item(vtkCollection* sself, int i);
extern "C" void vtk_collection_remove_all_items(vtkCollection* sself);
extern "C" int vtk_collection_get_number_of_items(vtkCollection* sself);
extern "C" void vtk_collection_init_traversal(vtkCollection* sself);
extern "C" vtkCollectionIterator * vtkCollectionIterator_new () ;
extern "C" void vtkCollectionIterator_destructor (vtkCollectionIterator * sself) ;
extern "C" void vtk_collection_iterator_init_traversal(vtkCollectionIterator* sself);
extern "C" void vtk_collection_iterator_go_to_first_item(vtkCollectionIterator* sself);
extern "C" void vtk_collection_iterator_go_to_next_item(vtkCollectionIterator* sself);
extern "C" int vtk_collection_iterator_is_done_with_traversal(vtkCollectionIterator* sself);
extern "C" vtkCriticalSection * vtkCriticalSection_new () ;
extern "C" void vtkCriticalSection_destructor (vtkCriticalSection * sself) ;
extern "C" void vtk_critical_section_lock(vtkCriticalSection* sself);
extern "C" void vtk_critical_section_unlock(vtkCriticalSection* sself);
extern "C" vtkDataArrayCollection * vtkDataArrayCollection_new () ;
extern "C" void vtkDataArrayCollection_destructor (vtkDataArrayCollection * sself) ;
extern "C" int vtk_data_array_collection_get_number_of_items(vtkDataArrayCollection* sself);
extern "C" vtkDataArrayCollectionIterator * vtkDataArrayCollectionIterator_new () ;
extern "C" void vtkDataArrayCollectionIterator_destructor (vtkDataArrayCollectionIterator * sself) ;
extern "C" vtkDataArraySelection * vtkDataArraySelection_new () ;
extern "C" void vtkDataArraySelection_destructor (vtkDataArraySelection * sself) ;
extern "C" void vtk_data_array_selection_enable_array(vtkDataArraySelection* sself, const char* name);
extern "C" void vtk_data_array_selection_disable_array(vtkDataArraySelection* sself, const char* name);
extern "C" int vtk_data_array_selection_array_is_enabled(vtkDataArraySelection* sself, const char* name);
extern "C" int vtk_data_array_selection_array_exists(vtkDataArraySelection* sself, const char* name);
extern "C" void vtk_data_array_selection_enable_all_arrays(vtkDataArraySelection* sself);
extern "C" void vtk_data_array_selection_disable_all_arrays(vtkDataArraySelection* sself);
extern "C" int vtk_data_array_selection_get_number_of_arrays(vtkDataArraySelection* sself);
extern "C" int vtk_data_array_selection_get_number_of_arrays_enabled(vtkDataArraySelection* sself);
extern "C" const char* vtk_data_array_selection_get_array_name(vtkDataArraySelection* sself, int index);
extern "C" int vtk_data_array_selection_get_array_index(vtkDataArraySelection* sself, const char* name);
extern "C" int vtk_data_array_selection_get_enabled_array_index(vtkDataArraySelection* sself, const char* name);
extern "C" int vtk_data_array_selection_get_array_setting(vtkDataArraySelection* sself, int index);
extern "C" void vtk_data_array_selection_set_array_setting(vtkDataArraySelection* sself, const char* name, int setting);
extern "C" void vtk_data_array_selection_remove_all_arrays(vtkDataArraySelection* sself);
extern "C" int vtk_data_array_selection_add_array(vtkDataArraySelection* sself, const char* name, bool state);
extern "C" void vtk_data_array_selection_remove_array_by_index(vtkDataArraySelection* sself, int index);
extern "C" void vtk_data_array_selection_remove_array_by_name(vtkDataArraySelection* sself, const char* name);
extern "C" void vtk_data_array_selection_set_unknown_array_setting(vtkDataArraySelection* sself, int _arg);
extern "C" int vtk_data_array_selection_get_unknown_array_setting(vtkDataArraySelection* sself);
extern "C" vtkDebugLeaks * vtkDebugLeaks_new () ;
extern "C" void vtkDebugLeaks_destructor (vtkDebugLeaks * sself) ;
extern "C" int vtk_debug_leaks_print_current_leaks(vtkDebugLeaks* sself);
extern "C" int vtk_debug_leaks_get_exit_error(vtkDebugLeaks* sself);
extern "C" void vtk_debug_leaks_set_exit_error(vtkDebugLeaks* sself, int p0);
extern "C" vtkDoubleArray * vtkDoubleArray_new () ;
extern "C" void vtkDoubleArray_destructor (vtkDoubleArray * sself) ;
extern "C" int vtk_double_array_get_data_type(vtkDoubleArray* sself);
extern "C" double vtk_double_array_get_value(vtkDoubleArray* sself, long long id);
extern "C" void vtk_double_array_set_value(vtkDoubleArray* sself, long long id, double value);
extern "C" bool vtk_double_array_set_number_of_values(vtkDoubleArray* sself, long long number);
extern "C" void vtk_double_array_insert_value(vtkDoubleArray* sself, long long id, double f);
extern "C" long long vtk_double_array_insert_next_value(vtkDoubleArray* sself, double f);
extern "C" double vtk_double_array_get_data_type_value_min(vtkDoubleArray* sself);
extern "C" double vtk_double_array_get_data_type_value_max(vtkDoubleArray* sself);
extern "C" vtkDynamicLoader * vtkDynamicLoader_new () ;
extern "C" void vtkDynamicLoader_destructor (vtkDynamicLoader * sself) ;
extern "C" const char* vtk_dynamic_loader_lib_prefix(vtkDynamicLoader* sself);
extern "C" const char* vtk_dynamic_loader_lib_extension(vtkDynamicLoader* sself);
extern "C" const char* vtk_dynamic_loader_last_error(vtkDynamicLoader* sself);
extern "C" vtkEventDataDevice3D * vtkEventDataDevice3D_new () ;
extern "C" void vtkEventDataDevice3D_destructor (vtkEventDataDevice3D * sself) ;
extern "C" void vtk_event_data_device_3_d_set_track_pad_position(vtkEventDataDevice3D* sself, double x, double y);
extern "C" vtkEventDataForDevice * vtkEventDataForDevice_new () ;
extern "C" void vtkEventDataForDevice_destructor (vtkEventDataForDevice * sself) ;
extern "C" vtkEventForwarderCommand * vtkEventForwarderCommand_new () ;
extern "C" void vtkEventForwarderCommand_destructor (vtkEventForwarderCommand * sself) ;
extern "C" void* vtk_event_forwarder_command_get_target(vtkEventForwarderCommand* sself);
extern "C" vtkFileOutputWindow * vtkFileOutputWindow_new () ;
extern "C" void vtkFileOutputWindow_destructor (vtkFileOutputWindow * sself) ;
extern "C" void vtk_file_output_window_display_text(vtkFileOutputWindow* sself, const char* p0);
extern "C" void vtk_file_output_window_set_file_name(vtkFileOutputWindow* sself, const char* _arg);
extern "C" void vtk_file_output_window_set_flush(vtkFileOutputWindow* sself, int _arg);
extern "C" int vtk_file_output_window_get_flush(vtkFileOutputWindow* sself);
extern "C" void vtk_file_output_window_flush_on(vtkFileOutputWindow* sself);
extern "C" void vtk_file_output_window_flush_off(vtkFileOutputWindow* sself);
extern "C" void vtk_file_output_window_set_append(vtkFileOutputWindow* sself, int _arg);
extern "C" int vtk_file_output_window_get_append(vtkFileOutputWindow* sself);
extern "C" void vtk_file_output_window_append_on(vtkFileOutputWindow* sself);
extern "C" void vtk_file_output_window_append_off(vtkFileOutputWindow* sself);
extern "C" vtkFloatArray * vtkFloatArray_new () ;
extern "C" void vtkFloatArray_destructor (vtkFloatArray * sself) ;
extern "C" int vtk_float_array_get_data_type(vtkFloatArray* sself);
extern "C" float vtk_float_array_get_value(vtkFloatArray* sself, long long id);
extern "C" void vtk_float_array_set_value(vtkFloatArray* sself, long long id, float value);
extern "C" bool vtk_float_array_set_number_of_values(vtkFloatArray* sself, long long number);
extern "C" void vtk_float_array_insert_value(vtkFloatArray* sself, long long id, float f);
extern "C" long long vtk_float_array_insert_next_value(vtkFloatArray* sself, float f);
extern "C" float vtk_float_array_get_data_type_value_min(vtkFloatArray* sself);
extern "C" float vtk_float_array_get_data_type_value_max(vtkFloatArray* sself);
extern "C" vtkGarbageCollector * vtkGarbageCollector_new () ;
extern "C" void vtkGarbageCollector_destructor (vtkGarbageCollector * sself) ;
extern "C" void vtk_garbage_collector_collect(vtkGarbageCollector* sself);
extern "C" void vtk_garbage_collector_deferred_collection_push(vtkGarbageCollector* sself);
extern "C" void vtk_garbage_collector_deferred_collection_pop(vtkGarbageCollector* sself);
extern "C" void vtk_garbage_collector_set_global_debug_flag(vtkGarbageCollector* sself, bool flag);
extern "C" bool vtk_garbage_collector_get_global_debug_flag(vtkGarbageCollector* sself);
extern "C" vtkIdList * vtkIdList_new () ;
extern "C" void vtkIdList_destructor (vtkIdList * sself) ;
extern "C" void vtk_id_list_initialize(vtkIdList* sself);
extern "C" int vtk_id_list_allocate(vtkIdList* sself, const long long sz, const int strategy);
extern "C" long long vtk_id_list_get_number_of_ids(vtkIdList* sself);
extern "C" long long vtk_id_list_get_id(vtkIdList* sself, const long long i);
extern "C" long long vtk_id_list_find_id_location(vtkIdList* sself, const long long id);
extern "C" void vtk_id_list_set_number_of_ids(vtkIdList* sself, const long long number);
extern "C" void vtk_id_list_set_id(vtkIdList* sself, const long long i, const long long vtkid);
extern "C" void vtk_id_list_insert_id(vtkIdList* sself, const long long i, const long long vtkid);
extern "C" long long vtk_id_list_insert_next_id(vtkIdList* sself, const long long vtkid);
extern "C" long long vtk_id_list_insert_unique_id(vtkIdList* sself, const long long vtkid);
extern "C" void vtk_id_list_sort(vtkIdList* sself);
extern "C" void vtk_id_list_fill(vtkIdList* sself, long long value);
extern "C" void vtk_id_list_reset(vtkIdList* sself);
extern "C" void vtk_id_list_squeeze(vtkIdList* sself);
extern "C" void vtk_id_list_delete_id(vtkIdList* sself, long long vtkid);
extern "C" long long vtk_id_list_is_id(vtkIdList* sself, long long vtkid);
extern "C" vtkIdListCollection * vtkIdListCollection_new () ;
extern "C" void vtkIdListCollection_destructor (vtkIdListCollection * sself) ;
extern "C" int vtk_id_list_collection_get_number_of_items(vtkIdListCollection* sself);
extern "C" vtkIdTypeArray * vtkIdTypeArray_new () ;
extern "C" void vtkIdTypeArray_destructor (vtkIdTypeArray * sself) ;
extern "C" int vtk_id_type_array_get_data_type(vtkIdTypeArray* sself);
extern "C" long long vtk_id_type_array_get_value(vtkIdTypeArray* sself, long long id);
extern "C" void vtk_id_type_array_set_value(vtkIdTypeArray* sself, long long id, long long value);
extern "C" bool vtk_id_type_array_set_number_of_values(vtkIdTypeArray* sself, long long number);
extern "C" void vtk_id_type_array_insert_value(vtkIdTypeArray* sself, long long id, long long f);
extern "C" long long vtk_id_type_array_insert_next_value(vtkIdTypeArray* sself, long long f);
extern "C" long long vtk_id_type_array_get_data_type_value_min(vtkIdTypeArray* sself);
extern "C" long long vtk_id_type_array_get_data_type_value_max(vtkIdTypeArray* sself);
extern "C" vtkInformation * vtkInformation_new () ;
extern "C" void vtkInformation_destructor (vtkInformation * sself) ;
extern "C" void vtk_information_modified(vtkInformation* sself);
extern "C" void vtk_information_clear(vtkInformation* sself);
extern "C" int vtk_information_get_number_of_keys(vtkInformation* sself);
extern "C" vtkInformationIterator * vtkInformationIterator_new () ;
extern "C" void vtkInformationIterator_destructor (vtkInformationIterator * sself) ;
extern "C" void vtk_information_iterator_init_traversal(vtkInformationIterator* sself);
extern "C" void vtk_information_iterator_go_to_first_item(vtkInformationIterator* sself);
extern "C" void vtk_information_iterator_go_to_next_item(vtkInformationIterator* sself);
extern "C" int vtk_information_iterator_is_done_with_traversal(vtkInformationIterator* sself);
extern "C" vtkInformationKeyLookup * vtkInformationKeyLookup_new () ;
extern "C" void vtkInformationKeyLookup_destructor (vtkInformationKeyLookup * sself) ;
extern "C" vtkInformationVector * vtkInformationVector_new () ;
extern "C" void vtkInformationVector_destructor (vtkInformationVector * sself) ;
extern "C" int vtk_information_vector_get_number_of_information_objects(vtkInformationVector* sself);
extern "C" void vtk_information_vector_set_number_of_information_objects(vtkInformationVector* sself, int n);
extern "C" vtkIntArray * vtkIntArray_new () ;
extern "C" void vtkIntArray_destructor (vtkIntArray * sself) ;
extern "C" int vtk_int_array_get_data_type(vtkIntArray* sself);
extern "C" int vtk_int_array_get_value(vtkIntArray* sself, long long id);
extern "C" void vtk_int_array_set_value(vtkIntArray* sself, long long id, int value);
extern "C" bool vtk_int_array_set_number_of_values(vtkIntArray* sself, long long number);
extern "C" void vtk_int_array_insert_value(vtkIntArray* sself, long long id, int f);
extern "C" long long vtk_int_array_insert_next_value(vtkIntArray* sself, int f);
extern "C" int vtk_int_array_get_data_type_value_min(vtkIntArray* sself);
extern "C" int vtk_int_array_get_data_type_value_max(vtkIntArray* sself);
extern "C" vtkLongArray * vtkLongArray_new () ;
extern "C" void vtkLongArray_destructor (vtkLongArray * sself) ;
extern "C" int vtk_long_array_get_data_type(vtkLongArray* sself);
extern "C" long vtk_long_array_get_value(vtkLongArray* sself, long long id);
extern "C" void vtk_long_array_set_value(vtkLongArray* sself, long long id, long value);
extern "C" bool vtk_long_array_set_number_of_values(vtkLongArray* sself, long long number);
extern "C" void vtk_long_array_insert_value(vtkLongArray* sself, long long id, long f);
extern "C" long long vtk_long_array_insert_next_value(vtkLongArray* sself, long f);
extern "C" long vtk_long_array_get_data_type_value_min(vtkLongArray* sself);
extern "C" long vtk_long_array_get_data_type_value_max(vtkLongArray* sself);
extern "C" vtkLongLongArray * vtkLongLongArray_new () ;
extern "C" void vtkLongLongArray_destructor (vtkLongLongArray * sself) ;
extern "C" int vtk_long_long_array_get_data_type(vtkLongLongArray* sself);
extern "C" long long vtk_long_long_array_get_value(vtkLongLongArray* sself, long long id);
extern "C" void vtk_long_long_array_set_value(vtkLongLongArray* sself, long long id, long long value);
extern "C" bool vtk_long_long_array_set_number_of_values(vtkLongLongArray* sself, long long number);
extern "C" void vtk_long_long_array_insert_value(vtkLongLongArray* sself, long long id, long long f);
extern "C" long long vtk_long_long_array_insert_next_value(vtkLongLongArray* sself, long long f);
extern "C" long long vtk_long_long_array_get_data_type_value_min(vtkLongLongArray* sself);
extern "C" long long vtk_long_long_array_get_data_type_value_max(vtkLongLongArray* sself);
extern "C" vtkLookupTable * vtkLookupTable_new () ;
extern "C" void vtkLookupTable_destructor (vtkLookupTable * sself) ;
extern "C" int vtk_lookup_table_is_opaque(vtkLookupTable* sself);
extern "C" int vtk_lookup_table_allocate(vtkLookupTable* sself, int sz, int ext);
extern "C" void vtk_lookup_table_build(vtkLookupTable* sself);
extern "C" void vtk_lookup_table_force_build(vtkLookupTable* sself);
extern "C" void vtk_lookup_table_build_special_colors(vtkLookupTable* sself);
extern "C" void vtk_lookup_table_set_ramp(vtkLookupTable* sself, int _arg);
extern "C" void vtk_lookup_table_set_ramp_to_linear(vtkLookupTable* sself);
extern "C" void vtk_lookup_table_set_ramp_to_s_curve(vtkLookupTable* sself);
extern "C" void vtk_lookup_table_set_ramp_to_sqrt(vtkLookupTable* sself);
extern "C" int vtk_lookup_table_get_ramp(vtkLookupTable* sself);
extern "C" void vtk_lookup_table_set_scale(vtkLookupTable* sself, int scale);
extern "C" void vtk_lookup_table_set_scale_to_linear(vtkLookupTable* sself);
extern "C" void vtk_lookup_table_set_scale_to_log_10(vtkLookupTable* sself);
extern "C" int vtk_lookup_table_get_scale(vtkLookupTable* sself);
extern "C" void vtk_lookup_table_set_table_range(vtkLookupTable* sself, double min, double max);
extern "C" void vtk_lookup_table_set_hue_range(vtkLookupTable* sself, double _arg1, double _arg2);
extern "C" void vtk_lookup_table_set_saturation_range(vtkLookupTable* sself, double _arg1, double _arg2);
extern "C" void vtk_lookup_table_set_value_range(vtkLookupTable* sself, double _arg1, double _arg2);
extern "C" void vtk_lookup_table_set_alpha_range(vtkLookupTable* sself, double _arg1, double _arg2);
extern "C" void vtk_lookup_table_set_nan_color(vtkLookupTable* sself, double _arg1, double _arg2, double _arg3, double _arg4);
extern "C" void vtk_lookup_table_set_below_range_color(vtkLookupTable* sself, double _arg1, double _arg2, double _arg3, double _arg4);
extern "C" void vtk_lookup_table_set_use_below_range_color(vtkLookupTable* sself, int _arg);
extern "C" int vtk_lookup_table_get_use_below_range_color(vtkLookupTable* sself);
extern "C" void vtk_lookup_table_use_below_range_color_on(vtkLookupTable* sself);
extern "C" void vtk_lookup_table_use_below_range_color_off(vtkLookupTable* sself);
extern "C" void vtk_lookup_table_set_above_range_color(vtkLookupTable* sself, double _arg1, double _arg2, double _arg3, double _arg4);
extern "C" void vtk_lookup_table_set_use_above_range_color(vtkLookupTable* sself, int _arg);
extern "C" int vtk_lookup_table_get_use_above_range_color(vtkLookupTable* sself);
extern "C" void vtk_lookup_table_use_above_range_color_on(vtkLookupTable* sself);
extern "C" void vtk_lookup_table_use_above_range_color_off(vtkLookupTable* sself);
extern "C" double vtk_lookup_table_get_opacity(vtkLookupTable* sself, double v);
extern "C" long long vtk_lookup_table_get_index(vtkLookupTable* sself, double v);
extern "C" void vtk_lookup_table_set_number_of_table_values(vtkLookupTable* sself, long long number);
extern "C" long long vtk_lookup_table_get_number_of_table_values(vtkLookupTable* sself);
extern "C" void vtk_lookup_table_set_table_value(vtkLookupTable* sself, long long indx, double r, double g, double b, double a);
extern "C" void vtk_lookup_table_set_number_of_colors(vtkLookupTable* sself, long long _arg);
extern "C" long long vtk_lookup_table_get_number_of_colors_min_value(vtkLookupTable* sself);
extern "C" long long vtk_lookup_table_get_number_of_colors_max_value(vtkLookupTable* sself);
extern "C" long long vtk_lookup_table_get_number_of_colors(vtkLookupTable* sself);
extern "C" int vtk_lookup_table_using_log_scale(vtkLookupTable* sself);
extern "C" vtkMath * vtkMath_new () ;
extern "C" void vtkMath_destructor (vtkMath * sself) ;
extern "C" double vtk_math_pi(vtkMath* sself);
extern "C" float vtk_math_radians_from_degrees(vtkMath* sself, float degrees);
extern "C" float vtk_math_degrees_from_radians(vtkMath* sself, float radians);
extern "C" int vtk_math_round(vtkMath* sself, float f);
extern "C" int vtk_math_floor(vtkMath* sself, double x);
extern "C" int vtk_math_ceil(vtkMath* sself, double x);
extern "C" int vtk_math_ceil_log_2(vtkMath* sself, unsigned long long x);
extern "C" bool vtk_math_is_power_of_two(vtkMath* sself, unsigned long long x);
extern "C" int vtk_math_nearest_power_of_two(vtkMath* sself, int x);
extern "C" long long vtk_math_factorial(vtkMath* sself, int N);
extern "C" long long vtk_math_binomial(vtkMath* sself, int m, int n);
extern "C" void vtk_math_random_seed(vtkMath* sself, int s);
extern "C" int vtk_math_get_seed(vtkMath* sself);
extern "C" double vtk_math_random(vtkMath* sself);
extern "C" double vtk_math_gaussian(vtkMath* sself);
extern "C" double vtk_math_gaussian_amplitude(vtkMath* sself, const double variance, const double distanceFromMean);
extern "C" double vtk_math_gaussian_weight(vtkMath* sself, const double variance, const double distanceFromMean);
extern "C" double vtk_math_determinant_2_x_2(vtkMath* sself, double a, double b, double c, double d);
extern "C" double vtk_math_determinant_3_x_3(vtkMath* sself, double a1, double a2, double a3, double b1, double b2, double b3, double c1, double c2, double c3);
extern "C" int vtk_math_solve_linear_system_gepp_2_x_2(vtkMath* sself, double a00, double a01, double a10, double a11, double b0, double b1, double& x0, double& x1);
extern "C" int vtk_math_get_scalar_type_fitting_range(vtkMath* sself, double range_min, double range_max, double scale, double shift);
extern "C" double vtk_math_inf(vtkMath* sself);
extern "C" double vtk_math_neg_inf(vtkMath* sself);
extern "C" double vtk_math_nan(vtkMath* sself);
extern "C" int vtk_math_is_inf(vtkMath* sself, double x);
extern "C" int vtk_math_is_nan(vtkMath* sself, double x);
extern "C" bool vtk_math_is_finite(vtkMath* sself, double x);
extern "C" vtkMersenneTwister * vtkMersenneTwister_new () ;
extern "C" void vtkMersenneTwister_destructor (vtkMersenneTwister * sself) ;
extern "C" void vtk_mersenne_twister_initialize(vtkMersenneTwister* sself, unsigned int seed);
extern "C" unsigned int vtk_mersenne_twister_initialize_new_sequence(vtkMersenneTwister* sself, unsigned int seed, int p);
extern "C" void vtk_mersenne_twister_initialize_sequence(vtkMersenneTwister* sself, unsigned int id, unsigned int seed, int p);
extern "C" double vtk_mersenne_twister_get_value(vtkMersenneTwister* sself, unsigned int id);
extern "C" void vtk_mersenne_twister_next(vtkMersenneTwister* sself, unsigned int id);
extern "C" vtkMinimalStandardRandomSequence * vtkMinimalStandardRandomSequence_new () ;
extern "C" void vtkMinimalStandardRandomSequence_destructor (vtkMinimalStandardRandomSequence * sself) ;
extern "C" void vtk_minimal_standard_random_sequence_initialize(vtkMinimalStandardRandomSequence* sself, unsigned int seed);
extern "C" void vtk_minimal_standard_random_sequence_set_seed(vtkMinimalStandardRandomSequence* sself, int value);
extern "C" void vtk_minimal_standard_random_sequence_set_seed_only(vtkMinimalStandardRandomSequence* sself, int value);
extern "C" int vtk_minimal_standard_random_sequence_get_seed(vtkMinimalStandardRandomSequence* sself);
extern "C" double vtk_minimal_standard_random_sequence_get_value(vtkMinimalStandardRandomSequence* sself);
extern "C" void vtk_minimal_standard_random_sequence_next(vtkMinimalStandardRandomSequence* sself);
extern "C" double vtk_minimal_standard_random_sequence_get_range_value(vtkMinimalStandardRandomSequence* sself, double rangeMin, double rangeMax);
extern "C" double vtk_minimal_standard_random_sequence_get_next_range_value(vtkMinimalStandardRandomSequence* sself, double rangeMin, double rangeMax);
extern "C" vtkMultiThreader * vtkMultiThreader_new () ;
extern "C" void vtkMultiThreader_destructor (vtkMultiThreader * sself) ;
extern "C" void vtk_multi_threader_set_number_of_threads(vtkMultiThreader* sself, int _arg);
extern "C" int vtk_multi_threader_get_number_of_threads_min_value(vtkMultiThreader* sself);
extern "C" int vtk_multi_threader_get_number_of_threads_max_value(vtkMultiThreader* sself);
extern "C" int vtk_multi_threader_get_number_of_threads(vtkMultiThreader* sself);
extern "C" int vtk_multi_threader_get_global_static_maximum_number_of_threads(vtkMultiThreader* sself);
extern "C" void vtk_multi_threader_set_global_maximum_number_of_threads(vtkMultiThreader* sself, int val);
extern "C" int vtk_multi_threader_get_global_maximum_number_of_threads(vtkMultiThreader* sself);
extern "C" void vtk_multi_threader_set_global_default_number_of_threads(vtkMultiThreader* sself, int val);
extern "C" int vtk_multi_threader_get_global_default_number_of_threads(vtkMultiThreader* sself);
extern "C" void vtk_multi_threader_single_method_execute(vtkMultiThreader* sself);
extern "C" void vtk_multi_threader_multiple_method_execute(vtkMultiThreader* sself);
extern "C" void vtk_multi_threader_terminate_thread(vtkMultiThreader* sself, int threadId);
extern "C" int vtk_multi_threader_is_thread_active(vtkMultiThreader* sself, int threadId);
extern "C" vtkObject * vtkObject_new () ;
extern "C" void vtkObject_destructor (vtkObject * sself) ;
extern "C" int vtk_object_is_type_of(vtkObject* sself, const char* type);
extern "C" int vtk_object_is_a(vtkObject* sself, const char* type);
extern "C" long long vtk_object_get_number_of_generations_from_base_type(vtkObject* sself, const char* type);
extern "C" long long vtk_object_get_number_of_generations_from_base(vtkObject* sself, const char* type);
extern "C" void vtk_object_debug_on(vtkObject* sself);
extern "C" void vtk_object_debug_off(vtkObject* sself);
extern "C" bool vtk_object_get_debug(vtkObject* sself);
extern "C" void vtk_object_set_debug(vtkObject* sself, bool debugFlag);
extern "C" void vtk_object_break_on_error(vtkObject* sself);
extern "C" void vtk_object_modified(vtkObject* sself);
extern "C" unsigned long vtk_object_get_m_time(vtkObject* sself);
extern "C" void vtk_object_set_global_warning_display(vtkObject* sself, int val);
extern "C" void vtk_object_global_warning_display_on(vtkObject* sself);
extern "C" void vtk_object_global_warning_display_off(vtkObject* sself);
extern "C" int vtk_object_get_global_warning_display(vtkObject* sself);
extern "C" void vtk_object_remove_all_observers(vtkObject* sself);
extern "C" int vtk_object_invoke_event(vtkObject* sself, unsigned long event, void* callData);
extern "C" vtkObjectFactoryCollection * vtkObjectFactoryCollection_new () ;
extern "C" void vtkObjectFactoryCollection_destructor (vtkObjectFactoryCollection * sself) ;
extern "C" vtkOldStyleCallbackCommand * vtkOldStyleCallbackCommand_new () ;
extern "C" void vtkOldStyleCallbackCommand_destructor (vtkOldStyleCallbackCommand * sself) ;
extern "C" void vtk_old_style_callback_command_set_client_data(vtkOldStyleCallbackCommand* sself, void* cd);
extern "C" vtkOutputWindow * vtkOutputWindow_new () ;
extern "C" void vtkOutputWindow_destructor (vtkOutputWindow * sself) ;
extern "C" void vtk_output_window_display_text(vtkOutputWindow* sself, const char* p0);
extern "C" void vtk_output_window_display_error_text(vtkOutputWindow* sself, const char* p0);
extern "C" void vtk_output_window_display_warning_text(vtkOutputWindow* sself, const char* p0);
extern "C" void vtk_output_window_display_generic_warning_text(vtkOutputWindow* sself, const char* p0);
extern "C" void vtk_output_window_display_debug_text(vtkOutputWindow* sself, const char* p0);
extern "C" void vtk_output_window_prompt_user_on(vtkOutputWindow* sself);
extern "C" void vtk_output_window_prompt_user_off(vtkOutputWindow* sself);
extern "C" void vtk_output_window_set_prompt_user(vtkOutputWindow* sself, bool _arg);
extern "C" void vtk_output_window_set_use_std_error_for_all_messages(vtkOutputWindow* sself, bool p0);
extern "C" bool vtk_output_window_get_use_std_error_for_all_messages(vtkOutputWindow* sself);
extern "C" void vtk_output_window_use_std_error_for_all_messages_on(vtkOutputWindow* sself);
extern "C" void vtk_output_window_use_std_error_for_all_messages_off(vtkOutputWindow* sself);
extern "C" void vtk_output_window_set_display_mode(vtkOutputWindow* sself, int _arg);
extern "C" int vtk_output_window_get_display_mode_min_value(vtkOutputWindow* sself);
extern "C" int vtk_output_window_get_display_mode_max_value(vtkOutputWindow* sself);
extern "C" int vtk_output_window_get_display_mode(vtkOutputWindow* sself);
extern "C" void vtk_output_window_set_display_mode_to_default(vtkOutputWindow* sself);
extern "C" void vtk_output_window_set_display_mode_to_never(vtkOutputWindow* sself);
extern "C" void vtk_output_window_set_display_mode_to_always(vtkOutputWindow* sself);
extern "C" void vtk_output_window_set_display_mode_to_always_std_err(vtkOutputWindow* sself);
extern "C" vtkOverrideInformationCollection * vtkOverrideInformationCollection_new () ;
extern "C" void vtkOverrideInformationCollection_destructor (vtkOverrideInformationCollection * sself) ;
extern "C" vtkPoints * vtkPoints_new () ;
extern "C" void vtkPoints_destructor (vtkPoints * sself) ;
extern "C" int vtk_points_allocate(vtkPoints* sself, long long sz, long long ext);
extern "C" void vtk_points_initialize(vtkPoints* sself);
extern "C" int vtk_points_get_data_type(vtkPoints* sself);
extern "C" void vtk_points_set_data_type(vtkPoints* sself, int dataType);
extern "C" void vtk_points_set_data_type_to_bit(vtkPoints* sself);
extern "C" void vtk_points_set_data_type_to_char(vtkPoints* sself);
extern "C" void vtk_points_set_data_type_to_unsigned_char(vtkPoints* sself);
extern "C" void vtk_points_set_data_type_to_short(vtkPoints* sself);
extern "C" void vtk_points_set_data_type_to_unsigned_short(vtkPoints* sself);
extern "C" void vtk_points_set_data_type_to_int(vtkPoints* sself);
extern "C" void vtk_points_set_data_type_to_unsigned_int(vtkPoints* sself);
extern "C" void vtk_points_set_data_type_to_long(vtkPoints* sself);
extern "C" void vtk_points_set_data_type_to_unsigned_long(vtkPoints* sself);
extern "C" void vtk_points_set_data_type_to_float(vtkPoints* sself);
extern "C" void vtk_points_set_data_type_to_double(vtkPoints* sself);
extern "C" void* vtk_points_get_void_pointer(vtkPoints* sself, const int id);
extern "C" void vtk_points_squeeze(vtkPoints* sself);
extern "C" void vtk_points_reset(vtkPoints* sself);
extern "C" unsigned long vtk_points_get_actual_memory_size(vtkPoints* sself);
extern "C" long long vtk_points_get_number_of_points(vtkPoints* sself);
extern "C" void vtk_points_set_point(vtkPoints* sself, long long id, double x, double y, double z);
extern "C" void vtk_points_insert_point(vtkPoints* sself, long long id, double x, double y, double z);
extern "C" long long vtk_points_insert_next_point(vtkPoints* sself, double x, double y, double z);
extern "C" void vtk_points_set_number_of_points(vtkPoints* sself, long long numPoints);
extern "C" int vtk_points_resize(vtkPoints* sself, long long numPoints);
extern "C" void vtk_points_compute_bounds(vtkPoints* sself);
extern "C" unsigned long vtk_points_get_m_time(vtkPoints* sself);
extern "C" void vtk_points_modified(vtkPoints* sself);
extern "C" vtkPoints2D * vtkPoints2D_new () ;
extern "C" void vtkPoints2D_destructor (vtkPoints2D * sself) ;
extern "C" int vtk_points_2_d_allocate(vtkPoints2D* sself, long long sz, long long ext);
extern "C" void vtk_points_2_d_initialize(vtkPoints2D* sself);
extern "C" int vtk_points_2_d_get_data_type(vtkPoints2D* sself);
extern "C" void vtk_points_2_d_set_data_type(vtkPoints2D* sself, int dataType);
extern "C" void vtk_points_2_d_set_data_type_to_bit(vtkPoints2D* sself);
extern "C" void vtk_points_2_d_set_data_type_to_char(vtkPoints2D* sself);
extern "C" void vtk_points_2_d_set_data_type_to_unsigned_char(vtkPoints2D* sself);
extern "C" void vtk_points_2_d_set_data_type_to_short(vtkPoints2D* sself);
extern "C" void vtk_points_2_d_set_data_type_to_unsigned_short(vtkPoints2D* sself);
extern "C" void vtk_points_2_d_set_data_type_to_int(vtkPoints2D* sself);
extern "C" void vtk_points_2_d_set_data_type_to_unsigned_int(vtkPoints2D* sself);
extern "C" void vtk_points_2_d_set_data_type_to_long(vtkPoints2D* sself);
extern "C" void vtk_points_2_d_set_data_type_to_unsigned_long(vtkPoints2D* sself);
extern "C" void vtk_points_2_d_set_data_type_to_float(vtkPoints2D* sself);
extern "C" void vtk_points_2_d_set_data_type_to_double(vtkPoints2D* sself);
extern "C" void* vtk_points_2_d_get_void_pointer(vtkPoints2D* sself, const int id);
extern "C" void vtk_points_2_d_squeeze(vtkPoints2D* sself);
extern "C" void vtk_points_2_d_reset(vtkPoints2D* sself);
extern "C" unsigned long vtk_points_2_d_get_actual_memory_size(vtkPoints2D* sself);
extern "C" long long vtk_points_2_d_get_number_of_points(vtkPoints2D* sself);
extern "C" void vtk_points_2_d_set_point(vtkPoints2D* sself, long long id, double x, double y);
extern "C" void vtk_points_2_d_insert_point(vtkPoints2D* sself, long long id, double x, double y);
extern "C" long long vtk_points_2_d_insert_next_point(vtkPoints2D* sself, double x, double y);
extern "C" void vtk_points_2_d_remove_point(vtkPoints2D* sself, long long id);
extern "C" void vtk_points_2_d_set_number_of_points(vtkPoints2D* sself, long long numPoints);
extern "C" int vtk_points_2_d_resize(vtkPoints2D* sself, long long numPoints);
extern "C" void vtk_points_2_d_compute_bounds(vtkPoints2D* sself);
extern "C" vtkPriorityQueue * vtkPriorityQueue_new () ;
extern "C" void vtkPriorityQueue_destructor (vtkPriorityQueue * sself) ;
extern "C" void vtk_priority_queue_allocate(vtkPriorityQueue* sself, long long sz, long long ext);
extern "C" void vtk_priority_queue_insert(vtkPriorityQueue* sself, double priority, long long id);
extern "C" long long vtk_priority_queue_pop(vtkPriorityQueue* sself, long long location, double& priority);
extern "C" long long vtk_priority_queue_peek(vtkPriorityQueue* sself, long long location, double& priority);
extern "C" double vtk_priority_queue_delete_id(vtkPriorityQueue* sself, long long id);
extern "C" double vtk_priority_queue_get_priority(vtkPriorityQueue* sself, long long id);
extern "C" long long vtk_priority_queue_get_number_of_items(vtkPriorityQueue* sself);
extern "C" void vtk_priority_queue_reset(vtkPriorityQueue* sself);
extern "C" vtkRandomPool * vtkRandomPool_new () ;
extern "C" void vtkRandomPool_destructor (vtkRandomPool * sself) ;
extern "C" void vtk_random_pool_set_size(vtkRandomPool* sself, long long _arg);
extern "C" long long vtk_random_pool_get_size_min_value(vtkRandomPool* sself);
extern "C" long long vtk_random_pool_get_size_max_value(vtkRandomPool* sself);
extern "C" long long vtk_random_pool_get_size(vtkRandomPool* sself);
extern "C" void vtk_random_pool_set_number_of_components(vtkRandomPool* sself, long long _arg);
extern "C" long long vtk_random_pool_get_number_of_components_min_value(vtkRandomPool* sself);
extern "C" long long vtk_random_pool_get_number_of_components_max_value(vtkRandomPool* sself);
extern "C" long long vtk_random_pool_get_number_of_components(vtkRandomPool* sself);
extern "C" long long vtk_random_pool_get_total_size(vtkRandomPool* sself);
extern "C" double vtk_random_pool_get_value(vtkRandomPool* sself, long long i);
extern "C" void vtk_random_pool_set_chunk_size(vtkRandomPool* sself, long long _arg);
extern "C" long long vtk_random_pool_get_chunk_size_min_value(vtkRandomPool* sself);
extern "C" long long vtk_random_pool_get_chunk_size_max_value(vtkRandomPool* sself);
extern "C" long long vtk_random_pool_get_chunk_size(vtkRandomPool* sself);
extern "C" vtkReferenceCount * vtkReferenceCount_new () ;
extern "C" void vtkReferenceCount_destructor (vtkReferenceCount * sself) ;
extern "C" vtkScalarsToColors * vtkScalarsToColors_new () ;
extern "C" void vtkScalarsToColors_destructor (vtkScalarsToColors * sself) ;
extern "C" int vtk_scalars_to_colors_is_opaque(vtkScalarsToColors* sself);
extern "C" void vtk_scalars_to_colors_build(vtkScalarsToColors* sself);
extern "C" void vtk_scalars_to_colors_set_range(vtkScalarsToColors* sself, double min, double max);
extern "C" double vtk_scalars_to_colors_get_opacity(vtkScalarsToColors* sself, double v);
extern "C" double vtk_scalars_to_colors_get_luminance(vtkScalarsToColors* sself, double x);
extern "C" void vtk_scalars_to_colors_set_alpha(vtkScalarsToColors* sself, double alpha);
extern "C" double vtk_scalars_to_colors_get_alpha(vtkScalarsToColors* sself);
extern "C" void vtk_scalars_to_colors_set_vector_mode(vtkScalarsToColors* sself, int _arg);
extern "C" int vtk_scalars_to_colors_get_vector_mode(vtkScalarsToColors* sself);
extern "C" void vtk_scalars_to_colors_set_vector_mode_to_magnitude(vtkScalarsToColors* sself);
extern "C" void vtk_scalars_to_colors_set_vector_mode_to_component(vtkScalarsToColors* sself);
extern "C" void vtk_scalars_to_colors_set_vector_mode_to_rgb_colors(vtkScalarsToColors* sself);
extern "C" void vtk_scalars_to_colors_set_vector_component(vtkScalarsToColors* sself, int _arg);
extern "C" int vtk_scalars_to_colors_get_vector_component(vtkScalarsToColors* sself);
extern "C" void vtk_scalars_to_colors_set_vector_size(vtkScalarsToColors* sself, int _arg);
extern "C" int vtk_scalars_to_colors_get_vector_size(vtkScalarsToColors* sself);
extern "C" int vtk_scalars_to_colors_using_log_scale(vtkScalarsToColors* sself);
extern "C" long long vtk_scalars_to_colors_get_number_of_available_colors(vtkScalarsToColors* sself);
extern "C" long long vtk_scalars_to_colors_get_number_of_annotated_values(vtkScalarsToColors* sself);
extern "C" void vtk_scalars_to_colors_reset_annotations(vtkScalarsToColors* sself);
extern "C" void vtk_scalars_to_colors_set_indexed_lookup(vtkScalarsToColors* sself, int _arg);
extern "C" int vtk_scalars_to_colors_get_indexed_lookup(vtkScalarsToColors* sself);
extern "C" void vtk_scalars_to_colors_indexed_lookup_on(vtkScalarsToColors* sself);
extern "C" void vtk_scalars_to_colors_indexed_lookup_off(vtkScalarsToColors* sself);
extern "C" vtkShortArray * vtkShortArray_new () ;
extern "C" void vtkShortArray_destructor (vtkShortArray * sself) ;
extern "C" int vtk_short_array_get_data_type(vtkShortArray* sself);
extern "C" short vtk_short_array_get_value(vtkShortArray* sself, long long id);
extern "C" void vtk_short_array_set_value(vtkShortArray* sself, long long id, short value);
extern "C" bool vtk_short_array_set_number_of_values(vtkShortArray* sself, long long number);
extern "C" void vtk_short_array_insert_value(vtkShortArray* sself, long long id, short f);
extern "C" long long vtk_short_array_insert_next_value(vtkShortArray* sself, short f);
extern "C" short vtk_short_array_get_data_type_value_min(vtkShortArray* sself);
extern "C" short vtk_short_array_get_data_type_value_max(vtkShortArray* sself);
extern "C" vtkSignedCharArray * vtkSignedCharArray_new () ;
extern "C" void vtkSignedCharArray_destructor (vtkSignedCharArray * sself) ;
extern "C" int vtk_signed_char_array_get_data_type(vtkSignedCharArray* sself);
extern "C" signed char vtk_signed_char_array_get_value(vtkSignedCharArray* sself, long long id);
extern "C" void vtk_signed_char_array_set_value(vtkSignedCharArray* sself, long long id, signed char value);
extern "C" bool vtk_signed_char_array_set_number_of_values(vtkSignedCharArray* sself, long long number);
extern "C" void vtk_signed_char_array_insert_value(vtkSignedCharArray* sself, long long id, signed char f);
extern "C" long long vtk_signed_char_array_insert_next_value(vtkSignedCharArray* sself, signed char f);
extern "C" signed char vtk_signed_char_array_get_data_type_value_min(vtkSignedCharArray* sself);
extern "C" signed char vtk_signed_char_array_get_data_type_value_max(vtkSignedCharArray* sself);
extern "C" vtkSortDataArray * vtkSortDataArray_new () ;
extern "C" void vtkSortDataArray_destructor (vtkSortDataArray * sself) ;
extern "C" vtkStringArray * vtkStringArray_new () ;
extern "C" void vtkStringArray_destructor (vtkStringArray * sself) ;
extern "C" int vtk_string_array_get_data_type(vtkStringArray* sself);
extern "C" int vtk_string_array_is_numeric(vtkStringArray* sself);
extern "C" void vtk_string_array_initialize(vtkStringArray* sself);
extern "C" int vtk_string_array_get_data_type_size(vtkStringArray* sself);
extern "C" void vtk_string_array_squeeze(vtkStringArray* sself);
extern "C" int vtk_string_array_resize(vtkStringArray* sself, long long numTuples);
extern "C" int vtk_string_array_allocate(vtkStringArray* sself, long long sz, long long ext);
extern "C" void vtk_string_array_set_number_of_tuples(vtkStringArray* sself, long long number);
extern "C" long long vtk_string_array_get_number_of_values(vtkStringArray* sself);
extern "C" int vtk_string_array_get_number_of_element_components(vtkStringArray* sself);
extern "C" int vtk_string_array_get_element_component_size(vtkStringArray* sself);
extern "C" void* vtk_string_array_get_void_pointer(vtkStringArray* sself, long long id);
extern "C" void vtk_string_array_set_void_array(vtkStringArray* sself, void* array, long long size, int save);
extern "C" unsigned long vtk_string_array_get_actual_memory_size(vtkStringArray* sself);
extern "C" long long vtk_string_array_get_data_size(vtkStringArray* sself);
extern "C" void vtk_string_array_data_changed(vtkStringArray* sself);
extern "C" void vtk_string_array_data_element_changed(vtkStringArray* sself, long long id);
extern "C" void vtk_string_array_clear_lookup(vtkStringArray* sself);
extern "C" vtkStringOutputWindow * vtkStringOutputWindow_new () ;
extern "C" void vtkStringOutputWindow_destructor (vtkStringOutputWindow * sself) ;
extern "C" void vtk_string_output_window_display_text(vtkStringOutputWindow* sself, const char* p0);
extern "C" vtkTimePointUtility * vtkTimePointUtility_new () ;
extern "C" void vtkTimePointUtility_destructor (vtkTimePointUtility * sself) ;
extern "C" unsigned long long vtk_time_point_utility_date_to_time_point(vtkTimePointUtility* sself, int year, int month, int day);
extern "C" unsigned long long vtk_time_point_utility_time_to_time_point(vtkTimePointUtility* sself, int hour, int minute, int second, int millis);
extern "C" unsigned long long vtk_time_point_utility_date_time_to_time_point(vtkTimePointUtility* sself, int year, int month, int day, int hour, int minute, int sec, int millis);
extern "C" void vtk_time_point_utility_get_date(vtkTimePointUtility* sself, unsigned long long time, int& year, int& month, int& day);
extern "C" void vtk_time_point_utility_get_time(vtkTimePointUtility* sself, unsigned long long time, int& hour, int& minute, int& second, int& millis);
extern "C" void vtk_time_point_utility_get_date_time(vtkTimePointUtility* sself, unsigned long long time, int& year, int& month, int& day, int& hour, int& minute, int& second, int& millis);
extern "C" int vtk_time_point_utility_get_year(vtkTimePointUtility* sself, unsigned long long time);
extern "C" int vtk_time_point_utility_get_month(vtkTimePointUtility* sself, unsigned long long time);
extern "C" int vtk_time_point_utility_get_day(vtkTimePointUtility* sself, unsigned long long time);
extern "C" int vtk_time_point_utility_get_hour(vtkTimePointUtility* sself, unsigned long long time);
extern "C" int vtk_time_point_utility_get_minute(vtkTimePointUtility* sself, unsigned long long time);
extern "C" int vtk_time_point_utility_get_second(vtkTimePointUtility* sself, unsigned long long time);
extern "C" int vtk_time_point_utility_get_millisecond(vtkTimePointUtility* sself, unsigned long long time);
extern "C" const char* vtk_time_point_utility_time_point_to_iso_8601(vtkTimePointUtility* sself, unsigned long long p0, int format);
extern "C" vtkTypeFloat32Array * vtkTypeFloat32Array_new () ;
extern "C" void vtkTypeFloat32Array_destructor (vtkTypeFloat32Array * sself) ;
extern "C" vtkTypeFloat64Array * vtkTypeFloat64Array_new () ;
extern "C" void vtkTypeFloat64Array_destructor (vtkTypeFloat64Array * sself) ;
extern "C" vtkTypeInt16Array * vtkTypeInt16Array_new () ;
extern "C" void vtkTypeInt16Array_destructor (vtkTypeInt16Array * sself) ;
extern "C" vtkTypeInt32Array * vtkTypeInt32Array_new () ;
extern "C" void vtkTypeInt32Array_destructor (vtkTypeInt32Array * sself) ;
extern "C" vtkTypeInt64Array * vtkTypeInt64Array_new () ;
extern "C" void vtkTypeInt64Array_destructor (vtkTypeInt64Array * sself) ;
extern "C" vtkTypeInt8Array * vtkTypeInt8Array_new () ;
extern "C" void vtkTypeInt8Array_destructor (vtkTypeInt8Array * sself) ;
extern "C" vtkTypeUInt16Array * vtkTypeUInt16Array_new () ;
extern "C" void vtkTypeUInt16Array_destructor (vtkTypeUInt16Array * sself) ;
extern "C" vtkTypeUInt32Array * vtkTypeUInt32Array_new () ;
extern "C" void vtkTypeUInt32Array_destructor (vtkTypeUInt32Array * sself) ;
extern "C" vtkTypeUInt64Array * vtkTypeUInt64Array_new () ;
extern "C" void vtkTypeUInt64Array_destructor (vtkTypeUInt64Array * sself) ;
extern "C" vtkTypeUInt8Array * vtkTypeUInt8Array_new () ;
extern "C" void vtkTypeUInt8Array_destructor (vtkTypeUInt8Array * sself) ;
extern "C" vtkUnicodeStringArray * vtkUnicodeStringArray_new () ;
extern "C" void vtkUnicodeStringArray_destructor (vtkUnicodeStringArray * sself) ;
extern "C" int vtk_unicode_string_array_allocate(vtkUnicodeStringArray* sself, long long sz, long long ext);
extern "C" void vtk_unicode_string_array_initialize(vtkUnicodeStringArray* sself);
extern "C" int vtk_unicode_string_array_get_data_type(vtkUnicodeStringArray* sself);
extern "C" int vtk_unicode_string_array_get_data_type_size(vtkUnicodeStringArray* sself);
extern "C" int vtk_unicode_string_array_get_element_component_size(vtkUnicodeStringArray* sself);
extern "C" void vtk_unicode_string_array_set_number_of_tuples(vtkUnicodeStringArray* sself, long long number);
extern "C" void* vtk_unicode_string_array_get_void_pointer(vtkUnicodeStringArray* sself, long long id);
extern "C" void vtk_unicode_string_array_squeeze(vtkUnicodeStringArray* sself);
extern "C" int vtk_unicode_string_array_resize(vtkUnicodeStringArray* sself, long long numTuples);
extern "C" void vtk_unicode_string_array_set_void_array(vtkUnicodeStringArray* sself, void* array, long long size, int save);
extern "C" unsigned long vtk_unicode_string_array_get_actual_memory_size(vtkUnicodeStringArray* sself);
extern "C" int vtk_unicode_string_array_is_numeric(vtkUnicodeStringArray* sself);
extern "C" void vtk_unicode_string_array_data_changed(vtkUnicodeStringArray* sself);
extern "C" void vtk_unicode_string_array_clear_lookup(vtkUnicodeStringArray* sself);
extern "C" void vtk_unicode_string_array_insert_next_utf_8_value(vtkUnicodeStringArray* sself, const char* p0);
extern "C" void vtk_unicode_string_array_set_utf_8_value(vtkUnicodeStringArray* sself, long long i, const char* p1);
extern "C" const char* vtk_unicode_string_array_get_utf_8_value(vtkUnicodeStringArray* sself, long long i);
extern "C" vtkUnsignedCharArray * vtkUnsignedCharArray_new () ;
extern "C" void vtkUnsignedCharArray_destructor (vtkUnsignedCharArray * sself) ;
extern "C" int vtk_unsigned_char_array_get_data_type(vtkUnsignedCharArray* sself);
extern "C" unsigned char vtk_unsigned_char_array_get_value(vtkUnsignedCharArray* sself, long long id);
extern "C" void vtk_unsigned_char_array_set_value(vtkUnsignedCharArray* sself, long long id, unsigned char value);
extern "C" bool vtk_unsigned_char_array_set_number_of_values(vtkUnsignedCharArray* sself, long long number);
extern "C" void vtk_unsigned_char_array_insert_value(vtkUnsignedCharArray* sself, long long id, unsigned char f);
extern "C" long long vtk_unsigned_char_array_insert_next_value(vtkUnsignedCharArray* sself, unsigned char f);
extern "C" unsigned char vtk_unsigned_char_array_get_data_type_value_min(vtkUnsignedCharArray* sself);
extern "C" unsigned char vtk_unsigned_char_array_get_data_type_value_max(vtkUnsignedCharArray* sself);
extern "C" vtkUnsignedIntArray * vtkUnsignedIntArray_new () ;
extern "C" void vtkUnsignedIntArray_destructor (vtkUnsignedIntArray * sself) ;
extern "C" int vtk_unsigned_int_array_get_data_type(vtkUnsignedIntArray* sself);
extern "C" unsigned int vtk_unsigned_int_array_get_value(vtkUnsignedIntArray* sself, long long id);
extern "C" void vtk_unsigned_int_array_set_value(vtkUnsignedIntArray* sself, long long id, unsigned int value);
extern "C" bool vtk_unsigned_int_array_set_number_of_values(vtkUnsignedIntArray* sself, long long number);
extern "C" void vtk_unsigned_int_array_insert_value(vtkUnsignedIntArray* sself, long long id, unsigned int f);
extern "C" long long vtk_unsigned_int_array_insert_next_value(vtkUnsignedIntArray* sself, unsigned int f);
extern "C" unsigned int vtk_unsigned_int_array_get_data_type_value_min(vtkUnsignedIntArray* sself);
extern "C" unsigned int vtk_unsigned_int_array_get_data_type_value_max(vtkUnsignedIntArray* sself);
extern "C" vtkUnsignedLongArray * vtkUnsignedLongArray_new () ;
extern "C" void vtkUnsignedLongArray_destructor (vtkUnsignedLongArray * sself) ;
extern "C" int vtk_unsigned_long_array_get_data_type(vtkUnsignedLongArray* sself);
extern "C" unsigned long vtk_unsigned_long_array_get_value(vtkUnsignedLongArray* sself, long long id);
extern "C" void vtk_unsigned_long_array_set_value(vtkUnsignedLongArray* sself, long long id, unsigned long value);
extern "C" bool vtk_unsigned_long_array_set_number_of_values(vtkUnsignedLongArray* sself, long long number);
extern "C" void vtk_unsigned_long_array_insert_value(vtkUnsignedLongArray* sself, long long id, unsigned long f);
extern "C" long long vtk_unsigned_long_array_insert_next_value(vtkUnsignedLongArray* sself, unsigned long f);
extern "C" unsigned long vtk_unsigned_long_array_get_data_type_value_min(vtkUnsignedLongArray* sself);
extern "C" unsigned long vtk_unsigned_long_array_get_data_type_value_max(vtkUnsignedLongArray* sself);
extern "C" vtkUnsignedLongLongArray * vtkUnsignedLongLongArray_new () ;
extern "C" void vtkUnsignedLongLongArray_destructor (vtkUnsignedLongLongArray * sself) ;
extern "C" int vtk_unsigned_long_long_array_get_data_type(vtkUnsignedLongLongArray* sself);
extern "C" unsigned long long vtk_unsigned_long_long_array_get_value(vtkUnsignedLongLongArray* sself, long long id);
extern "C" void vtk_unsigned_long_long_array_set_value(vtkUnsignedLongLongArray* sself, long long id, unsigned long long value);
extern "C" bool vtk_unsigned_long_long_array_set_number_of_values(vtkUnsignedLongLongArray* sself, long long number);
extern "C" void vtk_unsigned_long_long_array_insert_value(vtkUnsignedLongLongArray* sself, long long id, unsigned long long f);
extern "C" long long vtk_unsigned_long_long_array_insert_next_value(vtkUnsignedLongLongArray* sself, unsigned long long f);
extern "C" unsigned long long vtk_unsigned_long_long_array_get_data_type_value_min(vtkUnsignedLongLongArray* sself);
extern "C" unsigned long long vtk_unsigned_long_long_array_get_data_type_value_max(vtkUnsignedLongLongArray* sself);
extern "C" vtkUnsignedShortArray * vtkUnsignedShortArray_new () ;
extern "C" void vtkUnsignedShortArray_destructor (vtkUnsignedShortArray * sself) ;
extern "C" int vtk_unsigned_short_array_get_data_type(vtkUnsignedShortArray* sself);
extern "C" unsigned short vtk_unsigned_short_array_get_value(vtkUnsignedShortArray* sself, long long id);
extern "C" void vtk_unsigned_short_array_set_value(vtkUnsignedShortArray* sself, long long id, unsigned short value);
extern "C" bool vtk_unsigned_short_array_set_number_of_values(vtkUnsignedShortArray* sself, long long number);
extern "C" void vtk_unsigned_short_array_insert_value(vtkUnsignedShortArray* sself, long long id, unsigned short f);
extern "C" long long vtk_unsigned_short_array_insert_next_value(vtkUnsignedShortArray* sself, unsigned short f);
extern "C" unsigned short vtk_unsigned_short_array_get_data_type_value_min(vtkUnsignedShortArray* sself);
extern "C" unsigned short vtk_unsigned_short_array_get_data_type_value_max(vtkUnsignedShortArray* sself);
extern "C" vtkVariantArray * vtkVariantArray_new () ;
extern "C" void vtkVariantArray_destructor (vtkVariantArray * sself) ;
extern "C" int vtk_variant_array_allocate(vtkVariantArray* sself, long long sz, long long ext);
extern "C" void vtk_variant_array_initialize(vtkVariantArray* sself);
extern "C" int vtk_variant_array_get_data_type(vtkVariantArray* sself);
extern "C" int vtk_variant_array_get_data_type_size(vtkVariantArray* sself);
extern "C" int vtk_variant_array_get_element_component_size(vtkVariantArray* sself);
extern "C" void vtk_variant_array_set_number_of_tuples(vtkVariantArray* sself, long long number);
extern "C" void* vtk_variant_array_get_void_pointer(vtkVariantArray* sself, long long id);
extern "C" void vtk_variant_array_squeeze(vtkVariantArray* sself);
extern "C" int vtk_variant_array_resize(vtkVariantArray* sself, long long numTuples);
extern "C" void vtk_variant_array_set_void_array(vtkVariantArray* sself, void* arr, long long size, int save);
extern "C" unsigned long vtk_variant_array_get_actual_memory_size(vtkVariantArray* sself);
extern "C" int vtk_variant_array_is_numeric(vtkVariantArray* sself);
extern "C" long long vtk_variant_array_get_number_of_values(vtkVariantArray* sself);
extern "C" void vtk_variant_array_data_changed(vtkVariantArray* sself);
extern "C" void vtk_variant_array_data_element_changed(vtkVariantArray* sself, long long id);
extern "C" void vtk_variant_array_clear_lookup(vtkVariantArray* sself);
extern "C" vtkVersion * vtkVersion_new () ;
extern "C" void vtkVersion_destructor (vtkVersion * sself) ;
extern "C" const char* vtk_version_get_vtk_version(vtkVersion* sself);
extern "C" const char* vtk_version_get_vtk_version_full(vtkVersion* sself);
extern "C" int vtk_version_get_vtk_major_version(vtkVersion* sself);
extern "C" int vtk_version_get_vtk_minor_version(vtkVersion* sself);
extern "C" int vtk_version_get_vtk_build_version(vtkVersion* sself);
extern "C" const char* vtk_version_get_vtk_source_version(vtkVersion* sself);
extern "C" vtkVoidArray * vtkVoidArray_new () ;
extern "C" void vtkVoidArray_destructor (vtkVoidArray * sself) ;
extern "C" int vtk_void_array_allocate(vtkVoidArray* sself, long long sz, long long ext);
extern "C" void vtk_void_array_initialize(vtkVoidArray* sself);
extern "C" int vtk_void_array_get_data_type(vtkVoidArray* sself);
extern "C" int vtk_void_array_get_data_type_size(vtkVoidArray* sself);
extern "C" void vtk_void_array_set_number_of_pointers(vtkVoidArray* sself, long long number);
extern "C" long long vtk_void_array_get_number_of_pointers(vtkVoidArray* sself);
extern "C" void* vtk_void_array_get_void_pointer(vtkVoidArray* sself, long long id);
extern "C" void vtk_void_array_set_void_pointer(vtkVoidArray* sself, long long id, void* ptr);
extern "C" void vtk_void_array_insert_void_pointer(vtkVoidArray* sself, long long i, void* ptr);
extern "C" long long vtk_void_array_insert_next_void_pointer(vtkVoidArray* sself, void* tuple);
extern "C" void vtk_void_array_reset(vtkVoidArray* sself);
extern "C" void vtk_void_array_squeeze(vtkVoidArray* sself);
extern "C" vtkWeakReference * vtkWeakReference_new () ;
extern "C" void vtkWeakReference_destructor (vtkWeakReference * sself) ;
extern "C" vtkXMLFileOutputWindow * vtkXMLFileOutputWindow_new () ;
extern "C" void vtkXMLFileOutputWindow_destructor (vtkXMLFileOutputWindow * sself) ;
extern "C" void vtk_xml_file_output_window_display_text(vtkXMLFileOutputWindow* sself, const char* p0);
extern "C" void vtk_xml_file_output_window_display_tag(vtkXMLFileOutputWindow* sself, const char* p0);
