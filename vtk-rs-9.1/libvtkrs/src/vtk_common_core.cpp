// Include header file
#include <vtk_common_core.h>

// Default include in all modules
#include <vtkNew.h>
#include <vtkObjectBase.h>

// Include objects of this module
#include <vtkAOSDataArrayTemplate.h>
#include <vtkAbstractArray.h>
#include <vtkAffineCharArray.h>
#include <vtkAffineDoubleArray.h>
#include <vtkAffineFloatArray.h>
#include <vtkAffineIdTypeArray.h>
#include <vtkAffineIntArray.h>
#include <vtkAffineLongArray.h>
#include <vtkAffineLongLongArray.h>
#include <vtkAffineShortArray.h>
#include <vtkAffineSignedCharArray.h>
#include <vtkAffineTypeFloat32Array.h>
#include <vtkAffineTypeFloat64Array.h>
#include <vtkAffineTypeInt16Array.h>
#include <vtkAffineTypeInt32Array.h>
#include <vtkAffineTypeInt64Array.h>
#include <vtkAffineTypeInt8Array.h>
#include <vtkAffineTypeUInt16Array.h>
#include <vtkAffineTypeUInt32Array.h>
#include <vtkAffineTypeUInt64Array.h>
#include <vtkAffineTypeUInt8Array.h>
#include <vtkAffineUnsignedCharArray.h>
#include <vtkAffineUnsignedIntArray.h>
#include <vtkAffineUnsignedLongArray.h>
#include <vtkAffineUnsignedLongLongArray.h>
#include <vtkAffineUnsignedShortArray.h>
#include <vtkAnimationCue.h>
#include <vtkArchiver.h>
#include <vtkArray.h>
#include <vtkArrayCoordinates.h>
#include <vtkArrayExtents.h>
#include <vtkArrayExtentsList.h>
#include <vtkArrayIterator.h>
#include <vtkArrayIteratorTemplate.h>
#include <vtkArrayRange.h>
#include <vtkArraySort.h>
#include <vtkArrayWeights.h>
#include <vtkAtomicMutex.h>
#include <vtkBatch.h>
#include <vtkBitArray.h>
#include <vtkBitArrayIterator.h>
#include <vtkBoxMuellerRandomSequence.h>
#include <vtkBreakPoint.h>
#include <vtkBuffer.h>
#include <vtkByteSwap.h>
#include <vtkCallbackCommand.h>
#include <vtkCharArray.h>
#include <vtkCollection.h>
#include <vtkCollectionIterator.h>
#include <vtkCommand.h>
#include <vtkCommonInformationKeyManager.h>
#include <vtkCompositeCharArray.h>
#include <vtkCompositeDoubleArray.h>
#include <vtkCompositeFloatArray.h>
#include <vtkCompositeIdTypeArray.h>
#include <vtkCompositeIntArray.h>
#include <vtkCompositeLongArray.h>
#include <vtkCompositeLongLongArray.h>
#include <vtkCompositeShortArray.h>
#include <vtkCompositeSignedCharArray.h>
#include <vtkCompositeTypeFloat32Array.h>
#include <vtkCompositeTypeFloat64Array.h>
#include <vtkCompositeTypeInt16Array.h>
#include <vtkCompositeTypeInt32Array.h>
#include <vtkCompositeTypeInt64Array.h>
#include <vtkCompositeTypeInt8Array.h>
#include <vtkCompositeTypeUInt16Array.h>
#include <vtkCompositeTypeUInt32Array.h>
#include <vtkCompositeTypeUInt64Array.h>
#include <vtkCompositeTypeUInt8Array.h>
#include <vtkCompositeUnsignedCharArray.h>
#include <vtkCompositeUnsignedIntArray.h>
#include <vtkCompositeUnsignedLongArray.h>
#include <vtkCompositeUnsignedLongLongArray.h>
#include <vtkCompositeUnsignedShortArray.h>
#include <vtkConstantCharArray.h>
#include <vtkConstantDoubleArray.h>
#include <vtkConstantFloatArray.h>
#include <vtkConstantIdTypeArray.h>
#include <vtkConstantIntArray.h>
#include <vtkConstantLongArray.h>
#include <vtkConstantLongLongArray.h>
#include <vtkConstantShortArray.h>
#include <vtkConstantSignedCharArray.h>
#include <vtkConstantTypeFloat32Array.h>
#include <vtkConstantTypeFloat64Array.h>
#include <vtkConstantTypeInt16Array.h>
#include <vtkConstantTypeInt32Array.h>
#include <vtkConstantTypeInt64Array.h>
#include <vtkConstantTypeInt8Array.h>
#include <vtkConstantTypeUInt16Array.h>
#include <vtkConstantTypeUInt32Array.h>
#include <vtkConstantTypeUInt64Array.h>
#include <vtkConstantTypeUInt8Array.h>
#include <vtkConstantUnsignedCharArray.h>
#include <vtkConstantUnsignedIntArray.h>
#include <vtkConstantUnsignedLongArray.h>
#include <vtkConstantUnsignedLongLongArray.h>
#include <vtkConstantUnsignedShortArray.h>
#include <vtkDataArray.h>
#include <vtkDataArrayCollection.h>
#include <vtkDataArrayCollectionIterator.h>
#include <vtkDataArraySelection.h>
#include <vtkDebugLeaks.h>
#include <vtkDebugLeaksManager.h>
#include <vtkDenseArray.h>
#include <vtkDeserializer.h>
#include <vtkDoubleArray.h>
#include <vtkDynamicLoader.h>
#include <vtkEventData.h>
#include <vtkEventForwarderCommand.h>
#include <vtkFileOutputWindow.h>
#include <vtkFloatArray.h>
#include <vtkFloatingPointExceptions.h>
#include <vtkGarbageCollector.h>
#include <vtkGarbageCollectorManager.h>
#include <vtkGaussianRandomSequence.h>
#include <vtkGenericDataArray.h>
#include <vtkGenericDataArrayLookupHelper.h>
#include <vtkIdList.h>
#include <vtkIdListCollection.h>
#include <vtkIdTypeArray.h>
#include <vtkIndent.h>
#include <vtkIndexedCharArray.h>
#include <vtkIndexedDoubleArray.h>
#include <vtkIndexedFloatArray.h>
#include <vtkIndexedIdTypeArray.h>
#include <vtkIndexedIntArray.h>
#include <vtkIndexedLongArray.h>
#include <vtkIndexedLongLongArray.h>
#include <vtkIndexedShortArray.h>
#include <vtkIndexedSignedCharArray.h>
#include <vtkIndexedTypeFloat32Array.h>
#include <vtkIndexedTypeFloat64Array.h>
#include <vtkIndexedTypeInt16Array.h>
#include <vtkIndexedTypeInt32Array.h>
#include <vtkIndexedTypeInt64Array.h>
#include <vtkIndexedTypeInt8Array.h>
#include <vtkIndexedTypeUInt16Array.h>
#include <vtkIndexedTypeUInt32Array.h>
#include <vtkIndexedTypeUInt64Array.h>
#include <vtkIndexedTypeUInt8Array.h>
#include <vtkIndexedUnsignedCharArray.h>
#include <vtkIndexedUnsignedIntArray.h>
#include <vtkIndexedUnsignedLongArray.h>
#include <vtkIndexedUnsignedLongLongArray.h>
#include <vtkIndexedUnsignedShortArray.h>
#include <vtkInformation.h>
#include <vtkInformationDataObjectKey.h>
#include <vtkInformationDoubleKey.h>
#include <vtkInformationDoubleVectorKey.h>
#include <vtkInformationIdTypeKey.h>
#include <vtkInformationInformationKey.h>
#include <vtkInformationInformationVectorKey.h>
#include <vtkInformationIntegerKey.h>
#include <vtkInformationIntegerPointerKey.h>
#include <vtkInformationIntegerVectorKey.h>
#include <vtkInformationInternals.h>
#include <vtkInformationIterator.h>
#include <vtkInformationKey.h>
#include <vtkInformationKeyLookup.h>
#include <vtkInformationKeyVectorKey.h>
#include <vtkInformationObjectBaseKey.h>
#include <vtkInformationObjectBaseVectorKey.h>
#include <vtkInformationRequestKey.h>
#include <vtkInformationStringKey.h>
#include <vtkInformationStringVectorKey.h>
#include <vtkInformationUnsignedLongKey.h>
#include <vtkInformationVariantKey.h>
#include <vtkInformationVariantVectorKey.h>
#include <vtkInformationVector.h>
#include <vtkIntArray.h>
#include <vtkInvoker.h>
#include <vtkLargeInteger.h>
#include <vtkLogger.h>
#include <vtkLongArray.h>
#include <vtkLongLongArray.h>
#include <vtkLookupTable.h>
#include <vtkMappedDataArray.h>
#include <vtkMarshalContext.h>
#include <vtkMath.h>
#include <vtkMersenneTwister.h>
#include <vtkMinimalStandardRandomSequence.h>
#include <vtkMultiThreader.h>
#include <vtkNew.h>
#include <vtkNumberToString.h>
#include <vtkOStrStreamWrapper.h>
#include <vtkObject.h>
#include <vtkObjectBase.h>
#include <vtkObjectFactory.h>
#include <vtkObjectFactoryCollection.h>
#include <vtkOldStyleCallbackCommand.h>
#include <vtkOutputWindow.h>
#include <vtkOverrideInformation.h>
#include <vtkOverrideInformationCollection.h>
#include <vtkPoints.h>
#include <vtkPoints2D.h>
#include <vtkPriorityQueue.h>
#include <vtkRandomPool.h>
#include <vtkRandomSequence.h>
#include <vtkReferenceCount.h>
#include <vtkSMPThreadLocal.h>
#include <vtkSMPThreadLocalObject.h>
#include <vtkSMPTools.h>
#include <vtkSOADataArrayTemplate.h>
#include <vtkSOATypeFloat32Array.h>
#include <vtkSOATypeFloat64Array.h>
#include <vtkSOATypeInt16Array.h>
#include <vtkSOATypeInt32Array.h>
#include <vtkSOATypeInt64Array.h>
#include <vtkSOATypeInt8Array.h>
#include <vtkSOATypeUInt16Array.h>
#include <vtkSOATypeUInt32Array.h>
#include <vtkSOATypeUInt64Array.h>
#include <vtkSOATypeUInt8Array.h>
#include <vtkScalarsToColors.h>
#include <vtkScaledSOADataArrayTemplate.h>
#include <vtkScaledSOATypeFloat32Array.h>
#include <vtkScaledSOATypeFloat64Array.h>
#include <vtkScaledSOATypeInt16Array.h>
#include <vtkScaledSOATypeInt32Array.h>
#include <vtkScaledSOATypeInt64Array.h>
#include <vtkScaledSOATypeInt8Array.h>
#include <vtkScaledSOATypeUInt16Array.h>
#include <vtkScaledSOATypeUInt32Array.h>
#include <vtkScaledSOATypeUInt64Array.h>
#include <vtkScaledSOATypeUInt8Array.h>
#include <vtkSerializer.h>
#include <vtkShortArray.h>
#include <vtkSignedCharArray.h>
#include <vtkSmartPointer.h>
#include <vtkSmartPointerBase.h>
#include <vtkSortDataArray.h>
#include <vtkSparseArray.h>
#include <vtkStdFunctionTypeFloat32Array.h>
#include <vtkStdFunctionTypeFloat64Array.h>
#include <vtkStdFunctionTypeInt16Array.h>
#include <vtkStdFunctionTypeInt32Array.h>
#include <vtkStdFunctionTypeInt64Array.h>
#include <vtkStdFunctionTypeInt8Array.h>
#include <vtkStdFunctionTypeUInt16Array.h>
#include <vtkStdFunctionTypeUInt32Array.h>
#include <vtkStdFunctionTypeUInt64Array.h>
#include <vtkStdFunctionTypeUInt8Array.h>
#include <vtkStdString.h>
#include <vtkStridedTypeFloat32Array.h>
#include <vtkStridedTypeFloat64Array.h>
#include <vtkStridedTypeInt16Array.h>
#include <vtkStridedTypeInt32Array.h>
#include <vtkStridedTypeInt64Array.h>
#include <vtkStridedTypeInt8Array.h>
#include <vtkStridedTypeUInt16Array.h>
#include <vtkStridedTypeUInt32Array.h>
#include <vtkStridedTypeUInt64Array.h>
#include <vtkStridedTypeUInt8Array.h>
#include <vtkStringArray.h>
#include <vtkStringOutputWindow.h>
#include <vtkStringToken.h>
#include <vtkTestDataArray.h>
#include <vtkTimePointUtility.h>
#include <vtkTimeStamp.h>
#include <vtkTypeFloat32Array.h>
#include <vtkTypeFloat64Array.h>
#include <vtkTypeInt16Array.h>
#include <vtkTypeInt32Array.h>
#include <vtkTypeInt64Array.h>
#include <vtkTypeInt8Array.h>
#include <vtkTypeUInt16Array.h>
#include <vtkTypeUInt32Array.h>
#include <vtkTypeUInt64Array.h>
#include <vtkTypeUInt8Array.h>
#include <vtkTypedArray.h>
#include <vtkTypedDataArray.h>
#include <vtkTypedDataArrayIterator.h>
#include <vtkUnsignedCharArray.h>
#include <vtkUnsignedIntArray.h>
#include <vtkUnsignedLongArray.h>
#include <vtkUnsignedLongLongArray.h>
#include <vtkUnsignedShortArray.h>
#include <vtkVariant.h>
#include <vtkVariantArray.h>
#include <vtkVersion.h>
#include <vtkVoidArray.h>
#include <vtkWeakPointer.h>
#include <vtkWeakPointerBase.h>
#include <vtkWeakReference.h>
#include <vtkWindow.h>
#include <vtkXMLFileOutputWindow.h>

