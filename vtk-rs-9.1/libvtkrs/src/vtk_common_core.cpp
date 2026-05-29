// Include header file
#include<vtk_common_core.h>

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

// Implement declared functions
extern "C" vtkAnimationCue * vtkAnimationCue_new () {return vtkAnimationCue :: New () ;}
extern "C" void vtkAnimationCue_destructor (vtkAnimationCue * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_animation_cue_set_time_mode(vtkAnimationCue* sself, int mode) { sself->SetTimeMode(mode); }
extern "C" int vtk_animation_cue_get_time_mode(vtkAnimationCue* sself) { return sself->GetTimeMode(); }
extern "C" void vtk_animation_cue_set_time_mode_to_relative(vtkAnimationCue* sself) { sself->SetTimeModeToRelative(); }
extern "C" void vtk_animation_cue_set_time_mode_to_normalized(vtkAnimationCue* sself) { sself->SetTimeModeToNormalized(); }
extern "C" void vtk_animation_cue_set_start_time(vtkAnimationCue* sself, double _arg) { sself->SetStartTime(_arg); }
extern "C" double vtk_animation_cue_get_start_time(vtkAnimationCue* sself) { return sself->GetStartTime(); }
extern "C" void vtk_animation_cue_set_end_time(vtkAnimationCue* sself, double _arg) { sself->SetEndTime(_arg); }
extern "C" double vtk_animation_cue_get_end_time(vtkAnimationCue* sself) { return sself->GetEndTime(); }
extern "C" void vtk_animation_cue_tick(vtkAnimationCue* sself, double currenttime, double deltatime, double clocktime) { sself->Tick(currenttime, deltatime, clocktime); }
extern "C" void vtk_animation_cue_initialize(vtkAnimationCue* sself) { sself->Initialize(); }
extern "C" void vtk_animation_cue_finalize(vtkAnimationCue* sself) { sself->Finalize(); }
extern "C" double vtk_animation_cue_get_animation_time(vtkAnimationCue* sself) { return sself->GetAnimationTime(); }
extern "C" double vtk_animation_cue_get_delta_time(vtkAnimationCue* sself) { return sself->GetDeltaTime(); }
extern "C" double vtk_animation_cue_get_clock_time(vtkAnimationCue* sself) { return sself->GetClockTime(); }
extern "C" vtkArchiver * vtkArchiver_new () {return vtkArchiver :: New () ;}
extern "C" void vtkArchiver_destructor (vtkArchiver * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_archiver_set_archive_name(vtkArchiver* sself, const char* _arg) { sself->SetArchiveName(_arg); }
extern "C" void vtk_archiver_open_archive(vtkArchiver* sself) { sself->OpenArchive(); }
extern "C" void vtk_archiver_close_archive(vtkArchiver* sself) { sself->CloseArchive(); }
extern "C" void vtk_archiver_insert_into_archive(vtkArchiver* sself, const char*& relativePath, const char* data, size_t size) { sself->InsertIntoArchive(relativePath, data, size); }
extern "C" bool vtk_archiver_contains(vtkArchiver* sself, const char*& relativePath) { return sself->Contains(relativePath); }
extern "C" vtkBitArray * vtkBitArray_new () {return vtkBitArray :: New () ;}
extern "C" void vtkBitArray_destructor (vtkBitArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_bit_array_allocate(vtkBitArray* sself, long long sz, long long ext) { return sself->Allocate(sz, ext); }
extern "C" void vtk_bit_array_initialize(vtkBitArray* sself) { sself->Initialize(); }
extern "C" int vtk_bit_array_get_data_type(vtkBitArray* sself) { return sself->GetDataType(); }
extern "C" int vtk_bit_array_get_data_type_size(vtkBitArray* sself) { return sself->GetDataTypeSize(); }
extern "C" void vtk_bit_array_set_number_of_tuples(vtkBitArray* sself, long long number) { sself->SetNumberOfTuples(number); }
extern "C" bool vtk_bit_array_set_number_of_values(vtkBitArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_bit_array_remove_tuple(vtkBitArray* sself, long long id) { sself->RemoveTuple(id); }
extern "C" void vtk_bit_array_set_component(vtkBitArray* sself, long long i, int j, double c) { sself->SetComponent(i, j, c); }
extern "C" void vtk_bit_array_squeeze(vtkBitArray* sself) { sself->Squeeze(); }
extern "C" int vtk_bit_array_resize(vtkBitArray* sself, long long numTuples) { return sself->Resize(numTuples); }
extern "C" int vtk_bit_array_get_value(vtkBitArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_bit_array_set_value(vtkBitArray* sself, long long id, int value) { sself->SetValue(id, value); }
extern "C" void vtk_bit_array_insert_value(vtkBitArray* sself, long long id, int i) { sself->InsertValue(id, i); }
extern "C" long long vtk_bit_array_insert_next_value(vtkBitArray* sself, int i) { return sself->InsertNextValue(i); }
extern "C" void vtk_bit_array_insert_component(vtkBitArray* sself, long long i, int j, double c) { sself->InsertComponent(i, j, c); }
extern "C" void* vtk_bit_array_write_void_pointer(vtkBitArray* sself, long long id, long long number) { return sself->WriteVoidPointer(id, number); }
extern "C" void* vtk_bit_array_get_void_pointer(vtkBitArray* sself, long long id) { return sself->GetVoidPointer(id); }
extern "C" void vtk_bit_array_set_void_array(vtkBitArray* sself, void* array, long long size, int save) { sself->SetVoidArray(array, size, save); }
extern "C" void vtk_bit_array_data_changed(vtkBitArray* sself) { sself->DataChanged(); }
extern "C" void vtk_bit_array_clear_lookup(vtkBitArray* sself) { sself->ClearLookup(); }
extern "C" vtkBitArrayIterator * vtkBitArrayIterator_new () {return vtkBitArrayIterator :: New () ;}
extern "C" void vtkBitArrayIterator_destructor (vtkBitArrayIterator * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_bit_array_iterator_get_value(vtkBitArrayIterator* sself, long long id) { return sself->GetValue(id); }
extern "C" long long vtk_bit_array_iterator_get_number_of_tuples(vtkBitArrayIterator* sself) { return sself->GetNumberOfTuples(); }
extern "C" long long vtk_bit_array_iterator_get_number_of_values(vtkBitArrayIterator* sself) { return sself->GetNumberOfValues(); }
extern "C" int vtk_bit_array_iterator_get_number_of_components(vtkBitArrayIterator* sself) { return sself->GetNumberOfComponents(); }
extern "C" int vtk_bit_array_iterator_get_data_type(vtkBitArrayIterator* sself) { return sself->GetDataType(); }
extern "C" int vtk_bit_array_iterator_get_data_type_size(vtkBitArrayIterator* sself) { return sself->GetDataTypeSize(); }
extern "C" void vtk_bit_array_iterator_set_value(vtkBitArrayIterator* sself, long long id, int value) { sself->SetValue(id, value); }
extern "C" vtkBoxMuellerRandomSequence * vtkBoxMuellerRandomSequence_new () {return vtkBoxMuellerRandomSequence :: New () ;}
extern "C" void vtkBoxMuellerRandomSequence_destructor (vtkBoxMuellerRandomSequence * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_box_mueller_random_sequence_initialize(vtkBoxMuellerRandomSequence* sself, unsigned int seed) { sself->Initialize(seed); }
extern "C" double vtk_box_mueller_random_sequence_get_value(vtkBoxMuellerRandomSequence* sself) { return sself->GetValue(); }
extern "C" void vtk_box_mueller_random_sequence_next(vtkBoxMuellerRandomSequence* sself) { sself->Next(); }
extern "C" vtkByteSwap * vtkByteSwap_new () {return vtkByteSwap :: New () ;}
extern "C" void vtkByteSwap_destructor (vtkByteSwap * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_byte_swap_swap_2_le(vtkByteSwap* sself, void* p) { sself->Swap2LE(p); }
extern "C" void vtk_byte_swap_swap_4_le(vtkByteSwap* sself, void* p) { sself->Swap4LE(p); }
extern "C" void vtk_byte_swap_swap_8_le(vtkByteSwap* sself, void* p) { sself->Swap8LE(p); }
extern "C" void vtk_byte_swap_swap_2_le_range(vtkByteSwap* sself, void* p, size_t num) { sself->Swap2LERange(p, num); }
extern "C" void vtk_byte_swap_swap_4_le_range(vtkByteSwap* sself, void* p, size_t num) { sself->Swap4LERange(p, num); }
extern "C" void vtk_byte_swap_swap_8_le_range(vtkByteSwap* sself, void* p, size_t num) { sself->Swap8LERange(p, num); }
extern "C" void vtk_byte_swap_swap_2_be(vtkByteSwap* sself, void* p) { sself->Swap2BE(p); }
extern "C" void vtk_byte_swap_swap_4_be(vtkByteSwap* sself, void* p) { sself->Swap4BE(p); }
extern "C" void vtk_byte_swap_swap_8_be(vtkByteSwap* sself, void* p) { sself->Swap8BE(p); }
extern "C" void vtk_byte_swap_swap_2_be_range(vtkByteSwap* sself, void* p, size_t num) { sself->Swap2BERange(p, num); }
extern "C" void vtk_byte_swap_swap_4_be_range(vtkByteSwap* sself, void* p, size_t num) { sself->Swap4BERange(p, num); }
extern "C" void vtk_byte_swap_swap_8_be_range(vtkByteSwap* sself, void* p, size_t num) { sself->Swap8BERange(p, num); }
extern "C" void vtk_byte_swap_swap_void_range(vtkByteSwap* sself, void* buffer, size_t numWords, size_t wordSize) { sself->SwapVoidRange(buffer, numWords, wordSize); }
extern "C" vtkCallbackCommand * vtkCallbackCommand_new () {return vtkCallbackCommand :: New () ;}
extern "C" void vtkCallbackCommand_destructor (vtkCallbackCommand * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_callback_command_set_client_data(vtkCallbackCommand* sself, void* cd) { sself->SetClientData(cd); }
extern "C" void* vtk_callback_command_get_client_data(vtkCallbackCommand* sself) { return sself->GetClientData(); }
extern "C" void vtk_callback_command_set_abort_flag_on_execute(vtkCallbackCommand* sself, int f) { sself->SetAbortFlagOnExecute(f); }
extern "C" int vtk_callback_command_get_abort_flag_on_execute(vtkCallbackCommand* sself) { return sself->GetAbortFlagOnExecute(); }
extern "C" void vtk_callback_command_abort_flag_on_execute_on(vtkCallbackCommand* sself) { sself->AbortFlagOnExecuteOn(); }
extern "C" void vtk_callback_command_abort_flag_on_execute_off(vtkCallbackCommand* sself) { sself->AbortFlagOnExecuteOff(); }
extern "C" vtkCharArray * vtkCharArray_new () {return vtkCharArray :: New () ;}
extern "C" void vtkCharArray_destructor (vtkCharArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_char_array_get_data_type(vtkCharArray* sself) { return sself->GetDataType(); }
extern "C" void vtk_char_array_set_typed_tuple(vtkCharArray* sself, long long i, const char* tuple) { sself->SetTypedTuple(i, tuple); }
extern "C" void vtk_char_array_insert_typed_tuple(vtkCharArray* sself, long long i, const char* tuple) { sself->InsertTypedTuple(i, tuple); }
extern "C" long long vtk_char_array_insert_next_typed_tuple(vtkCharArray* sself, const char* tuple) { return sself->InsertNextTypedTuple(tuple); }
extern "C" char vtk_char_array_get_value(vtkCharArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_char_array_set_value(vtkCharArray* sself, long long id, char value) { sself->SetValue(id, value); }
extern "C" bool vtk_char_array_set_number_of_values(vtkCharArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_char_array_insert_value(vtkCharArray* sself, long long id, char f) { sself->InsertValue(id, f); }
extern "C" long long vtk_char_array_insert_next_value(vtkCharArray* sself, char f) { return sself->InsertNextValue(f); }
extern "C" char vtk_char_array_get_data_type_value_min(vtkCharArray* sself) { return sself->GetDataTypeValueMin(); }
extern "C" char vtk_char_array_get_data_type_value_max(vtkCharArray* sself) { return sself->GetDataTypeValueMax(); }
extern "C" vtkCollection * vtkCollection_new () {return vtkCollection :: New () ;}
extern "C" void vtkCollection_destructor (vtkCollection * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_collection_remove_item(vtkCollection* sself, int i) { sself->RemoveItem(i); }
extern "C" void vtk_collection_remove_all_items(vtkCollection* sself) { sself->RemoveAllItems(); }
extern "C" int vtk_collection_get_number_of_items(vtkCollection* sself) { return sself->GetNumberOfItems(); }
extern "C" void vtk_collection_init_traversal(vtkCollection* sself) { sself->InitTraversal(); }
extern "C" vtkCollectionIterator * vtkCollectionIterator_new () {return vtkCollectionIterator :: New () ;}
extern "C" void vtkCollectionIterator_destructor (vtkCollectionIterator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_collection_iterator_init_traversal(vtkCollectionIterator* sself) { sself->InitTraversal(); }
extern "C" void vtk_collection_iterator_go_to_first_item(vtkCollectionIterator* sself) { sself->GoToFirstItem(); }
extern "C" void vtk_collection_iterator_go_to_next_item(vtkCollectionIterator* sself) { sself->GoToNextItem(); }
extern "C" int vtk_collection_iterator_is_done_with_traversal(vtkCollectionIterator* sself) { return sself->IsDoneWithTraversal(); }
extern "C" vtkCriticalSection * vtkCriticalSection_new () {return vtkCriticalSection :: New () ;}
extern "C" void vtkCriticalSection_destructor (vtkCriticalSection * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_critical_section_lock(vtkCriticalSection* sself) { sself->Lock(); }
extern "C" void vtk_critical_section_unlock(vtkCriticalSection* sself) { sself->Unlock(); }
extern "C" vtkDataArrayCollection * vtkDataArrayCollection_new () {return vtkDataArrayCollection :: New () ;}
extern "C" void vtkDataArrayCollection_destructor (vtkDataArrayCollection * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_data_array_collection_get_number_of_items(vtkDataArrayCollection* sself) { return sself->GetNumberOfItems(); }
extern "C" vtkDataArrayCollectionIterator * vtkDataArrayCollectionIterator_new () {return vtkDataArrayCollectionIterator :: New () ;}
extern "C" void vtkDataArrayCollectionIterator_destructor (vtkDataArrayCollectionIterator * sself) {sself -> Delete () ; return ;}
extern "C" vtkDataArraySelection * vtkDataArraySelection_new () {return vtkDataArraySelection :: New () ;}
extern "C" void vtkDataArraySelection_destructor (vtkDataArraySelection * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_data_array_selection_enable_array(vtkDataArraySelection* sself, const char* name) { sself->EnableArray(name); }
extern "C" void vtk_data_array_selection_disable_array(vtkDataArraySelection* sself, const char* name) { sself->DisableArray(name); }
extern "C" int vtk_data_array_selection_array_is_enabled(vtkDataArraySelection* sself, const char* name) { return sself->ArrayIsEnabled(name); }
extern "C" int vtk_data_array_selection_array_exists(vtkDataArraySelection* sself, const char* name) { return sself->ArrayExists(name); }
extern "C" void vtk_data_array_selection_enable_all_arrays(vtkDataArraySelection* sself) { sself->EnableAllArrays(); }
extern "C" void vtk_data_array_selection_disable_all_arrays(vtkDataArraySelection* sself) { sself->DisableAllArrays(); }
extern "C" int vtk_data_array_selection_get_number_of_arrays(vtkDataArraySelection* sself) { return sself->GetNumberOfArrays(); }
extern "C" int vtk_data_array_selection_get_number_of_arrays_enabled(vtkDataArraySelection* sself) { return sself->GetNumberOfArraysEnabled(); }
extern "C" const char* vtk_data_array_selection_get_array_name(vtkDataArraySelection* sself, int index) { return sself->GetArrayName(index); }
extern "C" int vtk_data_array_selection_get_array_index(vtkDataArraySelection* sself, const char* name) { return sself->GetArrayIndex(name); }
extern "C" int vtk_data_array_selection_get_enabled_array_index(vtkDataArraySelection* sself, const char* name) { return sself->GetEnabledArrayIndex(name); }
extern "C" int vtk_data_array_selection_get_array_setting(vtkDataArraySelection* sself, int index) { return sself->GetArraySetting(index); }
extern "C" void vtk_data_array_selection_set_array_setting(vtkDataArraySelection* sself, const char* name, int setting) { sself->SetArraySetting(name, setting); }
extern "C" void vtk_data_array_selection_remove_all_arrays(vtkDataArraySelection* sself) { sself->RemoveAllArrays(); }
extern "C" int vtk_data_array_selection_add_array(vtkDataArraySelection* sself, const char* name, bool state) { return sself->AddArray(name, state); }
extern "C" void vtk_data_array_selection_remove_array_by_index(vtkDataArraySelection* sself, int index) { sself->RemoveArrayByIndex(index); }
extern "C" void vtk_data_array_selection_remove_array_by_name(vtkDataArraySelection* sself, const char* name) { sself->RemoveArrayByName(name); }
extern "C" void vtk_data_array_selection_set_unknown_array_setting(vtkDataArraySelection* sself, int _arg) { sself->SetUnknownArraySetting(_arg); }
extern "C" int vtk_data_array_selection_get_unknown_array_setting(vtkDataArraySelection* sself) { return sself->GetUnknownArraySetting(); }
extern "C" vtkDebugLeaks * vtkDebugLeaks_new () {return vtkDebugLeaks :: New () ;}
extern "C" void vtkDebugLeaks_destructor (vtkDebugLeaks * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_debug_leaks_print_current_leaks(vtkDebugLeaks* sself) { return sself->PrintCurrentLeaks(); }
extern "C" int vtk_debug_leaks_get_exit_error(vtkDebugLeaks* sself) { return sself->GetExitError(); }
extern "C" void vtk_debug_leaks_set_exit_error(vtkDebugLeaks* sself, int p0) { sself->SetExitError(p0); }
extern "C" vtkDoubleArray * vtkDoubleArray_new () {return vtkDoubleArray :: New () ;}
extern "C" void vtkDoubleArray_destructor (vtkDoubleArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_double_array_get_data_type(vtkDoubleArray* sself) { return sself->GetDataType(); }
extern "C" double vtk_double_array_get_value(vtkDoubleArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_double_array_set_value(vtkDoubleArray* sself, long long id, double value) { sself->SetValue(id, value); }
extern "C" bool vtk_double_array_set_number_of_values(vtkDoubleArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_double_array_insert_value(vtkDoubleArray* sself, long long id, double f) { sself->InsertValue(id, f); }
extern "C" long long vtk_double_array_insert_next_value(vtkDoubleArray* sself, double f) { return sself->InsertNextValue(f); }
extern "C" double vtk_double_array_get_data_type_value_min(vtkDoubleArray* sself) { return sself->GetDataTypeValueMin(); }
extern "C" double vtk_double_array_get_data_type_value_max(vtkDoubleArray* sself) { return sself->GetDataTypeValueMax(); }
extern "C" vtkDynamicLoader * vtkDynamicLoader_new () {return vtkDynamicLoader :: New () ;}
extern "C" void vtkDynamicLoader_destructor (vtkDynamicLoader * sself) {sself -> Delete () ; return ;}
extern "C" const char* vtk_dynamic_loader_lib_prefix(vtkDynamicLoader* sself) { return sself->LibPrefix(); }
extern "C" const char* vtk_dynamic_loader_lib_extension(vtkDynamicLoader* sself) { return sself->LibExtension(); }
extern "C" const char* vtk_dynamic_loader_last_error(vtkDynamicLoader* sself) { return sself->LastError(); }
extern "C" vtkEventDataDevice3D * vtkEventDataDevice3D_new () {return vtkEventDataDevice3D :: New () ;}
extern "C" void vtkEventDataDevice3D_destructor (vtkEventDataDevice3D * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_event_data_device_3_d_set_track_pad_position(vtkEventDataDevice3D* sself, double x, double y) { sself->SetTrackPadPosition(x, y); }
extern "C" vtkEventDataForDevice * vtkEventDataForDevice_new () {return vtkEventDataForDevice :: New () ;}
extern "C" void vtkEventDataForDevice_destructor (vtkEventDataForDevice * sself) {sself -> Delete () ; return ;}
extern "C" vtkEventForwarderCommand * vtkEventForwarderCommand_new () {return vtkEventForwarderCommand :: New () ;}
extern "C" void vtkEventForwarderCommand_destructor (vtkEventForwarderCommand * sself) {sself -> Delete () ; return ;}
extern "C" void* vtk_event_forwarder_command_get_target(vtkEventForwarderCommand* sself) { return sself->GetTarget(); }
extern "C" vtkFileOutputWindow * vtkFileOutputWindow_new () {return vtkFileOutputWindow :: New () ;}
extern "C" void vtkFileOutputWindow_destructor (vtkFileOutputWindow * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_file_output_window_display_text(vtkFileOutputWindow* sself, const char* p0) { sself->DisplayText(p0); }
extern "C" void vtk_file_output_window_set_file_name(vtkFileOutputWindow* sself, const char* _arg) { sself->SetFileName(_arg); }
extern "C" void vtk_file_output_window_set_flush(vtkFileOutputWindow* sself, int _arg) { sself->SetFlush(_arg); }
extern "C" int vtk_file_output_window_get_flush(vtkFileOutputWindow* sself) { return sself->GetFlush(); }
extern "C" void vtk_file_output_window_flush_on(vtkFileOutputWindow* sself) { sself->FlushOn(); }
extern "C" void vtk_file_output_window_flush_off(vtkFileOutputWindow* sself) { sself->FlushOff(); }
extern "C" void vtk_file_output_window_set_append(vtkFileOutputWindow* sself, int _arg) { sself->SetAppend(_arg); }
extern "C" int vtk_file_output_window_get_append(vtkFileOutputWindow* sself) { return sself->GetAppend(); }
extern "C" void vtk_file_output_window_append_on(vtkFileOutputWindow* sself) { sself->AppendOn(); }
extern "C" void vtk_file_output_window_append_off(vtkFileOutputWindow* sself) { sself->AppendOff(); }
extern "C" vtkFloatArray * vtkFloatArray_new () {return vtkFloatArray :: New () ;}
extern "C" void vtkFloatArray_destructor (vtkFloatArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_float_array_get_data_type(vtkFloatArray* sself) { return sself->GetDataType(); }
extern "C" float vtk_float_array_get_value(vtkFloatArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_float_array_set_value(vtkFloatArray* sself, long long id, float value) { sself->SetValue(id, value); }
extern "C" bool vtk_float_array_set_number_of_values(vtkFloatArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_float_array_insert_value(vtkFloatArray* sself, long long id, float f) { sself->InsertValue(id, f); }
extern "C" long long vtk_float_array_insert_next_value(vtkFloatArray* sself, float f) { return sself->InsertNextValue(f); }
extern "C" float vtk_float_array_get_data_type_value_min(vtkFloatArray* sself) { return sself->GetDataTypeValueMin(); }
extern "C" float vtk_float_array_get_data_type_value_max(vtkFloatArray* sself) { return sself->GetDataTypeValueMax(); }
extern "C" vtkGarbageCollector * vtkGarbageCollector_new () {return vtkGarbageCollector :: New () ;}
extern "C" void vtkGarbageCollector_destructor (vtkGarbageCollector * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_garbage_collector_collect(vtkGarbageCollector* sself) { sself->Collect(); }
extern "C" void vtk_garbage_collector_deferred_collection_push(vtkGarbageCollector* sself) { sself->DeferredCollectionPush(); }
extern "C" void vtk_garbage_collector_deferred_collection_pop(vtkGarbageCollector* sself) { sself->DeferredCollectionPop(); }
extern "C" void vtk_garbage_collector_set_global_debug_flag(vtkGarbageCollector* sself, bool flag) { sself->SetGlobalDebugFlag(flag); }
extern "C" bool vtk_garbage_collector_get_global_debug_flag(vtkGarbageCollector* sself) { return sself->GetGlobalDebugFlag(); }
extern "C" vtkIdList * vtkIdList_new () {return vtkIdList :: New () ;}
extern "C" void vtkIdList_destructor (vtkIdList * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_id_list_initialize(vtkIdList* sself) { sself->Initialize(); }
extern "C" int vtk_id_list_allocate(vtkIdList* sself, const long long sz, const int strategy) { return sself->Allocate(sz, strategy); }
extern "C" long long vtk_id_list_get_number_of_ids(vtkIdList* sself) { return sself->GetNumberOfIds(); }
extern "C" long long vtk_id_list_get_id(vtkIdList* sself, const long long i) { return sself->GetId(i); }
extern "C" long long vtk_id_list_find_id_location(vtkIdList* sself, const long long id) { return sself->FindIdLocation(id); }
extern "C" void vtk_id_list_set_number_of_ids(vtkIdList* sself, const long long number) { sself->SetNumberOfIds(number); }
extern "C" void vtk_id_list_set_id(vtkIdList* sself, const long long i, const long long vtkid) { sself->SetId(i, vtkid); }
extern "C" void vtk_id_list_insert_id(vtkIdList* sself, const long long i, const long long vtkid) { sself->InsertId(i, vtkid); }
extern "C" long long vtk_id_list_insert_next_id(vtkIdList* sself, const long long vtkid) { return sself->InsertNextId(vtkid); }
extern "C" long long vtk_id_list_insert_unique_id(vtkIdList* sself, const long long vtkid) { return sself->InsertUniqueId(vtkid); }
extern "C" void vtk_id_list_sort(vtkIdList* sself) { sself->Sort(); }
extern "C" void vtk_id_list_fill(vtkIdList* sself, long long value) { sself->Fill(value); }
extern "C" void vtk_id_list_reset(vtkIdList* sself) { sself->Reset(); }
extern "C" void vtk_id_list_squeeze(vtkIdList* sself) { sself->Squeeze(); }
extern "C" void vtk_id_list_delete_id(vtkIdList* sself, long long vtkid) { sself->DeleteId(vtkid); }
extern "C" long long vtk_id_list_is_id(vtkIdList* sself, long long vtkid) { return sself->IsId(vtkid); }
extern "C" vtkIdListCollection * vtkIdListCollection_new () {return vtkIdListCollection :: New () ;}
extern "C" void vtkIdListCollection_destructor (vtkIdListCollection * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_id_list_collection_get_number_of_items(vtkIdListCollection* sself) { return sself->GetNumberOfItems(); }
extern "C" vtkIdTypeArray * vtkIdTypeArray_new () {return vtkIdTypeArray :: New () ;}
extern "C" void vtkIdTypeArray_destructor (vtkIdTypeArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_id_type_array_get_data_type(vtkIdTypeArray* sself) { return sself->GetDataType(); }
extern "C" long long vtk_id_type_array_get_value(vtkIdTypeArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_id_type_array_set_value(vtkIdTypeArray* sself, long long id, long long value) { sself->SetValue(id, value); }
extern "C" bool vtk_id_type_array_set_number_of_values(vtkIdTypeArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_id_type_array_insert_value(vtkIdTypeArray* sself, long long id, long long f) { sself->InsertValue(id, f); }
extern "C" long long vtk_id_type_array_insert_next_value(vtkIdTypeArray* sself, long long f) { return sself->InsertNextValue(f); }
extern "C" long long vtk_id_type_array_get_data_type_value_min(vtkIdTypeArray* sself) { return sself->GetDataTypeValueMin(); }
extern "C" long long vtk_id_type_array_get_data_type_value_max(vtkIdTypeArray* sself) { return sself->GetDataTypeValueMax(); }
extern "C" vtkInformation * vtkInformation_new () {return vtkInformation :: New () ;}
extern "C" void vtkInformation_destructor (vtkInformation * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_information_modified(vtkInformation* sself) { sself->Modified(); }
extern "C" void vtk_information_clear(vtkInformation* sself) { sself->Clear(); }
extern "C" int vtk_information_get_number_of_keys(vtkInformation* sself) { return sself->GetNumberOfKeys(); }
extern "C" vtkInformationIterator * vtkInformationIterator_new () {return vtkInformationIterator :: New () ;}
extern "C" void vtkInformationIterator_destructor (vtkInformationIterator * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_information_iterator_init_traversal(vtkInformationIterator* sself) { sself->InitTraversal(); }
extern "C" void vtk_information_iterator_go_to_first_item(vtkInformationIterator* sself) { sself->GoToFirstItem(); }
extern "C" void vtk_information_iterator_go_to_next_item(vtkInformationIterator* sself) { sself->GoToNextItem(); }
extern "C" int vtk_information_iterator_is_done_with_traversal(vtkInformationIterator* sself) { return sself->IsDoneWithTraversal(); }
extern "C" vtkInformationKeyLookup * vtkInformationKeyLookup_new () {return vtkInformationKeyLookup :: New () ;}
extern "C" void vtkInformationKeyLookup_destructor (vtkInformationKeyLookup * sself) {sself -> Delete () ; return ;}
extern "C" vtkInformationVector * vtkInformationVector_new () {return vtkInformationVector :: New () ;}
extern "C" void vtkInformationVector_destructor (vtkInformationVector * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_information_vector_get_number_of_information_objects(vtkInformationVector* sself) { return sself->GetNumberOfInformationObjects(); }
extern "C" void vtk_information_vector_set_number_of_information_objects(vtkInformationVector* sself, int n) { sself->SetNumberOfInformationObjects(n); }
extern "C" vtkIntArray * vtkIntArray_new () {return vtkIntArray :: New () ;}
extern "C" void vtkIntArray_destructor (vtkIntArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_int_array_get_data_type(vtkIntArray* sself) { return sself->GetDataType(); }
extern "C" int vtk_int_array_get_value(vtkIntArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_int_array_set_value(vtkIntArray* sself, long long id, int value) { sself->SetValue(id, value); }
extern "C" bool vtk_int_array_set_number_of_values(vtkIntArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_int_array_insert_value(vtkIntArray* sself, long long id, int f) { sself->InsertValue(id, f); }
extern "C" long long vtk_int_array_insert_next_value(vtkIntArray* sself, int f) { return sself->InsertNextValue(f); }
extern "C" int vtk_int_array_get_data_type_value_min(vtkIntArray* sself) { return sself->GetDataTypeValueMin(); }
extern "C" int vtk_int_array_get_data_type_value_max(vtkIntArray* sself) { return sself->GetDataTypeValueMax(); }
extern "C" vtkLongArray * vtkLongArray_new () {return vtkLongArray :: New () ;}
extern "C" void vtkLongArray_destructor (vtkLongArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_long_array_get_data_type(vtkLongArray* sself) { return sself->GetDataType(); }
extern "C" long vtk_long_array_get_value(vtkLongArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_long_array_set_value(vtkLongArray* sself, long long id, long value) { sself->SetValue(id, value); }
extern "C" bool vtk_long_array_set_number_of_values(vtkLongArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_long_array_insert_value(vtkLongArray* sself, long long id, long f) { sself->InsertValue(id, f); }
extern "C" long long vtk_long_array_insert_next_value(vtkLongArray* sself, long f) { return sself->InsertNextValue(f); }
extern "C" long vtk_long_array_get_data_type_value_min(vtkLongArray* sself) { return sself->GetDataTypeValueMin(); }
extern "C" long vtk_long_array_get_data_type_value_max(vtkLongArray* sself) { return sself->GetDataTypeValueMax(); }
extern "C" vtkLongLongArray * vtkLongLongArray_new () {return vtkLongLongArray :: New () ;}
extern "C" void vtkLongLongArray_destructor (vtkLongLongArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_long_long_array_get_data_type(vtkLongLongArray* sself) { return sself->GetDataType(); }
extern "C" long long vtk_long_long_array_get_value(vtkLongLongArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_long_long_array_set_value(vtkLongLongArray* sself, long long id, long long value) { sself->SetValue(id, value); }
extern "C" bool vtk_long_long_array_set_number_of_values(vtkLongLongArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_long_long_array_insert_value(vtkLongLongArray* sself, long long id, long long f) { sself->InsertValue(id, f); }
extern "C" long long vtk_long_long_array_insert_next_value(vtkLongLongArray* sself, long long f) { return sself->InsertNextValue(f); }
extern "C" long long vtk_long_long_array_get_data_type_value_min(vtkLongLongArray* sself) { return sself->GetDataTypeValueMin(); }
extern "C" long long vtk_long_long_array_get_data_type_value_max(vtkLongLongArray* sself) { return sself->GetDataTypeValueMax(); }
extern "C" vtkLookupTable * vtkLookupTable_new () {return vtkLookupTable :: New () ;}
extern "C" void vtkLookupTable_destructor (vtkLookupTable * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_lookup_table_is_opaque(vtkLookupTable* sself) { return sself->IsOpaque(); }
extern "C" int vtk_lookup_table_allocate(vtkLookupTable* sself, int sz, int ext) { return sself->Allocate(sz, ext); }
extern "C" void vtk_lookup_table_build(vtkLookupTable* sself) { sself->Build(); }
extern "C" void vtk_lookup_table_force_build(vtkLookupTable* sself) { sself->ForceBuild(); }
extern "C" void vtk_lookup_table_build_special_colors(vtkLookupTable* sself) { sself->BuildSpecialColors(); }
extern "C" void vtk_lookup_table_set_ramp(vtkLookupTable* sself, int _arg) { sself->SetRamp(_arg); }
extern "C" void vtk_lookup_table_set_ramp_to_linear(vtkLookupTable* sself) { sself->SetRampToLinear(); }
extern "C" void vtk_lookup_table_set_ramp_to_s_curve(vtkLookupTable* sself) { sself->SetRampToSCurve(); }
extern "C" void vtk_lookup_table_set_ramp_to_sqrt(vtkLookupTable* sself) { sself->SetRampToSQRT(); }
extern "C" int vtk_lookup_table_get_ramp(vtkLookupTable* sself) { return sself->GetRamp(); }
extern "C" void vtk_lookup_table_set_scale(vtkLookupTable* sself, int scale) { sself->SetScale(scale); }
extern "C" void vtk_lookup_table_set_scale_to_linear(vtkLookupTable* sself) { sself->SetScaleToLinear(); }
extern "C" void vtk_lookup_table_set_scale_to_log_10(vtkLookupTable* sself) { sself->SetScaleToLog10(); }
extern "C" int vtk_lookup_table_get_scale(vtkLookupTable* sself) { return sself->GetScale(); }
extern "C" void vtk_lookup_table_set_table_range(vtkLookupTable* sself, double min, double max) { sself->SetTableRange(min, max); }
extern "C" void vtk_lookup_table_set_hue_range(vtkLookupTable* sself, double _arg1, double _arg2) { sself->SetHueRange(_arg1, _arg2); }
extern "C" void vtk_lookup_table_set_saturation_range(vtkLookupTable* sself, double _arg1, double _arg2) { sself->SetSaturationRange(_arg1, _arg2); }
extern "C" void vtk_lookup_table_set_value_range(vtkLookupTable* sself, double _arg1, double _arg2) { sself->SetValueRange(_arg1, _arg2); }
extern "C" void vtk_lookup_table_set_alpha_range(vtkLookupTable* sself, double _arg1, double _arg2) { sself->SetAlphaRange(_arg1, _arg2); }
extern "C" void vtk_lookup_table_set_nan_color(vtkLookupTable* sself, double _arg1, double _arg2, double _arg3, double _arg4) { sself->SetNanColor(_arg1, _arg2, _arg3, _arg4); }
extern "C" void vtk_lookup_table_set_below_range_color(vtkLookupTable* sself, double _arg1, double _arg2, double _arg3, double _arg4) { sself->SetBelowRangeColor(_arg1, _arg2, _arg3, _arg4); }
extern "C" void vtk_lookup_table_set_use_below_range_color(vtkLookupTable* sself, int _arg) { sself->SetUseBelowRangeColor(_arg); }
extern "C" int vtk_lookup_table_get_use_below_range_color(vtkLookupTable* sself) { return sself->GetUseBelowRangeColor(); }
extern "C" void vtk_lookup_table_use_below_range_color_on(vtkLookupTable* sself) { sself->UseBelowRangeColorOn(); }
extern "C" void vtk_lookup_table_use_below_range_color_off(vtkLookupTable* sself) { sself->UseBelowRangeColorOff(); }
extern "C" void vtk_lookup_table_set_above_range_color(vtkLookupTable* sself, double _arg1, double _arg2, double _arg3, double _arg4) { sself->SetAboveRangeColor(_arg1, _arg2, _arg3, _arg4); }
extern "C" void vtk_lookup_table_set_use_above_range_color(vtkLookupTable* sself, int _arg) { sself->SetUseAboveRangeColor(_arg); }
extern "C" int vtk_lookup_table_get_use_above_range_color(vtkLookupTable* sself) { return sself->GetUseAboveRangeColor(); }
extern "C" void vtk_lookup_table_use_above_range_color_on(vtkLookupTable* sself) { sself->UseAboveRangeColorOn(); }
extern "C" void vtk_lookup_table_use_above_range_color_off(vtkLookupTable* sself) { sself->UseAboveRangeColorOff(); }
extern "C" double vtk_lookup_table_get_opacity(vtkLookupTable* sself, double v) { return sself->GetOpacity(v); }
extern "C" long long vtk_lookup_table_get_index(vtkLookupTable* sself, double v) { return sself->GetIndex(v); }
extern "C" void vtk_lookup_table_set_number_of_table_values(vtkLookupTable* sself, long long number) { sself->SetNumberOfTableValues(number); }
extern "C" long long vtk_lookup_table_get_number_of_table_values(vtkLookupTable* sself) { return sself->GetNumberOfTableValues(); }
extern "C" void vtk_lookup_table_set_table_value(vtkLookupTable* sself, long long indx, double r, double g, double b, double a) { sself->SetTableValue(indx, r, g, b, a); }
extern "C" void vtk_lookup_table_set_number_of_colors(vtkLookupTable* sself, long long _arg) { sself->SetNumberOfColors(_arg); }
extern "C" long long vtk_lookup_table_get_number_of_colors_min_value(vtkLookupTable* sself) { return sself->GetNumberOfColorsMinValue(); }
extern "C" long long vtk_lookup_table_get_number_of_colors_max_value(vtkLookupTable* sself) { return sself->GetNumberOfColorsMaxValue(); }
extern "C" long long vtk_lookup_table_get_number_of_colors(vtkLookupTable* sself) { return sself->GetNumberOfColors(); }
extern "C" int vtk_lookup_table_using_log_scale(vtkLookupTable* sself) { return sself->UsingLogScale(); }
extern "C" vtkMath * vtkMath_new () {return vtkMath :: New () ;}
extern "C" void vtkMath_destructor (vtkMath * sself) {sself -> Delete () ; return ;}
extern "C" double vtk_math_pi(vtkMath* sself) { return sself->Pi(); }
extern "C" float vtk_math_radians_from_degrees(vtkMath* sself, float degrees) { return sself->RadiansFromDegrees(degrees); }
extern "C" float vtk_math_degrees_from_radians(vtkMath* sself, float radians) { return sself->DegreesFromRadians(radians); }
extern "C" int vtk_math_round(vtkMath* sself, float f) { return sself->Round(f); }
extern "C" int vtk_math_floor(vtkMath* sself, double x) { return sself->Floor(x); }
extern "C" int vtk_math_ceil(vtkMath* sself, double x) { return sself->Ceil(x); }
extern "C" int vtk_math_ceil_log_2(vtkMath* sself, unsigned long long x) { return sself->CeilLog2(x); }
extern "C" bool vtk_math_is_power_of_two(vtkMath* sself, unsigned long long x) { return sself->IsPowerOfTwo(x); }
extern "C" int vtk_math_nearest_power_of_two(vtkMath* sself, int x) { return sself->NearestPowerOfTwo(x); }
extern "C" long long vtk_math_factorial(vtkMath* sself, int N) { return sself->Factorial(N); }
extern "C" long long vtk_math_binomial(vtkMath* sself, int m, int n) { return sself->Binomial(m, n); }
extern "C" void vtk_math_random_seed(vtkMath* sself, int s) { sself->RandomSeed(s); }
extern "C" int vtk_math_get_seed(vtkMath* sself) { return sself->GetSeed(); }
extern "C" double vtk_math_random(vtkMath* sself) { return sself->Random(); }
extern "C" double vtk_math_gaussian(vtkMath* sself) { return sself->Gaussian(); }
extern "C" double vtk_math_gaussian_amplitude(vtkMath* sself, const double variance, const double distanceFromMean) { return sself->GaussianAmplitude(variance, distanceFromMean); }
extern "C" double vtk_math_gaussian_weight(vtkMath* sself, const double variance, const double distanceFromMean) { return sself->GaussianWeight(variance, distanceFromMean); }
extern "C" double vtk_math_determinant_2_x_2(vtkMath* sself, double a, double b, double c, double d) { return sself->Determinant2x2(a, b, c, d); }
extern "C" double vtk_math_determinant_3_x_3(vtkMath* sself, double a1, double a2, double a3, double b1, double b2, double b3, double c1, double c2, double c3) { return sself->Determinant3x3(a1, a2, a3, b1, b2, b3, c1, c2, c3); }
extern "C" int vtk_math_solve_linear_system_gepp_2_x_2(vtkMath* sself, double a00, double a01, double a10, double a11, double b0, double b1, double& x0, double& x1) { return sself->SolveLinearSystemGEPP2x2(a00, a01, a10, a11, b0, b1, x0, x1); }
extern "C" int vtk_math_get_scalar_type_fitting_range(vtkMath* sself, double range_min, double range_max, double scale, double shift) { return sself->GetScalarTypeFittingRange(range_min, range_max, scale, shift); }
extern "C" double vtk_math_inf(vtkMath* sself) { return sself->Inf(); }
extern "C" double vtk_math_neg_inf(vtkMath* sself) { return sself->NegInf(); }
extern "C" double vtk_math_nan(vtkMath* sself) { return sself->Nan(); }
extern "C" int vtk_math_is_inf(vtkMath* sself, double x) { return sself->IsInf(x); }
extern "C" int vtk_math_is_nan(vtkMath* sself, double x) { return sself->IsNan(x); }
extern "C" bool vtk_math_is_finite(vtkMath* sself, double x) { return sself->IsFinite(x); }
extern "C" vtkMersenneTwister * vtkMersenneTwister_new () {return vtkMersenneTwister :: New () ;}
extern "C" void vtkMersenneTwister_destructor (vtkMersenneTwister * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_mersenne_twister_initialize(vtkMersenneTwister* sself, unsigned int seed) { sself->Initialize(seed); }
extern "C" unsigned int vtk_mersenne_twister_initialize_new_sequence(vtkMersenneTwister* sself, unsigned int seed, int p) { return sself->InitializeNewSequence(seed, p); }
extern "C" void vtk_mersenne_twister_initialize_sequence(vtkMersenneTwister* sself, unsigned int id, unsigned int seed, int p) { sself->InitializeSequence(id, seed, p); }
extern "C" double vtk_mersenne_twister_get_value(vtkMersenneTwister* sself, unsigned int id) { return sself->GetValue(id); }
extern "C" void vtk_mersenne_twister_next(vtkMersenneTwister* sself, unsigned int id) { sself->Next(id); }
extern "C" vtkMinimalStandardRandomSequence * vtkMinimalStandardRandomSequence_new () {return vtkMinimalStandardRandomSequence :: New () ;}
extern "C" void vtkMinimalStandardRandomSequence_destructor (vtkMinimalStandardRandomSequence * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_minimal_standard_random_sequence_initialize(vtkMinimalStandardRandomSequence* sself, unsigned int seed) { sself->Initialize(seed); }
extern "C" void vtk_minimal_standard_random_sequence_set_seed(vtkMinimalStandardRandomSequence* sself, int value) { sself->SetSeed(value); }
extern "C" void vtk_minimal_standard_random_sequence_set_seed_only(vtkMinimalStandardRandomSequence* sself, int value) { sself->SetSeedOnly(value); }
extern "C" int vtk_minimal_standard_random_sequence_get_seed(vtkMinimalStandardRandomSequence* sself) { return sself->GetSeed(); }
extern "C" double vtk_minimal_standard_random_sequence_get_value(vtkMinimalStandardRandomSequence* sself) { return sself->GetValue(); }
extern "C" void vtk_minimal_standard_random_sequence_next(vtkMinimalStandardRandomSequence* sself) { sself->Next(); }
extern "C" double vtk_minimal_standard_random_sequence_get_range_value(vtkMinimalStandardRandomSequence* sself, double rangeMin, double rangeMax) { return sself->GetRangeValue(rangeMin, rangeMax); }
extern "C" double vtk_minimal_standard_random_sequence_get_next_range_value(vtkMinimalStandardRandomSequence* sself, double rangeMin, double rangeMax) { return sself->GetNextRangeValue(rangeMin, rangeMax); }
extern "C" vtkMultiThreader * vtkMultiThreader_new () {return vtkMultiThreader :: New () ;}
extern "C" void vtkMultiThreader_destructor (vtkMultiThreader * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_multi_threader_set_number_of_threads(vtkMultiThreader* sself, int _arg) { sself->SetNumberOfThreads(_arg); }
extern "C" int vtk_multi_threader_get_number_of_threads_min_value(vtkMultiThreader* sself) { return sself->GetNumberOfThreadsMinValue(); }
extern "C" int vtk_multi_threader_get_number_of_threads_max_value(vtkMultiThreader* sself) { return sself->GetNumberOfThreadsMaxValue(); }
extern "C" int vtk_multi_threader_get_number_of_threads(vtkMultiThreader* sself) { return sself->GetNumberOfThreads(); }
extern "C" int vtk_multi_threader_get_global_static_maximum_number_of_threads(vtkMultiThreader* sself) { return sself->GetGlobalStaticMaximumNumberOfThreads(); }
extern "C" void vtk_multi_threader_set_global_maximum_number_of_threads(vtkMultiThreader* sself, int val) { sself->SetGlobalMaximumNumberOfThreads(val); }
extern "C" int vtk_multi_threader_get_global_maximum_number_of_threads(vtkMultiThreader* sself) { return sself->GetGlobalMaximumNumberOfThreads(); }
extern "C" void vtk_multi_threader_set_global_default_number_of_threads(vtkMultiThreader* sself, int val) { sself->SetGlobalDefaultNumberOfThreads(val); }
extern "C" int vtk_multi_threader_get_global_default_number_of_threads(vtkMultiThreader* sself) { return sself->GetGlobalDefaultNumberOfThreads(); }
extern "C" void vtk_multi_threader_single_method_execute(vtkMultiThreader* sself) { sself->SingleMethodExecute(); }
extern "C" void vtk_multi_threader_multiple_method_execute(vtkMultiThreader* sself) { sself->MultipleMethodExecute(); }
extern "C" void vtk_multi_threader_terminate_thread(vtkMultiThreader* sself, int threadId) { sself->TerminateThread(threadId); }
extern "C" int vtk_multi_threader_is_thread_active(vtkMultiThreader* sself, int threadId) { return sself->IsThreadActive(threadId); }
extern "C" vtkObject * vtkObject_new () {return vtkObject :: New () ;}
extern "C" void vtkObject_destructor (vtkObject * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_object_is_type_of(vtkObject* sself, const char* type) { return sself->IsTypeOf(type); }
extern "C" int vtk_object_is_a(vtkObject* sself, const char* type) { return sself->IsA(type); }
extern "C" long long vtk_object_get_number_of_generations_from_base_type(vtkObject* sself, const char* type) { return sself->GetNumberOfGenerationsFromBaseType(type); }
extern "C" long long vtk_object_get_number_of_generations_from_base(vtkObject* sself, const char* type) { return sself->GetNumberOfGenerationsFromBase(type); }
extern "C" void vtk_object_debug_on(vtkObject* sself) { sself->DebugOn(); }
extern "C" void vtk_object_debug_off(vtkObject* sself) { sself->DebugOff(); }
extern "C" bool vtk_object_get_debug(vtkObject* sself) { return sself->GetDebug(); }
extern "C" void vtk_object_set_debug(vtkObject* sself, bool debugFlag) { sself->SetDebug(debugFlag); }
extern "C" void vtk_object_break_on_error(vtkObject* sself) { sself->BreakOnError(); }
extern "C" void vtk_object_modified(vtkObject* sself) { sself->Modified(); }
extern "C" unsigned long vtk_object_get_m_time(vtkObject* sself) { return sself->GetMTime(); }
extern "C" void vtk_object_set_global_warning_display(vtkObject* sself, int val) { sself->SetGlobalWarningDisplay(val); }
extern "C" void vtk_object_global_warning_display_on(vtkObject* sself) { sself->GlobalWarningDisplayOn(); }
extern "C" void vtk_object_global_warning_display_off(vtkObject* sself) { sself->GlobalWarningDisplayOff(); }
extern "C" int vtk_object_get_global_warning_display(vtkObject* sself) { return sself->GetGlobalWarningDisplay(); }
extern "C" void vtk_object_remove_all_observers(vtkObject* sself) { sself->RemoveAllObservers(); }
extern "C" int vtk_object_invoke_event(vtkObject* sself, unsigned long event, void* callData) { return sself->InvokeEvent(event, callData); }
extern "C" vtkObjectFactoryCollection * vtkObjectFactoryCollection_new () {return vtkObjectFactoryCollection :: New () ;}
extern "C" void vtkObjectFactoryCollection_destructor (vtkObjectFactoryCollection * sself) {sself -> Delete () ; return ;}
extern "C" vtkOldStyleCallbackCommand * vtkOldStyleCallbackCommand_new () {return vtkOldStyleCallbackCommand :: New () ;}
extern "C" void vtkOldStyleCallbackCommand_destructor (vtkOldStyleCallbackCommand * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_old_style_callback_command_set_client_data(vtkOldStyleCallbackCommand* sself, void* cd) { sself->SetClientData(cd); }
extern "C" vtkOutputWindow * vtkOutputWindow_new () {return vtkOutputWindow :: New () ;}
extern "C" void vtkOutputWindow_destructor (vtkOutputWindow * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_output_window_display_text(vtkOutputWindow* sself, const char* p0) { sself->DisplayText(p0); }
extern "C" void vtk_output_window_display_error_text(vtkOutputWindow* sself, const char* p0) { sself->DisplayErrorText(p0); }
extern "C" void vtk_output_window_display_warning_text(vtkOutputWindow* sself, const char* p0) { sself->DisplayWarningText(p0); }
extern "C" void vtk_output_window_display_generic_warning_text(vtkOutputWindow* sself, const char* p0) { sself->DisplayGenericWarningText(p0); }
extern "C" void vtk_output_window_display_debug_text(vtkOutputWindow* sself, const char* p0) { sself->DisplayDebugText(p0); }
extern "C" void vtk_output_window_prompt_user_on(vtkOutputWindow* sself) { sself->PromptUserOn(); }
extern "C" void vtk_output_window_prompt_user_off(vtkOutputWindow* sself) { sself->PromptUserOff(); }
extern "C" void vtk_output_window_set_prompt_user(vtkOutputWindow* sself, bool _arg) { sself->SetPromptUser(_arg); }
extern "C" void vtk_output_window_set_use_std_error_for_all_messages(vtkOutputWindow* sself, bool p0) { sself->SetUseStdErrorForAllMessages(p0); }
extern "C" bool vtk_output_window_get_use_std_error_for_all_messages(vtkOutputWindow* sself) { return sself->GetUseStdErrorForAllMessages(); }
extern "C" void vtk_output_window_use_std_error_for_all_messages_on(vtkOutputWindow* sself) { sself->UseStdErrorForAllMessagesOn(); }
extern "C" void vtk_output_window_use_std_error_for_all_messages_off(vtkOutputWindow* sself) { sself->UseStdErrorForAllMessagesOff(); }
extern "C" void vtk_output_window_set_display_mode(vtkOutputWindow* sself, int _arg) { sself->SetDisplayMode(_arg); }
extern "C" int vtk_output_window_get_display_mode_min_value(vtkOutputWindow* sself) { return sself->GetDisplayModeMinValue(); }
extern "C" int vtk_output_window_get_display_mode_max_value(vtkOutputWindow* sself) { return sself->GetDisplayModeMaxValue(); }
extern "C" int vtk_output_window_get_display_mode(vtkOutputWindow* sself) { return sself->GetDisplayMode(); }
extern "C" void vtk_output_window_set_display_mode_to_default(vtkOutputWindow* sself) { sself->SetDisplayModeToDefault(); }
extern "C" void vtk_output_window_set_display_mode_to_never(vtkOutputWindow* sself) { sself->SetDisplayModeToNever(); }
extern "C" void vtk_output_window_set_display_mode_to_always(vtkOutputWindow* sself) { sself->SetDisplayModeToAlways(); }
extern "C" void vtk_output_window_set_display_mode_to_always_std_err(vtkOutputWindow* sself) { sself->SetDisplayModeToAlwaysStdErr(); }
extern "C" vtkOverrideInformationCollection * vtkOverrideInformationCollection_new () {return vtkOverrideInformationCollection :: New () ;}
extern "C" void vtkOverrideInformationCollection_destructor (vtkOverrideInformationCollection * sself) {sself -> Delete () ; return ;}
extern "C" vtkPoints * vtkPoints_new () {return vtkPoints :: New () ;}
extern "C" void vtkPoints_destructor (vtkPoints * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_points_allocate(vtkPoints* sself, long long sz, long long ext) { return sself->Allocate(sz, ext); }
extern "C" void vtk_points_initialize(vtkPoints* sself) { sself->Initialize(); }
extern "C" int vtk_points_get_data_type(vtkPoints* sself) { return sself->GetDataType(); }
extern "C" void vtk_points_set_data_type(vtkPoints* sself, int dataType) { sself->SetDataType(dataType); }
extern "C" void vtk_points_set_data_type_to_bit(vtkPoints* sself) { sself->SetDataTypeToBit(); }
extern "C" void vtk_points_set_data_type_to_char(vtkPoints* sself) { sself->SetDataTypeToChar(); }
extern "C" void vtk_points_set_data_type_to_unsigned_char(vtkPoints* sself) { sself->SetDataTypeToUnsignedChar(); }
extern "C" void vtk_points_set_data_type_to_short(vtkPoints* sself) { sself->SetDataTypeToShort(); }
extern "C" void vtk_points_set_data_type_to_unsigned_short(vtkPoints* sself) { sself->SetDataTypeToUnsignedShort(); }
extern "C" void vtk_points_set_data_type_to_int(vtkPoints* sself) { sself->SetDataTypeToInt(); }
extern "C" void vtk_points_set_data_type_to_unsigned_int(vtkPoints* sself) { sself->SetDataTypeToUnsignedInt(); }
extern "C" void vtk_points_set_data_type_to_long(vtkPoints* sself) { sself->SetDataTypeToLong(); }
extern "C" void vtk_points_set_data_type_to_unsigned_long(vtkPoints* sself) { sself->SetDataTypeToUnsignedLong(); }
extern "C" void vtk_points_set_data_type_to_float(vtkPoints* sself) { sself->SetDataTypeToFloat(); }
extern "C" void vtk_points_set_data_type_to_double(vtkPoints* sself) { sself->SetDataTypeToDouble(); }
extern "C" void* vtk_points_get_void_pointer(vtkPoints* sself, const int id) { return sself->GetVoidPointer(id); }
extern "C" void vtk_points_squeeze(vtkPoints* sself) { sself->Squeeze(); }
extern "C" void vtk_points_reset(vtkPoints* sself) { sself->Reset(); }
extern "C" unsigned long vtk_points_get_actual_memory_size(vtkPoints* sself) { return sself->GetActualMemorySize(); }
extern "C" long long vtk_points_get_number_of_points(vtkPoints* sself) { return sself->GetNumberOfPoints(); }
extern "C" void vtk_points_set_point(vtkPoints* sself, long long id, double x, double y, double z) { sself->SetPoint(id, x, y, z); }
extern "C" void vtk_points_insert_point(vtkPoints* sself, long long id, double x, double y, double z) { sself->InsertPoint(id, x, y, z); }
extern "C" long long vtk_points_insert_next_point(vtkPoints* sself, double x, double y, double z) { return sself->InsertNextPoint(x, y, z); }
extern "C" void vtk_points_set_number_of_points(vtkPoints* sself, long long numPoints) { sself->SetNumberOfPoints(numPoints); }
extern "C" int vtk_points_resize(vtkPoints* sself, long long numPoints) { return sself->Resize(numPoints); }
extern "C" void vtk_points_compute_bounds(vtkPoints* sself) { sself->ComputeBounds(); }
extern "C" unsigned long vtk_points_get_m_time(vtkPoints* sself) { return sself->GetMTime(); }
extern "C" void vtk_points_modified(vtkPoints* sself) { sself->Modified(); }
extern "C" vtkPoints2D * vtkPoints2D_new () {return vtkPoints2D :: New () ;}
extern "C" void vtkPoints2D_destructor (vtkPoints2D * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_points_2_d_allocate(vtkPoints2D* sself, long long sz, long long ext) { return sself->Allocate(sz, ext); }
extern "C" void vtk_points_2_d_initialize(vtkPoints2D* sself) { sself->Initialize(); }
extern "C" int vtk_points_2_d_get_data_type(vtkPoints2D* sself) { return sself->GetDataType(); }
extern "C" void vtk_points_2_d_set_data_type(vtkPoints2D* sself, int dataType) { sself->SetDataType(dataType); }
extern "C" void vtk_points_2_d_set_data_type_to_bit(vtkPoints2D* sself) { sself->SetDataTypeToBit(); }
extern "C" void vtk_points_2_d_set_data_type_to_char(vtkPoints2D* sself) { sself->SetDataTypeToChar(); }
extern "C" void vtk_points_2_d_set_data_type_to_unsigned_char(vtkPoints2D* sself) { sself->SetDataTypeToUnsignedChar(); }
extern "C" void vtk_points_2_d_set_data_type_to_short(vtkPoints2D* sself) { sself->SetDataTypeToShort(); }
extern "C" void vtk_points_2_d_set_data_type_to_unsigned_short(vtkPoints2D* sself) { sself->SetDataTypeToUnsignedShort(); }
extern "C" void vtk_points_2_d_set_data_type_to_int(vtkPoints2D* sself) { sself->SetDataTypeToInt(); }
extern "C" void vtk_points_2_d_set_data_type_to_unsigned_int(vtkPoints2D* sself) { sself->SetDataTypeToUnsignedInt(); }
extern "C" void vtk_points_2_d_set_data_type_to_long(vtkPoints2D* sself) { sself->SetDataTypeToLong(); }
extern "C" void vtk_points_2_d_set_data_type_to_unsigned_long(vtkPoints2D* sself) { sself->SetDataTypeToUnsignedLong(); }
extern "C" void vtk_points_2_d_set_data_type_to_float(vtkPoints2D* sself) { sself->SetDataTypeToFloat(); }
extern "C" void vtk_points_2_d_set_data_type_to_double(vtkPoints2D* sself) { sself->SetDataTypeToDouble(); }
extern "C" void* vtk_points_2_d_get_void_pointer(vtkPoints2D* sself, const int id) { return sself->GetVoidPointer(id); }
extern "C" void vtk_points_2_d_squeeze(vtkPoints2D* sself) { sself->Squeeze(); }
extern "C" void vtk_points_2_d_reset(vtkPoints2D* sself) { sself->Reset(); }
extern "C" unsigned long vtk_points_2_d_get_actual_memory_size(vtkPoints2D* sself) { return sself->GetActualMemorySize(); }
extern "C" long long vtk_points_2_d_get_number_of_points(vtkPoints2D* sself) { return sself->GetNumberOfPoints(); }
extern "C" void vtk_points_2_d_set_point(vtkPoints2D* sself, long long id, double x, double y) { sself->SetPoint(id, x, y); }
extern "C" void vtk_points_2_d_insert_point(vtkPoints2D* sself, long long id, double x, double y) { sself->InsertPoint(id, x, y); }
extern "C" long long vtk_points_2_d_insert_next_point(vtkPoints2D* sself, double x, double y) { return sself->InsertNextPoint(x, y); }
extern "C" void vtk_points_2_d_remove_point(vtkPoints2D* sself, long long id) { sself->RemovePoint(id); }
extern "C" void vtk_points_2_d_set_number_of_points(vtkPoints2D* sself, long long numPoints) { sself->SetNumberOfPoints(numPoints); }
extern "C" int vtk_points_2_d_resize(vtkPoints2D* sself, long long numPoints) { return sself->Resize(numPoints); }
extern "C" void vtk_points_2_d_compute_bounds(vtkPoints2D* sself) { sself->ComputeBounds(); }
extern "C" vtkPriorityQueue * vtkPriorityQueue_new () {return vtkPriorityQueue :: New () ;}
extern "C" void vtkPriorityQueue_destructor (vtkPriorityQueue * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_priority_queue_allocate(vtkPriorityQueue* sself, long long sz, long long ext) { sself->Allocate(sz, ext); }
extern "C" void vtk_priority_queue_insert(vtkPriorityQueue* sself, double priority, long long id) { sself->Insert(priority, id); }
extern "C" long long vtk_priority_queue_pop(vtkPriorityQueue* sself, long long location, double& priority) { return sself->Pop(location, priority); }
extern "C" long long vtk_priority_queue_peek(vtkPriorityQueue* sself, long long location, double& priority) { return sself->Peek(location, priority); }
extern "C" double vtk_priority_queue_delete_id(vtkPriorityQueue* sself, long long id) { return sself->DeleteId(id); }
extern "C" double vtk_priority_queue_get_priority(vtkPriorityQueue* sself, long long id) { return sself->GetPriority(id); }
extern "C" long long vtk_priority_queue_get_number_of_items(vtkPriorityQueue* sself) { return sself->GetNumberOfItems(); }
extern "C" void vtk_priority_queue_reset(vtkPriorityQueue* sself) { sself->Reset(); }
extern "C" vtkRandomPool * vtkRandomPool_new () {return vtkRandomPool :: New () ;}
extern "C" void vtkRandomPool_destructor (vtkRandomPool * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_random_pool_set_size(vtkRandomPool* sself, long long _arg) { sself->SetSize(_arg); }
extern "C" long long vtk_random_pool_get_size_min_value(vtkRandomPool* sself) { return sself->GetSizeMinValue(); }
extern "C" long long vtk_random_pool_get_size_max_value(vtkRandomPool* sself) { return sself->GetSizeMaxValue(); }
extern "C" long long vtk_random_pool_get_size(vtkRandomPool* sself) { return sself->GetSize(); }
extern "C" void vtk_random_pool_set_number_of_components(vtkRandomPool* sself, long long _arg) { sself->SetNumberOfComponents(_arg); }
extern "C" long long vtk_random_pool_get_number_of_components_min_value(vtkRandomPool* sself) { return sself->GetNumberOfComponentsMinValue(); }
extern "C" long long vtk_random_pool_get_number_of_components_max_value(vtkRandomPool* sself) { return sself->GetNumberOfComponentsMaxValue(); }
extern "C" long long vtk_random_pool_get_number_of_components(vtkRandomPool* sself) { return sself->GetNumberOfComponents(); }
extern "C" long long vtk_random_pool_get_total_size(vtkRandomPool* sself) { return sself->GetTotalSize(); }
extern "C" double vtk_random_pool_get_value(vtkRandomPool* sself, long long i) { return sself->GetValue(i); }
extern "C" void vtk_random_pool_set_chunk_size(vtkRandomPool* sself, long long _arg) { sself->SetChunkSize(_arg); }
extern "C" long long vtk_random_pool_get_chunk_size_min_value(vtkRandomPool* sself) { return sself->GetChunkSizeMinValue(); }
extern "C" long long vtk_random_pool_get_chunk_size_max_value(vtkRandomPool* sself) { return sself->GetChunkSizeMaxValue(); }
extern "C" long long vtk_random_pool_get_chunk_size(vtkRandomPool* sself) { return sself->GetChunkSize(); }
extern "C" vtkReferenceCount * vtkReferenceCount_new () {return vtkReferenceCount :: New () ;}
extern "C" void vtkReferenceCount_destructor (vtkReferenceCount * sself) {sself -> Delete () ; return ;}
extern "C" vtkScalarsToColors * vtkScalarsToColors_new () {return vtkScalarsToColors :: New () ;}
extern "C" void vtkScalarsToColors_destructor (vtkScalarsToColors * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_scalars_to_colors_is_opaque(vtkScalarsToColors* sself) { return sself->IsOpaque(); }
extern "C" void vtk_scalars_to_colors_build(vtkScalarsToColors* sself) { sself->Build(); }
extern "C" void vtk_scalars_to_colors_set_range(vtkScalarsToColors* sself, double min, double max) { sself->SetRange(min, max); }
extern "C" double vtk_scalars_to_colors_get_opacity(vtkScalarsToColors* sself, double v) { return sself->GetOpacity(v); }
extern "C" double vtk_scalars_to_colors_get_luminance(vtkScalarsToColors* sself, double x) { return sself->GetLuminance(x); }
extern "C" void vtk_scalars_to_colors_set_alpha(vtkScalarsToColors* sself, double alpha) { sself->SetAlpha(alpha); }
extern "C" double vtk_scalars_to_colors_get_alpha(vtkScalarsToColors* sself) { return sself->GetAlpha(); }
extern "C" void vtk_scalars_to_colors_set_vector_mode(vtkScalarsToColors* sself, int _arg) { sself->SetVectorMode(_arg); }
extern "C" int vtk_scalars_to_colors_get_vector_mode(vtkScalarsToColors* sself) { return sself->GetVectorMode(); }
extern "C" void vtk_scalars_to_colors_set_vector_mode_to_magnitude(vtkScalarsToColors* sself) { sself->SetVectorModeToMagnitude(); }
extern "C" void vtk_scalars_to_colors_set_vector_mode_to_component(vtkScalarsToColors* sself) { sself->SetVectorModeToComponent(); }
extern "C" void vtk_scalars_to_colors_set_vector_mode_to_rgb_colors(vtkScalarsToColors* sself) { sself->SetVectorModeToRGBColors(); }
extern "C" void vtk_scalars_to_colors_set_vector_component(vtkScalarsToColors* sself, int _arg) { sself->SetVectorComponent(_arg); }
extern "C" int vtk_scalars_to_colors_get_vector_component(vtkScalarsToColors* sself) { return sself->GetVectorComponent(); }
extern "C" void vtk_scalars_to_colors_set_vector_size(vtkScalarsToColors* sself, int _arg) { sself->SetVectorSize(_arg); }
extern "C" int vtk_scalars_to_colors_get_vector_size(vtkScalarsToColors* sself) { return sself->GetVectorSize(); }
extern "C" int vtk_scalars_to_colors_using_log_scale(vtkScalarsToColors* sself) { return sself->UsingLogScale(); }
extern "C" long long vtk_scalars_to_colors_get_number_of_available_colors(vtkScalarsToColors* sself) { return sself->GetNumberOfAvailableColors(); }
extern "C" long long vtk_scalars_to_colors_get_number_of_annotated_values(vtkScalarsToColors* sself) { return sself->GetNumberOfAnnotatedValues(); }
extern "C" void vtk_scalars_to_colors_reset_annotations(vtkScalarsToColors* sself) { sself->ResetAnnotations(); }
extern "C" void vtk_scalars_to_colors_set_indexed_lookup(vtkScalarsToColors* sself, int _arg) { sself->SetIndexedLookup(_arg); }
extern "C" int vtk_scalars_to_colors_get_indexed_lookup(vtkScalarsToColors* sself) { return sself->GetIndexedLookup(); }
extern "C" void vtk_scalars_to_colors_indexed_lookup_on(vtkScalarsToColors* sself) { sself->IndexedLookupOn(); }
extern "C" void vtk_scalars_to_colors_indexed_lookup_off(vtkScalarsToColors* sself) { sself->IndexedLookupOff(); }
extern "C" vtkShortArray * vtkShortArray_new () {return vtkShortArray :: New () ;}
extern "C" void vtkShortArray_destructor (vtkShortArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_short_array_get_data_type(vtkShortArray* sself) { return sself->GetDataType(); }
extern "C" short vtk_short_array_get_value(vtkShortArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_short_array_set_value(vtkShortArray* sself, long long id, short value) { sself->SetValue(id, value); }
extern "C" bool vtk_short_array_set_number_of_values(vtkShortArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_short_array_insert_value(vtkShortArray* sself, long long id, short f) { sself->InsertValue(id, f); }
extern "C" long long vtk_short_array_insert_next_value(vtkShortArray* sself, short f) { return sself->InsertNextValue(f); }
extern "C" short vtk_short_array_get_data_type_value_min(vtkShortArray* sself) { return sself->GetDataTypeValueMin(); }
extern "C" short vtk_short_array_get_data_type_value_max(vtkShortArray* sself) { return sself->GetDataTypeValueMax(); }
extern "C" vtkSignedCharArray * vtkSignedCharArray_new () {return vtkSignedCharArray :: New () ;}
extern "C" void vtkSignedCharArray_destructor (vtkSignedCharArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_signed_char_array_get_data_type(vtkSignedCharArray* sself) { return sself->GetDataType(); }
extern "C" signed char vtk_signed_char_array_get_value(vtkSignedCharArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_signed_char_array_set_value(vtkSignedCharArray* sself, long long id, signed char value) { sself->SetValue(id, value); }
extern "C" bool vtk_signed_char_array_set_number_of_values(vtkSignedCharArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_signed_char_array_insert_value(vtkSignedCharArray* sself, long long id, signed char f) { sself->InsertValue(id, f); }
extern "C" long long vtk_signed_char_array_insert_next_value(vtkSignedCharArray* sself, signed char f) { return sself->InsertNextValue(f); }
extern "C" signed char vtk_signed_char_array_get_data_type_value_min(vtkSignedCharArray* sself) { return sself->GetDataTypeValueMin(); }
extern "C" signed char vtk_signed_char_array_get_data_type_value_max(vtkSignedCharArray* sself) { return sself->GetDataTypeValueMax(); }
extern "C" vtkSortDataArray * vtkSortDataArray_new () {return vtkSortDataArray :: New () ;}
extern "C" void vtkSortDataArray_destructor (vtkSortDataArray * sself) {sself -> Delete () ; return ;}
extern "C" vtkStringArray * vtkStringArray_new () {return vtkStringArray :: New () ;}
extern "C" void vtkStringArray_destructor (vtkStringArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_string_array_get_data_type(vtkStringArray* sself) { return sself->GetDataType(); }
extern "C" int vtk_string_array_is_numeric(vtkStringArray* sself) { return sself->IsNumeric(); }
extern "C" void vtk_string_array_initialize(vtkStringArray* sself) { sself->Initialize(); }
extern "C" int vtk_string_array_get_data_type_size(vtkStringArray* sself) { return sself->GetDataTypeSize(); }
extern "C" void vtk_string_array_squeeze(vtkStringArray* sself) { sself->Squeeze(); }
extern "C" int vtk_string_array_resize(vtkStringArray* sself, long long numTuples) { return sself->Resize(numTuples); }
extern "C" int vtk_string_array_allocate(vtkStringArray* sself, long long sz, long long ext) { return sself->Allocate(sz, ext); }
extern "C" void vtk_string_array_set_number_of_tuples(vtkStringArray* sself, long long number) { sself->SetNumberOfTuples(number); }
extern "C" long long vtk_string_array_get_number_of_values(vtkStringArray* sself) { return sself->GetNumberOfValues(); }
extern "C" int vtk_string_array_get_number_of_element_components(vtkStringArray* sself) { return sself->GetNumberOfElementComponents(); }
extern "C" int vtk_string_array_get_element_component_size(vtkStringArray* sself) { return sself->GetElementComponentSize(); }
extern "C" void* vtk_string_array_get_void_pointer(vtkStringArray* sself, long long id) { return sself->GetVoidPointer(id); }
extern "C" void vtk_string_array_set_void_array(vtkStringArray* sself, void* array, long long size, int save) { sself->SetVoidArray(array, size, save); }
extern "C" unsigned long vtk_string_array_get_actual_memory_size(vtkStringArray* sself) { return sself->GetActualMemorySize(); }
extern "C" long long vtk_string_array_get_data_size(vtkStringArray* sself) { return sself->GetDataSize(); }
extern "C" void vtk_string_array_data_changed(vtkStringArray* sself) { sself->DataChanged(); }
extern "C" void vtk_string_array_data_element_changed(vtkStringArray* sself, long long id) { sself->DataElementChanged(id); }
extern "C" void vtk_string_array_clear_lookup(vtkStringArray* sself) { sself->ClearLookup(); }
extern "C" vtkStringOutputWindow * vtkStringOutputWindow_new () {return vtkStringOutputWindow :: New () ;}
extern "C" void vtkStringOutputWindow_destructor (vtkStringOutputWindow * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_string_output_window_display_text(vtkStringOutputWindow* sself, const char* p0) { sself->DisplayText(p0); }
extern "C" vtkTimePointUtility * vtkTimePointUtility_new () {return vtkTimePointUtility :: New () ;}
extern "C" void vtkTimePointUtility_destructor (vtkTimePointUtility * sself) {sself -> Delete () ; return ;}
extern "C" unsigned long long vtk_time_point_utility_date_to_time_point(vtkTimePointUtility* sself, int year, int month, int day) { return sself->DateToTimePoint(year, month, day); }
extern "C" unsigned long long vtk_time_point_utility_time_to_time_point(vtkTimePointUtility* sself, int hour, int minute, int second, int millis) { return sself->TimeToTimePoint(hour, minute, second, millis); }
extern "C" unsigned long long vtk_time_point_utility_date_time_to_time_point(vtkTimePointUtility* sself, int year, int month, int day, int hour, int minute, int sec, int millis) { return sself->DateTimeToTimePoint(year, month, day, hour, minute, sec, millis); }
extern "C" void vtk_time_point_utility_get_date(vtkTimePointUtility* sself, unsigned long long time, int& year, int& month, int& day) { sself->GetDate(time, year, month, day); }
extern "C" void vtk_time_point_utility_get_time(vtkTimePointUtility* sself, unsigned long long time, int& hour, int& minute, int& second, int& millis) { sself->GetTime(time, hour, minute, second, millis); }
extern "C" void vtk_time_point_utility_get_date_time(vtkTimePointUtility* sself, unsigned long long time, int& year, int& month, int& day, int& hour, int& minute, int& second, int& millis) { sself->GetDateTime(time, year, month, day, hour, minute, second, millis); }
extern "C" int vtk_time_point_utility_get_year(vtkTimePointUtility* sself, unsigned long long time) { return sself->GetYear(time); }
extern "C" int vtk_time_point_utility_get_month(vtkTimePointUtility* sself, unsigned long long time) { return sself->GetMonth(time); }
extern "C" int vtk_time_point_utility_get_day(vtkTimePointUtility* sself, unsigned long long time) { return sself->GetDay(time); }
extern "C" int vtk_time_point_utility_get_hour(vtkTimePointUtility* sself, unsigned long long time) { return sself->GetHour(time); }
extern "C" int vtk_time_point_utility_get_minute(vtkTimePointUtility* sself, unsigned long long time) { return sself->GetMinute(time); }
extern "C" int vtk_time_point_utility_get_second(vtkTimePointUtility* sself, unsigned long long time) { return sself->GetSecond(time); }
extern "C" int vtk_time_point_utility_get_millisecond(vtkTimePointUtility* sself, unsigned long long time) { return sself->GetMillisecond(time); }
extern "C" const char* vtk_time_point_utility_time_point_to_iso_8601(vtkTimePointUtility* sself, unsigned long long p0, int format) { return sself->TimePointToISO8601(p0, format); }
extern "C" vtkTypeFloat32Array * vtkTypeFloat32Array_new () {return vtkTypeFloat32Array :: New () ;}
extern "C" void vtkTypeFloat32Array_destructor (vtkTypeFloat32Array * sself) {sself -> Delete () ; return ;}
extern "C" vtkTypeFloat64Array * vtkTypeFloat64Array_new () {return vtkTypeFloat64Array :: New () ;}
extern "C" void vtkTypeFloat64Array_destructor (vtkTypeFloat64Array * sself) {sself -> Delete () ; return ;}
extern "C" vtkTypeInt16Array * vtkTypeInt16Array_new () {return vtkTypeInt16Array :: New () ;}
extern "C" void vtkTypeInt16Array_destructor (vtkTypeInt16Array * sself) {sself -> Delete () ; return ;}
extern "C" vtkTypeInt32Array * vtkTypeInt32Array_new () {return vtkTypeInt32Array :: New () ;}
extern "C" void vtkTypeInt32Array_destructor (vtkTypeInt32Array * sself) {sself -> Delete () ; return ;}
extern "C" vtkTypeInt64Array * vtkTypeInt64Array_new () {return vtkTypeInt64Array :: New () ;}
extern "C" void vtkTypeInt64Array_destructor (vtkTypeInt64Array * sself) {sself -> Delete () ; return ;}
extern "C" vtkTypeInt8Array * vtkTypeInt8Array_new () {return vtkTypeInt8Array :: New () ;}
extern "C" void vtkTypeInt8Array_destructor (vtkTypeInt8Array * sself) {sself -> Delete () ; return ;}
extern "C" vtkTypeUInt16Array * vtkTypeUInt16Array_new () {return vtkTypeUInt16Array :: New () ;}
extern "C" void vtkTypeUInt16Array_destructor (vtkTypeUInt16Array * sself) {sself -> Delete () ; return ;}
extern "C" vtkTypeUInt32Array * vtkTypeUInt32Array_new () {return vtkTypeUInt32Array :: New () ;}
extern "C" void vtkTypeUInt32Array_destructor (vtkTypeUInt32Array * sself) {sself -> Delete () ; return ;}
extern "C" vtkTypeUInt64Array * vtkTypeUInt64Array_new () {return vtkTypeUInt64Array :: New () ;}
extern "C" void vtkTypeUInt64Array_destructor (vtkTypeUInt64Array * sself) {sself -> Delete () ; return ;}
extern "C" vtkTypeUInt8Array * vtkTypeUInt8Array_new () {return vtkTypeUInt8Array :: New () ;}
extern "C" void vtkTypeUInt8Array_destructor (vtkTypeUInt8Array * sself) {sself -> Delete () ; return ;}
extern "C" vtkUnicodeStringArray * vtkUnicodeStringArray_new () {return vtkUnicodeStringArray :: New () ;}
extern "C" void vtkUnicodeStringArray_destructor (vtkUnicodeStringArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_unicode_string_array_allocate(vtkUnicodeStringArray* sself, long long sz, long long ext) { return sself->Allocate(sz, ext); }
extern "C" void vtk_unicode_string_array_initialize(vtkUnicodeStringArray* sself) { sself->Initialize(); }
extern "C" int vtk_unicode_string_array_get_data_type(vtkUnicodeStringArray* sself) { return sself->GetDataType(); }
extern "C" int vtk_unicode_string_array_get_data_type_size(vtkUnicodeStringArray* sself) { return sself->GetDataTypeSize(); }
extern "C" int vtk_unicode_string_array_get_element_component_size(vtkUnicodeStringArray* sself) { return sself->GetElementComponentSize(); }
extern "C" void vtk_unicode_string_array_set_number_of_tuples(vtkUnicodeStringArray* sself, long long number) { sself->SetNumberOfTuples(number); }
extern "C" void* vtk_unicode_string_array_get_void_pointer(vtkUnicodeStringArray* sself, long long id) { return sself->GetVoidPointer(id); }
extern "C" void vtk_unicode_string_array_squeeze(vtkUnicodeStringArray* sself) { sself->Squeeze(); }
extern "C" int vtk_unicode_string_array_resize(vtkUnicodeStringArray* sself, long long numTuples) { return sself->Resize(numTuples); }
extern "C" void vtk_unicode_string_array_set_void_array(vtkUnicodeStringArray* sself, void* array, long long size, int save) { sself->SetVoidArray(array, size, save); }
extern "C" unsigned long vtk_unicode_string_array_get_actual_memory_size(vtkUnicodeStringArray* sself) { return sself->GetActualMemorySize(); }
extern "C" int vtk_unicode_string_array_is_numeric(vtkUnicodeStringArray* sself) { return sself->IsNumeric(); }
extern "C" void vtk_unicode_string_array_data_changed(vtkUnicodeStringArray* sself) { sself->DataChanged(); }
extern "C" void vtk_unicode_string_array_clear_lookup(vtkUnicodeStringArray* sself) { sself->ClearLookup(); }
extern "C" void vtk_unicode_string_array_insert_next_utf_8_value(vtkUnicodeStringArray* sself, const char* p0) { sself->InsertNextUTF8Value(p0); }
extern "C" void vtk_unicode_string_array_set_utf_8_value(vtkUnicodeStringArray* sself, long long i, const char* p1) { sself->SetUTF8Value(i, p1); }
extern "C" const char* vtk_unicode_string_array_get_utf_8_value(vtkUnicodeStringArray* sself, long long i) { return sself->GetUTF8Value(i); }
extern "C" vtkUnsignedCharArray * vtkUnsignedCharArray_new () {return vtkUnsignedCharArray :: New () ;}
extern "C" void vtkUnsignedCharArray_destructor (vtkUnsignedCharArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_unsigned_char_array_get_data_type(vtkUnsignedCharArray* sself) { return sself->GetDataType(); }
extern "C" unsigned char vtk_unsigned_char_array_get_value(vtkUnsignedCharArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_unsigned_char_array_set_value(vtkUnsignedCharArray* sself, long long id, unsigned char value) { sself->SetValue(id, value); }
extern "C" bool vtk_unsigned_char_array_set_number_of_values(vtkUnsignedCharArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_unsigned_char_array_insert_value(vtkUnsignedCharArray* sself, long long id, unsigned char f) { sself->InsertValue(id, f); }
extern "C" long long vtk_unsigned_char_array_insert_next_value(vtkUnsignedCharArray* sself, unsigned char f) { return sself->InsertNextValue(f); }
extern "C" unsigned char vtk_unsigned_char_array_get_data_type_value_min(vtkUnsignedCharArray* sself) { return sself->GetDataTypeValueMin(); }
extern "C" unsigned char vtk_unsigned_char_array_get_data_type_value_max(vtkUnsignedCharArray* sself) { return sself->GetDataTypeValueMax(); }
extern "C" vtkUnsignedIntArray * vtkUnsignedIntArray_new () {return vtkUnsignedIntArray :: New () ;}
extern "C" void vtkUnsignedIntArray_destructor (vtkUnsignedIntArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_unsigned_int_array_get_data_type(vtkUnsignedIntArray* sself) { return sself->GetDataType(); }
extern "C" unsigned int vtk_unsigned_int_array_get_value(vtkUnsignedIntArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_unsigned_int_array_set_value(vtkUnsignedIntArray* sself, long long id, unsigned int value) { sself->SetValue(id, value); }
extern "C" bool vtk_unsigned_int_array_set_number_of_values(vtkUnsignedIntArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_unsigned_int_array_insert_value(vtkUnsignedIntArray* sself, long long id, unsigned int f) { sself->InsertValue(id, f); }
extern "C" long long vtk_unsigned_int_array_insert_next_value(vtkUnsignedIntArray* sself, unsigned int f) { return sself->InsertNextValue(f); }
extern "C" unsigned int vtk_unsigned_int_array_get_data_type_value_min(vtkUnsignedIntArray* sself) { return sself->GetDataTypeValueMin(); }
extern "C" unsigned int vtk_unsigned_int_array_get_data_type_value_max(vtkUnsignedIntArray* sself) { return sself->GetDataTypeValueMax(); }
extern "C" vtkUnsignedLongArray * vtkUnsignedLongArray_new () {return vtkUnsignedLongArray :: New () ;}
extern "C" void vtkUnsignedLongArray_destructor (vtkUnsignedLongArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_unsigned_long_array_get_data_type(vtkUnsignedLongArray* sself) { return sself->GetDataType(); }
extern "C" unsigned long vtk_unsigned_long_array_get_value(vtkUnsignedLongArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_unsigned_long_array_set_value(vtkUnsignedLongArray* sself, long long id, unsigned long value) { sself->SetValue(id, value); }
extern "C" bool vtk_unsigned_long_array_set_number_of_values(vtkUnsignedLongArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_unsigned_long_array_insert_value(vtkUnsignedLongArray* sself, long long id, unsigned long f) { sself->InsertValue(id, f); }
extern "C" long long vtk_unsigned_long_array_insert_next_value(vtkUnsignedLongArray* sself, unsigned long f) { return sself->InsertNextValue(f); }
extern "C" unsigned long vtk_unsigned_long_array_get_data_type_value_min(vtkUnsignedLongArray* sself) { return sself->GetDataTypeValueMin(); }
extern "C" unsigned long vtk_unsigned_long_array_get_data_type_value_max(vtkUnsignedLongArray* sself) { return sself->GetDataTypeValueMax(); }
extern "C" vtkUnsignedLongLongArray * vtkUnsignedLongLongArray_new () {return vtkUnsignedLongLongArray :: New () ;}
extern "C" void vtkUnsignedLongLongArray_destructor (vtkUnsignedLongLongArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_unsigned_long_long_array_get_data_type(vtkUnsignedLongLongArray* sself) { return sself->GetDataType(); }
extern "C" unsigned long long vtk_unsigned_long_long_array_get_value(vtkUnsignedLongLongArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_unsigned_long_long_array_set_value(vtkUnsignedLongLongArray* sself, long long id, unsigned long long value) { sself->SetValue(id, value); }
extern "C" bool vtk_unsigned_long_long_array_set_number_of_values(vtkUnsignedLongLongArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_unsigned_long_long_array_insert_value(vtkUnsignedLongLongArray* sself, long long id, unsigned long long f) { sself->InsertValue(id, f); }
extern "C" long long vtk_unsigned_long_long_array_insert_next_value(vtkUnsignedLongLongArray* sself, unsigned long long f) { return sself->InsertNextValue(f); }
extern "C" unsigned long long vtk_unsigned_long_long_array_get_data_type_value_min(vtkUnsignedLongLongArray* sself) { return sself->GetDataTypeValueMin(); }
extern "C" unsigned long long vtk_unsigned_long_long_array_get_data_type_value_max(vtkUnsignedLongLongArray* sself) { return sself->GetDataTypeValueMax(); }
extern "C" vtkUnsignedShortArray * vtkUnsignedShortArray_new () {return vtkUnsignedShortArray :: New () ;}
extern "C" void vtkUnsignedShortArray_destructor (vtkUnsignedShortArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_unsigned_short_array_get_data_type(vtkUnsignedShortArray* sself) { return sself->GetDataType(); }
extern "C" unsigned short vtk_unsigned_short_array_get_value(vtkUnsignedShortArray* sself, long long id) { return sself->GetValue(id); }
extern "C" void vtk_unsigned_short_array_set_value(vtkUnsignedShortArray* sself, long long id, unsigned short value) { sself->SetValue(id, value); }
extern "C" bool vtk_unsigned_short_array_set_number_of_values(vtkUnsignedShortArray* sself, long long number) { return sself->SetNumberOfValues(number); }
extern "C" void vtk_unsigned_short_array_insert_value(vtkUnsignedShortArray* sself, long long id, unsigned short f) { sself->InsertValue(id, f); }
extern "C" long long vtk_unsigned_short_array_insert_next_value(vtkUnsignedShortArray* sself, unsigned short f) { return sself->InsertNextValue(f); }
extern "C" unsigned short vtk_unsigned_short_array_get_data_type_value_min(vtkUnsignedShortArray* sself) { return sself->GetDataTypeValueMin(); }
extern "C" unsigned short vtk_unsigned_short_array_get_data_type_value_max(vtkUnsignedShortArray* sself) { return sself->GetDataTypeValueMax(); }
extern "C" vtkVariantArray * vtkVariantArray_new () {return vtkVariantArray :: New () ;}
extern "C" void vtkVariantArray_destructor (vtkVariantArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_variant_array_allocate(vtkVariantArray* sself, long long sz, long long ext) { return sself->Allocate(sz, ext); }
extern "C" void vtk_variant_array_initialize(vtkVariantArray* sself) { sself->Initialize(); }
extern "C" int vtk_variant_array_get_data_type(vtkVariantArray* sself) { return sself->GetDataType(); }
extern "C" int vtk_variant_array_get_data_type_size(vtkVariantArray* sself) { return sself->GetDataTypeSize(); }
extern "C" int vtk_variant_array_get_element_component_size(vtkVariantArray* sself) { return sself->GetElementComponentSize(); }
extern "C" void vtk_variant_array_set_number_of_tuples(vtkVariantArray* sself, long long number) { sself->SetNumberOfTuples(number); }
extern "C" void* vtk_variant_array_get_void_pointer(vtkVariantArray* sself, long long id) { return sself->GetVoidPointer(id); }
extern "C" void vtk_variant_array_squeeze(vtkVariantArray* sself) { sself->Squeeze(); }
extern "C" int vtk_variant_array_resize(vtkVariantArray* sself, long long numTuples) { return sself->Resize(numTuples); }
extern "C" void vtk_variant_array_set_void_array(vtkVariantArray* sself, void* arr, long long size, int save) { sself->SetVoidArray(arr, size, save); }
extern "C" unsigned long vtk_variant_array_get_actual_memory_size(vtkVariantArray* sself) { return sself->GetActualMemorySize(); }
extern "C" int vtk_variant_array_is_numeric(vtkVariantArray* sself) { return sself->IsNumeric(); }
extern "C" long long vtk_variant_array_get_number_of_values(vtkVariantArray* sself) { return sself->GetNumberOfValues(); }
extern "C" void vtk_variant_array_data_changed(vtkVariantArray* sself) { sself->DataChanged(); }
extern "C" void vtk_variant_array_data_element_changed(vtkVariantArray* sself, long long id) { sself->DataElementChanged(id); }
extern "C" void vtk_variant_array_clear_lookup(vtkVariantArray* sself) { sself->ClearLookup(); }
extern "C" vtkVersion * vtkVersion_new () {return vtkVersion :: New () ;}
extern "C" void vtkVersion_destructor (vtkVersion * sself) {sself -> Delete () ; return ;}
extern "C" const char* vtk_version_get_vtk_version(vtkVersion* sself) { return sself->GetVTKVersion(); }
extern "C" const char* vtk_version_get_vtk_version_full(vtkVersion* sself) { return sself->GetVTKVersionFull(); }
extern "C" int vtk_version_get_vtk_major_version(vtkVersion* sself) { return sself->GetVTKMajorVersion(); }
extern "C" int vtk_version_get_vtk_minor_version(vtkVersion* sself) { return sself->GetVTKMinorVersion(); }
extern "C" int vtk_version_get_vtk_build_version(vtkVersion* sself) { return sself->GetVTKBuildVersion(); }
extern "C" const char* vtk_version_get_vtk_source_version(vtkVersion* sself) { return sself->GetVTKSourceVersion(); }
extern "C" vtkVoidArray * vtkVoidArray_new () {return vtkVoidArray :: New () ;}
extern "C" void vtkVoidArray_destructor (vtkVoidArray * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_void_array_allocate(vtkVoidArray* sself, long long sz, long long ext) { return sself->Allocate(sz, ext); }
extern "C" void vtk_void_array_initialize(vtkVoidArray* sself) { sself->Initialize(); }
extern "C" int vtk_void_array_get_data_type(vtkVoidArray* sself) { return sself->GetDataType(); }
extern "C" int vtk_void_array_get_data_type_size(vtkVoidArray* sself) { return sself->GetDataTypeSize(); }
extern "C" void vtk_void_array_set_number_of_pointers(vtkVoidArray* sself, long long number) { sself->SetNumberOfPointers(number); }
extern "C" long long vtk_void_array_get_number_of_pointers(vtkVoidArray* sself) { return sself->GetNumberOfPointers(); }
extern "C" void* vtk_void_array_get_void_pointer(vtkVoidArray* sself, long long id) { return sself->GetVoidPointer(id); }
extern "C" void vtk_void_array_set_void_pointer(vtkVoidArray* sself, long long id, void* ptr) { sself->SetVoidPointer(id, ptr); }
extern "C" void vtk_void_array_insert_void_pointer(vtkVoidArray* sself, long long i, void* ptr) { sself->InsertVoidPointer(i, ptr); }
extern "C" long long vtk_void_array_insert_next_void_pointer(vtkVoidArray* sself, void* tuple) { return sself->InsertNextVoidPointer(tuple); }
extern "C" void vtk_void_array_reset(vtkVoidArray* sself) { sself->Reset(); }
extern "C" void vtk_void_array_squeeze(vtkVoidArray* sself) { sself->Squeeze(); }
extern "C" vtkWeakReference * vtkWeakReference_new () {return vtkWeakReference :: New () ;}
extern "C" void vtkWeakReference_destructor (vtkWeakReference * sself) {sself -> Delete () ; return ;}
extern "C" vtkXMLFileOutputWindow * vtkXMLFileOutputWindow_new () {return vtkXMLFileOutputWindow :: New () ;}
extern "C" void vtkXMLFileOutputWindow_destructor (vtkXMLFileOutputWindow * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_xml_file_output_window_display_text(vtkXMLFileOutputWindow* sself, const char* p0) { sself->DisplayText(p0); }
extern "C" void vtk_xml_file_output_window_display_tag(vtkXMLFileOutputWindow* sself, const char* p0) { sself->DisplayTag(p0); }