// Implement declared functions
extern "C" vtkNew<vtkAffineCharArray> vtkAffineCharArray_new() {
    return vtkNew<vtkAffineCharArray>();
}
extern "C" void vtkAffineCharArray_destructor(vtkNew<vtkAffineCharArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineCharArray_get_ptr(vtkNew<vtkAffineCharArray> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_affine_char_array_get_data_type_value_min(vtkNew<vtkAffineCharArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_affine_char_array_get_data_type_value_max(vtkNew<vtkAffineCharArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_affine_char_array_get_value(vtkNew<vtkAffineCharArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_char_array_construct_backend(
    vtkNew<vtkAffineCharArray> self, char slope, char intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_char_array_get_number_of_values(vtkNew<vtkAffineCharArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineDoubleArray> vtkAffineDoubleArray_new() {
    return vtkNew<vtkAffineDoubleArray>();
}
extern "C" void vtkAffineDoubleArray_destructor(vtkNew<vtkAffineDoubleArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineDoubleArray_get_ptr(vtkNew<vtkAffineDoubleArray> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_affine_double_array_get_data_type_value_min(
    vtkNew<vtkAffineDoubleArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" double vtk_affine_double_array_get_data_type_value_max(
    vtkNew<vtkAffineDoubleArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" double vtk_affine_double_array_get_value(
    vtkNew<vtkAffineDoubleArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_double_array_construct_backend(
    vtkNew<vtkAffineDoubleArray> self, double slope, double intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_double_array_get_number_of_values(
    vtkNew<vtkAffineDoubleArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineFloatArray> vtkAffineFloatArray_new() {
    return vtkNew<vtkAffineFloatArray>();
}
extern "C" void vtkAffineFloatArray_destructor(vtkNew<vtkAffineFloatArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineFloatArray_get_ptr(vtkNew<vtkAffineFloatArray> sself) {
    return sself.GetPointer();
}
extern "C" float vtk_affine_float_array_get_data_type_value_min(vtkNew<vtkAffineFloatArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" float vtk_affine_float_array_get_data_type_value_max(vtkNew<vtkAffineFloatArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" float vtk_affine_float_array_get_value(vtkNew<vtkAffineFloatArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_float_array_construct_backend(
    vtkNew<vtkAffineFloatArray> self, float slope, float intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_float_array_get_number_of_values(vtkNew<vtkAffineFloatArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineIdTypeArray> vtkAffineIdTypeArray_new() {
    return vtkNew<vtkAffineIdTypeArray>();
}
extern "C" void vtkAffineIdTypeArray_destructor(vtkNew<vtkAffineIdTypeArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineIdTypeArray_get_ptr(vtkNew<vtkAffineIdTypeArray> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_affine_id_type_array_get_data_type_value_min(
    vtkNew<vtkAffineIdTypeArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_affine_id_type_array_get_data_type_value_max(
    vtkNew<vtkAffineIdTypeArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_affine_id_type_array_get_value(
    vtkNew<vtkAffineIdTypeArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_id_type_array_construct_backend(
    vtkNew<vtkAffineIdTypeArray> self, long long slope, long long intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_id_type_array_get_number_of_values(
    vtkNew<vtkAffineIdTypeArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineIntArray> vtkAffineIntArray_new() {
    return vtkNew<vtkAffineIntArray>();
}
extern "C" void vtkAffineIntArray_destructor(vtkNew<vtkAffineIntArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineIntArray_get_ptr(vtkNew<vtkAffineIntArray> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_affine_int_array_get_data_type_value_min(vtkNew<vtkAffineIntArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" int vtk_affine_int_array_get_data_type_value_max(vtkNew<vtkAffineIntArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" int vtk_affine_int_array_get_value(vtkNew<vtkAffineIntArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_int_array_construct_backend(
    vtkNew<vtkAffineIntArray> self, int slope, int intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_int_array_get_number_of_values(vtkNew<vtkAffineIntArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineLongArray> vtkAffineLongArray_new() {
    return vtkNew<vtkAffineLongArray>();
}
extern "C" void vtkAffineLongArray_destructor(vtkNew<vtkAffineLongArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineLongArray_get_ptr(vtkNew<vtkAffineLongArray> sself) {
    return sself.GetPointer();
}
extern "C" long vtk_affine_long_array_get_data_type_value_min(vtkNew<vtkAffineLongArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" long vtk_affine_long_array_get_data_type_value_max(vtkNew<vtkAffineLongArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" long vtk_affine_long_array_get_value(vtkNew<vtkAffineLongArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_long_array_construct_backend(
    vtkNew<vtkAffineLongArray> self, long slope, long intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_long_array_get_number_of_values(vtkNew<vtkAffineLongArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineLongLongArray> vtkAffineLongLongArray_new() {
    return vtkNew<vtkAffineLongLongArray>();
}
extern "C" void vtkAffineLongLongArray_destructor(vtkNew<vtkAffineLongLongArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineLongLongArray_get_ptr(vtkNew<vtkAffineLongLongArray> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_affine_long_long_array_get_data_type_value_min(
    vtkNew<vtkAffineLongLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_affine_long_long_array_get_data_type_value_max(
    vtkNew<vtkAffineLongLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_affine_long_long_array_get_value(
    vtkNew<vtkAffineLongLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_long_long_array_construct_backend(
    vtkNew<vtkAffineLongLongArray> self, long long slope, long long intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_long_long_array_get_number_of_values(
    vtkNew<vtkAffineLongLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineShortArray> vtkAffineShortArray_new() {
    return vtkNew<vtkAffineShortArray>();
}
extern "C" void vtkAffineShortArray_destructor(vtkNew<vtkAffineShortArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineShortArray_get_ptr(vtkNew<vtkAffineShortArray> sself) {
    return sself.GetPointer();
}
extern "C" short vtk_affine_short_array_get_data_type_value_min(vtkNew<vtkAffineShortArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" short vtk_affine_short_array_get_data_type_value_max(vtkNew<vtkAffineShortArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" short vtk_affine_short_array_get_value(vtkNew<vtkAffineShortArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_short_array_construct_backend(
    vtkNew<vtkAffineShortArray> self, short slope, short intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_short_array_get_number_of_values(vtkNew<vtkAffineShortArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineSignedCharArray> vtkAffineSignedCharArray_new() {
    return vtkNew<vtkAffineSignedCharArray>();
}
extern "C" void vtkAffineSignedCharArray_destructor(vtkNew<vtkAffineSignedCharArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineSignedCharArray_get_ptr(vtkNew<vtkAffineSignedCharArray> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_affine_signed_char_array_get_data_type_value_min(
    vtkNew<vtkAffineSignedCharArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_affine_signed_char_array_get_data_type_value_max(
    vtkNew<vtkAffineSignedCharArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_affine_signed_char_array_get_value(
    vtkNew<vtkAffineSignedCharArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_signed_char_array_construct_backend(
    vtkNew<vtkAffineSignedCharArray> self, char slope, char intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_signed_char_array_get_number_of_values(
    vtkNew<vtkAffineSignedCharArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineTypeFloat32Array> vtkAffineTypeFloat32Array_new() {
    return vtkNew<vtkAffineTypeFloat32Array>();
}
extern "C" void vtkAffineTypeFloat32Array_destructor(vtkNew<vtkAffineTypeFloat32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineTypeFloat32Array_get_ptr(vtkNew<vtkAffineTypeFloat32Array> sself) {
    return sself.GetPointer();
}
extern "C" float vtk_affine_type_float_32_array_get_data_type_value_min(
    vtkNew<vtkAffineTypeFloat32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" float vtk_affine_type_float_32_array_get_data_type_value_max(
    vtkNew<vtkAffineTypeFloat32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" float vtk_affine_type_float_32_array_get_value(
    vtkNew<vtkAffineTypeFloat32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_type_float_32_array_construct_backend(
    vtkNew<vtkAffineTypeFloat32Array> self, float slope, float intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_type_float_32_array_get_number_of_values(
    vtkNew<vtkAffineTypeFloat32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineTypeFloat64Array> vtkAffineTypeFloat64Array_new() {
    return vtkNew<vtkAffineTypeFloat64Array>();
}
extern "C" void vtkAffineTypeFloat64Array_destructor(vtkNew<vtkAffineTypeFloat64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineTypeFloat64Array_get_ptr(vtkNew<vtkAffineTypeFloat64Array> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_affine_type_float_64_array_get_data_type_value_min(
    vtkNew<vtkAffineTypeFloat64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" double vtk_affine_type_float_64_array_get_data_type_value_max(
    vtkNew<vtkAffineTypeFloat64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" double vtk_affine_type_float_64_array_get_value(
    vtkNew<vtkAffineTypeFloat64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_type_float_64_array_construct_backend(
    vtkNew<vtkAffineTypeFloat64Array> self, double slope, double intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_type_float_64_array_get_number_of_values(
    vtkNew<vtkAffineTypeFloat64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineTypeInt16Array> vtkAffineTypeInt16Array_new() {
    return vtkNew<vtkAffineTypeInt16Array>();
}
extern "C" void vtkAffineTypeInt16Array_destructor(vtkNew<vtkAffineTypeInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineTypeInt16Array_get_ptr(vtkNew<vtkAffineTypeInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" short vtk_affine_type_int_16_array_get_data_type_value_min(
    vtkNew<vtkAffineTypeInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" short vtk_affine_type_int_16_array_get_data_type_value_max(
    vtkNew<vtkAffineTypeInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" short vtk_affine_type_int_16_array_get_value(
    vtkNew<vtkAffineTypeInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_type_int_16_array_construct_backend(
    vtkNew<vtkAffineTypeInt16Array> self, short slope, short intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_type_int_16_array_get_number_of_values(
    vtkNew<vtkAffineTypeInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineTypeInt32Array> vtkAffineTypeInt32Array_new() {
    return vtkNew<vtkAffineTypeInt32Array>();
}
extern "C" void vtkAffineTypeInt32Array_destructor(vtkNew<vtkAffineTypeInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineTypeInt32Array_get_ptr(vtkNew<vtkAffineTypeInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_affine_type_int_32_array_get_data_type_value_min(
    vtkNew<vtkAffineTypeInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" int vtk_affine_type_int_32_array_get_data_type_value_max(
    vtkNew<vtkAffineTypeInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" int vtk_affine_type_int_32_array_get_value(
    vtkNew<vtkAffineTypeInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_type_int_32_array_construct_backend(
    vtkNew<vtkAffineTypeInt32Array> self, int slope, int intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_type_int_32_array_get_number_of_values(
    vtkNew<vtkAffineTypeInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineTypeInt64Array> vtkAffineTypeInt64Array_new() {
    return vtkNew<vtkAffineTypeInt64Array>();
}
extern "C" void vtkAffineTypeInt64Array_destructor(vtkNew<vtkAffineTypeInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineTypeInt64Array_get_ptr(vtkNew<vtkAffineTypeInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_affine_type_int_64_array_get_data_type_value_min(
    vtkNew<vtkAffineTypeInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_affine_type_int_64_array_get_data_type_value_max(
    vtkNew<vtkAffineTypeInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_affine_type_int_64_array_get_value(
    vtkNew<vtkAffineTypeInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_type_int_64_array_construct_backend(
    vtkNew<vtkAffineTypeInt64Array> self, long long slope, long long intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_type_int_64_array_get_number_of_values(
    vtkNew<vtkAffineTypeInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineTypeInt8Array> vtkAffineTypeInt8Array_new() {
    return vtkNew<vtkAffineTypeInt8Array>();
}
extern "C" void vtkAffineTypeInt8Array_destructor(vtkNew<vtkAffineTypeInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineTypeInt8Array_get_ptr(vtkNew<vtkAffineTypeInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_affine_type_int_8_array_get_data_type_value_min(
    vtkNew<vtkAffineTypeInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_affine_type_int_8_array_get_data_type_value_max(
    vtkNew<vtkAffineTypeInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_affine_type_int_8_array_get_value(
    vtkNew<vtkAffineTypeInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_type_int_8_array_construct_backend(
    vtkNew<vtkAffineTypeInt8Array> self, char slope, char intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_type_int_8_array_get_number_of_values(
    vtkNew<vtkAffineTypeInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineTypeUInt16Array> vtkAffineTypeUInt16Array_new() {
    return vtkNew<vtkAffineTypeUInt16Array>();
}
extern "C" void vtkAffineTypeUInt16Array_destructor(vtkNew<vtkAffineTypeUInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineTypeUInt16Array_get_ptr(vtkNew<vtkAffineTypeUInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned short vtk_affine_type_u_int_16_array_get_data_type_value_min(
    vtkNew<vtkAffineTypeUInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned short vtk_affine_type_u_int_16_array_get_data_type_value_max(
    vtkNew<vtkAffineTypeUInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned short vtk_affine_type_u_int_16_array_get_value(
    vtkNew<vtkAffineTypeUInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_type_u_int_16_array_construct_backend(
    vtkNew<vtkAffineTypeUInt16Array> self, unsigned short slope, unsigned short intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_type_u_int_16_array_get_number_of_values(
    vtkNew<vtkAffineTypeUInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineTypeUInt32Array> vtkAffineTypeUInt32Array_new() {
    return vtkNew<vtkAffineTypeUInt32Array>();
}
extern "C" void vtkAffineTypeUInt32Array_destructor(vtkNew<vtkAffineTypeUInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineTypeUInt32Array_get_ptr(vtkNew<vtkAffineTypeUInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned int vtk_affine_type_u_int_32_array_get_data_type_value_min(
    vtkNew<vtkAffineTypeUInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned int vtk_affine_type_u_int_32_array_get_data_type_value_max(
    vtkNew<vtkAffineTypeUInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned int vtk_affine_type_u_int_32_array_get_value(
    vtkNew<vtkAffineTypeUInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_type_u_int_32_array_construct_backend(
    vtkNew<vtkAffineTypeUInt32Array> self, unsigned int slope, unsigned int intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_type_u_int_32_array_get_number_of_values(
    vtkNew<vtkAffineTypeUInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineTypeUInt64Array> vtkAffineTypeUInt64Array_new() {
    return vtkNew<vtkAffineTypeUInt64Array>();
}
extern "C" void vtkAffineTypeUInt64Array_destructor(vtkNew<vtkAffineTypeUInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineTypeUInt64Array_get_ptr(vtkNew<vtkAffineTypeUInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long long vtk_affine_type_u_int_64_array_get_data_type_value_min(
    vtkNew<vtkAffineTypeUInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long long vtk_affine_type_u_int_64_array_get_data_type_value_max(
    vtkNew<vtkAffineTypeUInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long long vtk_affine_type_u_int_64_array_get_value(
    vtkNew<vtkAffineTypeUInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_type_u_int_64_array_construct_backend(
    vtkNew<vtkAffineTypeUInt64Array> self, unsigned long long slope, unsigned long long intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_type_u_int_64_array_get_number_of_values(
    vtkNew<vtkAffineTypeUInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineTypeUInt8Array> vtkAffineTypeUInt8Array_new() {
    return vtkNew<vtkAffineTypeUInt8Array>();
}
extern "C" void vtkAffineTypeUInt8Array_destructor(vtkNew<vtkAffineTypeUInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineTypeUInt8Array_get_ptr(vtkNew<vtkAffineTypeUInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned char vtk_affine_type_u_int_8_array_get_data_type_value_min(
    vtkNew<vtkAffineTypeUInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned char vtk_affine_type_u_int_8_array_get_data_type_value_max(
    vtkNew<vtkAffineTypeUInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned char vtk_affine_type_u_int_8_array_get_value(
    vtkNew<vtkAffineTypeUInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_type_u_int_8_array_construct_backend(
    vtkNew<vtkAffineTypeUInt8Array> self, unsigned char slope, unsigned char intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_type_u_int_8_array_get_number_of_values(
    vtkNew<vtkAffineTypeUInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineUnsignedCharArray> vtkAffineUnsignedCharArray_new() {
    return vtkNew<vtkAffineUnsignedCharArray>();
}
extern "C" void vtkAffineUnsignedCharArray_destructor(vtkNew<vtkAffineUnsignedCharArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineUnsignedCharArray_get_ptr(vtkNew<vtkAffineUnsignedCharArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned char vtk_affine_unsigned_char_array_get_data_type_value_min(
    vtkNew<vtkAffineUnsignedCharArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned char vtk_affine_unsigned_char_array_get_data_type_value_max(
    vtkNew<vtkAffineUnsignedCharArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned char vtk_affine_unsigned_char_array_get_value(
    vtkNew<vtkAffineUnsignedCharArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_unsigned_char_array_construct_backend(
    vtkNew<vtkAffineUnsignedCharArray> self, unsigned char slope, unsigned char intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_unsigned_char_array_get_number_of_values(
    vtkNew<vtkAffineUnsignedCharArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineUnsignedIntArray> vtkAffineUnsignedIntArray_new() {
    return vtkNew<vtkAffineUnsignedIntArray>();
}
extern "C" void vtkAffineUnsignedIntArray_destructor(vtkNew<vtkAffineUnsignedIntArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineUnsignedIntArray_get_ptr(vtkNew<vtkAffineUnsignedIntArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned int vtk_affine_unsigned_int_array_get_data_type_value_min(
    vtkNew<vtkAffineUnsignedIntArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned int vtk_affine_unsigned_int_array_get_data_type_value_max(
    vtkNew<vtkAffineUnsignedIntArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned int vtk_affine_unsigned_int_array_get_value(
    vtkNew<vtkAffineUnsignedIntArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_unsigned_int_array_construct_backend(
    vtkNew<vtkAffineUnsignedIntArray> self, unsigned int slope, unsigned int intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_unsigned_int_array_get_number_of_values(
    vtkNew<vtkAffineUnsignedIntArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineUnsignedLongArray> vtkAffineUnsignedLongArray_new() {
    return vtkNew<vtkAffineUnsignedLongArray>();
}
extern "C" void vtkAffineUnsignedLongArray_destructor(vtkNew<vtkAffineUnsignedLongArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineUnsignedLongArray_get_ptr(vtkNew<vtkAffineUnsignedLongArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long vtk_affine_unsigned_long_array_get_data_type_value_min(
    vtkNew<vtkAffineUnsignedLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long vtk_affine_unsigned_long_array_get_data_type_value_max(
    vtkNew<vtkAffineUnsignedLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long vtk_affine_unsigned_long_array_get_value(
    vtkNew<vtkAffineUnsignedLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_unsigned_long_array_construct_backend(
    vtkNew<vtkAffineUnsignedLongArray> self, unsigned long slope, unsigned long intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_unsigned_long_array_get_number_of_values(
    vtkNew<vtkAffineUnsignedLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineUnsignedLongLongArray> vtkAffineUnsignedLongLongArray_new() {
    return vtkNew<vtkAffineUnsignedLongLongArray>();
}
extern "C" void vtkAffineUnsignedLongLongArray_destructor(
    vtkNew<vtkAffineUnsignedLongLongArray> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineUnsignedLongLongArray_get_ptr(
    vtkNew<vtkAffineUnsignedLongLongArray> sself
) {
    return sself.GetPointer();
}
extern "C" unsigned long long vtk_affine_unsigned_long_long_array_get_data_type_value_min(
    vtkNew<vtkAffineUnsignedLongLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long long vtk_affine_unsigned_long_long_array_get_data_type_value_max(
    vtkNew<vtkAffineUnsignedLongLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long long vtk_affine_unsigned_long_long_array_get_value(
    vtkNew<vtkAffineUnsignedLongLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_unsigned_long_long_array_construct_backend(
    vtkNew<vtkAffineUnsignedLongLongArray> self,
    unsigned long long slope,
    unsigned long long intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_unsigned_long_long_array_get_number_of_values(
    vtkNew<vtkAffineUnsignedLongLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAffineUnsignedShortArray> vtkAffineUnsignedShortArray_new() {
    return vtkNew<vtkAffineUnsignedShortArray>();
}
extern "C" void vtkAffineUnsignedShortArray_destructor(vtkNew<vtkAffineUnsignedShortArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAffineUnsignedShortArray_get_ptr(vtkNew<vtkAffineUnsignedShortArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned short vtk_affine_unsigned_short_array_get_data_type_value_min(
    vtkNew<vtkAffineUnsignedShortArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned short vtk_affine_unsigned_short_array_get_data_type_value_max(
    vtkNew<vtkAffineUnsignedShortArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned short vtk_affine_unsigned_short_array_get_value(
    vtkNew<vtkAffineUnsignedShortArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_affine_unsigned_short_array_construct_backend(
    vtkNew<vtkAffineUnsignedShortArray> self, unsigned short slope, unsigned short intercept
) {
    return self->ConstructBackend(slope, intercept);
}
extern "C" long long vtk_affine_unsigned_short_array_get_number_of_values(
    vtkNew<vtkAffineUnsignedShortArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkAnimationCue> vtkAnimationCue_new() {
    return vtkNew<vtkAnimationCue>();
}
extern "C" void vtkAnimationCue_destructor(vtkNew<vtkAnimationCue> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAnimationCue_get_ptr(vtkNew<vtkAnimationCue> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_animation_cue_set_time_mode(vtkNew<vtkAnimationCue> self, int mode) {
    return self->SetTimeMode(mode);
}
extern "C" int vtk_animation_cue_get_time_mode(vtkNew<vtkAnimationCue> self) {
    return self->GetTimeMode();
}
extern "C" void vtk_animation_cue_set_time_mode_to_relative(vtkNew<vtkAnimationCue> self) {
    return self->SetTimeModeToRelative();
}
extern "C" void vtk_animation_cue_set_time_mode_to_normalized(vtkNew<vtkAnimationCue> self) {
    return self->SetTimeModeToNormalized();
}
extern "C" void vtk_animation_cue_set_start_time(vtkNew<vtkAnimationCue> self, double _arg) {
    return self->SetStartTime(_arg);
}
extern "C" double vtk_animation_cue_get_start_time(vtkNew<vtkAnimationCue> self) {
    return self->GetStartTime();
}
extern "C" void vtk_animation_cue_set_end_time(vtkNew<vtkAnimationCue> self, double _arg) {
    return self->SetEndTime(_arg);
}
extern "C" double vtk_animation_cue_get_end_time(vtkNew<vtkAnimationCue> self) {
    return self->GetEndTime();
}
extern "C" void vtk_animation_cue_tick(
    vtkNew<vtkAnimationCue> self, double currenttime, double deltatime, double clocktime
) {
    return self->Tick(currenttime, deltatime, clocktime);
}
extern "C" void vtk_animation_cue_initialize(vtkNew<vtkAnimationCue> self) {
    return self->Initialize();
}
extern "C" void vtk_animation_cue_finalize(vtkNew<vtkAnimationCue> self) {
    return self->Finalize();
}
extern "C" double vtk_animation_cue_get_animation_time(vtkNew<vtkAnimationCue> self) {
    return self->GetAnimationTime();
}
extern "C" double vtk_animation_cue_get_delta_time(vtkNew<vtkAnimationCue> self) {
    return self->GetDeltaTime();
}
extern "C" double vtk_animation_cue_get_clock_time(vtkNew<vtkAnimationCue> self) {
    return self->GetClockTime();
}
extern "C" vtkNew<vtkArchiver> vtkArchiver_new() {
    return vtkNew<vtkArchiver>();
}
extern "C" void vtkArchiver_destructor(vtkNew<vtkArchiver> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkArchiver_get_ptr(vtkNew<vtkArchiver> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_archiver_open_archive(vtkNew<vtkArchiver> self) {
    return self->OpenArchive();
}
extern "C" void vtk_archiver_close_archive(vtkNew<vtkArchiver> self) {
    return self->CloseArchive();
}
extern "C" vtkNew<vtkBitArray> vtkBitArray_new() {
    return vtkNew<vtkBitArray>();
}
extern "C" void vtkBitArray_destructor(vtkNew<vtkBitArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBitArray_get_ptr(vtkNew<vtkBitArray> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_bit_array_allocate(vtkNew<vtkBitArray> self, long long sz, long long ext) {
    return self->Allocate(sz, ext);
}
extern "C" void vtk_bit_array_initialize(vtkNew<vtkBitArray> self) {
    return self->Initialize();
}
extern "C" int vtk_bit_array_get_data_type_size(vtkNew<vtkBitArray> self) {
    return self->GetDataTypeSize();
}
extern "C" void vtk_bit_array_set_number_of_tuples(vtkNew<vtkBitArray> self, long long number) {
    return self->SetNumberOfTuples(number);
}
extern "C" bool vtk_bit_array_set_number_of_values(vtkNew<vtkBitArray> self, long long number) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_bit_array_remove_tuple(vtkNew<vtkBitArray> self, long long id) {
    return self->RemoveTuple(id);
}
extern "C" void vtk_bit_array_set_component(
    vtkNew<vtkBitArray> self, long long i, int j, double c
) {
    return self->SetComponent(i, j, c);
}
extern "C" void vtk_bit_array_squeeze(vtkNew<vtkBitArray> self) {
    return self->Squeeze();
}
extern "C" int vtk_bit_array_resize(vtkNew<vtkBitArray> self, long long numTuples) {
    return self->Resize(numTuples);
}
extern "C" int vtk_bit_array_get_value(vtkNew<vtkBitArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_bit_array_set_value(vtkNew<vtkBitArray> self, long long id, int value) {
    return self->SetValue(id, value);
}
extern "C" void vtk_bit_array_insert_value(vtkNew<vtkBitArray> self, long long id, int i) {
    return self->InsertValue(id, i);
}
extern "C" long long vtk_bit_array_insert_next_value(vtkNew<vtkBitArray> self, int i) {
    return self->InsertNextValue(i);
}
extern "C" void vtk_bit_array_insert_component(
    vtkNew<vtkBitArray> self, long long i, int j, double c
) {
    return self->InsertComponent(i, j, c);
}
extern "C" long long vtk_bit_array_lookup_value_c_int(vtkNew<vtkBitArray> self, int value) {
    return self->LookupValue(value);
}
extern "C" void vtk_bit_array_data_changed(vtkNew<vtkBitArray> self) {
    return self->DataChanged();
}
extern "C" void vtk_bit_array_clear_lookup(vtkNew<vtkBitArray> self) {
    return self->ClearLookup();
}
extern "C" long long vtk_bit_array_get_number_of_values(vtkNew<vtkBitArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkBitArrayIterator> vtkBitArrayIterator_new() {
    return vtkNew<vtkBitArrayIterator>();
}
extern "C" void vtkBitArrayIterator_destructor(vtkNew<vtkBitArrayIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBitArrayIterator_get_ptr(vtkNew<vtkBitArrayIterator> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_bit_array_iterator_get_value(vtkNew<vtkBitArrayIterator> self, long long id) {
    return self->GetValue(id);
}
extern "C" long long vtk_bit_array_iterator_get_number_of_tuples(vtkNew<vtkBitArrayIterator> self) {
    return self->GetNumberOfTuples();
}
extern "C" long long vtk_bit_array_iterator_get_number_of_values(vtkNew<vtkBitArrayIterator> self) {
    return self->GetNumberOfValues();
}
extern "C" int vtk_bit_array_iterator_get_number_of_components(vtkNew<vtkBitArrayIterator> self) {
    return self->GetNumberOfComponents();
}
extern "C" int vtk_bit_array_iterator_get_data_type(vtkNew<vtkBitArrayIterator> self) {
    return self->GetDataType();
}
extern "C" int vtk_bit_array_iterator_get_data_type_size(vtkNew<vtkBitArrayIterator> self) {
    return self->GetDataTypeSize();
}
extern "C" void vtk_bit_array_iterator_set_value(
    vtkNew<vtkBitArrayIterator> self, long long id, int value
) {
    return self->SetValue(id, value);
}
extern "C" vtkNew<vtkBoxMuellerRandomSequence> vtkBoxMuellerRandomSequence_new() {
    return vtkNew<vtkBoxMuellerRandomSequence>();
}
extern "C" void vtkBoxMuellerRandomSequence_destructor(vtkNew<vtkBoxMuellerRandomSequence> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkBoxMuellerRandomSequence_get_ptr(vtkNew<vtkBoxMuellerRandomSequence> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_box_mueller_random_sequence_initialize(
    vtkNew<vtkBoxMuellerRandomSequence> self, unsigned int seed
) {
    return self->Initialize(seed);
}
extern "C" double vtk_box_mueller_random_sequence_get_value(
    vtkNew<vtkBoxMuellerRandomSequence> self
) {
    return self->GetValue();
}
extern "C" void vtk_box_mueller_random_sequence_next(vtkNew<vtkBoxMuellerRandomSequence> self) {
    return self->Next();
}
extern "C" vtkNew<vtkByteSwap> vtkByteSwap_new() {
    return vtkNew<vtkByteSwap>();
}
extern "C" void vtkByteSwap_destructor(vtkNew<vtkByteSwap> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkByteSwap_get_ptr(vtkNew<vtkByteSwap> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkCallbackCommand> vtkCallbackCommand_new() {
    return vtkNew<vtkCallbackCommand>();
}
extern "C" void vtkCallbackCommand_destructor(vtkNew<vtkCallbackCommand> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCallbackCommand_get_ptr(vtkNew<vtkCallbackCommand> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_callback_command_set_abort_flag_on_execute(
    vtkNew<vtkCallbackCommand> self, int f
) {
    return self->SetAbortFlagOnExecute(f);
}
extern "C" int vtk_callback_command_get_abort_flag_on_execute(vtkNew<vtkCallbackCommand> self) {
    return self->GetAbortFlagOnExecute();
}
extern "C" void vtk_callback_command_abort_flag_on_execute_on(vtkNew<vtkCallbackCommand> self) {
    return self->AbortFlagOnExecuteOn();
}
extern "C" void vtk_callback_command_abort_flag_on_execute_off(vtkNew<vtkCallbackCommand> self) {
    return self->AbortFlagOnExecuteOff();
}
extern "C" vtkNew<vtkCharArray> vtkCharArray_new() {
    return vtkNew<vtkCharArray>();
}
extern "C" void vtkCharArray_destructor(vtkNew<vtkCharArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCharArray_get_ptr(vtkNew<vtkCharArray> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_char_array_get_data_type_value_min(vtkNew<vtkCharArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_char_array_get_data_type_value_max(vtkNew<vtkCharArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_char_array_get_value(vtkNew<vtkCharArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_char_array_set_value(vtkNew<vtkCharArray> self, long long id, char value) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_char_array_set_number_of_values(vtkNew<vtkCharArray> self, long long number) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_char_array_insert_value(vtkNew<vtkCharArray> self, long long id, char f) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_char_array_insert_next_value(vtkNew<vtkCharArray> self, char f) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_char_array_get_number_of_values(vtkNew<vtkCharArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCollection> vtkCollection_new() {
    return vtkNew<vtkCollection>();
}
extern "C" void vtkCollection_destructor(vtkNew<vtkCollection> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCollection_get_ptr(vtkNew<vtkCollection> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_collection_remove_item_c_int(vtkNew<vtkCollection> self, int i) {
    return self->RemoveItem(i);
}
extern "C" void vtk_collection_remove_all_items(vtkNew<vtkCollection> self) {
    return self->RemoveAllItems();
}
extern "C" int vtk_collection_get_number_of_items(vtkNew<vtkCollection> self) {
    return self->GetNumberOfItems();
}
extern "C" void vtk_collection_init_traversal(vtkNew<vtkCollection> self) {
    return self->InitTraversal();
}
extern "C" bool vtk_collection_uses_garbage_collector(vtkNew<vtkCollection> self) {
    return self->UsesGarbageCollector();
}
extern "C" vtkNew<vtkCollectionIterator> vtkCollectionIterator_new() {
    return vtkNew<vtkCollectionIterator>();
}
extern "C" void vtkCollectionIterator_destructor(vtkNew<vtkCollectionIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCollectionIterator_get_ptr(vtkNew<vtkCollectionIterator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_collection_iterator_init_traversal(vtkNew<vtkCollectionIterator> self) {
    return self->InitTraversal();
}
extern "C" void vtk_collection_iterator_go_to_first_item(vtkNew<vtkCollectionIterator> self) {
    return self->GoToFirstItem();
}
extern "C" void vtk_collection_iterator_go_to_next_item(vtkNew<vtkCollectionIterator> self) {
    return self->GoToNextItem();
}
extern "C" int vtk_collection_iterator_is_done_with_traversal(vtkNew<vtkCollectionIterator> self) {
    return self->IsDoneWithTraversal();
}
extern "C" vtkNew<vtkCompositeCharArray> vtkCompositeCharArray_new() {
    return vtkNew<vtkCompositeCharArray>();
}
extern "C" void vtkCompositeCharArray_destructor(vtkNew<vtkCompositeCharArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeCharArray_get_ptr(vtkNew<vtkCompositeCharArray> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_composite_char_array_get_data_type_value_min(
    vtkNew<vtkCompositeCharArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_composite_char_array_get_data_type_value_max(
    vtkNew<vtkCompositeCharArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_composite_char_array_get_value(
    vtkNew<vtkCompositeCharArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_char_array_get_number_of_values(
    vtkNew<vtkCompositeCharArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeDoubleArray> vtkCompositeDoubleArray_new() {
    return vtkNew<vtkCompositeDoubleArray>();
}
extern "C" void vtkCompositeDoubleArray_destructor(vtkNew<vtkCompositeDoubleArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeDoubleArray_get_ptr(vtkNew<vtkCompositeDoubleArray> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_composite_double_array_get_data_type_value_min(
    vtkNew<vtkCompositeDoubleArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" double vtk_composite_double_array_get_data_type_value_max(
    vtkNew<vtkCompositeDoubleArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" double vtk_composite_double_array_get_value(
    vtkNew<vtkCompositeDoubleArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_double_array_get_number_of_values(
    vtkNew<vtkCompositeDoubleArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeFloatArray> vtkCompositeFloatArray_new() {
    return vtkNew<vtkCompositeFloatArray>();
}
extern "C" void vtkCompositeFloatArray_destructor(vtkNew<vtkCompositeFloatArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeFloatArray_get_ptr(vtkNew<vtkCompositeFloatArray> sself) {
    return sself.GetPointer();
}
extern "C" float vtk_composite_float_array_get_data_type_value_min(
    vtkNew<vtkCompositeFloatArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" float vtk_composite_float_array_get_data_type_value_max(
    vtkNew<vtkCompositeFloatArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" float vtk_composite_float_array_get_value(
    vtkNew<vtkCompositeFloatArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_float_array_get_number_of_values(
    vtkNew<vtkCompositeFloatArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeIdTypeArray> vtkCompositeIdTypeArray_new() {
    return vtkNew<vtkCompositeIdTypeArray>();
}
extern "C" void vtkCompositeIdTypeArray_destructor(vtkNew<vtkCompositeIdTypeArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeIdTypeArray_get_ptr(vtkNew<vtkCompositeIdTypeArray> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_composite_id_type_array_get_data_type_value_min(
    vtkNew<vtkCompositeIdTypeArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_composite_id_type_array_get_data_type_value_max(
    vtkNew<vtkCompositeIdTypeArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_composite_id_type_array_get_value(
    vtkNew<vtkCompositeIdTypeArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_id_type_array_get_number_of_values(
    vtkNew<vtkCompositeIdTypeArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeIntArray> vtkCompositeIntArray_new() {
    return vtkNew<vtkCompositeIntArray>();
}
extern "C" void vtkCompositeIntArray_destructor(vtkNew<vtkCompositeIntArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeIntArray_get_ptr(vtkNew<vtkCompositeIntArray> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_composite_int_array_get_data_type_value_min(vtkNew<vtkCompositeIntArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" int vtk_composite_int_array_get_data_type_value_max(vtkNew<vtkCompositeIntArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" int vtk_composite_int_array_get_value(vtkNew<vtkCompositeIntArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_int_array_get_number_of_values(
    vtkNew<vtkCompositeIntArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeLongArray> vtkCompositeLongArray_new() {
    return vtkNew<vtkCompositeLongArray>();
}
extern "C" void vtkCompositeLongArray_destructor(vtkNew<vtkCompositeLongArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeLongArray_get_ptr(vtkNew<vtkCompositeLongArray> sself) {
    return sself.GetPointer();
}
extern "C" long vtk_composite_long_array_get_data_type_value_min(
    vtkNew<vtkCompositeLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long vtk_composite_long_array_get_data_type_value_max(
    vtkNew<vtkCompositeLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long vtk_composite_long_array_get_value(
    vtkNew<vtkCompositeLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_long_array_get_number_of_values(
    vtkNew<vtkCompositeLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeLongLongArray> vtkCompositeLongLongArray_new() {
    return vtkNew<vtkCompositeLongLongArray>();
}
extern "C" void vtkCompositeLongLongArray_destructor(vtkNew<vtkCompositeLongLongArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeLongLongArray_get_ptr(vtkNew<vtkCompositeLongLongArray> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_composite_long_long_array_get_data_type_value_min(
    vtkNew<vtkCompositeLongLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_composite_long_long_array_get_data_type_value_max(
    vtkNew<vtkCompositeLongLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_composite_long_long_array_get_value(
    vtkNew<vtkCompositeLongLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_long_long_array_get_number_of_values(
    vtkNew<vtkCompositeLongLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeShortArray> vtkCompositeShortArray_new() {
    return vtkNew<vtkCompositeShortArray>();
}
extern "C" void vtkCompositeShortArray_destructor(vtkNew<vtkCompositeShortArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeShortArray_get_ptr(vtkNew<vtkCompositeShortArray> sself) {
    return sself.GetPointer();
}
extern "C" short vtk_composite_short_array_get_data_type_value_min(
    vtkNew<vtkCompositeShortArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" short vtk_composite_short_array_get_data_type_value_max(
    vtkNew<vtkCompositeShortArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" short vtk_composite_short_array_get_value(
    vtkNew<vtkCompositeShortArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_short_array_get_number_of_values(
    vtkNew<vtkCompositeShortArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeSignedCharArray> vtkCompositeSignedCharArray_new() {
    return vtkNew<vtkCompositeSignedCharArray>();
}
extern "C" void vtkCompositeSignedCharArray_destructor(vtkNew<vtkCompositeSignedCharArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeSignedCharArray_get_ptr(vtkNew<vtkCompositeSignedCharArray> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_composite_signed_char_array_get_data_type_value_min(
    vtkNew<vtkCompositeSignedCharArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_composite_signed_char_array_get_data_type_value_max(
    vtkNew<vtkCompositeSignedCharArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_composite_signed_char_array_get_value(
    vtkNew<vtkCompositeSignedCharArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_signed_char_array_get_number_of_values(
    vtkNew<vtkCompositeSignedCharArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeTypeFloat32Array> vtkCompositeTypeFloat32Array_new() {
    return vtkNew<vtkCompositeTypeFloat32Array>();
}
extern "C" void vtkCompositeTypeFloat32Array_destructor(
    vtkNew<vtkCompositeTypeFloat32Array> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeTypeFloat32Array_get_ptr(vtkNew<vtkCompositeTypeFloat32Array> sself) {
    return sself.GetPointer();
}
extern "C" float vtk_composite_type_float_32_array_get_data_type_value_min(
    vtkNew<vtkCompositeTypeFloat32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" float vtk_composite_type_float_32_array_get_data_type_value_max(
    vtkNew<vtkCompositeTypeFloat32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" float vtk_composite_type_float_32_array_get_value(
    vtkNew<vtkCompositeTypeFloat32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_type_float_32_array_get_number_of_values(
    vtkNew<vtkCompositeTypeFloat32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeTypeFloat64Array> vtkCompositeTypeFloat64Array_new() {
    return vtkNew<vtkCompositeTypeFloat64Array>();
}
extern "C" void vtkCompositeTypeFloat64Array_destructor(
    vtkNew<vtkCompositeTypeFloat64Array> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeTypeFloat64Array_get_ptr(vtkNew<vtkCompositeTypeFloat64Array> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_composite_type_float_64_array_get_data_type_value_min(
    vtkNew<vtkCompositeTypeFloat64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" double vtk_composite_type_float_64_array_get_data_type_value_max(
    vtkNew<vtkCompositeTypeFloat64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" double vtk_composite_type_float_64_array_get_value(
    vtkNew<vtkCompositeTypeFloat64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_type_float_64_array_get_number_of_values(
    vtkNew<vtkCompositeTypeFloat64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeTypeInt16Array> vtkCompositeTypeInt16Array_new() {
    return vtkNew<vtkCompositeTypeInt16Array>();
}
extern "C" void vtkCompositeTypeInt16Array_destructor(vtkNew<vtkCompositeTypeInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeTypeInt16Array_get_ptr(vtkNew<vtkCompositeTypeInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" short vtk_composite_type_int_16_array_get_data_type_value_min(
    vtkNew<vtkCompositeTypeInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" short vtk_composite_type_int_16_array_get_data_type_value_max(
    vtkNew<vtkCompositeTypeInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" short vtk_composite_type_int_16_array_get_value(
    vtkNew<vtkCompositeTypeInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_type_int_16_array_get_number_of_values(
    vtkNew<vtkCompositeTypeInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeTypeInt32Array> vtkCompositeTypeInt32Array_new() {
    return vtkNew<vtkCompositeTypeInt32Array>();
}
extern "C" void vtkCompositeTypeInt32Array_destructor(vtkNew<vtkCompositeTypeInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeTypeInt32Array_get_ptr(vtkNew<vtkCompositeTypeInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_composite_type_int_32_array_get_data_type_value_min(
    vtkNew<vtkCompositeTypeInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" int vtk_composite_type_int_32_array_get_data_type_value_max(
    vtkNew<vtkCompositeTypeInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" int vtk_composite_type_int_32_array_get_value(
    vtkNew<vtkCompositeTypeInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_type_int_32_array_get_number_of_values(
    vtkNew<vtkCompositeTypeInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeTypeInt64Array> vtkCompositeTypeInt64Array_new() {
    return vtkNew<vtkCompositeTypeInt64Array>();
}
extern "C" void vtkCompositeTypeInt64Array_destructor(vtkNew<vtkCompositeTypeInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeTypeInt64Array_get_ptr(vtkNew<vtkCompositeTypeInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_composite_type_int_64_array_get_data_type_value_min(
    vtkNew<vtkCompositeTypeInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_composite_type_int_64_array_get_data_type_value_max(
    vtkNew<vtkCompositeTypeInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_composite_type_int_64_array_get_value(
    vtkNew<vtkCompositeTypeInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_type_int_64_array_get_number_of_values(
    vtkNew<vtkCompositeTypeInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeTypeInt8Array> vtkCompositeTypeInt8Array_new() {
    return vtkNew<vtkCompositeTypeInt8Array>();
}
extern "C" void vtkCompositeTypeInt8Array_destructor(vtkNew<vtkCompositeTypeInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeTypeInt8Array_get_ptr(vtkNew<vtkCompositeTypeInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_composite_type_int_8_array_get_data_type_value_min(
    vtkNew<vtkCompositeTypeInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_composite_type_int_8_array_get_data_type_value_max(
    vtkNew<vtkCompositeTypeInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_composite_type_int_8_array_get_value(
    vtkNew<vtkCompositeTypeInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_type_int_8_array_get_number_of_values(
    vtkNew<vtkCompositeTypeInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeTypeUInt16Array> vtkCompositeTypeUInt16Array_new() {
    return vtkNew<vtkCompositeTypeUInt16Array>();
}
extern "C" void vtkCompositeTypeUInt16Array_destructor(vtkNew<vtkCompositeTypeUInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeTypeUInt16Array_get_ptr(vtkNew<vtkCompositeTypeUInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned short vtk_composite_type_u_int_16_array_get_data_type_value_min(
    vtkNew<vtkCompositeTypeUInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned short vtk_composite_type_u_int_16_array_get_data_type_value_max(
    vtkNew<vtkCompositeTypeUInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned short vtk_composite_type_u_int_16_array_get_value(
    vtkNew<vtkCompositeTypeUInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_type_u_int_16_array_get_number_of_values(
    vtkNew<vtkCompositeTypeUInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeTypeUInt32Array> vtkCompositeTypeUInt32Array_new() {
    return vtkNew<vtkCompositeTypeUInt32Array>();
}
extern "C" void vtkCompositeTypeUInt32Array_destructor(vtkNew<vtkCompositeTypeUInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeTypeUInt32Array_get_ptr(vtkNew<vtkCompositeTypeUInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned int vtk_composite_type_u_int_32_array_get_data_type_value_min(
    vtkNew<vtkCompositeTypeUInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned int vtk_composite_type_u_int_32_array_get_data_type_value_max(
    vtkNew<vtkCompositeTypeUInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned int vtk_composite_type_u_int_32_array_get_value(
    vtkNew<vtkCompositeTypeUInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_type_u_int_32_array_get_number_of_values(
    vtkNew<vtkCompositeTypeUInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeTypeUInt64Array> vtkCompositeTypeUInt64Array_new() {
    return vtkNew<vtkCompositeTypeUInt64Array>();
}
extern "C" void vtkCompositeTypeUInt64Array_destructor(vtkNew<vtkCompositeTypeUInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeTypeUInt64Array_get_ptr(vtkNew<vtkCompositeTypeUInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long long vtk_composite_type_u_int_64_array_get_data_type_value_min(
    vtkNew<vtkCompositeTypeUInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long long vtk_composite_type_u_int_64_array_get_data_type_value_max(
    vtkNew<vtkCompositeTypeUInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long long vtk_composite_type_u_int_64_array_get_value(
    vtkNew<vtkCompositeTypeUInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_type_u_int_64_array_get_number_of_values(
    vtkNew<vtkCompositeTypeUInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeTypeUInt8Array> vtkCompositeTypeUInt8Array_new() {
    return vtkNew<vtkCompositeTypeUInt8Array>();
}
extern "C" void vtkCompositeTypeUInt8Array_destructor(vtkNew<vtkCompositeTypeUInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeTypeUInt8Array_get_ptr(vtkNew<vtkCompositeTypeUInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned char vtk_composite_type_u_int_8_array_get_data_type_value_min(
    vtkNew<vtkCompositeTypeUInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned char vtk_composite_type_u_int_8_array_get_data_type_value_max(
    vtkNew<vtkCompositeTypeUInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned char vtk_composite_type_u_int_8_array_get_value(
    vtkNew<vtkCompositeTypeUInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_type_u_int_8_array_get_number_of_values(
    vtkNew<vtkCompositeTypeUInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeUnsignedCharArray> vtkCompositeUnsignedCharArray_new() {
    return vtkNew<vtkCompositeUnsignedCharArray>();
}
extern "C" void vtkCompositeUnsignedCharArray_destructor(
    vtkNew<vtkCompositeUnsignedCharArray> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeUnsignedCharArray_get_ptr(
    vtkNew<vtkCompositeUnsignedCharArray> sself
) {
    return sself.GetPointer();
}
extern "C" unsigned char vtk_composite_unsigned_char_array_get_data_type_value_min(
    vtkNew<vtkCompositeUnsignedCharArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned char vtk_composite_unsigned_char_array_get_data_type_value_max(
    vtkNew<vtkCompositeUnsignedCharArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned char vtk_composite_unsigned_char_array_get_value(
    vtkNew<vtkCompositeUnsignedCharArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_unsigned_char_array_get_number_of_values(
    vtkNew<vtkCompositeUnsignedCharArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeUnsignedIntArray> vtkCompositeUnsignedIntArray_new() {
    return vtkNew<vtkCompositeUnsignedIntArray>();
}
extern "C" void vtkCompositeUnsignedIntArray_destructor(
    vtkNew<vtkCompositeUnsignedIntArray> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeUnsignedIntArray_get_ptr(vtkNew<vtkCompositeUnsignedIntArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned int vtk_composite_unsigned_int_array_get_data_type_value_min(
    vtkNew<vtkCompositeUnsignedIntArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned int vtk_composite_unsigned_int_array_get_data_type_value_max(
    vtkNew<vtkCompositeUnsignedIntArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned int vtk_composite_unsigned_int_array_get_value(
    vtkNew<vtkCompositeUnsignedIntArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_unsigned_int_array_get_number_of_values(
    vtkNew<vtkCompositeUnsignedIntArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeUnsignedLongArray> vtkCompositeUnsignedLongArray_new() {
    return vtkNew<vtkCompositeUnsignedLongArray>();
}
extern "C" void vtkCompositeUnsignedLongArray_destructor(
    vtkNew<vtkCompositeUnsignedLongArray> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeUnsignedLongArray_get_ptr(
    vtkNew<vtkCompositeUnsignedLongArray> sself
) {
    return sself.GetPointer();
}
extern "C" unsigned long vtk_composite_unsigned_long_array_get_data_type_value_min(
    vtkNew<vtkCompositeUnsignedLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long vtk_composite_unsigned_long_array_get_data_type_value_max(
    vtkNew<vtkCompositeUnsignedLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long vtk_composite_unsigned_long_array_get_value(
    vtkNew<vtkCompositeUnsignedLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_unsigned_long_array_get_number_of_values(
    vtkNew<vtkCompositeUnsignedLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeUnsignedLongLongArray> vtkCompositeUnsignedLongLongArray_new() {
    return vtkNew<vtkCompositeUnsignedLongLongArray>();
}
extern "C" void vtkCompositeUnsignedLongLongArray_destructor(
    vtkNew<vtkCompositeUnsignedLongLongArray> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeUnsignedLongLongArray_get_ptr(
    vtkNew<vtkCompositeUnsignedLongLongArray> sself
) {
    return sself.GetPointer();
}
extern "C" unsigned long long vtk_composite_unsigned_long_long_array_get_data_type_value_min(
    vtkNew<vtkCompositeUnsignedLongLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long long vtk_composite_unsigned_long_long_array_get_data_type_value_max(
    vtkNew<vtkCompositeUnsignedLongLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long long vtk_composite_unsigned_long_long_array_get_value(
    vtkNew<vtkCompositeUnsignedLongLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_unsigned_long_long_array_get_number_of_values(
    vtkNew<vtkCompositeUnsignedLongLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkCompositeUnsignedShortArray> vtkCompositeUnsignedShortArray_new() {
    return vtkNew<vtkCompositeUnsignedShortArray>();
}
extern "C" void vtkCompositeUnsignedShortArray_destructor(
    vtkNew<vtkCompositeUnsignedShortArray> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkCompositeUnsignedShortArray_get_ptr(
    vtkNew<vtkCompositeUnsignedShortArray> sself
) {
    return sself.GetPointer();
}
extern "C" unsigned short vtk_composite_unsigned_short_array_get_data_type_value_min(
    vtkNew<vtkCompositeUnsignedShortArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned short vtk_composite_unsigned_short_array_get_data_type_value_max(
    vtkNew<vtkCompositeUnsignedShortArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned short vtk_composite_unsigned_short_array_get_value(
    vtkNew<vtkCompositeUnsignedShortArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_composite_unsigned_short_array_get_number_of_values(
    vtkNew<vtkCompositeUnsignedShortArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantCharArray> vtkConstantCharArray_new() {
    return vtkNew<vtkConstantCharArray>();
}
extern "C" void vtkConstantCharArray_destructor(vtkNew<vtkConstantCharArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantCharArray_get_ptr(vtkNew<vtkConstantCharArray> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_constant_char_array_get_data_type_value_min(vtkNew<vtkConstantCharArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_constant_char_array_get_data_type_value_max(vtkNew<vtkConstantCharArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_constant_char_array_get_value(vtkNew<vtkConstantCharArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_char_array_construct_backend(
    vtkNew<vtkConstantCharArray> self, char value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_char_array_get_number_of_values(
    vtkNew<vtkConstantCharArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantDoubleArray> vtkConstantDoubleArray_new() {
    return vtkNew<vtkConstantDoubleArray>();
}
extern "C" void vtkConstantDoubleArray_destructor(vtkNew<vtkConstantDoubleArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantDoubleArray_get_ptr(vtkNew<vtkConstantDoubleArray> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_constant_double_array_get_data_type_value_min(
    vtkNew<vtkConstantDoubleArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" double vtk_constant_double_array_get_data_type_value_max(
    vtkNew<vtkConstantDoubleArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" double vtk_constant_double_array_get_value(
    vtkNew<vtkConstantDoubleArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_double_array_construct_backend(
    vtkNew<vtkConstantDoubleArray> self, double value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_double_array_get_number_of_values(
    vtkNew<vtkConstantDoubleArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantFloatArray> vtkConstantFloatArray_new() {
    return vtkNew<vtkConstantFloatArray>();
}
extern "C" void vtkConstantFloatArray_destructor(vtkNew<vtkConstantFloatArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantFloatArray_get_ptr(vtkNew<vtkConstantFloatArray> sself) {
    return sself.GetPointer();
}
extern "C" float vtk_constant_float_array_get_data_type_value_min(
    vtkNew<vtkConstantFloatArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" float vtk_constant_float_array_get_data_type_value_max(
    vtkNew<vtkConstantFloatArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" float vtk_constant_float_array_get_value(
    vtkNew<vtkConstantFloatArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_float_array_construct_backend(
    vtkNew<vtkConstantFloatArray> self, float value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_float_array_get_number_of_values(
    vtkNew<vtkConstantFloatArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantIdTypeArray> vtkConstantIdTypeArray_new() {
    return vtkNew<vtkConstantIdTypeArray>();
}
extern "C" void vtkConstantIdTypeArray_destructor(vtkNew<vtkConstantIdTypeArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantIdTypeArray_get_ptr(vtkNew<vtkConstantIdTypeArray> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_constant_id_type_array_get_data_type_value_min(
    vtkNew<vtkConstantIdTypeArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_constant_id_type_array_get_data_type_value_max(
    vtkNew<vtkConstantIdTypeArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_constant_id_type_array_get_value(
    vtkNew<vtkConstantIdTypeArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_id_type_array_construct_backend(
    vtkNew<vtkConstantIdTypeArray> self, long long value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_id_type_array_get_number_of_values(
    vtkNew<vtkConstantIdTypeArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantIntArray> vtkConstantIntArray_new() {
    return vtkNew<vtkConstantIntArray>();
}
extern "C" void vtkConstantIntArray_destructor(vtkNew<vtkConstantIntArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantIntArray_get_ptr(vtkNew<vtkConstantIntArray> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_constant_int_array_get_data_type_value_min(vtkNew<vtkConstantIntArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" int vtk_constant_int_array_get_data_type_value_max(vtkNew<vtkConstantIntArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" int vtk_constant_int_array_get_value(vtkNew<vtkConstantIntArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_int_array_construct_backend(
    vtkNew<vtkConstantIntArray> self, int value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_int_array_get_number_of_values(vtkNew<vtkConstantIntArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantLongArray> vtkConstantLongArray_new() {
    return vtkNew<vtkConstantLongArray>();
}
extern "C" void vtkConstantLongArray_destructor(vtkNew<vtkConstantLongArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantLongArray_get_ptr(vtkNew<vtkConstantLongArray> sself) {
    return sself.GetPointer();
}
extern "C" long vtk_constant_long_array_get_data_type_value_min(vtkNew<vtkConstantLongArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" long vtk_constant_long_array_get_data_type_value_max(vtkNew<vtkConstantLongArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" long vtk_constant_long_array_get_value(vtkNew<vtkConstantLongArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_long_array_construct_backend(
    vtkNew<vtkConstantLongArray> self, long value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_long_array_get_number_of_values(
    vtkNew<vtkConstantLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantLongLongArray> vtkConstantLongLongArray_new() {
    return vtkNew<vtkConstantLongLongArray>();
}
extern "C" void vtkConstantLongLongArray_destructor(vtkNew<vtkConstantLongLongArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantLongLongArray_get_ptr(vtkNew<vtkConstantLongLongArray> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_constant_long_long_array_get_data_type_value_min(
    vtkNew<vtkConstantLongLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_constant_long_long_array_get_data_type_value_max(
    vtkNew<vtkConstantLongLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_constant_long_long_array_get_value(
    vtkNew<vtkConstantLongLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_long_long_array_construct_backend(
    vtkNew<vtkConstantLongLongArray> self, long long value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_long_long_array_get_number_of_values(
    vtkNew<vtkConstantLongLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantShortArray> vtkConstantShortArray_new() {
    return vtkNew<vtkConstantShortArray>();
}
extern "C" void vtkConstantShortArray_destructor(vtkNew<vtkConstantShortArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantShortArray_get_ptr(vtkNew<vtkConstantShortArray> sself) {
    return sself.GetPointer();
}
extern "C" short vtk_constant_short_array_get_data_type_value_min(
    vtkNew<vtkConstantShortArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" short vtk_constant_short_array_get_data_type_value_max(
    vtkNew<vtkConstantShortArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" short vtk_constant_short_array_get_value(
    vtkNew<vtkConstantShortArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_short_array_construct_backend(
    vtkNew<vtkConstantShortArray> self, short value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_short_array_get_number_of_values(
    vtkNew<vtkConstantShortArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantSignedCharArray> vtkConstantSignedCharArray_new() {
    return vtkNew<vtkConstantSignedCharArray>();
}
extern "C" void vtkConstantSignedCharArray_destructor(vtkNew<vtkConstantSignedCharArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantSignedCharArray_get_ptr(vtkNew<vtkConstantSignedCharArray> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_constant_signed_char_array_get_data_type_value_min(
    vtkNew<vtkConstantSignedCharArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_constant_signed_char_array_get_data_type_value_max(
    vtkNew<vtkConstantSignedCharArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_constant_signed_char_array_get_value(
    vtkNew<vtkConstantSignedCharArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_signed_char_array_construct_backend(
    vtkNew<vtkConstantSignedCharArray> self, char value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_signed_char_array_get_number_of_values(
    vtkNew<vtkConstantSignedCharArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantTypeFloat32Array> vtkConstantTypeFloat32Array_new() {
    return vtkNew<vtkConstantTypeFloat32Array>();
}
extern "C" void vtkConstantTypeFloat32Array_destructor(vtkNew<vtkConstantTypeFloat32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantTypeFloat32Array_get_ptr(vtkNew<vtkConstantTypeFloat32Array> sself) {
    return sself.GetPointer();
}
extern "C" float vtk_constant_type_float_32_array_get_data_type_value_min(
    vtkNew<vtkConstantTypeFloat32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" float vtk_constant_type_float_32_array_get_data_type_value_max(
    vtkNew<vtkConstantTypeFloat32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" float vtk_constant_type_float_32_array_get_value(
    vtkNew<vtkConstantTypeFloat32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_type_float_32_array_construct_backend(
    vtkNew<vtkConstantTypeFloat32Array> self, float value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_type_float_32_array_get_number_of_values(
    vtkNew<vtkConstantTypeFloat32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantTypeFloat64Array> vtkConstantTypeFloat64Array_new() {
    return vtkNew<vtkConstantTypeFloat64Array>();
}
extern "C" void vtkConstantTypeFloat64Array_destructor(vtkNew<vtkConstantTypeFloat64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantTypeFloat64Array_get_ptr(vtkNew<vtkConstantTypeFloat64Array> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_constant_type_float_64_array_get_data_type_value_min(
    vtkNew<vtkConstantTypeFloat64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" double vtk_constant_type_float_64_array_get_data_type_value_max(
    vtkNew<vtkConstantTypeFloat64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" double vtk_constant_type_float_64_array_get_value(
    vtkNew<vtkConstantTypeFloat64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_type_float_64_array_construct_backend(
    vtkNew<vtkConstantTypeFloat64Array> self, double value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_type_float_64_array_get_number_of_values(
    vtkNew<vtkConstantTypeFloat64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantTypeInt16Array> vtkConstantTypeInt16Array_new() {
    return vtkNew<vtkConstantTypeInt16Array>();
}
extern "C" void vtkConstantTypeInt16Array_destructor(vtkNew<vtkConstantTypeInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantTypeInt16Array_get_ptr(vtkNew<vtkConstantTypeInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" short vtk_constant_type_int_16_array_get_data_type_value_min(
    vtkNew<vtkConstantTypeInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" short vtk_constant_type_int_16_array_get_data_type_value_max(
    vtkNew<vtkConstantTypeInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" short vtk_constant_type_int_16_array_get_value(
    vtkNew<vtkConstantTypeInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_type_int_16_array_construct_backend(
    vtkNew<vtkConstantTypeInt16Array> self, short value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_type_int_16_array_get_number_of_values(
    vtkNew<vtkConstantTypeInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantTypeInt32Array> vtkConstantTypeInt32Array_new() {
    return vtkNew<vtkConstantTypeInt32Array>();
}
extern "C" void vtkConstantTypeInt32Array_destructor(vtkNew<vtkConstantTypeInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantTypeInt32Array_get_ptr(vtkNew<vtkConstantTypeInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_constant_type_int_32_array_get_data_type_value_min(
    vtkNew<vtkConstantTypeInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" int vtk_constant_type_int_32_array_get_data_type_value_max(
    vtkNew<vtkConstantTypeInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" int vtk_constant_type_int_32_array_get_value(
    vtkNew<vtkConstantTypeInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_type_int_32_array_construct_backend(
    vtkNew<vtkConstantTypeInt32Array> self, int value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_type_int_32_array_get_number_of_values(
    vtkNew<vtkConstantTypeInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantTypeInt64Array> vtkConstantTypeInt64Array_new() {
    return vtkNew<vtkConstantTypeInt64Array>();
}
extern "C" void vtkConstantTypeInt64Array_destructor(vtkNew<vtkConstantTypeInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantTypeInt64Array_get_ptr(vtkNew<vtkConstantTypeInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_constant_type_int_64_array_get_data_type_value_min(
    vtkNew<vtkConstantTypeInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_constant_type_int_64_array_get_data_type_value_max(
    vtkNew<vtkConstantTypeInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_constant_type_int_64_array_get_value(
    vtkNew<vtkConstantTypeInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_type_int_64_array_construct_backend(
    vtkNew<vtkConstantTypeInt64Array> self, long long value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_type_int_64_array_get_number_of_values(
    vtkNew<vtkConstantTypeInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantTypeInt8Array> vtkConstantTypeInt8Array_new() {
    return vtkNew<vtkConstantTypeInt8Array>();
}
extern "C" void vtkConstantTypeInt8Array_destructor(vtkNew<vtkConstantTypeInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantTypeInt8Array_get_ptr(vtkNew<vtkConstantTypeInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_constant_type_int_8_array_get_data_type_value_min(
    vtkNew<vtkConstantTypeInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_constant_type_int_8_array_get_data_type_value_max(
    vtkNew<vtkConstantTypeInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_constant_type_int_8_array_get_value(
    vtkNew<vtkConstantTypeInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_type_int_8_array_construct_backend(
    vtkNew<vtkConstantTypeInt8Array> self, char value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_type_int_8_array_get_number_of_values(
    vtkNew<vtkConstantTypeInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantTypeUInt16Array> vtkConstantTypeUInt16Array_new() {
    return vtkNew<vtkConstantTypeUInt16Array>();
}
extern "C" void vtkConstantTypeUInt16Array_destructor(vtkNew<vtkConstantTypeUInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantTypeUInt16Array_get_ptr(vtkNew<vtkConstantTypeUInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned short vtk_constant_type_u_int_16_array_get_data_type_value_min(
    vtkNew<vtkConstantTypeUInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned short vtk_constant_type_u_int_16_array_get_data_type_value_max(
    vtkNew<vtkConstantTypeUInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned short vtk_constant_type_u_int_16_array_get_value(
    vtkNew<vtkConstantTypeUInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_type_u_int_16_array_construct_backend(
    vtkNew<vtkConstantTypeUInt16Array> self, unsigned short value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_type_u_int_16_array_get_number_of_values(
    vtkNew<vtkConstantTypeUInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantTypeUInt32Array> vtkConstantTypeUInt32Array_new() {
    return vtkNew<vtkConstantTypeUInt32Array>();
}
extern "C" void vtkConstantTypeUInt32Array_destructor(vtkNew<vtkConstantTypeUInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantTypeUInt32Array_get_ptr(vtkNew<vtkConstantTypeUInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned int vtk_constant_type_u_int_32_array_get_data_type_value_min(
    vtkNew<vtkConstantTypeUInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned int vtk_constant_type_u_int_32_array_get_data_type_value_max(
    vtkNew<vtkConstantTypeUInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned int vtk_constant_type_u_int_32_array_get_value(
    vtkNew<vtkConstantTypeUInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_type_u_int_32_array_construct_backend(
    vtkNew<vtkConstantTypeUInt32Array> self, unsigned int value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_type_u_int_32_array_get_number_of_values(
    vtkNew<vtkConstantTypeUInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantTypeUInt64Array> vtkConstantTypeUInt64Array_new() {
    return vtkNew<vtkConstantTypeUInt64Array>();
}
extern "C" void vtkConstantTypeUInt64Array_destructor(vtkNew<vtkConstantTypeUInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantTypeUInt64Array_get_ptr(vtkNew<vtkConstantTypeUInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long long vtk_constant_type_u_int_64_array_get_data_type_value_min(
    vtkNew<vtkConstantTypeUInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long long vtk_constant_type_u_int_64_array_get_data_type_value_max(
    vtkNew<vtkConstantTypeUInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long long vtk_constant_type_u_int_64_array_get_value(
    vtkNew<vtkConstantTypeUInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_type_u_int_64_array_construct_backend(
    vtkNew<vtkConstantTypeUInt64Array> self, unsigned long long value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_type_u_int_64_array_get_number_of_values(
    vtkNew<vtkConstantTypeUInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantTypeUInt8Array> vtkConstantTypeUInt8Array_new() {
    return vtkNew<vtkConstantTypeUInt8Array>();
}
extern "C" void vtkConstantTypeUInt8Array_destructor(vtkNew<vtkConstantTypeUInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantTypeUInt8Array_get_ptr(vtkNew<vtkConstantTypeUInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned char vtk_constant_type_u_int_8_array_get_data_type_value_min(
    vtkNew<vtkConstantTypeUInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned char vtk_constant_type_u_int_8_array_get_data_type_value_max(
    vtkNew<vtkConstantTypeUInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned char vtk_constant_type_u_int_8_array_get_value(
    vtkNew<vtkConstantTypeUInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_type_u_int_8_array_construct_backend(
    vtkNew<vtkConstantTypeUInt8Array> self, unsigned char value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_type_u_int_8_array_get_number_of_values(
    vtkNew<vtkConstantTypeUInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantUnsignedCharArray> vtkConstantUnsignedCharArray_new() {
    return vtkNew<vtkConstantUnsignedCharArray>();
}
extern "C" void vtkConstantUnsignedCharArray_destructor(
    vtkNew<vtkConstantUnsignedCharArray> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantUnsignedCharArray_get_ptr(vtkNew<vtkConstantUnsignedCharArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned char vtk_constant_unsigned_char_array_get_data_type_value_min(
    vtkNew<vtkConstantUnsignedCharArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned char vtk_constant_unsigned_char_array_get_data_type_value_max(
    vtkNew<vtkConstantUnsignedCharArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned char vtk_constant_unsigned_char_array_get_value(
    vtkNew<vtkConstantUnsignedCharArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_unsigned_char_array_construct_backend(
    vtkNew<vtkConstantUnsignedCharArray> self, unsigned char value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_unsigned_char_array_get_number_of_values(
    vtkNew<vtkConstantUnsignedCharArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantUnsignedIntArray> vtkConstantUnsignedIntArray_new() {
    return vtkNew<vtkConstantUnsignedIntArray>();
}
extern "C" void vtkConstantUnsignedIntArray_destructor(vtkNew<vtkConstantUnsignedIntArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantUnsignedIntArray_get_ptr(vtkNew<vtkConstantUnsignedIntArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned int vtk_constant_unsigned_int_array_get_data_type_value_min(
    vtkNew<vtkConstantUnsignedIntArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned int vtk_constant_unsigned_int_array_get_data_type_value_max(
    vtkNew<vtkConstantUnsignedIntArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned int vtk_constant_unsigned_int_array_get_value(
    vtkNew<vtkConstantUnsignedIntArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_unsigned_int_array_construct_backend(
    vtkNew<vtkConstantUnsignedIntArray> self, unsigned int value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_unsigned_int_array_get_number_of_values(
    vtkNew<vtkConstantUnsignedIntArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantUnsignedLongArray> vtkConstantUnsignedLongArray_new() {
    return vtkNew<vtkConstantUnsignedLongArray>();
}
extern "C" void vtkConstantUnsignedLongArray_destructor(
    vtkNew<vtkConstantUnsignedLongArray> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantUnsignedLongArray_get_ptr(vtkNew<vtkConstantUnsignedLongArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long vtk_constant_unsigned_long_array_get_data_type_value_min(
    vtkNew<vtkConstantUnsignedLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long vtk_constant_unsigned_long_array_get_data_type_value_max(
    vtkNew<vtkConstantUnsignedLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long vtk_constant_unsigned_long_array_get_value(
    vtkNew<vtkConstantUnsignedLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_unsigned_long_array_construct_backend(
    vtkNew<vtkConstantUnsignedLongArray> self, unsigned long value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_unsigned_long_array_get_number_of_values(
    vtkNew<vtkConstantUnsignedLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantUnsignedLongLongArray> vtkConstantUnsignedLongLongArray_new() {
    return vtkNew<vtkConstantUnsignedLongLongArray>();
}
extern "C" void vtkConstantUnsignedLongLongArray_destructor(
    vtkNew<vtkConstantUnsignedLongLongArray> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantUnsignedLongLongArray_get_ptr(
    vtkNew<vtkConstantUnsignedLongLongArray> sself
) {
    return sself.GetPointer();
}
extern "C" unsigned long long vtk_constant_unsigned_long_long_array_get_data_type_value_min(
    vtkNew<vtkConstantUnsignedLongLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long long vtk_constant_unsigned_long_long_array_get_data_type_value_max(
    vtkNew<vtkConstantUnsignedLongLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long long vtk_constant_unsigned_long_long_array_get_value(
    vtkNew<vtkConstantUnsignedLongLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_unsigned_long_long_array_construct_backend(
    vtkNew<vtkConstantUnsignedLongLongArray> self, unsigned long long value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_unsigned_long_long_array_get_number_of_values(
    vtkNew<vtkConstantUnsignedLongLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkConstantUnsignedShortArray> vtkConstantUnsignedShortArray_new() {
    return vtkNew<vtkConstantUnsignedShortArray>();
}
extern "C" void vtkConstantUnsignedShortArray_destructor(
    vtkNew<vtkConstantUnsignedShortArray> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkConstantUnsignedShortArray_get_ptr(
    vtkNew<vtkConstantUnsignedShortArray> sself
) {
    return sself.GetPointer();
}
extern "C" unsigned short vtk_constant_unsigned_short_array_get_data_type_value_min(
    vtkNew<vtkConstantUnsignedShortArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned short vtk_constant_unsigned_short_array_get_data_type_value_max(
    vtkNew<vtkConstantUnsignedShortArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned short vtk_constant_unsigned_short_array_get_value(
    vtkNew<vtkConstantUnsignedShortArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_constant_unsigned_short_array_construct_backend(
    vtkNew<vtkConstantUnsignedShortArray> self, unsigned short value
) {
    return self->ConstructBackend(value);
}
extern "C" long long vtk_constant_unsigned_short_array_get_number_of_values(
    vtkNew<vtkConstantUnsignedShortArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkDataArrayCollection> vtkDataArrayCollection_new() {
    return vtkNew<vtkDataArrayCollection>();
}
extern "C" void vtkDataArrayCollection_destructor(vtkNew<vtkDataArrayCollection> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDataArrayCollection_get_ptr(vtkNew<vtkDataArrayCollection> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_data_array_collection_get_number_of_items(vtkNew<vtkDataArrayCollection> self) {
    return self->GetNumberOfItems();
}
extern "C" vtkNew<vtkDataArrayCollectionIterator> vtkDataArrayCollectionIterator_new() {
    return vtkNew<vtkDataArrayCollectionIterator>();
}
extern "C" void vtkDataArrayCollectionIterator_destructor(
    vtkNew<vtkDataArrayCollectionIterator> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkDataArrayCollectionIterator_get_ptr(
    vtkNew<vtkDataArrayCollectionIterator> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkDataArraySelection> vtkDataArraySelection_new() {
    return vtkNew<vtkDataArraySelection>();
}
extern "C" void vtkDataArraySelection_destructor(vtkNew<vtkDataArraySelection> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDataArraySelection_get_ptr(vtkNew<vtkDataArraySelection> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_data_array_selection_enable_all_arrays(vtkNew<vtkDataArraySelection> self) {
    return self->EnableAllArrays();
}
extern "C" void vtk_data_array_selection_disable_all_arrays(vtkNew<vtkDataArraySelection> self) {
    return self->DisableAllArrays();
}
extern "C" int vtk_data_array_selection_get_number_of_arrays(vtkNew<vtkDataArraySelection> self) {
    return self->GetNumberOfArrays();
}
extern "C" int vtk_data_array_selection_get_number_of_arrays_enabled(
    vtkNew<vtkDataArraySelection> self
) {
    return self->GetNumberOfArraysEnabled();
}
extern "C" int vtk_data_array_selection_get_array_setting_c_int(
    vtkNew<vtkDataArraySelection> self, int index
) {
    return self->GetArraySetting(index);
}
extern "C" void vtk_data_array_selection_remove_all_arrays(vtkNew<vtkDataArraySelection> self) {
    return self->RemoveAllArrays();
}
extern "C" void vtk_data_array_selection_remove_array_by_index(
    vtkNew<vtkDataArraySelection> self, int index
) {
    return self->RemoveArrayByIndex(index);
}
extern "C" void vtk_data_array_selection_set_unknown_array_setting(
    vtkNew<vtkDataArraySelection> self, int _arg
) {
    return self->SetUnknownArraySetting(_arg);
}
extern "C" int vtk_data_array_selection_get_unknown_array_setting(
    vtkNew<vtkDataArraySelection> self
) {
    return self->GetUnknownArraySetting();
}
extern "C" vtkNew<vtkDebugLeaks> vtkDebugLeaks_new() {
    return vtkNew<vtkDebugLeaks>();
}
extern "C" void vtkDebugLeaks_destructor(vtkNew<vtkDebugLeaks> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDebugLeaks_get_ptr(vtkNew<vtkDebugLeaks> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_debug_leaks_print_current_leaks(vtkNew<vtkDebugLeaks> self) {
    return self->PrintCurrentLeaks();
}
extern "C" int vtk_debug_leaks_get_exit_error(vtkNew<vtkDebugLeaks> self) {
    return self->GetExitError();
}
extern "C" void vtk_debug_leaks_set_exit_error(vtkNew<vtkDebugLeaks> self, int p0) {
    return self->SetExitError(p0);
}
extern "C" vtkNew<vtkDeserializer> vtkDeserializer_new() {
    return vtkNew<vtkDeserializer>();
}
extern "C" void vtkDeserializer_destructor(vtkNew<vtkDeserializer> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDeserializer_get_ptr(vtkNew<vtkDeserializer> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkDoubleArray> vtkDoubleArray_new() {
    return vtkNew<vtkDoubleArray>();
}
extern "C" void vtkDoubleArray_destructor(vtkNew<vtkDoubleArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDoubleArray_get_ptr(vtkNew<vtkDoubleArray> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_double_array_get_data_type_value_min(vtkNew<vtkDoubleArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" double vtk_double_array_get_data_type_value_max(vtkNew<vtkDoubleArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" double vtk_double_array_get_value(vtkNew<vtkDoubleArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_double_array_set_value(
    vtkNew<vtkDoubleArray> self, long long id, double value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_double_array_set_number_of_values(
    vtkNew<vtkDoubleArray> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_double_array_insert_value(vtkNew<vtkDoubleArray> self, long long id, double f) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_double_array_insert_next_value(vtkNew<vtkDoubleArray> self, double f) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_double_array_get_number_of_values(vtkNew<vtkDoubleArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkDynamicLoader> vtkDynamicLoader_new() {
    return vtkNew<vtkDynamicLoader>();
}
extern "C" void vtkDynamicLoader_destructor(vtkNew<vtkDynamicLoader> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkDynamicLoader_get_ptr(vtkNew<vtkDynamicLoader> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkEventDataDevice3D> vtkEventDataDevice3D_new() {
    return vtkNew<vtkEventDataDevice3D>();
}
extern "C" void vtkEventDataDevice3D_destructor(vtkNew<vtkEventDataDevice3D> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkEventDataDevice3D_get_ptr(vtkNew<vtkEventDataDevice3D> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_event_data_device_3_d_get_world_position_double_3(
    vtkNew<vtkEventDataDevice3D> self, double* v
) {
    return self->GetWorldPosition(v);
}
extern "C" void vtk_event_data_device_3_d_set_world_position(
    vtkNew<vtkEventDataDevice3D> self, const double* p
) {
    return self->SetWorldPosition(p);
}
extern "C" void vtk_event_data_device_3_d_get_world_direction_double_3(
    vtkNew<vtkEventDataDevice3D> self, double* v
) {
    return self->GetWorldDirection(v);
}
extern "C" void vtk_event_data_device_3_d_set_world_direction(
    vtkNew<vtkEventDataDevice3D> self, const double* p
) {
    return self->SetWorldDirection(p);
}
extern "C" void vtk_event_data_device_3_d_get_world_orientation_double_4(
    vtkNew<vtkEventDataDevice3D> self, double* v
) {
    return self->GetWorldOrientation(v);
}
extern "C" void vtk_event_data_device_3_d_set_world_orientation(
    vtkNew<vtkEventDataDevice3D> self, const double* p
) {
    return self->SetWorldOrientation(p);
}
extern "C" void vtk_event_data_device_3_d_get_track_pad_position_double_2(
    vtkNew<vtkEventDataDevice3D> self, double* v
) {
    return self->GetTrackPadPosition(v);
}
extern "C" void vtk_event_data_device_3_d_set_track_pad_position_const_double_2(
    vtkNew<vtkEventDataDevice3D> self, const double* p
) {
    return self->SetTrackPadPosition(p);
}
extern "C" void vtk_event_data_device_3_d_set_track_pad_position_double_double(
    vtkNew<vtkEventDataDevice3D> self, double x, double y
) {
    return self->SetTrackPadPosition(x, y);
}
extern "C" vtkNew<vtkEventDataForDevice> vtkEventDataForDevice_new() {
    return vtkNew<vtkEventDataForDevice>();
}
extern "C" void vtkEventDataForDevice_destructor(vtkNew<vtkEventDataForDevice> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkEventDataForDevice_get_ptr(vtkNew<vtkEventDataForDevice> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkEventForwarderCommand> vtkEventForwarderCommand_new() {
    return vtkNew<vtkEventForwarderCommand>();
}
extern "C" void vtkEventForwarderCommand_destructor(vtkNew<vtkEventForwarderCommand> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkEventForwarderCommand_get_ptr(vtkNew<vtkEventForwarderCommand> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkFileOutputWindow> vtkFileOutputWindow_new() {
    return vtkNew<vtkFileOutputWindow>();
}
extern "C" void vtkFileOutputWindow_destructor(vtkNew<vtkFileOutputWindow> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkFileOutputWindow_get_ptr(vtkNew<vtkFileOutputWindow> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_file_output_window_set_flush(vtkNew<vtkFileOutputWindow> self, int _arg) {
    return self->SetFlush(_arg);
}
extern "C" int vtk_file_output_window_get_flush(vtkNew<vtkFileOutputWindow> self) {
    return self->GetFlush();
}
extern "C" void vtk_file_output_window_flush_on(vtkNew<vtkFileOutputWindow> self) {
    return self->FlushOn();
}
extern "C" void vtk_file_output_window_flush_off(vtkNew<vtkFileOutputWindow> self) {
    return self->FlushOff();
}
extern "C" void vtk_file_output_window_set_append(vtkNew<vtkFileOutputWindow> self, int _arg) {
    return self->SetAppend(_arg);
}
extern "C" int vtk_file_output_window_get_append(vtkNew<vtkFileOutputWindow> self) {
    return self->GetAppend();
}
extern "C" void vtk_file_output_window_append_on(vtkNew<vtkFileOutputWindow> self) {
    return self->AppendOn();
}
extern "C" void vtk_file_output_window_append_off(vtkNew<vtkFileOutputWindow> self) {
    return self->AppendOff();
}
extern "C" vtkNew<vtkFloatArray> vtkFloatArray_new() {
    return vtkNew<vtkFloatArray>();
}
extern "C" void vtkFloatArray_destructor(vtkNew<vtkFloatArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkFloatArray_get_ptr(vtkNew<vtkFloatArray> sself) {
    return sself.GetPointer();
}
extern "C" float vtk_float_array_get_data_type_value_min(vtkNew<vtkFloatArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" float vtk_float_array_get_data_type_value_max(vtkNew<vtkFloatArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" float vtk_float_array_get_value(vtkNew<vtkFloatArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_float_array_set_value(vtkNew<vtkFloatArray> self, long long id, float value) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_float_array_set_number_of_values(vtkNew<vtkFloatArray> self, long long number) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_float_array_insert_value(vtkNew<vtkFloatArray> self, long long id, float f) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_float_array_insert_next_value(vtkNew<vtkFloatArray> self, float f) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_float_array_get_number_of_values(vtkNew<vtkFloatArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkGarbageCollector> vtkGarbageCollector_new() {
    return vtkNew<vtkGarbageCollector>();
}
extern "C" void vtkGarbageCollector_destructor(vtkNew<vtkGarbageCollector> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkGarbageCollector_get_ptr(vtkNew<vtkGarbageCollector> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_garbage_collector_collect(vtkNew<vtkGarbageCollector> self) {
    return self->Collect();
}
extern "C" void vtk_garbage_collector_deferred_collection_push(vtkNew<vtkGarbageCollector> self) {
    return self->DeferredCollectionPush();
}
extern "C" void vtk_garbage_collector_deferred_collection_pop(vtkNew<vtkGarbageCollector> self) {
    return self->DeferredCollectionPop();
}
extern "C" void vtk_garbage_collector_set_global_debug_flag(
    vtkNew<vtkGarbageCollector> self, bool flag
) {
    return self->SetGlobalDebugFlag(flag);
}
extern "C" bool vtk_garbage_collector_get_global_debug_flag(vtkNew<vtkGarbageCollector> self) {
    return self->GetGlobalDebugFlag();
}
extern "C" vtkNew<vtkIdList> vtkIdList_new() {
    return vtkNew<vtkIdList>();
}
extern "C" void vtkIdList_destructor(vtkNew<vtkIdList> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIdList_get_ptr(vtkNew<vtkIdList> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_id_list_initialize(vtkNew<vtkIdList> self) {
    return self->Initialize();
}
extern "C" int vtk_id_list_allocate(vtkNew<vtkIdList> self, long long sz, int strategy) {
    return self->Allocate(sz, strategy);
}
extern "C" long long vtk_id_list_get_number_of_ids(vtkNew<vtkIdList> self) {
    return self->GetNumberOfIds();
}
extern "C" long long vtk_id_list_get_id(vtkNew<vtkIdList> self, long long i) {
    return self->GetId(i);
}
extern "C" long long vtk_id_list_find_id_location(vtkNew<vtkIdList> self, long long id) {
    return self->FindIdLocation(id);
}
extern "C" void vtk_id_list_set_number_of_ids(vtkNew<vtkIdList> self, long long number) {
    return self->SetNumberOfIds(number);
}
extern "C" void vtk_id_list_set_id(vtkNew<vtkIdList> self, long long i, long long vtkid) {
    return self->SetId(i, vtkid);
}
extern "C" void vtk_id_list_insert_id(vtkNew<vtkIdList> self, long long i, long long vtkid) {
    return self->InsertId(i, vtkid);
}
extern "C" long long vtk_id_list_insert_next_id(vtkNew<vtkIdList> self, long long vtkid) {
    return self->InsertNextId(vtkid);
}
extern "C" long long vtk_id_list_insert_unique_id(vtkNew<vtkIdList> self, long long vtkid) {
    return self->InsertUniqueId(vtkid);
}
extern "C" void vtk_id_list_sort(vtkNew<vtkIdList> self) {
    return self->Sort();
}
extern "C" void vtk_id_list_fill(vtkNew<vtkIdList> self, long long value) {
    return self->Fill(value);
}
extern "C" void vtk_id_list_reset(vtkNew<vtkIdList> self) {
    return self->Reset();
}
extern "C" void vtk_id_list_squeeze(vtkNew<vtkIdList> self) {
    return self->Squeeze();
}
extern "C" void vtk_id_list_delete_id(vtkNew<vtkIdList> self, long long vtkid) {
    return self->DeleteId(vtkid);
}
extern "C" long long vtk_id_list_is_id(vtkNew<vtkIdList> self, long long vtkid) {
    return self->IsId(vtkid);
}
extern "C" vtkNew<vtkIdListCollection> vtkIdListCollection_new() {
    return vtkNew<vtkIdListCollection>();
}
extern "C" void vtkIdListCollection_destructor(vtkNew<vtkIdListCollection> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIdListCollection_get_ptr(vtkNew<vtkIdListCollection> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_id_list_collection_get_number_of_items(vtkNew<vtkIdListCollection> self) {
    return self->GetNumberOfItems();
}
extern "C" vtkNew<vtkIdTypeArray> vtkIdTypeArray_new() {
    return vtkNew<vtkIdTypeArray>();
}
extern "C" void vtkIdTypeArray_destructor(vtkNew<vtkIdTypeArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIdTypeArray_get_ptr(vtkNew<vtkIdTypeArray> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_id_type_array_get_data_type_value_min(vtkNew<vtkIdTypeArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_id_type_array_get_data_type_value_max(vtkNew<vtkIdTypeArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_id_type_array_get_value(vtkNew<vtkIdTypeArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_id_type_array_set_value(
    vtkNew<vtkIdTypeArray> self, long long id, long long value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_id_type_array_set_number_of_values(
    vtkNew<vtkIdTypeArray> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_id_type_array_insert_value(
    vtkNew<vtkIdTypeArray> self, long long id, long long f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_id_type_array_insert_next_value(vtkNew<vtkIdTypeArray> self, long long f) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_id_type_array_get_number_of_values(vtkNew<vtkIdTypeArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedCharArray> vtkIndexedCharArray_new() {
    return vtkNew<vtkIndexedCharArray>();
}
extern "C" void vtkIndexedCharArray_destructor(vtkNew<vtkIndexedCharArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedCharArray_get_ptr(vtkNew<vtkIndexedCharArray> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_indexed_char_array_get_data_type_value_min(vtkNew<vtkIndexedCharArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_indexed_char_array_get_data_type_value_max(vtkNew<vtkIndexedCharArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_indexed_char_array_get_value(vtkNew<vtkIndexedCharArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_char_array_get_number_of_values(vtkNew<vtkIndexedCharArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedDoubleArray> vtkIndexedDoubleArray_new() {
    return vtkNew<vtkIndexedDoubleArray>();
}
extern "C" void vtkIndexedDoubleArray_destructor(vtkNew<vtkIndexedDoubleArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedDoubleArray_get_ptr(vtkNew<vtkIndexedDoubleArray> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_indexed_double_array_get_data_type_value_min(
    vtkNew<vtkIndexedDoubleArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" double vtk_indexed_double_array_get_data_type_value_max(
    vtkNew<vtkIndexedDoubleArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" double vtk_indexed_double_array_get_value(
    vtkNew<vtkIndexedDoubleArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_double_array_get_number_of_values(
    vtkNew<vtkIndexedDoubleArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedFloatArray> vtkIndexedFloatArray_new() {
    return vtkNew<vtkIndexedFloatArray>();
}
extern "C" void vtkIndexedFloatArray_destructor(vtkNew<vtkIndexedFloatArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedFloatArray_get_ptr(vtkNew<vtkIndexedFloatArray> sself) {
    return sself.GetPointer();
}
extern "C" float vtk_indexed_float_array_get_data_type_value_min(
    vtkNew<vtkIndexedFloatArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" float vtk_indexed_float_array_get_data_type_value_max(
    vtkNew<vtkIndexedFloatArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" float vtk_indexed_float_array_get_value(
    vtkNew<vtkIndexedFloatArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_float_array_get_number_of_values(
    vtkNew<vtkIndexedFloatArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedIdTypeArray> vtkIndexedIdTypeArray_new() {
    return vtkNew<vtkIndexedIdTypeArray>();
}
extern "C" void vtkIndexedIdTypeArray_destructor(vtkNew<vtkIndexedIdTypeArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedIdTypeArray_get_ptr(vtkNew<vtkIndexedIdTypeArray> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_indexed_id_type_array_get_data_type_value_min(
    vtkNew<vtkIndexedIdTypeArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_indexed_id_type_array_get_data_type_value_max(
    vtkNew<vtkIndexedIdTypeArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_indexed_id_type_array_get_value(
    vtkNew<vtkIndexedIdTypeArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_id_type_array_get_number_of_values(
    vtkNew<vtkIndexedIdTypeArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedIntArray> vtkIndexedIntArray_new() {
    return vtkNew<vtkIndexedIntArray>();
}
extern "C" void vtkIndexedIntArray_destructor(vtkNew<vtkIndexedIntArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedIntArray_get_ptr(vtkNew<vtkIndexedIntArray> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_indexed_int_array_get_data_type_value_min(vtkNew<vtkIndexedIntArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" int vtk_indexed_int_array_get_data_type_value_max(vtkNew<vtkIndexedIntArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" int vtk_indexed_int_array_get_value(vtkNew<vtkIndexedIntArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_int_array_get_number_of_values(vtkNew<vtkIndexedIntArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedLongArray> vtkIndexedLongArray_new() {
    return vtkNew<vtkIndexedLongArray>();
}
extern "C" void vtkIndexedLongArray_destructor(vtkNew<vtkIndexedLongArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedLongArray_get_ptr(vtkNew<vtkIndexedLongArray> sself) {
    return sself.GetPointer();
}
extern "C" long vtk_indexed_long_array_get_data_type_value_min(vtkNew<vtkIndexedLongArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" long vtk_indexed_long_array_get_data_type_value_max(vtkNew<vtkIndexedLongArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" long vtk_indexed_long_array_get_value(vtkNew<vtkIndexedLongArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_long_array_get_number_of_values(vtkNew<vtkIndexedLongArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedLongLongArray> vtkIndexedLongLongArray_new() {
    return vtkNew<vtkIndexedLongLongArray>();
}
extern "C" void vtkIndexedLongLongArray_destructor(vtkNew<vtkIndexedLongLongArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedLongLongArray_get_ptr(vtkNew<vtkIndexedLongLongArray> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_indexed_long_long_array_get_data_type_value_min(
    vtkNew<vtkIndexedLongLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_indexed_long_long_array_get_data_type_value_max(
    vtkNew<vtkIndexedLongLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_indexed_long_long_array_get_value(
    vtkNew<vtkIndexedLongLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_long_long_array_get_number_of_values(
    vtkNew<vtkIndexedLongLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedShortArray> vtkIndexedShortArray_new() {
    return vtkNew<vtkIndexedShortArray>();
}
extern "C" void vtkIndexedShortArray_destructor(vtkNew<vtkIndexedShortArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedShortArray_get_ptr(vtkNew<vtkIndexedShortArray> sself) {
    return sself.GetPointer();
}
extern "C" short vtk_indexed_short_array_get_data_type_value_min(
    vtkNew<vtkIndexedShortArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" short vtk_indexed_short_array_get_data_type_value_max(
    vtkNew<vtkIndexedShortArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" short vtk_indexed_short_array_get_value(
    vtkNew<vtkIndexedShortArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_short_array_get_number_of_values(
    vtkNew<vtkIndexedShortArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedSignedCharArray> vtkIndexedSignedCharArray_new() {
    return vtkNew<vtkIndexedSignedCharArray>();
}
extern "C" void vtkIndexedSignedCharArray_destructor(vtkNew<vtkIndexedSignedCharArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedSignedCharArray_get_ptr(vtkNew<vtkIndexedSignedCharArray> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_indexed_signed_char_array_get_data_type_value_min(
    vtkNew<vtkIndexedSignedCharArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_indexed_signed_char_array_get_data_type_value_max(
    vtkNew<vtkIndexedSignedCharArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_indexed_signed_char_array_get_value(
    vtkNew<vtkIndexedSignedCharArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_signed_char_array_get_number_of_values(
    vtkNew<vtkIndexedSignedCharArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedTypeFloat32Array> vtkIndexedTypeFloat32Array_new() {
    return vtkNew<vtkIndexedTypeFloat32Array>();
}
extern "C" void vtkIndexedTypeFloat32Array_destructor(vtkNew<vtkIndexedTypeFloat32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedTypeFloat32Array_get_ptr(vtkNew<vtkIndexedTypeFloat32Array> sself) {
    return sself.GetPointer();
}
extern "C" float vtk_indexed_type_float_32_array_get_data_type_value_min(
    vtkNew<vtkIndexedTypeFloat32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" float vtk_indexed_type_float_32_array_get_data_type_value_max(
    vtkNew<vtkIndexedTypeFloat32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" float vtk_indexed_type_float_32_array_get_value(
    vtkNew<vtkIndexedTypeFloat32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_type_float_32_array_get_number_of_values(
    vtkNew<vtkIndexedTypeFloat32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedTypeFloat64Array> vtkIndexedTypeFloat64Array_new() {
    return vtkNew<vtkIndexedTypeFloat64Array>();
}
extern "C" void vtkIndexedTypeFloat64Array_destructor(vtkNew<vtkIndexedTypeFloat64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedTypeFloat64Array_get_ptr(vtkNew<vtkIndexedTypeFloat64Array> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_indexed_type_float_64_array_get_data_type_value_min(
    vtkNew<vtkIndexedTypeFloat64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" double vtk_indexed_type_float_64_array_get_data_type_value_max(
    vtkNew<vtkIndexedTypeFloat64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" double vtk_indexed_type_float_64_array_get_value(
    vtkNew<vtkIndexedTypeFloat64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_type_float_64_array_get_number_of_values(
    vtkNew<vtkIndexedTypeFloat64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedTypeInt16Array> vtkIndexedTypeInt16Array_new() {
    return vtkNew<vtkIndexedTypeInt16Array>();
}
extern "C" void vtkIndexedTypeInt16Array_destructor(vtkNew<vtkIndexedTypeInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedTypeInt16Array_get_ptr(vtkNew<vtkIndexedTypeInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" short vtk_indexed_type_int_16_array_get_data_type_value_min(
    vtkNew<vtkIndexedTypeInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" short vtk_indexed_type_int_16_array_get_data_type_value_max(
    vtkNew<vtkIndexedTypeInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" short vtk_indexed_type_int_16_array_get_value(
    vtkNew<vtkIndexedTypeInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_type_int_16_array_get_number_of_values(
    vtkNew<vtkIndexedTypeInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedTypeInt32Array> vtkIndexedTypeInt32Array_new() {
    return vtkNew<vtkIndexedTypeInt32Array>();
}
extern "C" void vtkIndexedTypeInt32Array_destructor(vtkNew<vtkIndexedTypeInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedTypeInt32Array_get_ptr(vtkNew<vtkIndexedTypeInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_indexed_type_int_32_array_get_data_type_value_min(
    vtkNew<vtkIndexedTypeInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" int vtk_indexed_type_int_32_array_get_data_type_value_max(
    vtkNew<vtkIndexedTypeInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" int vtk_indexed_type_int_32_array_get_value(
    vtkNew<vtkIndexedTypeInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_type_int_32_array_get_number_of_values(
    vtkNew<vtkIndexedTypeInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedTypeInt64Array> vtkIndexedTypeInt64Array_new() {
    return vtkNew<vtkIndexedTypeInt64Array>();
}
extern "C" void vtkIndexedTypeInt64Array_destructor(vtkNew<vtkIndexedTypeInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedTypeInt64Array_get_ptr(vtkNew<vtkIndexedTypeInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_indexed_type_int_64_array_get_data_type_value_min(
    vtkNew<vtkIndexedTypeInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_indexed_type_int_64_array_get_data_type_value_max(
    vtkNew<vtkIndexedTypeInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_indexed_type_int_64_array_get_value(
    vtkNew<vtkIndexedTypeInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_type_int_64_array_get_number_of_values(
    vtkNew<vtkIndexedTypeInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedTypeInt8Array> vtkIndexedTypeInt8Array_new() {
    return vtkNew<vtkIndexedTypeInt8Array>();
}
extern "C" void vtkIndexedTypeInt8Array_destructor(vtkNew<vtkIndexedTypeInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedTypeInt8Array_get_ptr(vtkNew<vtkIndexedTypeInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_indexed_type_int_8_array_get_data_type_value_min(
    vtkNew<vtkIndexedTypeInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_indexed_type_int_8_array_get_data_type_value_max(
    vtkNew<vtkIndexedTypeInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_indexed_type_int_8_array_get_value(
    vtkNew<vtkIndexedTypeInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_type_int_8_array_get_number_of_values(
    vtkNew<vtkIndexedTypeInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedTypeUInt16Array> vtkIndexedTypeUInt16Array_new() {
    return vtkNew<vtkIndexedTypeUInt16Array>();
}
extern "C" void vtkIndexedTypeUInt16Array_destructor(vtkNew<vtkIndexedTypeUInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedTypeUInt16Array_get_ptr(vtkNew<vtkIndexedTypeUInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned short vtk_indexed_type_u_int_16_array_get_data_type_value_min(
    vtkNew<vtkIndexedTypeUInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned short vtk_indexed_type_u_int_16_array_get_data_type_value_max(
    vtkNew<vtkIndexedTypeUInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned short vtk_indexed_type_u_int_16_array_get_value(
    vtkNew<vtkIndexedTypeUInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_type_u_int_16_array_get_number_of_values(
    vtkNew<vtkIndexedTypeUInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedTypeUInt32Array> vtkIndexedTypeUInt32Array_new() {
    return vtkNew<vtkIndexedTypeUInt32Array>();
}
extern "C" void vtkIndexedTypeUInt32Array_destructor(vtkNew<vtkIndexedTypeUInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedTypeUInt32Array_get_ptr(vtkNew<vtkIndexedTypeUInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned int vtk_indexed_type_u_int_32_array_get_data_type_value_min(
    vtkNew<vtkIndexedTypeUInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned int vtk_indexed_type_u_int_32_array_get_data_type_value_max(
    vtkNew<vtkIndexedTypeUInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned int vtk_indexed_type_u_int_32_array_get_value(
    vtkNew<vtkIndexedTypeUInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_type_u_int_32_array_get_number_of_values(
    vtkNew<vtkIndexedTypeUInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedTypeUInt64Array> vtkIndexedTypeUInt64Array_new() {
    return vtkNew<vtkIndexedTypeUInt64Array>();
}
extern "C" void vtkIndexedTypeUInt64Array_destructor(vtkNew<vtkIndexedTypeUInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedTypeUInt64Array_get_ptr(vtkNew<vtkIndexedTypeUInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long long vtk_indexed_type_u_int_64_array_get_data_type_value_min(
    vtkNew<vtkIndexedTypeUInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long long vtk_indexed_type_u_int_64_array_get_data_type_value_max(
    vtkNew<vtkIndexedTypeUInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long long vtk_indexed_type_u_int_64_array_get_value(
    vtkNew<vtkIndexedTypeUInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_type_u_int_64_array_get_number_of_values(
    vtkNew<vtkIndexedTypeUInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedTypeUInt8Array> vtkIndexedTypeUInt8Array_new() {
    return vtkNew<vtkIndexedTypeUInt8Array>();
}
extern "C" void vtkIndexedTypeUInt8Array_destructor(vtkNew<vtkIndexedTypeUInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedTypeUInt8Array_get_ptr(vtkNew<vtkIndexedTypeUInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned char vtk_indexed_type_u_int_8_array_get_data_type_value_min(
    vtkNew<vtkIndexedTypeUInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned char vtk_indexed_type_u_int_8_array_get_data_type_value_max(
    vtkNew<vtkIndexedTypeUInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned char vtk_indexed_type_u_int_8_array_get_value(
    vtkNew<vtkIndexedTypeUInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_type_u_int_8_array_get_number_of_values(
    vtkNew<vtkIndexedTypeUInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedUnsignedCharArray> vtkIndexedUnsignedCharArray_new() {
    return vtkNew<vtkIndexedUnsignedCharArray>();
}
extern "C" void vtkIndexedUnsignedCharArray_destructor(vtkNew<vtkIndexedUnsignedCharArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedUnsignedCharArray_get_ptr(vtkNew<vtkIndexedUnsignedCharArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned char vtk_indexed_unsigned_char_array_get_data_type_value_min(
    vtkNew<vtkIndexedUnsignedCharArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned char vtk_indexed_unsigned_char_array_get_data_type_value_max(
    vtkNew<vtkIndexedUnsignedCharArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned char vtk_indexed_unsigned_char_array_get_value(
    vtkNew<vtkIndexedUnsignedCharArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_unsigned_char_array_get_number_of_values(
    vtkNew<vtkIndexedUnsignedCharArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedUnsignedIntArray> vtkIndexedUnsignedIntArray_new() {
    return vtkNew<vtkIndexedUnsignedIntArray>();
}
extern "C" void vtkIndexedUnsignedIntArray_destructor(vtkNew<vtkIndexedUnsignedIntArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedUnsignedIntArray_get_ptr(vtkNew<vtkIndexedUnsignedIntArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned int vtk_indexed_unsigned_int_array_get_data_type_value_min(
    vtkNew<vtkIndexedUnsignedIntArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned int vtk_indexed_unsigned_int_array_get_data_type_value_max(
    vtkNew<vtkIndexedUnsignedIntArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned int vtk_indexed_unsigned_int_array_get_value(
    vtkNew<vtkIndexedUnsignedIntArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_unsigned_int_array_get_number_of_values(
    vtkNew<vtkIndexedUnsignedIntArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedUnsignedLongArray> vtkIndexedUnsignedLongArray_new() {
    return vtkNew<vtkIndexedUnsignedLongArray>();
}
extern "C" void vtkIndexedUnsignedLongArray_destructor(vtkNew<vtkIndexedUnsignedLongArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedUnsignedLongArray_get_ptr(vtkNew<vtkIndexedUnsignedLongArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long vtk_indexed_unsigned_long_array_get_data_type_value_min(
    vtkNew<vtkIndexedUnsignedLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long vtk_indexed_unsigned_long_array_get_data_type_value_max(
    vtkNew<vtkIndexedUnsignedLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long vtk_indexed_unsigned_long_array_get_value(
    vtkNew<vtkIndexedUnsignedLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_unsigned_long_array_get_number_of_values(
    vtkNew<vtkIndexedUnsignedLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedUnsignedLongLongArray> vtkIndexedUnsignedLongLongArray_new() {
    return vtkNew<vtkIndexedUnsignedLongLongArray>();
}
extern "C" void vtkIndexedUnsignedLongLongArray_destructor(
    vtkNew<vtkIndexedUnsignedLongLongArray> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedUnsignedLongLongArray_get_ptr(
    vtkNew<vtkIndexedUnsignedLongLongArray> sself
) {
    return sself.GetPointer();
}
extern "C" unsigned long long vtk_indexed_unsigned_long_long_array_get_data_type_value_min(
    vtkNew<vtkIndexedUnsignedLongLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long long vtk_indexed_unsigned_long_long_array_get_data_type_value_max(
    vtkNew<vtkIndexedUnsignedLongLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long long vtk_indexed_unsigned_long_long_array_get_value(
    vtkNew<vtkIndexedUnsignedLongLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_unsigned_long_long_array_get_number_of_values(
    vtkNew<vtkIndexedUnsignedLongLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkIndexedUnsignedShortArray> vtkIndexedUnsignedShortArray_new() {
    return vtkNew<vtkIndexedUnsignedShortArray>();
}
extern "C" void vtkIndexedUnsignedShortArray_destructor(
    vtkNew<vtkIndexedUnsignedShortArray> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkIndexedUnsignedShortArray_get_ptr(vtkNew<vtkIndexedUnsignedShortArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned short vtk_indexed_unsigned_short_array_get_data_type_value_min(
    vtkNew<vtkIndexedUnsignedShortArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned short vtk_indexed_unsigned_short_array_get_data_type_value_max(
    vtkNew<vtkIndexedUnsignedShortArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned short vtk_indexed_unsigned_short_array_get_value(
    vtkNew<vtkIndexedUnsignedShortArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_indexed_unsigned_short_array_get_number_of_values(
    vtkNew<vtkIndexedUnsignedShortArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkInformation> vtkInformation_new() {
    return vtkNew<vtkInformation>();
}
extern "C" void vtkInformation_destructor(vtkNew<vtkInformation> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkInformation_get_ptr(vtkNew<vtkInformation> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_information_modified(vtkNew<vtkInformation> self) {
    return self->Modified();
}
extern "C" void vtk_information_clear(vtkNew<vtkInformation> self) {
    return self->Clear();
}
extern "C" int vtk_information_get_number_of_keys(vtkNew<vtkInformation> self) {
    return self->GetNumberOfKeys();
}
extern "C" bool vtk_information_uses_garbage_collector(vtkNew<vtkInformation> self) {
    return self->UsesGarbageCollector();
}
extern "C" vtkNew<vtkInformationIterator> vtkInformationIterator_new() {
    return vtkNew<vtkInformationIterator>();
}
extern "C" void vtkInformationIterator_destructor(vtkNew<vtkInformationIterator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkInformationIterator_get_ptr(vtkNew<vtkInformationIterator> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_information_iterator_init_traversal(vtkNew<vtkInformationIterator> self) {
    return self->InitTraversal();
}
extern "C" void vtk_information_iterator_go_to_first_item(vtkNew<vtkInformationIterator> self) {
    return self->GoToFirstItem();
}
extern "C" void vtk_information_iterator_go_to_next_item(vtkNew<vtkInformationIterator> self) {
    return self->GoToNextItem();
}
extern "C" int vtk_information_iterator_is_done_with_traversal(
    vtkNew<vtkInformationIterator> self
) {
    return self->IsDoneWithTraversal();
}
extern "C" vtkNew<vtkInformationKeyLookup> vtkInformationKeyLookup_new() {
    return vtkNew<vtkInformationKeyLookup>();
}
extern "C" void vtkInformationKeyLookup_destructor(vtkNew<vtkInformationKeyLookup> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkInformationKeyLookup_get_ptr(vtkNew<vtkInformationKeyLookup> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkInformationVector> vtkInformationVector_new() {
    return vtkNew<vtkInformationVector>();
}
extern "C" void vtkInformationVector_destructor(vtkNew<vtkInformationVector> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkInformationVector_get_ptr(vtkNew<vtkInformationVector> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_information_vector_get_number_of_information_objects(
    vtkNew<vtkInformationVector> self
) {
    return self->GetNumberOfInformationObjects();
}
extern "C" void vtk_information_vector_set_number_of_information_objects(
    vtkNew<vtkInformationVector> self, int n
) {
    return self->SetNumberOfInformationObjects(n);
}
extern "C" void vtk_information_vector_remove_c_int(vtkNew<vtkInformationVector> self, int idx) {
    return self->Remove(idx);
}
extern "C" bool vtk_information_vector_uses_garbage_collector(vtkNew<vtkInformationVector> self) {
    return self->UsesGarbageCollector();
}
extern "C" vtkNew<vtkIntArray> vtkIntArray_new() {
    return vtkNew<vtkIntArray>();
}
extern "C" void vtkIntArray_destructor(vtkNew<vtkIntArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIntArray_get_ptr(vtkNew<vtkIntArray> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_int_array_get_data_type_value_min(vtkNew<vtkIntArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" int vtk_int_array_get_data_type_value_max(vtkNew<vtkIntArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" int vtk_int_array_get_value(vtkNew<vtkIntArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_int_array_set_value(vtkNew<vtkIntArray> self, long long id, int value) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_int_array_set_number_of_values(vtkNew<vtkIntArray> self, long long number) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_int_array_insert_value(vtkNew<vtkIntArray> self, long long id, int f) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_int_array_insert_next_value(vtkNew<vtkIntArray> self, int f) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_int_array_get_number_of_values(vtkNew<vtkIntArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkInvoker> vtkInvoker_new() {
    return vtkNew<vtkInvoker>();
}
extern "C" void vtkInvoker_destructor(vtkNew<vtkInvoker> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkInvoker_get_ptr(vtkNew<vtkInvoker> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkLongArray> vtkLongArray_new() {
    return vtkNew<vtkLongArray>();
}
extern "C" void vtkLongArray_destructor(vtkNew<vtkLongArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkLongArray_get_ptr(vtkNew<vtkLongArray> sself) {
    return sself.GetPointer();
}
extern "C" long vtk_long_array_get_data_type_value_min(vtkNew<vtkLongArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" long vtk_long_array_get_data_type_value_max(vtkNew<vtkLongArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" long vtk_long_array_get_value(vtkNew<vtkLongArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_long_array_set_value(vtkNew<vtkLongArray> self, long long id, long value) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_long_array_set_number_of_values(vtkNew<vtkLongArray> self, long long number) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_long_array_insert_value(vtkNew<vtkLongArray> self, long long id, long f) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_long_array_insert_next_value(vtkNew<vtkLongArray> self, long f) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_long_array_get_number_of_values(vtkNew<vtkLongArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkLongLongArray> vtkLongLongArray_new() {
    return vtkNew<vtkLongLongArray>();
}
extern "C" void vtkLongLongArray_destructor(vtkNew<vtkLongLongArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkLongLongArray_get_ptr(vtkNew<vtkLongLongArray> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_long_long_array_get_data_type_value_min(vtkNew<vtkLongLongArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_long_long_array_get_data_type_value_max(vtkNew<vtkLongLongArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_long_long_array_get_value(vtkNew<vtkLongLongArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_long_long_array_set_value(
    vtkNew<vtkLongLongArray> self, long long id, long long value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_long_long_array_set_number_of_values(
    vtkNew<vtkLongLongArray> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_long_long_array_insert_value(
    vtkNew<vtkLongLongArray> self, long long id, long long f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_long_long_array_insert_next_value(
    vtkNew<vtkLongLongArray> self, long long f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_long_long_array_get_number_of_values(vtkNew<vtkLongLongArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkLookupTable> vtkLookupTable_new() {
    return vtkNew<vtkLookupTable>();
}
extern "C" void vtkLookupTable_destructor(vtkNew<vtkLookupTable> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkLookupTable_get_ptr(vtkNew<vtkLookupTable> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_lookup_table_is_opaque(vtkNew<vtkLookupTable> self) {
    return self->IsOpaque();
}
extern "C" int vtk_lookup_table_allocate(vtkNew<vtkLookupTable> self, int sz, int ext) {
    return self->Allocate(sz, ext);
}
extern "C" void vtk_lookup_table_build(vtkNew<vtkLookupTable> self) {
    return self->Build();
}
extern "C" void vtk_lookup_table_force_build(vtkNew<vtkLookupTable> self) {
    return self->ForceBuild();
}
extern "C" void vtk_lookup_table_build_special_colors(vtkNew<vtkLookupTable> self) {
    return self->BuildSpecialColors();
}
extern "C" void vtk_lookup_table_set_ramp(vtkNew<vtkLookupTable> self, int _arg) {
    return self->SetRamp(_arg);
}
extern "C" void vtk_lookup_table_set_ramp_to_linear(vtkNew<vtkLookupTable> self) {
    return self->SetRampToLinear();
}
extern "C" void vtk_lookup_table_set_ramp_to_s_curve(vtkNew<vtkLookupTable> self) {
    return self->SetRampToSCurve();
}
extern "C" void vtk_lookup_table_set_ramp_to_sqrt(vtkNew<vtkLookupTable> self) {
    return self->SetRampToSQRT();
}
extern "C" int vtk_lookup_table_get_ramp(vtkNew<vtkLookupTable> self) {
    return self->GetRamp();
}
extern "C" void vtk_lookup_table_set_scale(vtkNew<vtkLookupTable> self, int scale) {
    return self->SetScale(scale);
}
extern "C" void vtk_lookup_table_set_scale_to_linear(vtkNew<vtkLookupTable> self) {
    return self->SetScaleToLinear();
}
extern "C" void vtk_lookup_table_set_scale_to_log_10(vtkNew<vtkLookupTable> self) {
    return self->SetScaleToLog10();
}
extern "C" int vtk_lookup_table_get_scale(vtkNew<vtkLookupTable> self) {
    return self->GetScale();
}
extern "C" void vtk_lookup_table_set_table_range_const_double_2(
    vtkNew<vtkLookupTable> self, const double* r
) {
    return self->SetTableRange(r);
}
extern "C" void vtk_lookup_table_set_table_range_double_double(
    vtkNew<vtkLookupTable> self, double min, double max
) {
    return self->SetTableRange(min, max);
}
extern "C" void vtk_lookup_table_get_table_range_double_2(
    vtkNew<vtkLookupTable> self, double* data
) {
    return self->GetTableRange(data);
}
extern "C" void vtk_lookup_table_set_hue_range_double_double(
    vtkNew<vtkLookupTable> self, double _arg1, double _arg2
) {
    return self->SetHueRange(_arg1, _arg2);
}
extern "C" void vtk_lookup_table_set_hue_range_const_double_2(
    vtkNew<vtkLookupTable> self, const double* _arg
) {
    return self->SetHueRange(_arg);
}
extern "C" void vtk_lookup_table_get_hue_range_double_2(vtkNew<vtkLookupTable> self, double* _arg) {
    return self->GetHueRange(_arg);
}
extern "C" void vtk_lookup_table_set_saturation_range_double_double(
    vtkNew<vtkLookupTable> self, double _arg1, double _arg2
) {
    return self->SetSaturationRange(_arg1, _arg2);
}
extern "C" void vtk_lookup_table_set_saturation_range_const_double_2(
    vtkNew<vtkLookupTable> self, const double* _arg
) {
    return self->SetSaturationRange(_arg);
}
extern "C" void vtk_lookup_table_get_saturation_range_double_2(
    vtkNew<vtkLookupTable> self, double* _arg
) {
    return self->GetSaturationRange(_arg);
}
extern "C" void vtk_lookup_table_set_value_range_double_double(
    vtkNew<vtkLookupTable> self, double _arg1, double _arg2
) {
    return self->SetValueRange(_arg1, _arg2);
}
extern "C" void vtk_lookup_table_set_value_range_const_double_2(
    vtkNew<vtkLookupTable> self, const double* _arg
) {
    return self->SetValueRange(_arg);
}
extern "C" void vtk_lookup_table_get_value_range_double_2(
    vtkNew<vtkLookupTable> self, double* _arg
) {
    return self->GetValueRange(_arg);
}
extern "C" void vtk_lookup_table_set_alpha_range_double_double(
    vtkNew<vtkLookupTable> self, double _arg1, double _arg2
) {
    return self->SetAlphaRange(_arg1, _arg2);
}
extern "C" void vtk_lookup_table_set_alpha_range_const_double_2(
    vtkNew<vtkLookupTable> self, const double* _arg
) {
    return self->SetAlphaRange(_arg);
}
extern "C" void vtk_lookup_table_get_alpha_range_double_2(
    vtkNew<vtkLookupTable> self, double* _arg
) {
    return self->GetAlphaRange(_arg);
}
extern "C" void vtk_lookup_table_set_nan_color_double_double_double_double(
    vtkNew<vtkLookupTable> self, double _arg1, double _arg2, double _arg3, double _arg4
) {
    return self->SetNanColor(_arg1, _arg2, _arg3, _arg4);
}
extern "C" void vtk_lookup_table_set_nan_color_const_double_4(
    vtkNew<vtkLookupTable> self, const double* _arg
) {
    return self->SetNanColor(_arg);
}
extern "C" void vtk_lookup_table_get_nan_color_double_4(vtkNew<vtkLookupTable> self, double* _arg) {
    return self->GetNanColor(_arg);
}
extern "C" void vtk_lookup_table_get_color_as_unsigned_chars(
    vtkNew<vtkLookupTable> self, const double* colorIn, unsigned char* colorOut
) {
    return self->GetColorAsUnsignedChars(colorIn, colorOut);
}
extern "C" void vtk_lookup_table_set_below_range_color_double_double_double_double(
    vtkNew<vtkLookupTable> self, double _arg1, double _arg2, double _arg3, double _arg4
) {
    return self->SetBelowRangeColor(_arg1, _arg2, _arg3, _arg4);
}
extern "C" void vtk_lookup_table_set_below_range_color_const_double_4(
    vtkNew<vtkLookupTable> self, const double* _arg
) {
    return self->SetBelowRangeColor(_arg);
}
extern "C" void vtk_lookup_table_get_below_range_color_double_4(
    vtkNew<vtkLookupTable> self, double* _arg
) {
    return self->GetBelowRangeColor(_arg);
}
extern "C" void vtk_lookup_table_set_use_below_range_color(vtkNew<vtkLookupTable> self, int _arg) {
    return self->SetUseBelowRangeColor(_arg);
}
extern "C" int vtk_lookup_table_get_use_below_range_color(vtkNew<vtkLookupTable> self) {
    return self->GetUseBelowRangeColor();
}
extern "C" void vtk_lookup_table_use_below_range_color_on(vtkNew<vtkLookupTable> self) {
    return self->UseBelowRangeColorOn();
}
extern "C" void vtk_lookup_table_use_below_range_color_off(vtkNew<vtkLookupTable> self) {
    return self->UseBelowRangeColorOff();
}
extern "C" void vtk_lookup_table_set_above_range_color_double_double_double_double(
    vtkNew<vtkLookupTable> self, double _arg1, double _arg2, double _arg3, double _arg4
) {
    return self->SetAboveRangeColor(_arg1, _arg2, _arg3, _arg4);
}
extern "C" void vtk_lookup_table_set_above_range_color_const_double_4(
    vtkNew<vtkLookupTable> self, const double* _arg
) {
    return self->SetAboveRangeColor(_arg);
}
extern "C" void vtk_lookup_table_get_above_range_color_double_4(
    vtkNew<vtkLookupTable> self, double* _arg
) {
    return self->GetAboveRangeColor(_arg);
}
extern "C" void vtk_lookup_table_set_use_above_range_color(vtkNew<vtkLookupTable> self, int _arg) {
    return self->SetUseAboveRangeColor(_arg);
}
extern "C" int vtk_lookup_table_get_use_above_range_color(vtkNew<vtkLookupTable> self) {
    return self->GetUseAboveRangeColor();
}
extern "C" void vtk_lookup_table_use_above_range_color_on(vtkNew<vtkLookupTable> self) {
    return self->UseAboveRangeColorOn();
}
extern "C" void vtk_lookup_table_use_above_range_color_off(vtkNew<vtkLookupTable> self) {
    return self->UseAboveRangeColorOff();
}
extern "C" void vtk_lookup_table_get_color(vtkNew<vtkLookupTable> self, double v, double* rgb) {
    return self->GetColor(v, rgb);
}
extern "C" double vtk_lookup_table_get_opacity(vtkNew<vtkLookupTable> self, double v) {
    return self->GetOpacity(v);
}
extern "C" long long vtk_lookup_table_get_index(vtkNew<vtkLookupTable> self, double v) {
    return self->GetIndex(v);
}
extern "C" void vtk_lookup_table_set_number_of_table_values(
    vtkNew<vtkLookupTable> self, long long number
) {
    return self->SetNumberOfTableValues(number);
}
extern "C" long long vtk_lookup_table_get_number_of_table_values(vtkNew<vtkLookupTable> self) {
    return self->GetNumberOfTableValues();
}
extern "C" void vtk_lookup_table_set_table_value_c_longlong_const_double_4(
    vtkNew<vtkLookupTable> self, long long indx, const double* rgba
) {
    return self->SetTableValue(indx, rgba);
}
extern "C" void vtk_lookup_table_set_table_value_c_longlong_double_double_double_double(
    vtkNew<vtkLookupTable> self, long long indx, double r, double g, double b, double a
) {
    return self->SetTableValue(indx, r, g, b, a);
}
extern "C" void vtk_lookup_table_get_table_value_c_longlong_double_4(
    vtkNew<vtkLookupTable> self, long long indx, double* rgba
) {
    return self->GetTableValue(indx, rgba);
}
extern "C" void vtk_lookup_table_get_log_range(
    vtkNew<vtkLookupTable> self, const double* range, double* log_range
) {
    return self->GetLogRange(range, log_range);
}
extern "C" double vtk_lookup_table_apply_log_scale(
    vtkNew<vtkLookupTable> self, double v, const double* range, const double* log_range
) {
    return self->ApplyLogScale(v, range, log_range);
}
extern "C" void vtk_lookup_table_set_number_of_colors(vtkNew<vtkLookupTable> self, long long _arg) {
    return self->SetNumberOfColors(_arg);
}
extern "C" long long vtk_lookup_table_get_number_of_colors_min_value(vtkNew<vtkLookupTable> self) {
    return self->GetNumberOfColorsMinValue();
}
extern "C" long long vtk_lookup_table_get_number_of_colors_max_value(vtkNew<vtkLookupTable> self) {
    return self->GetNumberOfColorsMaxValue();
}
extern "C" long long vtk_lookup_table_get_number_of_colors(vtkNew<vtkLookupTable> self) {
    return self->GetNumberOfColors();
}
extern "C" int vtk_lookup_table_using_log_scale(vtkNew<vtkLookupTable> self) {
    return self->UsingLogScale();
}
extern "C" void vtk_lookup_table_get_indexed_color(
    vtkNew<vtkLookupTable> self, long long idx, double* rgba
) {
    return self->GetIndexedColor(idx, rgba);
}
extern "C" vtkNew<vtkMarshalContext> vtkMarshalContext_new() {
    return vtkNew<vtkMarshalContext>();
}
extern "C" void vtkMarshalContext_destructor(vtkNew<vtkMarshalContext> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMarshalContext_get_ptr(vtkNew<vtkMarshalContext> sself) {
    return sself.GetPointer();
}
extern "C" bool vtk_marshal_context_un_register_state(
    vtkNew<vtkMarshalContext> self, unsigned int identifier
) {
    return self->UnRegisterState(identifier);
}
extern "C" bool vtk_marshal_context_un_register_object(
    vtkNew<vtkMarshalContext> self, unsigned int identifier
) {
    return self->UnRegisterObject(identifier);
}
extern "C" void vtk_marshal_context_reset_direct_dependencies(vtkNew<vtkMarshalContext> self) {
    return self->ResetDirectDependencies();
}
extern "C" void vtk_marshal_context_reset_direct_dependencies_for_node(
    vtkNew<vtkMarshalContext> self, unsigned int identifier
) {
    return self->ResetDirectDependenciesForNode(identifier);
}
extern "C" unsigned int vtk_marshal_context_make_id(vtkNew<vtkMarshalContext> self) {
    return self->MakeId();
}
extern "C" void vtk_marshal_context_push_parent(
    vtkNew<vtkMarshalContext> self, unsigned int identifier
) {
    return self->PushParent(identifier);
}
extern "C" void vtk_marshal_context_pop_parent(vtkNew<vtkMarshalContext> self) {
    return self->PopParent();
}
extern "C" vtkNew<vtkMath> vtkMath_new() {
    return vtkNew<vtkMath>();
}
extern "C" void vtkMath_destructor(vtkNew<vtkMath> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMath_get_ptr(vtkNew<vtkMath> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_math_dynamic_vector_size(vtkNew<vtkMath> self) {
    return self->DYNAMIC_VECTOR_SIZE();
}
extern "C" double vtk_math_pi(vtkNew<vtkMath> self) {
    return self->Pi();
}
extern "C" float vtk_math_radians_from_degrees_float(vtkNew<vtkMath> self, float degrees) {
    return self->RadiansFromDegrees(degrees);
}
extern "C" double vtk_math_radians_from_degrees_double(vtkNew<vtkMath> self, double degrees) {
    return self->RadiansFromDegrees(degrees);
}
extern "C" float vtk_math_degrees_from_radians_float(vtkNew<vtkMath> self, float radians) {
    return self->DegreesFromRadians(radians);
}
extern "C" double vtk_math_degrees_from_radians_double(vtkNew<vtkMath> self, double radians) {
    return self->DegreesFromRadians(radians);
}
extern "C" int vtk_math_round_float(vtkNew<vtkMath> self, float f) {
    return self->Round(f);
}
extern "C" int vtk_math_round_double(vtkNew<vtkMath> self, double f) {
    return self->Round(f);
}
extern "C" int vtk_math_floor(vtkNew<vtkMath> self, double x) {
    return self->Floor(x);
}
extern "C" int vtk_math_ceil(vtkNew<vtkMath> self, double x) {
    return self->Ceil(x);
}
extern "C" int vtk_math_ceil_log_2(vtkNew<vtkMath> self, unsigned long long x) {
    return self->CeilLog2(x);
}
extern "C" bool vtk_math_is_power_of_two(vtkNew<vtkMath> self, unsigned long long x) {
    return self->IsPowerOfTwo(x);
}
extern "C" int vtk_math_nearest_power_of_two(vtkNew<vtkMath> self, int x) {
    return self->NearestPowerOfTwo(x);
}
extern "C" long long vtk_math_factorial(vtkNew<vtkMath> self, int N) {
    return self->Factorial(N);
}
extern "C" long long vtk_math_binomial(vtkNew<vtkMath> self, int m, int n) {
    return self->Binomial(m, n);
}
extern "C" void vtk_math_random_seed(vtkNew<vtkMath> self, int s) {
    return self->RandomSeed(s);
}
extern "C" int vtk_math_get_seed(vtkNew<vtkMath> self) {
    return self->GetSeed();
}
extern "C" double vtk_math_random(vtkNew<vtkMath> self) {
    return self->Random();
}
extern "C" double vtk_math_random_double_double(vtkNew<vtkMath> self, double min, double max) {
    return self->Random(min, max);
}
extern "C" double vtk_math_gaussian(vtkNew<vtkMath> self) {
    return self->Gaussian();
}
extern "C" double vtk_math_gaussian_double_double(vtkNew<vtkMath> self, double mean, double std) {
    return self->Gaussian(mean, std);
}
extern "C" void vtk_math_assign(vtkNew<vtkMath> self, const double* a, double* b) {
    return self->Assign(a, b);
}
extern "C" void vtk_math_add_const_float_3_const_float_3_float_3(
    vtkNew<vtkMath> self, const float* a, const float* b, float* c
) {
    return self->Add(a, b, c);
}
extern "C" void vtk_math_add_const_double_3_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* a, const double* b, double* c
) {
    return self->Add(a, b, c);
}
extern "C" void vtk_math_subtract_const_float_3_const_float_3_float_3(
    vtkNew<vtkMath> self, const float* a, const float* b, float* c
) {
    return self->Subtract(a, b, c);
}
extern "C" void vtk_math_subtract_const_double_3_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* a, const double* b, double* c
) {
    return self->Subtract(a, b, c);
}
extern "C" void vtk_math_multiply_scalar_float_3_float(vtkNew<vtkMath> self, float* a, float s) {
    return self->MultiplyScalar(a, s);
}
extern "C" void vtk_math_multiply_scalar_2_d_float_2_float(
    vtkNew<vtkMath> self, float* a, float s
) {
    return self->MultiplyScalar2D(a, s);
}
extern "C" void vtk_math_multiply_scalar_double_3_double(
    vtkNew<vtkMath> self, double* a, double s
) {
    return self->MultiplyScalar(a, s);
}
extern "C" void vtk_math_multiply_scalar_2_d_double_2_double(
    vtkNew<vtkMath> self, double* a, double s
) {
    return self->MultiplyScalar2D(a, s);
}
extern "C" float vtk_math_dot_const_float_3_const_float_3(
    vtkNew<vtkMath> self, const float* a, const float* b
) {
    return self->Dot(a, b);
}
extern "C" double vtk_math_dot_const_double_3_const_double_3(
    vtkNew<vtkMath> self, const double* a, const double* b
) {
    return self->Dot(a, b);
}
extern "C" void vtk_math_cross_const_float_3_const_float_3_float_3(
    vtkNew<vtkMath> self, const float* a, const float* b, float* c
) {
    return self->Cross(a, b, c);
}
extern "C" void vtk_math_cross_const_double_3_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* a, const double* b, double* c
) {
    return self->Cross(a, b, c);
}
extern "C" float vtk_math_norm_const_float_3(vtkNew<vtkMath> self, const float* v) {
    return self->Norm(v);
}
extern "C" double vtk_math_norm_const_double_3(vtkNew<vtkMath> self, const double* v) {
    return self->Norm(v);
}
extern "C" float vtk_math_normalize_float_3(vtkNew<vtkMath> self, float* v) {
    return self->Normalize(v);
}
extern "C" double vtk_math_normalize_double_3(vtkNew<vtkMath> self, double* v) {
    return self->Normalize(v);
}
extern "C" void vtk_math_perpendiculars_const_double_3_double_3_double_3_double(
    vtkNew<vtkMath> self, const double* v1, double* v2, double* v3, double theta
) {
    return self->Perpendiculars(v1, v2, v3, theta);
}
extern "C" void vtk_math_perpendiculars_const_float_3_float_3_float_3_double(
    vtkNew<vtkMath> self, const float* v1, float* v2, float* v3, double theta
) {
    return self->Perpendiculars(v1, v2, v3, theta);
}
extern "C" bool vtk_math_project_vector_const_float_3_const_float_3_float_3(
    vtkNew<vtkMath> self, const float* a, const float* b, float* projection
) {
    return self->ProjectVector(a, b, projection);
}
extern "C" bool vtk_math_project_vector_const_double_3_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* a, const double* b, double* projection
) {
    return self->ProjectVector(a, b, projection);
}
extern "C" bool vtk_math_project_vector_2_d_const_float_2_const_float_2_float_2(
    vtkNew<vtkMath> self, const float* a, const float* b, float* projection
) {
    return self->ProjectVector2D(a, b, projection);
}
extern "C" bool vtk_math_project_vector_2_d_const_double_2_const_double_2_double_2(
    vtkNew<vtkMath> self, const double* a, const double* b, double* projection
) {
    return self->ProjectVector2D(a, b, projection);
}
extern "C" float vtk_math_distance_2_between_points_const_float_3_const_float_3(
    vtkNew<vtkMath> self, const float* p1, const float* p2
) {
    return self->Distance2BetweenPoints(p1, p2);
}
extern "C" double vtk_math_distance_2_between_points_const_double_3_const_double_3(
    vtkNew<vtkMath> self, const double* p1, const double* p2
) {
    return self->Distance2BetweenPoints(p1, p2);
}
extern "C" double vtk_math_angle_between_vectors(
    vtkNew<vtkMath> self, const double* v1, const double* v2
) {
    return self->AngleBetweenVectors(v1, v2);
}
extern "C" double vtk_math_signed_angle_between_vectors(
    vtkNew<vtkMath> self, const double* v1, const double* v2, const double* vn
) {
    return self->SignedAngleBetweenVectors(v1, v2, vn);
}
extern "C" double vtk_math_gaussian_amplitude_double_double(
    vtkNew<vtkMath> self, double variance, double distanceFromMean
) {
    return self->GaussianAmplitude(variance, distanceFromMean);
}
extern "C" double vtk_math_gaussian_amplitude_double_double_double(
    vtkNew<vtkMath> self, double mean, double variance, double position
) {
    return self->GaussianAmplitude(mean, variance, position);
}
extern "C" double vtk_math_gaussian_weight_double_double(
    vtkNew<vtkMath> self, double variance, double distanceFromMean
) {
    return self->GaussianWeight(variance, distanceFromMean);
}
extern "C" double vtk_math_gaussian_weight_double_double_double(
    vtkNew<vtkMath> self, double mean, double variance, double position
) {
    return self->GaussianWeight(mean, variance, position);
}
extern "C" float vtk_math_dot_2_d_const_float_2_const_float_2(
    vtkNew<vtkMath> self, const float* x, const float* y
) {
    return self->Dot2D(x, y);
}
extern "C" double vtk_math_dot_2_d_const_double_2_const_double_2(
    vtkNew<vtkMath> self, const double* x, const double* y
) {
    return self->Dot2D(x, y);
}
extern "C" float vtk_math_norm_2_d_const_float_2(vtkNew<vtkMath> self, const float* x) {
    return self->Norm2D(x);
}
extern "C" double vtk_math_norm_2_d_const_double_2(vtkNew<vtkMath> self, const double* x) {
    return self->Norm2D(x);
}
extern "C" float vtk_math_normalize_2_d_float_2(vtkNew<vtkMath> self, float* v) {
    return self->Normalize2D(v);
}
extern "C" double vtk_math_normalize_2_d_double_2(vtkNew<vtkMath> self, double* v) {
    return self->Normalize2D(v);
}
extern "C" float vtk_math_determinant_2_x_2_const_float_2_const_float_2(
    vtkNew<vtkMath> self, const float* c1, const float* c2
) {
    return self->Determinant2x2(c1, c2);
}
extern "C" double vtk_math_determinant_2_x_2_double_double_double_double(
    vtkNew<vtkMath> self, double a, double b, double c, double d
) {
    return self->Determinant2x2(a, b, c, d);
}
extern "C" double vtk_math_determinant_2_x_2_const_double_2_const_double_2(
    vtkNew<vtkMath> self, const double* c1, const double* c2
) {
    return self->Determinant2x2(c1, c2);
}
extern "C" float vtk_math_determinant_3_x_3_const_float_3_const_float_3_const_float_3(
    vtkNew<vtkMath> self, const float* c1, const float* c2, const float* c3
) {
    return self->Determinant3x3(c1, c2, c3);
}
extern "C" double vtk_math_determinant_3_x_3_const_double_3_const_double_3_const_double_3(
    vtkNew<vtkMath> self, const double* c1, const double* c2, const double* c3
) {
    return self->Determinant3x3(c1, c2, c3);
}
extern "C" double vtk_math_determinant_3_x_3_double_double_double_double_double_double_double_double_double(
    vtkNew<vtkMath> self,
    double a1,
    double a2,
    double a3,
    double b1,
    double b2,
    double b3,
    double c1,
    double c2,
    double c3
) {
    return self->Determinant3x3(a1, a2, a3, b1, b2, b3, c1, c2, c3);
}
extern "C" void vtk_math_multiply_quaternion_const_float_4_const_float_4_float_4(
    vtkNew<vtkMath> self, const float* q1, const float* q2, float* q
) {
    return self->MultiplyQuaternion(q1, q2, q);
}
extern "C" void vtk_math_multiply_quaternion_const_double_4_const_double_4_double_4(
    vtkNew<vtkMath> self, const double* q1, const double* q2, double* q
) {
    return self->MultiplyQuaternion(q1, q2, q);
}
extern "C" void vtk_math_rotate_vector_by_normalized_quaternion_const_float_3_const_float_4_float_3(
    vtkNew<vtkMath> self, const float* v, const float* q, float* r
) {
    return self->RotateVectorByNormalizedQuaternion(v, q, r);
}
extern "C" void vtk_math_rotate_vector_by_normalized_quaternion_const_double_3_const_double_4_double_3(
    vtkNew<vtkMath> self, const double* v, const double* q, double* r
) {
    return self->RotateVectorByNormalizedQuaternion(v, q, r);
}
extern "C" void vtk_math_rotate_vector_by_wxyz_const_float_3_const_float_4_float_3(
    vtkNew<vtkMath> self, const float* v, const float* q, float* r
) {
    return self->RotateVectorByWXYZ(v, q, r);
}
extern "C" void vtk_math_rotate_vector_by_wxyz_const_double_3_const_double_4_double_3(
    vtkNew<vtkMath> self, const double* v, const double* q, double* r
) {
    return self->RotateVectorByWXYZ(v, q, r);
}
extern "C" void vtk_math_rgb_to_hsv_const_float_3_float_3(
    vtkNew<vtkMath> self, const float* rgb, float* hsv
) {
    return self->RGBToHSV(rgb, hsv);
}
extern "C" void vtk_math_rgb_to_hsv_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* rgb, double* hsv
) {
    return self->RGBToHSV(rgb, hsv);
}
extern "C" void vtk_math_hsv_to_rgb_const_float_3_float_3(
    vtkNew<vtkMath> self, const float* hsv, float* rgb
) {
    return self->HSVToRGB(hsv, rgb);
}
extern "C" void vtk_math_hsv_to_rgb_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* hsv, double* rgb
) {
    return self->HSVToRGB(hsv, rgb);
}
extern "C" void vtk_math_pro_lab_to_xyz_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* prolab, double* xyz
) {
    return self->ProLabToXYZ(prolab, xyz);
}
extern "C" void vtk_math_xyz_to_pro_lab_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* xyz, double* prolab
) {
    return self->XYZToProLab(xyz, prolab);
}
extern "C" void vtk_math_lab_to_xyz_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* lab, double* xyz
) {
    return self->LabToXYZ(lab, xyz);
}
extern "C" void vtk_math_xyz_to_lab_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* xyz, double* lab
) {
    return self->XYZToLab(xyz, lab);
}
extern "C" void vtk_math_xyz_to_rgb_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* xyz, double* rgb
) {
    return self->XYZToRGB(xyz, rgb);
}
extern "C" void vtk_math_rgb_to_xyz_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* rgb, double* xyz
) {
    return self->RGBToXYZ(rgb, xyz);
}
extern "C" void vtk_math_rgb_to_lab_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* rgb, double* lab
) {
    return self->RGBToLab(rgb, lab);
}
extern "C" void vtk_math_pro_lab_to_rgb_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* prolab, double* rgb
) {
    return self->ProLabToRGB(prolab, rgb);
}
extern "C" void vtk_math_rgb_to_pro_lab_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* rgb, double* prolab
) {
    return self->RGBToProLab(rgb, prolab);
}
extern "C" void vtk_math_lab_to_rgb_const_double_3_double_3(
    vtkNew<vtkMath> self, const double* lab, double* rgb
) {
    return self->LabToRGB(lab, rgb);
}
extern "C" void vtk_math_uninitialize_bounds(vtkNew<vtkMath> self, double* bounds) {
    return self->UninitializeBounds(bounds);
}
extern "C" int vtk_math_are_bounds_initialized(vtkNew<vtkMath> self, const double* bounds) {
    return self->AreBoundsInitialized(bounds);
}
extern "C" double vtk_math_clamp_and_normalize_value(
    vtkNew<vtkMath> self, double value, const double* range
) {
    return self->ClampAndNormalizeValue(value, range);
}
extern "C" int vtk_math_get_scalar_type_fitting_range(
    vtkNew<vtkMath> self, double range_min, double range_max, double scale, double shift
) {
    return self->GetScalarTypeFittingRange(range_min, range_max, scale, shift);
}
extern "C" int vtk_math_extent_is_within_other_extent(
    vtkNew<vtkMath> self, const int* extent1, const int* extent2
) {
    return self->ExtentIsWithinOtherExtent(extent1, extent2);
}
extern "C" int vtk_math_bounds_is_within_other_bounds(
    vtkNew<vtkMath> self, const double* bounds1, const double* bounds2, const double* delta
) {
    return self->BoundsIsWithinOtherBounds(bounds1, bounds2, delta);
}
extern "C" int vtk_math_point_is_within_bounds(
    vtkNew<vtkMath> self, const double* point, const double* bounds, const double* delta
) {
    return self->PointIsWithinBounds(point, bounds, delta);
}
extern "C" int vtk_math_plane_intersects_aabb(
    vtkNew<vtkMath> self, const double* bounds, const double* normal, const double* point
) {
    return self->PlaneIntersectsAABB(bounds, normal, point);
}
extern "C" double vtk_math_solve_3_point_circle(
    vtkNew<vtkMath> self, const double* p1, const double* p2, const double* p3, double* center
) {
    return self->Solve3PointCircle(p1, p2, p3, center);
}
extern "C" double vtk_math_inf(vtkNew<vtkMath> self) {
    return self->Inf();
}
extern "C" double vtk_math_neg_inf(vtkNew<vtkMath> self) {
    return self->NegInf();
}
extern "C" double vtk_math_nan(vtkNew<vtkMath> self) {
    return self->Nan();
}
extern "C" int vtk_math_is_inf(vtkNew<vtkMath> self, double x) {
    return self->IsInf(x);
}
extern "C" int vtk_math_is_nan(vtkNew<vtkMath> self, double x) {
    return self->IsNan(x);
}
extern "C" bool vtk_math_is_finite(vtkNew<vtkMath> self, double x) {
    return self->IsFinite(x);
}
extern "C" long long vtk_math_compute_gcd(vtkNew<vtkMath> self, long long m, long long n) {
    return self->ComputeGCD(m, n);
}
extern "C" void vtk_math_get_point_along_line(
    vtkNew<vtkMath> self, double* result, double* p1, double* p2, double offset
) {
    return self->GetPointAlongLine(result, p1, p2, offset);
}
extern "C" vtkNew<vtkMersenneTwister> vtkMersenneTwister_new() {
    return vtkNew<vtkMersenneTwister>();
}
extern "C" void vtkMersenneTwister_destructor(vtkNew<vtkMersenneTwister> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMersenneTwister_get_ptr(vtkNew<vtkMersenneTwister> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_mersenne_twister_initialize(
    vtkNew<vtkMersenneTwister> self, unsigned int seed
) {
    return self->Initialize(seed);
}
extern "C" unsigned int vtk_mersenne_twister_initialize_new_sequence(
    vtkNew<vtkMersenneTwister> self, unsigned int seed, int p
) {
    return self->InitializeNewSequence(seed, p);
}
extern "C" void vtk_mersenne_twister_initialize_sequence(
    vtkNew<vtkMersenneTwister> self, unsigned int id, unsigned int seed, int p
) {
    return self->InitializeSequence(id, seed, p);
}
extern "C" double vtk_mersenne_twister_get_value_c_uint(
    vtkNew<vtkMersenneTwister> self, unsigned int id
) {
    return self->GetValue(id);
}
extern "C" double vtk_mersenne_twister_get_value(vtkNew<vtkMersenneTwister> self) {
    return self->GetValue();
}
extern "C" void vtk_mersenne_twister_next_c_uint(vtkNew<vtkMersenneTwister> self, unsigned int id) {
    return self->Next(id);
}
extern "C" void vtk_mersenne_twister_next(vtkNew<vtkMersenneTwister> self) {
    return self->Next();
}
extern "C" vtkNew<vtkMinimalStandardRandomSequence> vtkMinimalStandardRandomSequence_new() {
    return vtkNew<vtkMinimalStandardRandomSequence>();
}
extern "C" void vtkMinimalStandardRandomSequence_destructor(
    vtkNew<vtkMinimalStandardRandomSequence> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkMinimalStandardRandomSequence_get_ptr(
    vtkNew<vtkMinimalStandardRandomSequence> sself
) {
    return sself.GetPointer();
}
extern "C" void vtk_minimal_standard_random_sequence_initialize(
    vtkNew<vtkMinimalStandardRandomSequence> self, unsigned int seed
) {
    return self->Initialize(seed);
}
extern "C" void vtk_minimal_standard_random_sequence_set_seed(
    vtkNew<vtkMinimalStandardRandomSequence> self, int value
) {
    return self->SetSeed(value);
}
extern "C" void vtk_minimal_standard_random_sequence_set_seed_only(
    vtkNew<vtkMinimalStandardRandomSequence> self, int value
) {
    return self->SetSeedOnly(value);
}
extern "C" int vtk_minimal_standard_random_sequence_get_seed(
    vtkNew<vtkMinimalStandardRandomSequence> self
) {
    return self->GetSeed();
}
extern "C" double vtk_minimal_standard_random_sequence_get_value(
    vtkNew<vtkMinimalStandardRandomSequence> self
) {
    return self->GetValue();
}
extern "C" void vtk_minimal_standard_random_sequence_next(
    vtkNew<vtkMinimalStandardRandomSequence> self
) {
    return self->Next();
}
extern "C" double vtk_minimal_standard_random_sequence_get_range_value(
    vtkNew<vtkMinimalStandardRandomSequence> self, double rangeMin, double rangeMax
) {
    return self->GetRangeValue(rangeMin, rangeMax);
}
extern "C" double vtk_minimal_standard_random_sequence_get_next_range_value(
    vtkNew<vtkMinimalStandardRandomSequence> self, double rangeMin, double rangeMax
) {
    return self->GetNextRangeValue(rangeMin, rangeMax);
}
extern "C" vtkNew<vtkMultiThreader> vtkMultiThreader_new() {
    return vtkNew<vtkMultiThreader>();
}
extern "C" void vtkMultiThreader_destructor(vtkNew<vtkMultiThreader> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMultiThreader_get_ptr(vtkNew<vtkMultiThreader> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_multi_threader_set_number_of_threads(vtkNew<vtkMultiThreader> self, int _arg) {
    return self->SetNumberOfThreads(_arg);
}
extern "C" int vtk_multi_threader_get_number_of_threads_min_value(vtkNew<vtkMultiThreader> self) {
    return self->GetNumberOfThreadsMinValue();
}
extern "C" int vtk_multi_threader_get_number_of_threads_max_value(vtkNew<vtkMultiThreader> self) {
    return self->GetNumberOfThreadsMaxValue();
}
extern "C" int vtk_multi_threader_get_number_of_threads(vtkNew<vtkMultiThreader> self) {
    return self->GetNumberOfThreads();
}
extern "C" int vtk_multi_threader_get_global_static_maximum_number_of_threads(
    vtkNew<vtkMultiThreader> self
) {
    return self->GetGlobalStaticMaximumNumberOfThreads();
}
extern "C" void vtk_multi_threader_set_global_maximum_number_of_threads(
    vtkNew<vtkMultiThreader> self, int val
) {
    return self->SetGlobalMaximumNumberOfThreads(val);
}
extern "C" int vtk_multi_threader_get_global_maximum_number_of_threads(
    vtkNew<vtkMultiThreader> self
) {
    return self->GetGlobalMaximumNumberOfThreads();
}
extern "C" void vtk_multi_threader_set_global_default_number_of_threads(
    vtkNew<vtkMultiThreader> self, int val
) {
    return self->SetGlobalDefaultNumberOfThreads(val);
}
extern "C" int vtk_multi_threader_get_global_default_number_of_threads(
    vtkNew<vtkMultiThreader> self
) {
    return self->GetGlobalDefaultNumberOfThreads();
}
extern "C" void vtk_multi_threader_single_method_execute(vtkNew<vtkMultiThreader> self) {
    return self->SingleMethodExecute();
}
extern "C" void vtk_multi_threader_multiple_method_execute(vtkNew<vtkMultiThreader> self) {
    return self->MultipleMethodExecute();
}
extern "C" void vtk_multi_threader_terminate_thread(vtkNew<vtkMultiThreader> self, int threadId) {
    return self->TerminateThread(threadId);
}
extern "C" int vtk_multi_threader_is_thread_active(vtkNew<vtkMultiThreader> self, int threadId) {
    return self->IsThreadActive(threadId);
}
extern "C" vtkNew<vtkObject> vtkObject_new() {
    return vtkNew<vtkObject>();
}
extern "C" void vtkObject_destructor(vtkNew<vtkObject> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkObject_get_ptr(vtkNew<vtkObject> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_object_debug_on(vtkNew<vtkObject> self) {
    return self->DebugOn();
}
extern "C" void vtk_object_debug_off(vtkNew<vtkObject> self) {
    return self->DebugOff();
}
extern "C" bool vtk_object_get_debug(vtkNew<vtkObject> self) {
    return self->GetDebug();
}
extern "C" void vtk_object_set_debug(vtkNew<vtkObject> self, bool debugFlag) {
    return self->SetDebug(debugFlag);
}
extern "C" void vtk_object_break_on_error(vtkNew<vtkObject> self) {
    return self->BreakOnError();
}
extern "C" void vtk_object_modified(vtkNew<vtkObject> self) {
    return self->Modified();
}
extern "C" unsigned long vtk_object_get_m_time(vtkNew<vtkObject> self) {
    return self->GetMTime();
}
extern "C" void vtk_object_set_global_warning_display(vtkNew<vtkObject> self, int val) {
    return self->SetGlobalWarningDisplay(val);
}
extern "C" void vtk_object_global_warning_display_on(vtkNew<vtkObject> self) {
    return self->GlobalWarningDisplayOn();
}
extern "C" void vtk_object_global_warning_display_off(vtkNew<vtkObject> self) {
    return self->GlobalWarningDisplayOff();
}
extern "C" int vtk_object_get_global_warning_display(vtkNew<vtkObject> self) {
    return self->GetGlobalWarningDisplay();
}
extern "C" void vtk_object_remove_observer_c_ulong(vtkNew<vtkObject> self, unsigned long tag) {
    return self->RemoveObserver(tag);
}
extern "C" void vtk_object_remove_observers_c_ulong(vtkNew<vtkObject> self, unsigned long event) {
    return self->RemoveObservers(event);
}
extern "C" void vtk_object_remove_all_observers(vtkNew<vtkObject> self) {
    return self->RemoveAllObservers();
}
extern "C" int vtk_object_has_observer_c_ulong(vtkNew<vtkObject> self, unsigned long event) {
    return self->HasObserver(event);
}
extern "C" int vtk_object_invoke_event_c_ulong(vtkNew<vtkObject> self, unsigned long event) {
    return self->InvokeEvent(event);
}
extern "C" vtkNew<vtkObjectFactoryCollection> vtkObjectFactoryCollection_new() {
    return vtkNew<vtkObjectFactoryCollection>();
}
extern "C" void vtkObjectFactoryCollection_destructor(vtkNew<vtkObjectFactoryCollection> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkObjectFactoryCollection_get_ptr(vtkNew<vtkObjectFactoryCollection> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkOldStyleCallbackCommand> vtkOldStyleCallbackCommand_new() {
    return vtkNew<vtkOldStyleCallbackCommand>();
}
extern "C" void vtkOldStyleCallbackCommand_destructor(vtkNew<vtkOldStyleCallbackCommand> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkOldStyleCallbackCommand_get_ptr(vtkNew<vtkOldStyleCallbackCommand> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkOutputWindow> vtkOutputWindow_new() {
    return vtkNew<vtkOutputWindow>();
}
extern "C" void vtkOutputWindow_destructor(vtkNew<vtkOutputWindow> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkOutputWindow_get_ptr(vtkNew<vtkOutputWindow> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_output_window_prompt_user_on(vtkNew<vtkOutputWindow> self) {
    return self->PromptUserOn();
}
extern "C" void vtk_output_window_prompt_user_off(vtkNew<vtkOutputWindow> self) {
    return self->PromptUserOff();
}
extern "C" void vtk_output_window_set_prompt_user(vtkNew<vtkOutputWindow> self, bool _arg) {
    return self->SetPromptUser(_arg);
}
extern "C" void vtk_output_window_set_display_mode(vtkNew<vtkOutputWindow> self, int _arg) {
    return self->SetDisplayMode(_arg);
}
extern "C" int vtk_output_window_get_display_mode_min_value(vtkNew<vtkOutputWindow> self) {
    return self->GetDisplayModeMinValue();
}
extern "C" int vtk_output_window_get_display_mode_max_value(vtkNew<vtkOutputWindow> self) {
    return self->GetDisplayModeMaxValue();
}
extern "C" int vtk_output_window_get_display_mode(vtkNew<vtkOutputWindow> self) {
    return self->GetDisplayMode();
}
extern "C" void vtk_output_window_set_display_mode_to_default(vtkNew<vtkOutputWindow> self) {
    return self->SetDisplayModeToDefault();
}
extern "C" void vtk_output_window_set_display_mode_to_never(vtkNew<vtkOutputWindow> self) {
    return self->SetDisplayModeToNever();
}
extern "C" void vtk_output_window_set_display_mode_to_always(vtkNew<vtkOutputWindow> self) {
    return self->SetDisplayModeToAlways();
}
extern "C" void vtk_output_window_set_display_mode_to_always_std_err(vtkNew<vtkOutputWindow> self) {
    return self->SetDisplayModeToAlwaysStdErr();
}
extern "C" vtkNew<vtkOverrideInformationCollection> vtkOverrideInformationCollection_new() {
    return vtkNew<vtkOverrideInformationCollection>();
}
extern "C" void vtkOverrideInformationCollection_destructor(
    vtkNew<vtkOverrideInformationCollection> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkOverrideInformationCollection_get_ptr(
    vtkNew<vtkOverrideInformationCollection> sself
) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkPoints> vtkPoints_new() {
    return vtkNew<vtkPoints>();
}
extern "C" void vtkPoints_destructor(vtkNew<vtkPoints> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPoints_get_ptr(vtkNew<vtkPoints> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_points_allocate(vtkNew<vtkPoints> self, long long sz, long long ext) {
    return self->Allocate(sz, ext);
}
extern "C" void vtk_points_initialize(vtkNew<vtkPoints> self) {
    return self->Initialize();
}
extern "C" int vtk_points_get_data_type(vtkNew<vtkPoints> self) {
    return self->GetDataType();
}
extern "C" void vtk_points_set_data_type(vtkNew<vtkPoints> self, int dataType) {
    return self->SetDataType(dataType);
}
extern "C" void vtk_points_set_data_type_to_bit(vtkNew<vtkPoints> self) {
    return self->SetDataTypeToBit();
}
extern "C" void vtk_points_set_data_type_to_char(vtkNew<vtkPoints> self) {
    return self->SetDataTypeToChar();
}
extern "C" void vtk_points_set_data_type_to_unsigned_char(vtkNew<vtkPoints> self) {
    return self->SetDataTypeToUnsignedChar();
}
extern "C" void vtk_points_set_data_type_to_short(vtkNew<vtkPoints> self) {
    return self->SetDataTypeToShort();
}
extern "C" void vtk_points_set_data_type_to_unsigned_short(vtkNew<vtkPoints> self) {
    return self->SetDataTypeToUnsignedShort();
}
extern "C" void vtk_points_set_data_type_to_int(vtkNew<vtkPoints> self) {
    return self->SetDataTypeToInt();
}
extern "C" void vtk_points_set_data_type_to_unsigned_int(vtkNew<vtkPoints> self) {
    return self->SetDataTypeToUnsignedInt();
}
extern "C" void vtk_points_set_data_type_to_long(vtkNew<vtkPoints> self) {
    return self->SetDataTypeToLong();
}
extern "C" void vtk_points_set_data_type_to_unsigned_long(vtkNew<vtkPoints> self) {
    return self->SetDataTypeToUnsignedLong();
}
extern "C" void vtk_points_set_data_type_to_float(vtkNew<vtkPoints> self) {
    return self->SetDataTypeToFloat();
}
extern "C" void vtk_points_set_data_type_to_double(vtkNew<vtkPoints> self) {
    return self->SetDataTypeToDouble();
}
extern "C" void vtk_points_squeeze(vtkNew<vtkPoints> self) {
    return self->Squeeze();
}
extern "C" void vtk_points_reset(vtkNew<vtkPoints> self) {
    return self->Reset();
}
extern "C" unsigned long vtk_points_get_actual_memory_size(vtkNew<vtkPoints> self) {
    return self->GetActualMemorySize();
}
extern "C" long long vtk_points_get_number_of_points(vtkNew<vtkPoints> self) {
    return self->GetNumberOfPoints();
}
extern "C" void vtk_points_get_point_c_longlong_double_3(
    vtkNew<vtkPoints> self, long long id, double* x
) {
    return self->GetPoint(id, x);
}
extern "C" void vtk_points_set_point_c_longlong_const_float_3(
    vtkNew<vtkPoints> self, long long id, const float* x
) {
    return self->SetPoint(id, x);
}
extern "C" void vtk_points_set_point_c_longlong_const_double_3(
    vtkNew<vtkPoints> self, long long id, const double* x
) {
    return self->SetPoint(id, x);
}
extern "C" void vtk_points_set_point_c_longlong_double_double_double(
    vtkNew<vtkPoints> self, long long id, double x, double y, double z
) {
    return self->SetPoint(id, x, y, z);
}
extern "C" void vtk_points_insert_point_c_longlong_const_float_3(
    vtkNew<vtkPoints> self, long long id, const float* x
) {
    return self->InsertPoint(id, x);
}
extern "C" void vtk_points_insert_point_c_longlong_const_double_3(
    vtkNew<vtkPoints> self, long long id, const double* x
) {
    return self->InsertPoint(id, x);
}
extern "C" void vtk_points_insert_point_c_longlong_double_double_double(
    vtkNew<vtkPoints> self, long long id, double x, double y, double z
) {
    return self->InsertPoint(id, x, y, z);
}
extern "C" long long vtk_points_insert_next_point_const_float_3(
    vtkNew<vtkPoints> self, const float* x
) {
    return self->InsertNextPoint(x);
}
extern "C" long long vtk_points_insert_next_point_const_double_3(
    vtkNew<vtkPoints> self, const double* x
) {
    return self->InsertNextPoint(x);
}
extern "C" long long vtk_points_insert_next_point_double_double_double(
    vtkNew<vtkPoints> self, double x, double y, double z
) {
    return self->InsertNextPoint(x, y, z);
}
extern "C" void vtk_points_set_number_of_points(vtkNew<vtkPoints> self, long long numPoints) {
    return self->SetNumberOfPoints(numPoints);
}
extern "C" int vtk_points_resize(vtkNew<vtkPoints> self, long long numPoints) {
    return self->Resize(numPoints);
}
extern "C" void vtk_points_compute_bounds(vtkNew<vtkPoints> self) {
    return self->ComputeBounds();
}
extern "C" void vtk_points_get_bounds_double_6(vtkNew<vtkPoints> self, double* bounds) {
    return self->GetBounds(bounds);
}
extern "C" unsigned long vtk_points_get_m_time(vtkNew<vtkPoints> self) {
    return self->GetMTime();
}
extern "C" void vtk_points_modified(vtkNew<vtkPoints> self) {
    return self->Modified();
}
extern "C" vtkNew<vtkPoints2D> vtkPoints2D_new() {
    return vtkNew<vtkPoints2D>();
}
extern "C" void vtkPoints2D_destructor(vtkNew<vtkPoints2D> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPoints2D_get_ptr(vtkNew<vtkPoints2D> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_points_2_d_allocate(vtkNew<vtkPoints2D> self, long long sz, long long ext) {
    return self->Allocate(sz, ext);
}
extern "C" void vtk_points_2_d_initialize(vtkNew<vtkPoints2D> self) {
    return self->Initialize();
}
extern "C" int vtk_points_2_d_get_data_type(vtkNew<vtkPoints2D> self) {
    return self->GetDataType();
}
extern "C" void vtk_points_2_d_set_data_type(vtkNew<vtkPoints2D> self, int dataType) {
    return self->SetDataType(dataType);
}
extern "C" void vtk_points_2_d_set_data_type_to_bit(vtkNew<vtkPoints2D> self) {
    return self->SetDataTypeToBit();
}
extern "C" void vtk_points_2_d_set_data_type_to_char(vtkNew<vtkPoints2D> self) {
    return self->SetDataTypeToChar();
}
extern "C" void vtk_points_2_d_set_data_type_to_unsigned_char(vtkNew<vtkPoints2D> self) {
    return self->SetDataTypeToUnsignedChar();
}
extern "C" void vtk_points_2_d_set_data_type_to_short(vtkNew<vtkPoints2D> self) {
    return self->SetDataTypeToShort();
}
extern "C" void vtk_points_2_d_set_data_type_to_unsigned_short(vtkNew<vtkPoints2D> self) {
    return self->SetDataTypeToUnsignedShort();
}
extern "C" void vtk_points_2_d_set_data_type_to_int(vtkNew<vtkPoints2D> self) {
    return self->SetDataTypeToInt();
}
extern "C" void vtk_points_2_d_set_data_type_to_unsigned_int(vtkNew<vtkPoints2D> self) {
    return self->SetDataTypeToUnsignedInt();
}
extern "C" void vtk_points_2_d_set_data_type_to_long(vtkNew<vtkPoints2D> self) {
    return self->SetDataTypeToLong();
}
extern "C" void vtk_points_2_d_set_data_type_to_unsigned_long(vtkNew<vtkPoints2D> self) {
    return self->SetDataTypeToUnsignedLong();
}
extern "C" void vtk_points_2_d_set_data_type_to_float(vtkNew<vtkPoints2D> self) {
    return self->SetDataTypeToFloat();
}
extern "C" void vtk_points_2_d_set_data_type_to_double(vtkNew<vtkPoints2D> self) {
    return self->SetDataTypeToDouble();
}
extern "C" void vtk_points_2_d_squeeze(vtkNew<vtkPoints2D> self) {
    return self->Squeeze();
}
extern "C" void vtk_points_2_d_reset(vtkNew<vtkPoints2D> self) {
    return self->Reset();
}
extern "C" unsigned long vtk_points_2_d_get_actual_memory_size(vtkNew<vtkPoints2D> self) {
    return self->GetActualMemorySize();
}
extern "C" long long vtk_points_2_d_get_number_of_points(vtkNew<vtkPoints2D> self) {
    return self->GetNumberOfPoints();
}
extern "C" void vtk_points_2_d_get_point_c_longlong_double_2(
    vtkNew<vtkPoints2D> self, long long id, double* x
) {
    return self->GetPoint(id, x);
}
extern "C" void vtk_points_2_d_set_point_c_longlong_const_float_2(
    vtkNew<vtkPoints2D> self, long long id, const float* x
) {
    return self->SetPoint(id, x);
}
extern "C" void vtk_points_2_d_set_point_c_longlong_const_double_2(
    vtkNew<vtkPoints2D> self, long long id, const double* x
) {
    return self->SetPoint(id, x);
}
extern "C" void vtk_points_2_d_set_point_c_longlong_double_double(
    vtkNew<vtkPoints2D> self, long long id, double x, double y
) {
    return self->SetPoint(id, x, y);
}
extern "C" void vtk_points_2_d_insert_point_c_longlong_const_float_2(
    vtkNew<vtkPoints2D> self, long long id, const float* x
) {
    return self->InsertPoint(id, x);
}
extern "C" void vtk_points_2_d_insert_point_c_longlong_const_double_2(
    vtkNew<vtkPoints2D> self, long long id, const double* x
) {
    return self->InsertPoint(id, x);
}
extern "C" void vtk_points_2_d_insert_point_c_longlong_double_double(
    vtkNew<vtkPoints2D> self, long long id, double x, double y
) {
    return self->InsertPoint(id, x, y);
}
extern "C" long long vtk_points_2_d_insert_next_point_const_float_2(
    vtkNew<vtkPoints2D> self, const float* x
) {
    return self->InsertNextPoint(x);
}
extern "C" long long vtk_points_2_d_insert_next_point_const_double_2(
    vtkNew<vtkPoints2D> self, const double* x
) {
    return self->InsertNextPoint(x);
}
extern "C" long long vtk_points_2_d_insert_next_point_double_double(
    vtkNew<vtkPoints2D> self, double x, double y
) {
    return self->InsertNextPoint(x, y);
}
extern "C" void vtk_points_2_d_remove_point(vtkNew<vtkPoints2D> self, long long id) {
    return self->RemovePoint(id);
}
extern "C" void vtk_points_2_d_set_number_of_points(vtkNew<vtkPoints2D> self, long long numPoints) {
    return self->SetNumberOfPoints(numPoints);
}
extern "C" int vtk_points_2_d_resize(vtkNew<vtkPoints2D> self, long long numPoints) {
    return self->Resize(numPoints);
}
extern "C" void vtk_points_2_d_compute_bounds(vtkNew<vtkPoints2D> self) {
    return self->ComputeBounds();
}
extern "C" void vtk_points_2_d_get_bounds_double_4(vtkNew<vtkPoints2D> self, double* bounds) {
    return self->GetBounds(bounds);
}
extern "C" vtkNew<vtkPriorityQueue> vtkPriorityQueue_new() {
    return vtkNew<vtkPriorityQueue>();
}
extern "C" void vtkPriorityQueue_destructor(vtkNew<vtkPriorityQueue> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPriorityQueue_get_ptr(vtkNew<vtkPriorityQueue> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_priority_queue_allocate(
    vtkNew<vtkPriorityQueue> self, long long sz, long long ext
) {
    return self->Allocate(sz, ext);
}
extern "C" void vtk_priority_queue_insert(
    vtkNew<vtkPriorityQueue> self, double priority, long long id
) {
    return self->Insert(priority, id);
}
extern "C" long long vtk_priority_queue_pop_c_longlong(
    vtkNew<vtkPriorityQueue> self, long long location
) {
    return self->Pop(location);
}
extern "C" long long vtk_priority_queue_peek_c_longlong(
    vtkNew<vtkPriorityQueue> self, long long location
) {
    return self->Peek(location);
}
extern "C" double vtk_priority_queue_delete_id(vtkNew<vtkPriorityQueue> self, long long id) {
    return self->DeleteId(id);
}
extern "C" double vtk_priority_queue_get_priority(vtkNew<vtkPriorityQueue> self, long long id) {
    return self->GetPriority(id);
}
extern "C" long long vtk_priority_queue_get_number_of_items(vtkNew<vtkPriorityQueue> self) {
    return self->GetNumberOfItems();
}
extern "C" void vtk_priority_queue_reset(vtkNew<vtkPriorityQueue> self) {
    return self->Reset();
}
extern "C" vtkNew<vtkRandomPool> vtkRandomPool_new() {
    return vtkNew<vtkRandomPool>();
}
extern "C" void vtkRandomPool_destructor(vtkNew<vtkRandomPool> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkRandomPool_get_ptr(vtkNew<vtkRandomPool> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_random_pool_set_size(vtkNew<vtkRandomPool> self, long long _arg) {
    return self->SetSize(_arg);
}
extern "C" long long vtk_random_pool_get_size_min_value(vtkNew<vtkRandomPool> self) {
    return self->GetSizeMinValue();
}
extern "C" long long vtk_random_pool_get_size_max_value(vtkNew<vtkRandomPool> self) {
    return self->GetSizeMaxValue();
}
extern "C" long long vtk_random_pool_get_size(vtkNew<vtkRandomPool> self) {
    return self->GetSize();
}
extern "C" void vtk_random_pool_set_number_of_components(
    vtkNew<vtkRandomPool> self, long long _arg
) {
    return self->SetNumberOfComponents(_arg);
}
extern "C" long long vtk_random_pool_get_number_of_components_min_value(
    vtkNew<vtkRandomPool> self
) {
    return self->GetNumberOfComponentsMinValue();
}
extern "C" long long vtk_random_pool_get_number_of_components_max_value(
    vtkNew<vtkRandomPool> self
) {
    return self->GetNumberOfComponentsMaxValue();
}
extern "C" long long vtk_random_pool_get_number_of_components(vtkNew<vtkRandomPool> self) {
    return self->GetNumberOfComponents();
}
extern "C" long long vtk_random_pool_get_total_size(vtkNew<vtkRandomPool> self) {
    return self->GetTotalSize();
}
extern "C" double vtk_random_pool_get_value_c_longlong(vtkNew<vtkRandomPool> self, long long i) {
    return self->GetValue(i);
}
extern "C" double vtk_random_pool_get_value_c_longlong_c_int(
    vtkNew<vtkRandomPool> self, long long i, int compNum
) {
    return self->GetValue(i, compNum);
}
extern "C" void vtk_random_pool_set_chunk_size(vtkNew<vtkRandomPool> self, long long _arg) {
    return self->SetChunkSize(_arg);
}
extern "C" long long vtk_random_pool_get_chunk_size_min_value(vtkNew<vtkRandomPool> self) {
    return self->GetChunkSizeMinValue();
}
extern "C" long long vtk_random_pool_get_chunk_size_max_value(vtkNew<vtkRandomPool> self) {
    return self->GetChunkSizeMaxValue();
}
extern "C" long long vtk_random_pool_get_chunk_size(vtkNew<vtkRandomPool> self) {
    return self->GetChunkSize();
}
extern "C" vtkNew<vtkReferenceCount> vtkReferenceCount_new() {
    return vtkNew<vtkReferenceCount>();
}
extern "C" void vtkReferenceCount_destructor(vtkNew<vtkReferenceCount> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkReferenceCount_get_ptr(vtkNew<vtkReferenceCount> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkSOATypeFloat32Array> vtkSOATypeFloat32Array_new() {
    return vtkNew<vtkSOATypeFloat32Array>();
}
extern "C" void vtkSOATypeFloat32Array_destructor(vtkNew<vtkSOATypeFloat32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSOATypeFloat32Array_get_ptr(vtkNew<vtkSOATypeFloat32Array> sself) {
    return sself.GetPointer();
}
extern "C" float vtk_soa_type_float_32_array_get_data_type_value_min(
    vtkNew<vtkSOATypeFloat32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" float vtk_soa_type_float_32_array_get_data_type_value_max(
    vtkNew<vtkSOATypeFloat32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" float vtk_soa_type_float_32_array_get_value(
    vtkNew<vtkSOATypeFloat32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_soa_type_float_32_array_set_value(
    vtkNew<vtkSOATypeFloat32Array> self, long long id, float value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_soa_type_float_32_array_set_number_of_values(
    vtkNew<vtkSOATypeFloat32Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_soa_type_float_32_array_insert_value(
    vtkNew<vtkSOATypeFloat32Array> self, long long id, float f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_soa_type_float_32_array_insert_next_value(
    vtkNew<vtkSOATypeFloat32Array> self, float f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_soa_type_float_32_array_get_number_of_values(
    vtkNew<vtkSOATypeFloat32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkSOATypeFloat64Array> vtkSOATypeFloat64Array_new() {
    return vtkNew<vtkSOATypeFloat64Array>();
}
extern "C" void vtkSOATypeFloat64Array_destructor(vtkNew<vtkSOATypeFloat64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSOATypeFloat64Array_get_ptr(vtkNew<vtkSOATypeFloat64Array> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_soa_type_float_64_array_get_data_type_value_min(
    vtkNew<vtkSOATypeFloat64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" double vtk_soa_type_float_64_array_get_data_type_value_max(
    vtkNew<vtkSOATypeFloat64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" double vtk_soa_type_float_64_array_get_value(
    vtkNew<vtkSOATypeFloat64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_soa_type_float_64_array_set_value(
    vtkNew<vtkSOATypeFloat64Array> self, long long id, double value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_soa_type_float_64_array_set_number_of_values(
    vtkNew<vtkSOATypeFloat64Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_soa_type_float_64_array_insert_value(
    vtkNew<vtkSOATypeFloat64Array> self, long long id, double f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_soa_type_float_64_array_insert_next_value(
    vtkNew<vtkSOATypeFloat64Array> self, double f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_soa_type_float_64_array_get_number_of_values(
    vtkNew<vtkSOATypeFloat64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkSOATypeInt16Array> vtkSOATypeInt16Array_new() {
    return vtkNew<vtkSOATypeInt16Array>();
}
extern "C" void vtkSOATypeInt16Array_destructor(vtkNew<vtkSOATypeInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSOATypeInt16Array_get_ptr(vtkNew<vtkSOATypeInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" short vtk_soa_type_int_16_array_get_data_type_value_min(
    vtkNew<vtkSOATypeInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" short vtk_soa_type_int_16_array_get_data_type_value_max(
    vtkNew<vtkSOATypeInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" short vtk_soa_type_int_16_array_get_value(
    vtkNew<vtkSOATypeInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_soa_type_int_16_array_set_value(
    vtkNew<vtkSOATypeInt16Array> self, long long id, short value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_soa_type_int_16_array_set_number_of_values(
    vtkNew<vtkSOATypeInt16Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_soa_type_int_16_array_insert_value(
    vtkNew<vtkSOATypeInt16Array> self, long long id, short f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_soa_type_int_16_array_insert_next_value(
    vtkNew<vtkSOATypeInt16Array> self, short f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_soa_type_int_16_array_get_number_of_values(
    vtkNew<vtkSOATypeInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkSOATypeInt32Array> vtkSOATypeInt32Array_new() {
    return vtkNew<vtkSOATypeInt32Array>();
}
extern "C" void vtkSOATypeInt32Array_destructor(vtkNew<vtkSOATypeInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSOATypeInt32Array_get_ptr(vtkNew<vtkSOATypeInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_soa_type_int_32_array_get_data_type_value_min(
    vtkNew<vtkSOATypeInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" int vtk_soa_type_int_32_array_get_data_type_value_max(
    vtkNew<vtkSOATypeInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" int vtk_soa_type_int_32_array_get_value(
    vtkNew<vtkSOATypeInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_soa_type_int_32_array_set_value(
    vtkNew<vtkSOATypeInt32Array> self, long long id, int value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_soa_type_int_32_array_set_number_of_values(
    vtkNew<vtkSOATypeInt32Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_soa_type_int_32_array_insert_value(
    vtkNew<vtkSOATypeInt32Array> self, long long id, int f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_soa_type_int_32_array_insert_next_value(
    vtkNew<vtkSOATypeInt32Array> self, int f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_soa_type_int_32_array_get_number_of_values(
    vtkNew<vtkSOATypeInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkSOATypeInt64Array> vtkSOATypeInt64Array_new() {
    return vtkNew<vtkSOATypeInt64Array>();
}
extern "C" void vtkSOATypeInt64Array_destructor(vtkNew<vtkSOATypeInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSOATypeInt64Array_get_ptr(vtkNew<vtkSOATypeInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_soa_type_int_64_array_get_data_type_value_min(
    vtkNew<vtkSOATypeInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_soa_type_int_64_array_get_data_type_value_max(
    vtkNew<vtkSOATypeInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_soa_type_int_64_array_get_value(
    vtkNew<vtkSOATypeInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_soa_type_int_64_array_set_value(
    vtkNew<vtkSOATypeInt64Array> self, long long id, long long value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_soa_type_int_64_array_set_number_of_values(
    vtkNew<vtkSOATypeInt64Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_soa_type_int_64_array_insert_value(
    vtkNew<vtkSOATypeInt64Array> self, long long id, long long f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_soa_type_int_64_array_insert_next_value(
    vtkNew<vtkSOATypeInt64Array> self, long long f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_soa_type_int_64_array_get_number_of_values(
    vtkNew<vtkSOATypeInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkSOATypeInt8Array> vtkSOATypeInt8Array_new() {
    return vtkNew<vtkSOATypeInt8Array>();
}
extern "C" void vtkSOATypeInt8Array_destructor(vtkNew<vtkSOATypeInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSOATypeInt8Array_get_ptr(vtkNew<vtkSOATypeInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_soa_type_int_8_array_get_data_type_value_min(vtkNew<vtkSOATypeInt8Array> self) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_soa_type_int_8_array_get_data_type_value_max(vtkNew<vtkSOATypeInt8Array> self) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_soa_type_int_8_array_get_value(vtkNew<vtkSOATypeInt8Array> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_soa_type_int_8_array_set_value(
    vtkNew<vtkSOATypeInt8Array> self, long long id, char value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_soa_type_int_8_array_set_number_of_values(
    vtkNew<vtkSOATypeInt8Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_soa_type_int_8_array_insert_value(
    vtkNew<vtkSOATypeInt8Array> self, long long id, char f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_soa_type_int_8_array_insert_next_value(
    vtkNew<vtkSOATypeInt8Array> self, char f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_soa_type_int_8_array_get_number_of_values(
    vtkNew<vtkSOATypeInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkSOATypeUInt16Array> vtkSOATypeUInt16Array_new() {
    return vtkNew<vtkSOATypeUInt16Array>();
}
extern "C" void vtkSOATypeUInt16Array_destructor(vtkNew<vtkSOATypeUInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSOATypeUInt16Array_get_ptr(vtkNew<vtkSOATypeUInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned short vtk_soa_type_u_int_16_array_get_data_type_value_min(
    vtkNew<vtkSOATypeUInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned short vtk_soa_type_u_int_16_array_get_data_type_value_max(
    vtkNew<vtkSOATypeUInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned short vtk_soa_type_u_int_16_array_get_value(
    vtkNew<vtkSOATypeUInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_soa_type_u_int_16_array_set_value(
    vtkNew<vtkSOATypeUInt16Array> self, long long id, unsigned short value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_soa_type_u_int_16_array_set_number_of_values(
    vtkNew<vtkSOATypeUInt16Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_soa_type_u_int_16_array_insert_value(
    vtkNew<vtkSOATypeUInt16Array> self, long long id, unsigned short f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_soa_type_u_int_16_array_insert_next_value(
    vtkNew<vtkSOATypeUInt16Array> self, unsigned short f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_soa_type_u_int_16_array_get_number_of_values(
    vtkNew<vtkSOATypeUInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkSOATypeUInt32Array> vtkSOATypeUInt32Array_new() {
    return vtkNew<vtkSOATypeUInt32Array>();
}
extern "C" void vtkSOATypeUInt32Array_destructor(vtkNew<vtkSOATypeUInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSOATypeUInt32Array_get_ptr(vtkNew<vtkSOATypeUInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned int vtk_soa_type_u_int_32_array_get_data_type_value_min(
    vtkNew<vtkSOATypeUInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned int vtk_soa_type_u_int_32_array_get_data_type_value_max(
    vtkNew<vtkSOATypeUInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned int vtk_soa_type_u_int_32_array_get_value(
    vtkNew<vtkSOATypeUInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_soa_type_u_int_32_array_set_value(
    vtkNew<vtkSOATypeUInt32Array> self, long long id, unsigned int value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_soa_type_u_int_32_array_set_number_of_values(
    vtkNew<vtkSOATypeUInt32Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_soa_type_u_int_32_array_insert_value(
    vtkNew<vtkSOATypeUInt32Array> self, long long id, unsigned int f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_soa_type_u_int_32_array_insert_next_value(
    vtkNew<vtkSOATypeUInt32Array> self, unsigned int f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_soa_type_u_int_32_array_get_number_of_values(
    vtkNew<vtkSOATypeUInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkSOATypeUInt64Array> vtkSOATypeUInt64Array_new() {
    return vtkNew<vtkSOATypeUInt64Array>();
}
extern "C" void vtkSOATypeUInt64Array_destructor(vtkNew<vtkSOATypeUInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSOATypeUInt64Array_get_ptr(vtkNew<vtkSOATypeUInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long long vtk_soa_type_u_int_64_array_get_data_type_value_min(
    vtkNew<vtkSOATypeUInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long long vtk_soa_type_u_int_64_array_get_data_type_value_max(
    vtkNew<vtkSOATypeUInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long long vtk_soa_type_u_int_64_array_get_value(
    vtkNew<vtkSOATypeUInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_soa_type_u_int_64_array_set_value(
    vtkNew<vtkSOATypeUInt64Array> self, long long id, unsigned long long value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_soa_type_u_int_64_array_set_number_of_values(
    vtkNew<vtkSOATypeUInt64Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_soa_type_u_int_64_array_insert_value(
    vtkNew<vtkSOATypeUInt64Array> self, long long id, unsigned long long f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_soa_type_u_int_64_array_insert_next_value(
    vtkNew<vtkSOATypeUInt64Array> self, unsigned long long f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_soa_type_u_int_64_array_get_number_of_values(
    vtkNew<vtkSOATypeUInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkSOATypeUInt8Array> vtkSOATypeUInt8Array_new() {
    return vtkNew<vtkSOATypeUInt8Array>();
}
extern "C" void vtkSOATypeUInt8Array_destructor(vtkNew<vtkSOATypeUInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSOATypeUInt8Array_get_ptr(vtkNew<vtkSOATypeUInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned char vtk_soa_type_u_int_8_array_get_data_type_value_min(
    vtkNew<vtkSOATypeUInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned char vtk_soa_type_u_int_8_array_get_data_type_value_max(
    vtkNew<vtkSOATypeUInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned char vtk_soa_type_u_int_8_array_get_value(
    vtkNew<vtkSOATypeUInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_soa_type_u_int_8_array_set_value(
    vtkNew<vtkSOATypeUInt8Array> self, long long id, unsigned char value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_soa_type_u_int_8_array_set_number_of_values(
    vtkNew<vtkSOATypeUInt8Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_soa_type_u_int_8_array_insert_value(
    vtkNew<vtkSOATypeUInt8Array> self, long long id, unsigned char f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_soa_type_u_int_8_array_insert_next_value(
    vtkNew<vtkSOATypeUInt8Array> self, unsigned char f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_soa_type_u_int_8_array_get_number_of_values(
    vtkNew<vtkSOATypeUInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkScalarsToColors> vtkScalarsToColors_new() {
    return vtkNew<vtkScalarsToColors>();
}
extern "C" void vtkScalarsToColors_destructor(vtkNew<vtkScalarsToColors> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkScalarsToColors_get_ptr(vtkNew<vtkScalarsToColors> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_scalars_to_colors_is_opaque(vtkNew<vtkScalarsToColors> self) {
    return self->IsOpaque();
}
extern "C" void vtk_scalars_to_colors_build(vtkNew<vtkScalarsToColors> self) {
    return self->Build();
}
extern "C" void vtk_scalars_to_colors_set_range_double_double(
    vtkNew<vtkScalarsToColors> self, double min, double max
) {
    return self->SetRange(min, max);
}
extern "C" void vtk_scalars_to_colors_set_range_const_double_2(
    vtkNew<vtkScalarsToColors> self, const double* rng
) {
    return self->SetRange(rng);
}
extern "C" void vtk_scalars_to_colors_get_color_double_double_3(
    vtkNew<vtkScalarsToColors> self, double v, double* rgb
) {
    return self->GetColor(v, rgb);
}
extern "C" double vtk_scalars_to_colors_get_opacity(vtkNew<vtkScalarsToColors> self, double v) {
    return self->GetOpacity(v);
}
extern "C" double vtk_scalars_to_colors_get_luminance(vtkNew<vtkScalarsToColors> self, double x) {
    return self->GetLuminance(x);
}
extern "C" void vtk_scalars_to_colors_set_alpha(vtkNew<vtkScalarsToColors> self, double alpha) {
    return self->SetAlpha(alpha);
}
extern "C" double vtk_scalars_to_colors_get_alpha(vtkNew<vtkScalarsToColors> self) {
    return self->GetAlpha();
}
extern "C" void vtk_scalars_to_colors_set_vector_mode(vtkNew<vtkScalarsToColors> self, int _arg) {
    return self->SetVectorMode(_arg);
}
extern "C" int vtk_scalars_to_colors_get_vector_mode(vtkNew<vtkScalarsToColors> self) {
    return self->GetVectorMode();
}
extern "C" void vtk_scalars_to_colors_set_vector_mode_to_magnitude(
    vtkNew<vtkScalarsToColors> self
) {
    return self->SetVectorModeToMagnitude();
}
extern "C" void vtk_scalars_to_colors_set_vector_mode_to_component(
    vtkNew<vtkScalarsToColors> self
) {
    return self->SetVectorModeToComponent();
}
extern "C" void vtk_scalars_to_colors_set_vector_mode_to_rgb_colors(
    vtkNew<vtkScalarsToColors> self
) {
    return self->SetVectorModeToRGBColors();
}
extern "C" void vtk_scalars_to_colors_set_vector_component(
    vtkNew<vtkScalarsToColors> self, int _arg
) {
    return self->SetVectorComponent(_arg);
}
extern "C" int vtk_scalars_to_colors_get_vector_component(vtkNew<vtkScalarsToColors> self) {
    return self->GetVectorComponent();
}
extern "C" void vtk_scalars_to_colors_set_vector_size(vtkNew<vtkScalarsToColors> self, int _arg) {
    return self->SetVectorSize(_arg);
}
extern "C" int vtk_scalars_to_colors_get_vector_size(vtkNew<vtkScalarsToColors> self) {
    return self->GetVectorSize();
}
extern "C" int vtk_scalars_to_colors_using_log_scale(vtkNew<vtkScalarsToColors> self) {
    return self->UsingLogScale();
}
extern "C" long long vtk_scalars_to_colors_get_number_of_available_colors(
    vtkNew<vtkScalarsToColors> self
) {
    return self->GetNumberOfAvailableColors();
}
extern "C" long long vtk_scalars_to_colors_get_number_of_annotated_values(
    vtkNew<vtkScalarsToColors> self
) {
    return self->GetNumberOfAnnotatedValues();
}
extern "C" void vtk_scalars_to_colors_get_indexed_color(
    vtkNew<vtkScalarsToColors> self, long long i, double* rgba
) {
    return self->GetIndexedColor(i, rgba);
}
extern "C" void vtk_scalars_to_colors_reset_annotations(vtkNew<vtkScalarsToColors> self) {
    return self->ResetAnnotations();
}
extern "C" void vtk_scalars_to_colors_set_indexed_lookup(
    vtkNew<vtkScalarsToColors> self, int _arg
) {
    return self->SetIndexedLookup(_arg);
}
extern "C" int vtk_scalars_to_colors_get_indexed_lookup(vtkNew<vtkScalarsToColors> self) {
    return self->GetIndexedLookup();
}
extern "C" void vtk_scalars_to_colors_indexed_lookup_on(vtkNew<vtkScalarsToColors> self) {
    return self->IndexedLookupOn();
}
extern "C" void vtk_scalars_to_colors_indexed_lookup_off(vtkNew<vtkScalarsToColors> self) {
    return self->IndexedLookupOff();
}
extern "C" vtkNew<vtkScaledSOATypeFloat32Array> vtkScaledSOATypeFloat32Array_new() {
    return vtkNew<vtkScaledSOATypeFloat32Array>();
}
extern "C" void vtkScaledSOATypeFloat32Array_destructor(
    vtkNew<vtkScaledSOATypeFloat32Array> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkScaledSOATypeFloat32Array_get_ptr(vtkNew<vtkScaledSOATypeFloat32Array> sself) {
    return sself.GetPointer();
}
extern "C" float vtk_scaled_soa_type_float_32_array_get_data_type_value_min(
    vtkNew<vtkScaledSOATypeFloat32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" float vtk_scaled_soa_type_float_32_array_get_data_type_value_max(
    vtkNew<vtkScaledSOATypeFloat32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" float vtk_scaled_soa_type_float_32_array_get_value(
    vtkNew<vtkScaledSOATypeFloat32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_scaled_soa_type_float_32_array_set_value(
    vtkNew<vtkScaledSOATypeFloat32Array> self, long long id, float value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_scaled_soa_type_float_32_array_set_number_of_values(
    vtkNew<vtkScaledSOATypeFloat32Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_scaled_soa_type_float_32_array_insert_value(
    vtkNew<vtkScaledSOATypeFloat32Array> self, long long id, float f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_scaled_soa_type_float_32_array_insert_next_value(
    vtkNew<vtkScaledSOATypeFloat32Array> self, float f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_scaled_soa_type_float_32_array_get_number_of_values(
    vtkNew<vtkScaledSOATypeFloat32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkScaledSOATypeFloat64Array> vtkScaledSOATypeFloat64Array_new() {
    return vtkNew<vtkScaledSOATypeFloat64Array>();
}
extern "C" void vtkScaledSOATypeFloat64Array_destructor(
    vtkNew<vtkScaledSOATypeFloat64Array> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkScaledSOATypeFloat64Array_get_ptr(vtkNew<vtkScaledSOATypeFloat64Array> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_scaled_soa_type_float_64_array_get_data_type_value_min(
    vtkNew<vtkScaledSOATypeFloat64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" double vtk_scaled_soa_type_float_64_array_get_data_type_value_max(
    vtkNew<vtkScaledSOATypeFloat64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" double vtk_scaled_soa_type_float_64_array_get_value(
    vtkNew<vtkScaledSOATypeFloat64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_scaled_soa_type_float_64_array_set_value(
    vtkNew<vtkScaledSOATypeFloat64Array> self, long long id, double value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_scaled_soa_type_float_64_array_set_number_of_values(
    vtkNew<vtkScaledSOATypeFloat64Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_scaled_soa_type_float_64_array_insert_value(
    vtkNew<vtkScaledSOATypeFloat64Array> self, long long id, double f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_scaled_soa_type_float_64_array_insert_next_value(
    vtkNew<vtkScaledSOATypeFloat64Array> self, double f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_scaled_soa_type_float_64_array_get_number_of_values(
    vtkNew<vtkScaledSOATypeFloat64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkScaledSOATypeInt16Array> vtkScaledSOATypeInt16Array_new() {
    return vtkNew<vtkScaledSOATypeInt16Array>();
}
extern "C" void vtkScaledSOATypeInt16Array_destructor(vtkNew<vtkScaledSOATypeInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkScaledSOATypeInt16Array_get_ptr(vtkNew<vtkScaledSOATypeInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" short vtk_scaled_soa_type_int_16_array_get_data_type_value_min(
    vtkNew<vtkScaledSOATypeInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" short vtk_scaled_soa_type_int_16_array_get_data_type_value_max(
    vtkNew<vtkScaledSOATypeInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" short vtk_scaled_soa_type_int_16_array_get_value(
    vtkNew<vtkScaledSOATypeInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_scaled_soa_type_int_16_array_set_value(
    vtkNew<vtkScaledSOATypeInt16Array> self, long long id, short value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_scaled_soa_type_int_16_array_set_number_of_values(
    vtkNew<vtkScaledSOATypeInt16Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_scaled_soa_type_int_16_array_insert_value(
    vtkNew<vtkScaledSOATypeInt16Array> self, long long id, short f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_scaled_soa_type_int_16_array_insert_next_value(
    vtkNew<vtkScaledSOATypeInt16Array> self, short f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_scaled_soa_type_int_16_array_get_number_of_values(
    vtkNew<vtkScaledSOATypeInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkScaledSOATypeInt32Array> vtkScaledSOATypeInt32Array_new() {
    return vtkNew<vtkScaledSOATypeInt32Array>();
}
extern "C" void vtkScaledSOATypeInt32Array_destructor(vtkNew<vtkScaledSOATypeInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkScaledSOATypeInt32Array_get_ptr(vtkNew<vtkScaledSOATypeInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_scaled_soa_type_int_32_array_get_data_type_value_min(
    vtkNew<vtkScaledSOATypeInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" int vtk_scaled_soa_type_int_32_array_get_data_type_value_max(
    vtkNew<vtkScaledSOATypeInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" int vtk_scaled_soa_type_int_32_array_get_value(
    vtkNew<vtkScaledSOATypeInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_scaled_soa_type_int_32_array_set_value(
    vtkNew<vtkScaledSOATypeInt32Array> self, long long id, int value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_scaled_soa_type_int_32_array_set_number_of_values(
    vtkNew<vtkScaledSOATypeInt32Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_scaled_soa_type_int_32_array_insert_value(
    vtkNew<vtkScaledSOATypeInt32Array> self, long long id, int f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_scaled_soa_type_int_32_array_insert_next_value(
    vtkNew<vtkScaledSOATypeInt32Array> self, int f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_scaled_soa_type_int_32_array_get_number_of_values(
    vtkNew<vtkScaledSOATypeInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkScaledSOATypeInt64Array> vtkScaledSOATypeInt64Array_new() {
    return vtkNew<vtkScaledSOATypeInt64Array>();
}
extern "C" void vtkScaledSOATypeInt64Array_destructor(vtkNew<vtkScaledSOATypeInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkScaledSOATypeInt64Array_get_ptr(vtkNew<vtkScaledSOATypeInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_scaled_soa_type_int_64_array_get_data_type_value_min(
    vtkNew<vtkScaledSOATypeInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_scaled_soa_type_int_64_array_get_data_type_value_max(
    vtkNew<vtkScaledSOATypeInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_scaled_soa_type_int_64_array_get_value(
    vtkNew<vtkScaledSOATypeInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_scaled_soa_type_int_64_array_set_value(
    vtkNew<vtkScaledSOATypeInt64Array> self, long long id, long long value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_scaled_soa_type_int_64_array_set_number_of_values(
    vtkNew<vtkScaledSOATypeInt64Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_scaled_soa_type_int_64_array_insert_value(
    vtkNew<vtkScaledSOATypeInt64Array> self, long long id, long long f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_scaled_soa_type_int_64_array_insert_next_value(
    vtkNew<vtkScaledSOATypeInt64Array> self, long long f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_scaled_soa_type_int_64_array_get_number_of_values(
    vtkNew<vtkScaledSOATypeInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkScaledSOATypeInt8Array> vtkScaledSOATypeInt8Array_new() {
    return vtkNew<vtkScaledSOATypeInt8Array>();
}
extern "C" void vtkScaledSOATypeInt8Array_destructor(vtkNew<vtkScaledSOATypeInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkScaledSOATypeInt8Array_get_ptr(vtkNew<vtkScaledSOATypeInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_scaled_soa_type_int_8_array_get_data_type_value_min(
    vtkNew<vtkScaledSOATypeInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_scaled_soa_type_int_8_array_get_data_type_value_max(
    vtkNew<vtkScaledSOATypeInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_scaled_soa_type_int_8_array_get_value(
    vtkNew<vtkScaledSOATypeInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_scaled_soa_type_int_8_array_set_value(
    vtkNew<vtkScaledSOATypeInt8Array> self, long long id, char value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_scaled_soa_type_int_8_array_set_number_of_values(
    vtkNew<vtkScaledSOATypeInt8Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_scaled_soa_type_int_8_array_insert_value(
    vtkNew<vtkScaledSOATypeInt8Array> self, long long id, char f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_scaled_soa_type_int_8_array_insert_next_value(
    vtkNew<vtkScaledSOATypeInt8Array> self, char f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_scaled_soa_type_int_8_array_get_number_of_values(
    vtkNew<vtkScaledSOATypeInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkScaledSOATypeUInt16Array> vtkScaledSOATypeUInt16Array_new() {
    return vtkNew<vtkScaledSOATypeUInt16Array>();
}
extern "C" void vtkScaledSOATypeUInt16Array_destructor(vtkNew<vtkScaledSOATypeUInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkScaledSOATypeUInt16Array_get_ptr(vtkNew<vtkScaledSOATypeUInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned short vtk_scaled_soa_type_u_int_16_array_get_data_type_value_min(
    vtkNew<vtkScaledSOATypeUInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned short vtk_scaled_soa_type_u_int_16_array_get_data_type_value_max(
    vtkNew<vtkScaledSOATypeUInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned short vtk_scaled_soa_type_u_int_16_array_get_value(
    vtkNew<vtkScaledSOATypeUInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_scaled_soa_type_u_int_16_array_set_value(
    vtkNew<vtkScaledSOATypeUInt16Array> self, long long id, unsigned short value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_scaled_soa_type_u_int_16_array_set_number_of_values(
    vtkNew<vtkScaledSOATypeUInt16Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_scaled_soa_type_u_int_16_array_insert_value(
    vtkNew<vtkScaledSOATypeUInt16Array> self, long long id, unsigned short f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_scaled_soa_type_u_int_16_array_insert_next_value(
    vtkNew<vtkScaledSOATypeUInt16Array> self, unsigned short f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_scaled_soa_type_u_int_16_array_get_number_of_values(
    vtkNew<vtkScaledSOATypeUInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkScaledSOATypeUInt32Array> vtkScaledSOATypeUInt32Array_new() {
    return vtkNew<vtkScaledSOATypeUInt32Array>();
}
extern "C" void vtkScaledSOATypeUInt32Array_destructor(vtkNew<vtkScaledSOATypeUInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkScaledSOATypeUInt32Array_get_ptr(vtkNew<vtkScaledSOATypeUInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned int vtk_scaled_soa_type_u_int_32_array_get_data_type_value_min(
    vtkNew<vtkScaledSOATypeUInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned int vtk_scaled_soa_type_u_int_32_array_get_data_type_value_max(
    vtkNew<vtkScaledSOATypeUInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned int vtk_scaled_soa_type_u_int_32_array_get_value(
    vtkNew<vtkScaledSOATypeUInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_scaled_soa_type_u_int_32_array_set_value(
    vtkNew<vtkScaledSOATypeUInt32Array> self, long long id, unsigned int value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_scaled_soa_type_u_int_32_array_set_number_of_values(
    vtkNew<vtkScaledSOATypeUInt32Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_scaled_soa_type_u_int_32_array_insert_value(
    vtkNew<vtkScaledSOATypeUInt32Array> self, long long id, unsigned int f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_scaled_soa_type_u_int_32_array_insert_next_value(
    vtkNew<vtkScaledSOATypeUInt32Array> self, unsigned int f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_scaled_soa_type_u_int_32_array_get_number_of_values(
    vtkNew<vtkScaledSOATypeUInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkScaledSOATypeUInt64Array> vtkScaledSOATypeUInt64Array_new() {
    return vtkNew<vtkScaledSOATypeUInt64Array>();
}
extern "C" void vtkScaledSOATypeUInt64Array_destructor(vtkNew<vtkScaledSOATypeUInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkScaledSOATypeUInt64Array_get_ptr(vtkNew<vtkScaledSOATypeUInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long long vtk_scaled_soa_type_u_int_64_array_get_data_type_value_min(
    vtkNew<vtkScaledSOATypeUInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long long vtk_scaled_soa_type_u_int_64_array_get_data_type_value_max(
    vtkNew<vtkScaledSOATypeUInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long long vtk_scaled_soa_type_u_int_64_array_get_value(
    vtkNew<vtkScaledSOATypeUInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_scaled_soa_type_u_int_64_array_set_value(
    vtkNew<vtkScaledSOATypeUInt64Array> self, long long id, unsigned long long value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_scaled_soa_type_u_int_64_array_set_number_of_values(
    vtkNew<vtkScaledSOATypeUInt64Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_scaled_soa_type_u_int_64_array_insert_value(
    vtkNew<vtkScaledSOATypeUInt64Array> self, long long id, unsigned long long f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_scaled_soa_type_u_int_64_array_insert_next_value(
    vtkNew<vtkScaledSOATypeUInt64Array> self, unsigned long long f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_scaled_soa_type_u_int_64_array_get_number_of_values(
    vtkNew<vtkScaledSOATypeUInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkScaledSOATypeUInt8Array> vtkScaledSOATypeUInt8Array_new() {
    return vtkNew<vtkScaledSOATypeUInt8Array>();
}
extern "C" void vtkScaledSOATypeUInt8Array_destructor(vtkNew<vtkScaledSOATypeUInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkScaledSOATypeUInt8Array_get_ptr(vtkNew<vtkScaledSOATypeUInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned char vtk_scaled_soa_type_u_int_8_array_get_data_type_value_min(
    vtkNew<vtkScaledSOATypeUInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned char vtk_scaled_soa_type_u_int_8_array_get_data_type_value_max(
    vtkNew<vtkScaledSOATypeUInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned char vtk_scaled_soa_type_u_int_8_array_get_value(
    vtkNew<vtkScaledSOATypeUInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_scaled_soa_type_u_int_8_array_set_value(
    vtkNew<vtkScaledSOATypeUInt8Array> self, long long id, unsigned char value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_scaled_soa_type_u_int_8_array_set_number_of_values(
    vtkNew<vtkScaledSOATypeUInt8Array> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_scaled_soa_type_u_int_8_array_insert_value(
    vtkNew<vtkScaledSOATypeUInt8Array> self, long long id, unsigned char f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_scaled_soa_type_u_int_8_array_insert_next_value(
    vtkNew<vtkScaledSOATypeUInt8Array> self, unsigned char f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_scaled_soa_type_u_int_8_array_get_number_of_values(
    vtkNew<vtkScaledSOATypeUInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkSerializer> vtkSerializer_new() {
    return vtkNew<vtkSerializer>();
}
extern "C" void vtkSerializer_destructor(vtkNew<vtkSerializer> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSerializer_get_ptr(vtkNew<vtkSerializer> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkShortArray> vtkShortArray_new() {
    return vtkNew<vtkShortArray>();
}
extern "C" void vtkShortArray_destructor(vtkNew<vtkShortArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkShortArray_get_ptr(vtkNew<vtkShortArray> sself) {
    return sself.GetPointer();
}
extern "C" short vtk_short_array_get_data_type_value_min(vtkNew<vtkShortArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" short vtk_short_array_get_data_type_value_max(vtkNew<vtkShortArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" short vtk_short_array_get_value(vtkNew<vtkShortArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_short_array_set_value(vtkNew<vtkShortArray> self, long long id, short value) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_short_array_set_number_of_values(vtkNew<vtkShortArray> self, long long number) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_short_array_insert_value(vtkNew<vtkShortArray> self, long long id, short f) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_short_array_insert_next_value(vtkNew<vtkShortArray> self, short f) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_short_array_get_number_of_values(vtkNew<vtkShortArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkSignedCharArray> vtkSignedCharArray_new() {
    return vtkNew<vtkSignedCharArray>();
}
extern "C" void vtkSignedCharArray_destructor(vtkNew<vtkSignedCharArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSignedCharArray_get_ptr(vtkNew<vtkSignedCharArray> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_signed_char_array_get_data_type_value_min(vtkNew<vtkSignedCharArray> self) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_signed_char_array_get_data_type_value_max(vtkNew<vtkSignedCharArray> self) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_signed_char_array_get_value(vtkNew<vtkSignedCharArray> self, long long id) {
    return self->GetValue(id);
}
extern "C" void vtk_signed_char_array_set_value(
    vtkNew<vtkSignedCharArray> self, long long id, char value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_signed_char_array_set_number_of_values(
    vtkNew<vtkSignedCharArray> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_signed_char_array_insert_value(
    vtkNew<vtkSignedCharArray> self, long long id, char f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_signed_char_array_insert_next_value(
    vtkNew<vtkSignedCharArray> self, char f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_signed_char_array_get_number_of_values(vtkNew<vtkSignedCharArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkSortDataArray> vtkSortDataArray_new() {
    return vtkNew<vtkSortDataArray>();
}
extern "C" void vtkSortDataArray_destructor(vtkNew<vtkSortDataArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSortDataArray_get_ptr(vtkNew<vtkSortDataArray> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkStdFunctionTypeFloat32Array> vtkStdFunctionTypeFloat32Array_new() {
    return vtkNew<vtkStdFunctionTypeFloat32Array>();
}
extern "C" void vtkStdFunctionTypeFloat32Array_destructor(
    vtkNew<vtkStdFunctionTypeFloat32Array> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkStdFunctionTypeFloat32Array_get_ptr(
    vtkNew<vtkStdFunctionTypeFloat32Array> sself
) {
    return sself.GetPointer();
}
extern "C" float vtk_std_function_type_float_32_array_get_data_type_value_min(
    vtkNew<vtkStdFunctionTypeFloat32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" float vtk_std_function_type_float_32_array_get_data_type_value_max(
    vtkNew<vtkStdFunctionTypeFloat32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" float vtk_std_function_type_float_32_array_get_value(
    vtkNew<vtkStdFunctionTypeFloat32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_std_function_type_float_32_array_get_number_of_values(
    vtkNew<vtkStdFunctionTypeFloat32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStdFunctionTypeFloat64Array> vtkStdFunctionTypeFloat64Array_new() {
    return vtkNew<vtkStdFunctionTypeFloat64Array>();
}
extern "C" void vtkStdFunctionTypeFloat64Array_destructor(
    vtkNew<vtkStdFunctionTypeFloat64Array> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkStdFunctionTypeFloat64Array_get_ptr(
    vtkNew<vtkStdFunctionTypeFloat64Array> sself
) {
    return sself.GetPointer();
}
extern "C" double vtk_std_function_type_float_64_array_get_data_type_value_min(
    vtkNew<vtkStdFunctionTypeFloat64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" double vtk_std_function_type_float_64_array_get_data_type_value_max(
    vtkNew<vtkStdFunctionTypeFloat64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" double vtk_std_function_type_float_64_array_get_value(
    vtkNew<vtkStdFunctionTypeFloat64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_std_function_type_float_64_array_get_number_of_values(
    vtkNew<vtkStdFunctionTypeFloat64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStdFunctionTypeInt16Array> vtkStdFunctionTypeInt16Array_new() {
    return vtkNew<vtkStdFunctionTypeInt16Array>();
}
extern "C" void vtkStdFunctionTypeInt16Array_destructor(
    vtkNew<vtkStdFunctionTypeInt16Array> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkStdFunctionTypeInt16Array_get_ptr(vtkNew<vtkStdFunctionTypeInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" short vtk_std_function_type_int_16_array_get_data_type_value_min(
    vtkNew<vtkStdFunctionTypeInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" short vtk_std_function_type_int_16_array_get_data_type_value_max(
    vtkNew<vtkStdFunctionTypeInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" short vtk_std_function_type_int_16_array_get_value(
    vtkNew<vtkStdFunctionTypeInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_std_function_type_int_16_array_get_number_of_values(
    vtkNew<vtkStdFunctionTypeInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStdFunctionTypeInt32Array> vtkStdFunctionTypeInt32Array_new() {
    return vtkNew<vtkStdFunctionTypeInt32Array>();
}
extern "C" void vtkStdFunctionTypeInt32Array_destructor(
    vtkNew<vtkStdFunctionTypeInt32Array> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkStdFunctionTypeInt32Array_get_ptr(vtkNew<vtkStdFunctionTypeInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_std_function_type_int_32_array_get_data_type_value_min(
    vtkNew<vtkStdFunctionTypeInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" int vtk_std_function_type_int_32_array_get_data_type_value_max(
    vtkNew<vtkStdFunctionTypeInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" int vtk_std_function_type_int_32_array_get_value(
    vtkNew<vtkStdFunctionTypeInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_std_function_type_int_32_array_get_number_of_values(
    vtkNew<vtkStdFunctionTypeInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStdFunctionTypeInt64Array> vtkStdFunctionTypeInt64Array_new() {
    return vtkNew<vtkStdFunctionTypeInt64Array>();
}
extern "C" void vtkStdFunctionTypeInt64Array_destructor(
    vtkNew<vtkStdFunctionTypeInt64Array> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkStdFunctionTypeInt64Array_get_ptr(vtkNew<vtkStdFunctionTypeInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_std_function_type_int_64_array_get_data_type_value_min(
    vtkNew<vtkStdFunctionTypeInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_std_function_type_int_64_array_get_data_type_value_max(
    vtkNew<vtkStdFunctionTypeInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_std_function_type_int_64_array_get_value(
    vtkNew<vtkStdFunctionTypeInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_std_function_type_int_64_array_get_number_of_values(
    vtkNew<vtkStdFunctionTypeInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStdFunctionTypeInt8Array> vtkStdFunctionTypeInt8Array_new() {
    return vtkNew<vtkStdFunctionTypeInt8Array>();
}
extern "C" void vtkStdFunctionTypeInt8Array_destructor(vtkNew<vtkStdFunctionTypeInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStdFunctionTypeInt8Array_get_ptr(vtkNew<vtkStdFunctionTypeInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_std_function_type_int_8_array_get_data_type_value_min(
    vtkNew<vtkStdFunctionTypeInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_std_function_type_int_8_array_get_data_type_value_max(
    vtkNew<vtkStdFunctionTypeInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_std_function_type_int_8_array_get_value(
    vtkNew<vtkStdFunctionTypeInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_std_function_type_int_8_array_get_number_of_values(
    vtkNew<vtkStdFunctionTypeInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStdFunctionTypeUInt16Array> vtkStdFunctionTypeUInt16Array_new() {
    return vtkNew<vtkStdFunctionTypeUInt16Array>();
}
extern "C" void vtkStdFunctionTypeUInt16Array_destructor(
    vtkNew<vtkStdFunctionTypeUInt16Array> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkStdFunctionTypeUInt16Array_get_ptr(
    vtkNew<vtkStdFunctionTypeUInt16Array> sself
) {
    return sself.GetPointer();
}
extern "C" unsigned short vtk_std_function_type_u_int_16_array_get_data_type_value_min(
    vtkNew<vtkStdFunctionTypeUInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned short vtk_std_function_type_u_int_16_array_get_data_type_value_max(
    vtkNew<vtkStdFunctionTypeUInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned short vtk_std_function_type_u_int_16_array_get_value(
    vtkNew<vtkStdFunctionTypeUInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_std_function_type_u_int_16_array_get_number_of_values(
    vtkNew<vtkStdFunctionTypeUInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStdFunctionTypeUInt32Array> vtkStdFunctionTypeUInt32Array_new() {
    return vtkNew<vtkStdFunctionTypeUInt32Array>();
}
extern "C" void vtkStdFunctionTypeUInt32Array_destructor(
    vtkNew<vtkStdFunctionTypeUInt32Array> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkStdFunctionTypeUInt32Array_get_ptr(
    vtkNew<vtkStdFunctionTypeUInt32Array> sself
) {
    return sself.GetPointer();
}
extern "C" unsigned int vtk_std_function_type_u_int_32_array_get_data_type_value_min(
    vtkNew<vtkStdFunctionTypeUInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned int vtk_std_function_type_u_int_32_array_get_data_type_value_max(
    vtkNew<vtkStdFunctionTypeUInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned int vtk_std_function_type_u_int_32_array_get_value(
    vtkNew<vtkStdFunctionTypeUInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_std_function_type_u_int_32_array_get_number_of_values(
    vtkNew<vtkStdFunctionTypeUInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStdFunctionTypeUInt64Array> vtkStdFunctionTypeUInt64Array_new() {
    return vtkNew<vtkStdFunctionTypeUInt64Array>();
}
extern "C" void vtkStdFunctionTypeUInt64Array_destructor(
    vtkNew<vtkStdFunctionTypeUInt64Array> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkStdFunctionTypeUInt64Array_get_ptr(
    vtkNew<vtkStdFunctionTypeUInt64Array> sself
) {
    return sself.GetPointer();
}
extern "C" unsigned long long vtk_std_function_type_u_int_64_array_get_data_type_value_min(
    vtkNew<vtkStdFunctionTypeUInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long long vtk_std_function_type_u_int_64_array_get_data_type_value_max(
    vtkNew<vtkStdFunctionTypeUInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long long vtk_std_function_type_u_int_64_array_get_value(
    vtkNew<vtkStdFunctionTypeUInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_std_function_type_u_int_64_array_get_number_of_values(
    vtkNew<vtkStdFunctionTypeUInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStdFunctionTypeUInt8Array> vtkStdFunctionTypeUInt8Array_new() {
    return vtkNew<vtkStdFunctionTypeUInt8Array>();
}
extern "C" void vtkStdFunctionTypeUInt8Array_destructor(
    vtkNew<vtkStdFunctionTypeUInt8Array> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkStdFunctionTypeUInt8Array_get_ptr(vtkNew<vtkStdFunctionTypeUInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned char vtk_std_function_type_u_int_8_array_get_data_type_value_min(
    vtkNew<vtkStdFunctionTypeUInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned char vtk_std_function_type_u_int_8_array_get_data_type_value_max(
    vtkNew<vtkStdFunctionTypeUInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned char vtk_std_function_type_u_int_8_array_get_value(
    vtkNew<vtkStdFunctionTypeUInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_std_function_type_u_int_8_array_get_number_of_values(
    vtkNew<vtkStdFunctionTypeUInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStridedTypeFloat32Array> vtkStridedTypeFloat32Array_new() {
    return vtkNew<vtkStridedTypeFloat32Array>();
}
extern "C" void vtkStridedTypeFloat32Array_destructor(vtkNew<vtkStridedTypeFloat32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStridedTypeFloat32Array_get_ptr(vtkNew<vtkStridedTypeFloat32Array> sself) {
    return sself.GetPointer();
}
extern "C" float vtk_strided_type_float_32_array_get_data_type_value_min(
    vtkNew<vtkStridedTypeFloat32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" float vtk_strided_type_float_32_array_get_data_type_value_max(
    vtkNew<vtkStridedTypeFloat32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" float vtk_strided_type_float_32_array_get_value(
    vtkNew<vtkStridedTypeFloat32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_strided_type_float_32_array_get_number_of_values(
    vtkNew<vtkStridedTypeFloat32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStridedTypeFloat64Array> vtkStridedTypeFloat64Array_new() {
    return vtkNew<vtkStridedTypeFloat64Array>();
}
extern "C" void vtkStridedTypeFloat64Array_destructor(vtkNew<vtkStridedTypeFloat64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStridedTypeFloat64Array_get_ptr(vtkNew<vtkStridedTypeFloat64Array> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_strided_type_float_64_array_get_data_type_value_min(
    vtkNew<vtkStridedTypeFloat64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" double vtk_strided_type_float_64_array_get_data_type_value_max(
    vtkNew<vtkStridedTypeFloat64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" double vtk_strided_type_float_64_array_get_value(
    vtkNew<vtkStridedTypeFloat64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_strided_type_float_64_array_get_number_of_values(
    vtkNew<vtkStridedTypeFloat64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStridedTypeInt16Array> vtkStridedTypeInt16Array_new() {
    return vtkNew<vtkStridedTypeInt16Array>();
}
extern "C" void vtkStridedTypeInt16Array_destructor(vtkNew<vtkStridedTypeInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStridedTypeInt16Array_get_ptr(vtkNew<vtkStridedTypeInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" short vtk_strided_type_int_16_array_get_data_type_value_min(
    vtkNew<vtkStridedTypeInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" short vtk_strided_type_int_16_array_get_data_type_value_max(
    vtkNew<vtkStridedTypeInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" short vtk_strided_type_int_16_array_get_value(
    vtkNew<vtkStridedTypeInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_strided_type_int_16_array_get_number_of_values(
    vtkNew<vtkStridedTypeInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStridedTypeInt32Array> vtkStridedTypeInt32Array_new() {
    return vtkNew<vtkStridedTypeInt32Array>();
}
extern "C" void vtkStridedTypeInt32Array_destructor(vtkNew<vtkStridedTypeInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStridedTypeInt32Array_get_ptr(vtkNew<vtkStridedTypeInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_strided_type_int_32_array_get_data_type_value_min(
    vtkNew<vtkStridedTypeInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" int vtk_strided_type_int_32_array_get_data_type_value_max(
    vtkNew<vtkStridedTypeInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" int vtk_strided_type_int_32_array_get_value(
    vtkNew<vtkStridedTypeInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_strided_type_int_32_array_get_number_of_values(
    vtkNew<vtkStridedTypeInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStridedTypeInt64Array> vtkStridedTypeInt64Array_new() {
    return vtkNew<vtkStridedTypeInt64Array>();
}
extern "C" void vtkStridedTypeInt64Array_destructor(vtkNew<vtkStridedTypeInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStridedTypeInt64Array_get_ptr(vtkNew<vtkStridedTypeInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" long long vtk_strided_type_int_64_array_get_data_type_value_min(
    vtkNew<vtkStridedTypeInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" long long vtk_strided_type_int_64_array_get_data_type_value_max(
    vtkNew<vtkStridedTypeInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" long long vtk_strided_type_int_64_array_get_value(
    vtkNew<vtkStridedTypeInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_strided_type_int_64_array_get_number_of_values(
    vtkNew<vtkStridedTypeInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStridedTypeInt8Array> vtkStridedTypeInt8Array_new() {
    return vtkNew<vtkStridedTypeInt8Array>();
}
extern "C" void vtkStridedTypeInt8Array_destructor(vtkNew<vtkStridedTypeInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStridedTypeInt8Array_get_ptr(vtkNew<vtkStridedTypeInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" char vtk_strided_type_int_8_array_get_data_type_value_min(
    vtkNew<vtkStridedTypeInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" char vtk_strided_type_int_8_array_get_data_type_value_max(
    vtkNew<vtkStridedTypeInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" char vtk_strided_type_int_8_array_get_value(
    vtkNew<vtkStridedTypeInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_strided_type_int_8_array_get_number_of_values(
    vtkNew<vtkStridedTypeInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStridedTypeUInt16Array> vtkStridedTypeUInt16Array_new() {
    return vtkNew<vtkStridedTypeUInt16Array>();
}
extern "C" void vtkStridedTypeUInt16Array_destructor(vtkNew<vtkStridedTypeUInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStridedTypeUInt16Array_get_ptr(vtkNew<vtkStridedTypeUInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned short vtk_strided_type_u_int_16_array_get_data_type_value_min(
    vtkNew<vtkStridedTypeUInt16Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned short vtk_strided_type_u_int_16_array_get_data_type_value_max(
    vtkNew<vtkStridedTypeUInt16Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned short vtk_strided_type_u_int_16_array_get_value(
    vtkNew<vtkStridedTypeUInt16Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_strided_type_u_int_16_array_get_number_of_values(
    vtkNew<vtkStridedTypeUInt16Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStridedTypeUInt32Array> vtkStridedTypeUInt32Array_new() {
    return vtkNew<vtkStridedTypeUInt32Array>();
}
extern "C" void vtkStridedTypeUInt32Array_destructor(vtkNew<vtkStridedTypeUInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStridedTypeUInt32Array_get_ptr(vtkNew<vtkStridedTypeUInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned int vtk_strided_type_u_int_32_array_get_data_type_value_min(
    vtkNew<vtkStridedTypeUInt32Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned int vtk_strided_type_u_int_32_array_get_data_type_value_max(
    vtkNew<vtkStridedTypeUInt32Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned int vtk_strided_type_u_int_32_array_get_value(
    vtkNew<vtkStridedTypeUInt32Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_strided_type_u_int_32_array_get_number_of_values(
    vtkNew<vtkStridedTypeUInt32Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStridedTypeUInt64Array> vtkStridedTypeUInt64Array_new() {
    return vtkNew<vtkStridedTypeUInt64Array>();
}
extern "C" void vtkStridedTypeUInt64Array_destructor(vtkNew<vtkStridedTypeUInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStridedTypeUInt64Array_get_ptr(vtkNew<vtkStridedTypeUInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long long vtk_strided_type_u_int_64_array_get_data_type_value_min(
    vtkNew<vtkStridedTypeUInt64Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long long vtk_strided_type_u_int_64_array_get_data_type_value_max(
    vtkNew<vtkStridedTypeUInt64Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long long vtk_strided_type_u_int_64_array_get_value(
    vtkNew<vtkStridedTypeUInt64Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_strided_type_u_int_64_array_get_number_of_values(
    vtkNew<vtkStridedTypeUInt64Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStridedTypeUInt8Array> vtkStridedTypeUInt8Array_new() {
    return vtkNew<vtkStridedTypeUInt8Array>();
}
extern "C" void vtkStridedTypeUInt8Array_destructor(vtkNew<vtkStridedTypeUInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStridedTypeUInt8Array_get_ptr(vtkNew<vtkStridedTypeUInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" unsigned char vtk_strided_type_u_int_8_array_get_data_type_value_min(
    vtkNew<vtkStridedTypeUInt8Array> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned char vtk_strided_type_u_int_8_array_get_data_type_value_max(
    vtkNew<vtkStridedTypeUInt8Array> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned char vtk_strided_type_u_int_8_array_get_value(
    vtkNew<vtkStridedTypeUInt8Array> self, long long id
) {
    return self->GetValue(id);
}
extern "C" long long vtk_strided_type_u_int_8_array_get_number_of_values(
    vtkNew<vtkStridedTypeUInt8Array> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkStringArray> vtkStringArray_new() {
    return vtkNew<vtkStringArray>();
}
extern "C" void vtkStringArray_destructor(vtkNew<vtkStringArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStringArray_get_ptr(vtkNew<vtkStringArray> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_string_array_get_array_type(vtkNew<vtkStringArray> self) {
    return self->GetArrayType();
}
extern "C" int vtk_string_array_get_data_type(vtkNew<vtkStringArray> self) {
    return self->GetDataType();
}
extern "C" int vtk_string_array_is_numeric(vtkNew<vtkStringArray> self) {
    return self->IsNumeric();
}
extern "C" void vtk_string_array_initialize(vtkNew<vtkStringArray> self) {
    return self->Initialize();
}
extern "C" int vtk_string_array_get_data_type_size(vtkNew<vtkStringArray> self) {
    return self->GetDataTypeSize();
}
extern "C" void vtk_string_array_squeeze(vtkNew<vtkStringArray> self) {
    return self->Squeeze();
}
extern "C" int vtk_string_array_resize(vtkNew<vtkStringArray> self, long long numTuples) {
    return self->Resize(numTuples);
}
extern "C" int vtk_string_array_allocate(vtkNew<vtkStringArray> self, long long sz, long long ext) {
    return self->Allocate(sz, ext);
}
extern "C" void vtk_string_array_set_number_of_tuples(
    vtkNew<vtkStringArray> self, long long number
) {
    return self->SetNumberOfTuples(number);
}
extern "C" long long vtk_string_array_get_number_of_values(vtkNew<vtkStringArray> self) {
    return self->GetNumberOfValues();
}
extern "C" int vtk_string_array_get_number_of_element_components(vtkNew<vtkStringArray> self) {
    return self->GetNumberOfElementComponents();
}
extern "C" int vtk_string_array_get_element_component_size(vtkNew<vtkStringArray> self) {
    return self->GetElementComponentSize();
}
extern "C" unsigned long vtk_string_array_get_actual_memory_size(vtkNew<vtkStringArray> self) {
    return self->GetActualMemorySize();
}
extern "C" long long vtk_string_array_get_data_size(vtkNew<vtkStringArray> self) {
    return self->GetDataSize();
}
extern "C" void vtk_string_array_data_changed(vtkNew<vtkStringArray> self) {
    return self->DataChanged();
}
extern "C" void vtk_string_array_data_element_changed(vtkNew<vtkStringArray> self, long long id) {
    return self->DataElementChanged(id);
}
extern "C" void vtk_string_array_clear_lookup(vtkNew<vtkStringArray> self) {
    return self->ClearLookup();
}
extern "C" vtkNew<vtkStringOutputWindow> vtkStringOutputWindow_new() {
    return vtkNew<vtkStringOutputWindow>();
}
extern "C" void vtkStringOutputWindow_destructor(vtkNew<vtkStringOutputWindow> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkStringOutputWindow_get_ptr(vtkNew<vtkStringOutputWindow> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTimePointUtility> vtkTimePointUtility_new() {
    return vtkNew<vtkTimePointUtility>();
}
extern "C" void vtkTimePointUtility_destructor(vtkNew<vtkTimePointUtility> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTimePointUtility_get_ptr(vtkNew<vtkTimePointUtility> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long long vtk_time_point_utility_date_to_time_point(
    vtkNew<vtkTimePointUtility> self, int year, int month, int day
) {
    return self->DateToTimePoint(year, month, day);
}
extern "C" unsigned long long vtk_time_point_utility_time_to_time_point(
    vtkNew<vtkTimePointUtility> self, int hour, int minute, int second, int millis
) {
    return self->TimeToTimePoint(hour, minute, second, millis);
}
extern "C" unsigned long long vtk_time_point_utility_date_time_to_time_point(
    vtkNew<vtkTimePointUtility> self,
    int year,
    int month,
    int day,
    int hour,
    int minute,
    int sec,
    int millis
) {
    return self->DateTimeToTimePoint(year, month, day, hour, minute, sec, millis);
}
extern "C" int vtk_time_point_utility_get_year(
    vtkNew<vtkTimePointUtility> self, unsigned long long time
) {
    return self->GetYear(time);
}
extern "C" int vtk_time_point_utility_get_month(
    vtkNew<vtkTimePointUtility> self, unsigned long long time
) {
    return self->GetMonth(time);
}
extern "C" int vtk_time_point_utility_get_day(
    vtkNew<vtkTimePointUtility> self, unsigned long long time
) {
    return self->GetDay(time);
}
extern "C" int vtk_time_point_utility_get_hour(
    vtkNew<vtkTimePointUtility> self, unsigned long long time
) {
    return self->GetHour(time);
}
extern "C" int vtk_time_point_utility_get_minute(
    vtkNew<vtkTimePointUtility> self, unsigned long long time
) {
    return self->GetMinute(time);
}
extern "C" int vtk_time_point_utility_get_second(
    vtkNew<vtkTimePointUtility> self, unsigned long long time
) {
    return self->GetSecond(time);
}
extern "C" int vtk_time_point_utility_get_millisecond(
    vtkNew<vtkTimePointUtility> self, unsigned long long time
) {
    return self->GetMillisecond(time);
}
extern "C" vtkNew<vtkTypeFloat32Array> vtkTypeFloat32Array_new() {
    return vtkNew<vtkTypeFloat32Array>();
}
extern "C" void vtkTypeFloat32Array_destructor(vtkNew<vtkTypeFloat32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTypeFloat32Array_get_ptr(vtkNew<vtkTypeFloat32Array> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTypeFloat64Array> vtkTypeFloat64Array_new() {
    return vtkNew<vtkTypeFloat64Array>();
}
extern "C" void vtkTypeFloat64Array_destructor(vtkNew<vtkTypeFloat64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTypeFloat64Array_get_ptr(vtkNew<vtkTypeFloat64Array> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTypeInt16Array> vtkTypeInt16Array_new() {
    return vtkNew<vtkTypeInt16Array>();
}
extern "C" void vtkTypeInt16Array_destructor(vtkNew<vtkTypeInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTypeInt16Array_get_ptr(vtkNew<vtkTypeInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTypeInt32Array> vtkTypeInt32Array_new() {
    return vtkNew<vtkTypeInt32Array>();
}
extern "C" void vtkTypeInt32Array_destructor(vtkNew<vtkTypeInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTypeInt32Array_get_ptr(vtkNew<vtkTypeInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTypeInt64Array> vtkTypeInt64Array_new() {
    return vtkNew<vtkTypeInt64Array>();
}
extern "C" void vtkTypeInt64Array_destructor(vtkNew<vtkTypeInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTypeInt64Array_get_ptr(vtkNew<vtkTypeInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTypeInt8Array> vtkTypeInt8Array_new() {
    return vtkNew<vtkTypeInt8Array>();
}
extern "C" void vtkTypeInt8Array_destructor(vtkNew<vtkTypeInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTypeInt8Array_get_ptr(vtkNew<vtkTypeInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTypeUInt16Array> vtkTypeUInt16Array_new() {
    return vtkNew<vtkTypeUInt16Array>();
}
extern "C" void vtkTypeUInt16Array_destructor(vtkNew<vtkTypeUInt16Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTypeUInt16Array_get_ptr(vtkNew<vtkTypeUInt16Array> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTypeUInt32Array> vtkTypeUInt32Array_new() {
    return vtkNew<vtkTypeUInt32Array>();
}
extern "C" void vtkTypeUInt32Array_destructor(vtkNew<vtkTypeUInt32Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTypeUInt32Array_get_ptr(vtkNew<vtkTypeUInt32Array> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTypeUInt64Array> vtkTypeUInt64Array_new() {
    return vtkNew<vtkTypeUInt64Array>();
}
extern "C" void vtkTypeUInt64Array_destructor(vtkNew<vtkTypeUInt64Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTypeUInt64Array_get_ptr(vtkNew<vtkTypeUInt64Array> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkTypeUInt8Array> vtkTypeUInt8Array_new() {
    return vtkNew<vtkTypeUInt8Array>();
}
extern "C" void vtkTypeUInt8Array_destructor(vtkNew<vtkTypeUInt8Array> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTypeUInt8Array_get_ptr(vtkNew<vtkTypeUInt8Array> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkUnsignedCharArray> vtkUnsignedCharArray_new() {
    return vtkNew<vtkUnsignedCharArray>();
}
extern "C" void vtkUnsignedCharArray_destructor(vtkNew<vtkUnsignedCharArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkUnsignedCharArray_get_ptr(vtkNew<vtkUnsignedCharArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned char vtk_unsigned_char_array_get_data_type_value_min(
    vtkNew<vtkUnsignedCharArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned char vtk_unsigned_char_array_get_data_type_value_max(
    vtkNew<vtkUnsignedCharArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned char vtk_unsigned_char_array_get_value(
    vtkNew<vtkUnsignedCharArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_unsigned_char_array_set_value(
    vtkNew<vtkUnsignedCharArray> self, long long id, unsigned char value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_unsigned_char_array_set_number_of_values(
    vtkNew<vtkUnsignedCharArray> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_unsigned_char_array_insert_value(
    vtkNew<vtkUnsignedCharArray> self, long long id, unsigned char f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_unsigned_char_array_insert_next_value(
    vtkNew<vtkUnsignedCharArray> self, unsigned char f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_unsigned_char_array_get_number_of_values(
    vtkNew<vtkUnsignedCharArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkUnsignedIntArray> vtkUnsignedIntArray_new() {
    return vtkNew<vtkUnsignedIntArray>();
}
extern "C" void vtkUnsignedIntArray_destructor(vtkNew<vtkUnsignedIntArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkUnsignedIntArray_get_ptr(vtkNew<vtkUnsignedIntArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned int vtk_unsigned_int_array_get_data_type_value_min(
    vtkNew<vtkUnsignedIntArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned int vtk_unsigned_int_array_get_data_type_value_max(
    vtkNew<vtkUnsignedIntArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned int vtk_unsigned_int_array_get_value(
    vtkNew<vtkUnsignedIntArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_unsigned_int_array_set_value(
    vtkNew<vtkUnsignedIntArray> self, long long id, unsigned int value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_unsigned_int_array_set_number_of_values(
    vtkNew<vtkUnsignedIntArray> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_unsigned_int_array_insert_value(
    vtkNew<vtkUnsignedIntArray> self, long long id, unsigned int f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_unsigned_int_array_insert_next_value(
    vtkNew<vtkUnsignedIntArray> self, unsigned int f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_unsigned_int_array_get_number_of_values(vtkNew<vtkUnsignedIntArray> self) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkUnsignedLongArray> vtkUnsignedLongArray_new() {
    return vtkNew<vtkUnsignedLongArray>();
}
extern "C" void vtkUnsignedLongArray_destructor(vtkNew<vtkUnsignedLongArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkUnsignedLongArray_get_ptr(vtkNew<vtkUnsignedLongArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long vtk_unsigned_long_array_get_data_type_value_min(
    vtkNew<vtkUnsignedLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long vtk_unsigned_long_array_get_data_type_value_max(
    vtkNew<vtkUnsignedLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long vtk_unsigned_long_array_get_value(
    vtkNew<vtkUnsignedLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_unsigned_long_array_set_value(
    vtkNew<vtkUnsignedLongArray> self, long long id, unsigned long value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_unsigned_long_array_set_number_of_values(
    vtkNew<vtkUnsignedLongArray> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_unsigned_long_array_insert_value(
    vtkNew<vtkUnsignedLongArray> self, long long id, unsigned long f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_unsigned_long_array_insert_next_value(
    vtkNew<vtkUnsignedLongArray> self, unsigned long f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_unsigned_long_array_get_number_of_values(
    vtkNew<vtkUnsignedLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkUnsignedLongLongArray> vtkUnsignedLongLongArray_new() {
    return vtkNew<vtkUnsignedLongLongArray>();
}
extern "C" void vtkUnsignedLongLongArray_destructor(vtkNew<vtkUnsignedLongLongArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkUnsignedLongLongArray_get_ptr(vtkNew<vtkUnsignedLongLongArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned long long vtk_unsigned_long_long_array_get_data_type_value_min(
    vtkNew<vtkUnsignedLongLongArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned long long vtk_unsigned_long_long_array_get_data_type_value_max(
    vtkNew<vtkUnsignedLongLongArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned long long vtk_unsigned_long_long_array_get_value(
    vtkNew<vtkUnsignedLongLongArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_unsigned_long_long_array_set_value(
    vtkNew<vtkUnsignedLongLongArray> self, long long id, unsigned long long value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_unsigned_long_long_array_set_number_of_values(
    vtkNew<vtkUnsignedLongLongArray> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_unsigned_long_long_array_insert_value(
    vtkNew<vtkUnsignedLongLongArray> self, long long id, unsigned long long f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_unsigned_long_long_array_insert_next_value(
    vtkNew<vtkUnsignedLongLongArray> self, unsigned long long f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_unsigned_long_long_array_get_number_of_values(
    vtkNew<vtkUnsignedLongLongArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkUnsignedShortArray> vtkUnsignedShortArray_new() {
    return vtkNew<vtkUnsignedShortArray>();
}
extern "C" void vtkUnsignedShortArray_destructor(vtkNew<vtkUnsignedShortArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkUnsignedShortArray_get_ptr(vtkNew<vtkUnsignedShortArray> sself) {
    return sself.GetPointer();
}
extern "C" unsigned short vtk_unsigned_short_array_get_data_type_value_min(
    vtkNew<vtkUnsignedShortArray> self
) {
    return self->GetDataTypeValueMin();
}
extern "C" unsigned short vtk_unsigned_short_array_get_data_type_value_max(
    vtkNew<vtkUnsignedShortArray> self
) {
    return self->GetDataTypeValueMax();
}
extern "C" unsigned short vtk_unsigned_short_array_get_value(
    vtkNew<vtkUnsignedShortArray> self, long long id
) {
    return self->GetValue(id);
}
extern "C" void vtk_unsigned_short_array_set_value(
    vtkNew<vtkUnsignedShortArray> self, long long id, unsigned short value
) {
    return self->SetValue(id, value);
}
extern "C" bool vtk_unsigned_short_array_set_number_of_values(
    vtkNew<vtkUnsignedShortArray> self, long long number
) {
    return self->SetNumberOfValues(number);
}
extern "C" void vtk_unsigned_short_array_insert_value(
    vtkNew<vtkUnsignedShortArray> self, long long id, unsigned short f
) {
    return self->InsertValue(id, f);
}
extern "C" long long vtk_unsigned_short_array_insert_next_value(
    vtkNew<vtkUnsignedShortArray> self, unsigned short f
) {
    return self->InsertNextValue(f);
}
extern "C" long long vtk_unsigned_short_array_get_number_of_values(
    vtkNew<vtkUnsignedShortArray> self
) {
    return self->GetNumberOfValues();
}
extern "C" vtkNew<vtkVariantArray> vtkVariantArray_new() {
    return vtkNew<vtkVariantArray>();
}
extern "C" void vtkVariantArray_destructor(vtkNew<vtkVariantArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkVariantArray_get_ptr(vtkNew<vtkVariantArray> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_variant_array_allocate(
    vtkNew<vtkVariantArray> self, long long sz, long long ext
) {
    return self->Allocate(sz, ext);
}
extern "C" void vtk_variant_array_initialize(vtkNew<vtkVariantArray> self) {
    return self->Initialize();
}
extern "C" int vtk_variant_array_get_array_type(vtkNew<vtkVariantArray> self) {
    return self->GetArrayType();
}
extern "C" int vtk_variant_array_get_data_type(vtkNew<vtkVariantArray> self) {
    return self->GetDataType();
}
extern "C" int vtk_variant_array_get_data_type_size(vtkNew<vtkVariantArray> self) {
    return self->GetDataTypeSize();
}
extern "C" int vtk_variant_array_get_element_component_size(vtkNew<vtkVariantArray> self) {
    return self->GetElementComponentSize();
}
extern "C" void vtk_variant_array_set_number_of_tuples(
    vtkNew<vtkVariantArray> self, long long number
) {
    return self->SetNumberOfTuples(number);
}
extern "C" void vtk_variant_array_squeeze(vtkNew<vtkVariantArray> self) {
    return self->Squeeze();
}
extern "C" int vtk_variant_array_resize(vtkNew<vtkVariantArray> self, long long numTuples) {
    return self->Resize(numTuples);
}
extern "C" unsigned long vtk_variant_array_get_actual_memory_size(vtkNew<vtkVariantArray> self) {
    return self->GetActualMemorySize();
}
extern "C" int vtk_variant_array_is_numeric(vtkNew<vtkVariantArray> self) {
    return self->IsNumeric();
}
extern "C" long long vtk_variant_array_get_number_of_values(vtkNew<vtkVariantArray> self) {
    return self->GetNumberOfValues();
}
extern "C" void vtk_variant_array_data_changed(vtkNew<vtkVariantArray> self) {
    return self->DataChanged();
}
extern "C" void vtk_variant_array_data_element_changed(vtkNew<vtkVariantArray> self, long long id) {
    return self->DataElementChanged(id);
}
extern "C" void vtk_variant_array_clear_lookup(vtkNew<vtkVariantArray> self) {
    return self->ClearLookup();
}
extern "C" vtkNew<vtkVersion> vtkVersion_new() {
    return vtkNew<vtkVersion>();
}
extern "C" void vtkVersion_destructor(vtkNew<vtkVersion> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkVersion_get_ptr(vtkNew<vtkVersion> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_version_get_vtk_major_version(vtkNew<vtkVersion> self) {
    return self->GetVTKMajorVersion();
}
extern "C" int vtk_version_get_vtk_minor_version(vtkNew<vtkVersion> self) {
    return self->GetVTKMinorVersion();
}
extern "C" int vtk_version_get_vtk_build_version(vtkNew<vtkVersion> self) {
    return self->GetVTKBuildVersion();
}
extern "C" vtkNew<vtkVoidArray> vtkVoidArray_new() {
    return vtkNew<vtkVoidArray>();
}
extern "C" void vtkVoidArray_destructor(vtkNew<vtkVoidArray> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkVoidArray_get_ptr(vtkNew<vtkVoidArray> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_void_array_allocate(vtkNew<vtkVoidArray> self, long long sz, long long ext) {
    return self->Allocate(sz, ext);
}
extern "C" void vtk_void_array_initialize(vtkNew<vtkVoidArray> self) {
    return self->Initialize();
}
extern "C" int vtk_void_array_get_data_type(vtkNew<vtkVoidArray> self) {
    return self->GetDataType();
}
extern "C" int vtk_void_array_get_data_type_size(vtkNew<vtkVoidArray> self) {
    return self->GetDataTypeSize();
}
extern "C" void vtk_void_array_set_number_of_pointers(vtkNew<vtkVoidArray> self, long long number) {
    return self->SetNumberOfPointers(number);
}
extern "C" long long vtk_void_array_get_number_of_pointers(vtkNew<vtkVoidArray> self) {
    return self->GetNumberOfPointers();
}
extern "C" void vtk_void_array_reset(vtkNew<vtkVoidArray> self) {
    return self->Reset();
}
extern "C" void vtk_void_array_squeeze(vtkNew<vtkVoidArray> self) {
    return self->Squeeze();
}
extern "C" vtkNew<vtkWeakReference> vtkWeakReference_new() {
    return vtkNew<vtkWeakReference>();
}
extern "C" void vtkWeakReference_destructor(vtkNew<vtkWeakReference> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkWeakReference_get_ptr(vtkNew<vtkWeakReference> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkXMLFileOutputWindow> vtkXMLFileOutputWindow_new() {
    return vtkNew<vtkXMLFileOutputWindow>();
}
extern "C" void vtkXMLFileOutputWindow_destructor(vtkNew<vtkXMLFileOutputWindow> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkXMLFileOutputWindow_get_ptr(vtkNew<vtkXMLFileOutputWindow> sself) {
    return sself.GetPointer();
}
