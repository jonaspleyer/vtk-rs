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
#include<vtkAtomicMutex.h>
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
extern "C" vtkNew < vtkAnimationCue > vtkAnimationCue_new () {return vtkNew < vtkAnimationCue > () ;}
extern "C" void vtkAnimationCue_destructor (vtkNew < vtkAnimationCue > sself) {sself . Reset () ; return ;}
extern "C" void * vtkAnimationCue_get_ptr (vtkNew < vtkAnimationCue > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkArchiver > vtkArchiver_new () {return vtkNew < vtkArchiver > () ;}
extern "C" void vtkArchiver_destructor (vtkNew < vtkArchiver > sself) {sself . Reset () ; return ;}
extern "C" void * vtkArchiver_get_ptr (vtkNew < vtkArchiver > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBitArray > vtkBitArray_new () {return vtkNew < vtkBitArray > () ;}
extern "C" void vtkBitArray_destructor (vtkNew < vtkBitArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBitArray_get_ptr (vtkNew < vtkBitArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBitArrayIterator > vtkBitArrayIterator_new () {return vtkNew < vtkBitArrayIterator > () ;}
extern "C" void vtkBitArrayIterator_destructor (vtkNew < vtkBitArrayIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBitArrayIterator_get_ptr (vtkNew < vtkBitArrayIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkBoxMuellerRandomSequence > vtkBoxMuellerRandomSequence_new () {return vtkNew < vtkBoxMuellerRandomSequence > () ;}
extern "C" void vtkBoxMuellerRandomSequence_destructor (vtkNew < vtkBoxMuellerRandomSequence > sself) {sself . Reset () ; return ;}
extern "C" void * vtkBoxMuellerRandomSequence_get_ptr (vtkNew < vtkBoxMuellerRandomSequence > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkByteSwap > vtkByteSwap_new () {return vtkNew < vtkByteSwap > () ;}
extern "C" void vtkByteSwap_destructor (vtkNew < vtkByteSwap > sself) {sself . Reset () ; return ;}
extern "C" void * vtkByteSwap_get_ptr (vtkNew < vtkByteSwap > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCallbackCommand > vtkCallbackCommand_new () {return vtkNew < vtkCallbackCommand > () ;}
extern "C" void vtkCallbackCommand_destructor (vtkNew < vtkCallbackCommand > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCallbackCommand_get_ptr (vtkNew < vtkCallbackCommand > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCharArray > vtkCharArray_new () {return vtkNew < vtkCharArray > () ;}
extern "C" void vtkCharArray_destructor (vtkNew < vtkCharArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCharArray_get_ptr (vtkNew < vtkCharArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCollection > vtkCollection_new () {return vtkNew < vtkCollection > () ;}
extern "C" void vtkCollection_destructor (vtkNew < vtkCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCollection_get_ptr (vtkNew < vtkCollection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCollectionIterator > vtkCollectionIterator_new () {return vtkNew < vtkCollectionIterator > () ;}
extern "C" void vtkCollectionIterator_destructor (vtkNew < vtkCollectionIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCollectionIterator_get_ptr (vtkNew < vtkCollectionIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkCriticalSection > vtkCriticalSection_new () {return vtkNew < vtkCriticalSection > () ;}
extern "C" void vtkCriticalSection_destructor (vtkNew < vtkCriticalSection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkCriticalSection_get_ptr (vtkNew < vtkCriticalSection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDataArrayCollection > vtkDataArrayCollection_new () {return vtkNew < vtkDataArrayCollection > () ;}
extern "C" void vtkDataArrayCollection_destructor (vtkNew < vtkDataArrayCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDataArrayCollection_get_ptr (vtkNew < vtkDataArrayCollection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDataArrayCollectionIterator > vtkDataArrayCollectionIterator_new () {return vtkNew < vtkDataArrayCollectionIterator > () ;}
extern "C" void vtkDataArrayCollectionIterator_destructor (vtkNew < vtkDataArrayCollectionIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDataArrayCollectionIterator_get_ptr (vtkNew < vtkDataArrayCollectionIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDataArraySelection > vtkDataArraySelection_new () {return vtkNew < vtkDataArraySelection > () ;}
extern "C" void vtkDataArraySelection_destructor (vtkNew < vtkDataArraySelection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDataArraySelection_get_ptr (vtkNew < vtkDataArraySelection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDebugLeaks > vtkDebugLeaks_new () {return vtkNew < vtkDebugLeaks > () ;}
extern "C" void vtkDebugLeaks_destructor (vtkNew < vtkDebugLeaks > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDebugLeaks_get_ptr (vtkNew < vtkDebugLeaks > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDoubleArray > vtkDoubleArray_new () {return vtkNew < vtkDoubleArray > () ;}
extern "C" void vtkDoubleArray_destructor (vtkNew < vtkDoubleArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDoubleArray_get_ptr (vtkNew < vtkDoubleArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkDynamicLoader > vtkDynamicLoader_new () {return vtkNew < vtkDynamicLoader > () ;}
extern "C" void vtkDynamicLoader_destructor (vtkNew < vtkDynamicLoader > sself) {sself . Reset () ; return ;}
extern "C" void * vtkDynamicLoader_get_ptr (vtkNew < vtkDynamicLoader > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkEventDataDevice3D > vtkEventDataDevice3D_new () {return vtkNew < vtkEventDataDevice3D > () ;}
extern "C" void vtkEventDataDevice3D_destructor (vtkNew < vtkEventDataDevice3D > sself) {sself . Reset () ; return ;}
extern "C" void * vtkEventDataDevice3D_get_ptr (vtkNew < vtkEventDataDevice3D > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkEventDataForDevice > vtkEventDataForDevice_new () {return vtkNew < vtkEventDataForDevice > () ;}
extern "C" void vtkEventDataForDevice_destructor (vtkNew < vtkEventDataForDevice > sself) {sself . Reset () ; return ;}
extern "C" void * vtkEventDataForDevice_get_ptr (vtkNew < vtkEventDataForDevice > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkEventForwarderCommand > vtkEventForwarderCommand_new () {return vtkNew < vtkEventForwarderCommand > () ;}
extern "C" void vtkEventForwarderCommand_destructor (vtkNew < vtkEventForwarderCommand > sself) {sself . Reset () ; return ;}
extern "C" void * vtkEventForwarderCommand_get_ptr (vtkNew < vtkEventForwarderCommand > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkFileOutputWindow > vtkFileOutputWindow_new () {return vtkNew < vtkFileOutputWindow > () ;}
extern "C" void vtkFileOutputWindow_destructor (vtkNew < vtkFileOutputWindow > sself) {sself . Reset () ; return ;}
extern "C" void * vtkFileOutputWindow_get_ptr (vtkNew < vtkFileOutputWindow > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkFloatArray > vtkFloatArray_new () {return vtkNew < vtkFloatArray > () ;}
extern "C" void vtkFloatArray_destructor (vtkNew < vtkFloatArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkFloatArray_get_ptr (vtkNew < vtkFloatArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkGarbageCollector > vtkGarbageCollector_new () {return vtkNew < vtkGarbageCollector > () ;}
extern "C" void vtkGarbageCollector_destructor (vtkNew < vtkGarbageCollector > sself) {sself . Reset () ; return ;}
extern "C" void * vtkGarbageCollector_get_ptr (vtkNew < vtkGarbageCollector > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkIdList > vtkIdList_new () {return vtkNew < vtkIdList > () ;}
extern "C" void vtkIdList_destructor (vtkNew < vtkIdList > sself) {sself . Reset () ; return ;}
extern "C" void * vtkIdList_get_ptr (vtkNew < vtkIdList > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkIdListCollection > vtkIdListCollection_new () {return vtkNew < vtkIdListCollection > () ;}
extern "C" void vtkIdListCollection_destructor (vtkNew < vtkIdListCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkIdListCollection_get_ptr (vtkNew < vtkIdListCollection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkIdTypeArray > vtkIdTypeArray_new () {return vtkNew < vtkIdTypeArray > () ;}
extern "C" void vtkIdTypeArray_destructor (vtkNew < vtkIdTypeArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkIdTypeArray_get_ptr (vtkNew < vtkIdTypeArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkInformation > vtkInformation_new () {return vtkNew < vtkInformation > () ;}
extern "C" void vtkInformation_destructor (vtkNew < vtkInformation > sself) {sself . Reset () ; return ;}
extern "C" void * vtkInformation_get_ptr (vtkNew < vtkInformation > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkInformationIterator > vtkInformationIterator_new () {return vtkNew < vtkInformationIterator > () ;}
extern "C" void vtkInformationIterator_destructor (vtkNew < vtkInformationIterator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkInformationIterator_get_ptr (vtkNew < vtkInformationIterator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkInformationKeyLookup > vtkInformationKeyLookup_new () {return vtkNew < vtkInformationKeyLookup > () ;}
extern "C" void vtkInformationKeyLookup_destructor (vtkNew < vtkInformationKeyLookup > sself) {sself . Reset () ; return ;}
extern "C" void * vtkInformationKeyLookup_get_ptr (vtkNew < vtkInformationKeyLookup > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkInformationVector > vtkInformationVector_new () {return vtkNew < vtkInformationVector > () ;}
extern "C" void vtkInformationVector_destructor (vtkNew < vtkInformationVector > sself) {sself . Reset () ; return ;}
extern "C" void * vtkInformationVector_get_ptr (vtkNew < vtkInformationVector > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkIntArray > vtkIntArray_new () {return vtkNew < vtkIntArray > () ;}
extern "C" void vtkIntArray_destructor (vtkNew < vtkIntArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkIntArray_get_ptr (vtkNew < vtkIntArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkLongArray > vtkLongArray_new () {return vtkNew < vtkLongArray > () ;}
extern "C" void vtkLongArray_destructor (vtkNew < vtkLongArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkLongArray_get_ptr (vtkNew < vtkLongArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkLongLongArray > vtkLongLongArray_new () {return vtkNew < vtkLongLongArray > () ;}
extern "C" void vtkLongLongArray_destructor (vtkNew < vtkLongLongArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkLongLongArray_get_ptr (vtkNew < vtkLongLongArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkLookupTable > vtkLookupTable_new () {return vtkNew < vtkLookupTable > () ;}
extern "C" void vtkLookupTable_destructor (vtkNew < vtkLookupTable > sself) {sself . Reset () ; return ;}
extern "C" void * vtkLookupTable_get_ptr (vtkNew < vtkLookupTable > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMath > vtkMath_new () {return vtkNew < vtkMath > () ;}
extern "C" void vtkMath_destructor (vtkNew < vtkMath > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMath_get_ptr (vtkNew < vtkMath > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMersenneTwister > vtkMersenneTwister_new () {return vtkNew < vtkMersenneTwister > () ;}
extern "C" void vtkMersenneTwister_destructor (vtkNew < vtkMersenneTwister > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMersenneTwister_get_ptr (vtkNew < vtkMersenneTwister > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMinimalStandardRandomSequence > vtkMinimalStandardRandomSequence_new () {return vtkNew < vtkMinimalStandardRandomSequence > () ;}
extern "C" void vtkMinimalStandardRandomSequence_destructor (vtkNew < vtkMinimalStandardRandomSequence > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMinimalStandardRandomSequence_get_ptr (vtkNew < vtkMinimalStandardRandomSequence > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMultiThreader > vtkMultiThreader_new () {return vtkNew < vtkMultiThreader > () ;}
extern "C" void vtkMultiThreader_destructor (vtkNew < vtkMultiThreader > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMultiThreader_get_ptr (vtkNew < vtkMultiThreader > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkObject > vtkObject_new () {return vtkNew < vtkObject > () ;}
extern "C" void vtkObject_destructor (vtkNew < vtkObject > sself) {sself . Reset () ; return ;}
extern "C" void * vtkObject_get_ptr (vtkNew < vtkObject > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkObjectFactoryCollection > vtkObjectFactoryCollection_new () {return vtkNew < vtkObjectFactoryCollection > () ;}
extern "C" void vtkObjectFactoryCollection_destructor (vtkNew < vtkObjectFactoryCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkObjectFactoryCollection_get_ptr (vtkNew < vtkObjectFactoryCollection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkOldStyleCallbackCommand > vtkOldStyleCallbackCommand_new () {return vtkNew < vtkOldStyleCallbackCommand > () ;}
extern "C" void vtkOldStyleCallbackCommand_destructor (vtkNew < vtkOldStyleCallbackCommand > sself) {sself . Reset () ; return ;}
extern "C" void * vtkOldStyleCallbackCommand_get_ptr (vtkNew < vtkOldStyleCallbackCommand > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkOutputWindow > vtkOutputWindow_new () {return vtkNew < vtkOutputWindow > () ;}
extern "C" void vtkOutputWindow_destructor (vtkNew < vtkOutputWindow > sself) {sself . Reset () ; return ;}
extern "C" void * vtkOutputWindow_get_ptr (vtkNew < vtkOutputWindow > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkOverrideInformationCollection > vtkOverrideInformationCollection_new () {return vtkNew < vtkOverrideInformationCollection > () ;}
extern "C" void vtkOverrideInformationCollection_destructor (vtkNew < vtkOverrideInformationCollection > sself) {sself . Reset () ; return ;}
extern "C" void * vtkOverrideInformationCollection_get_ptr (vtkNew < vtkOverrideInformationCollection > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPoints > vtkPoints_new () {return vtkNew < vtkPoints > () ;}
extern "C" void vtkPoints_destructor (vtkNew < vtkPoints > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPoints_get_ptr (vtkNew < vtkPoints > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPoints2D > vtkPoints2D_new () {return vtkNew < vtkPoints2D > () ;}
extern "C" void vtkPoints2D_destructor (vtkNew < vtkPoints2D > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPoints2D_get_ptr (vtkNew < vtkPoints2D > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPriorityQueue > vtkPriorityQueue_new () {return vtkNew < vtkPriorityQueue > () ;}
extern "C" void vtkPriorityQueue_destructor (vtkNew < vtkPriorityQueue > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPriorityQueue_get_ptr (vtkNew < vtkPriorityQueue > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkRandomPool > vtkRandomPool_new () {return vtkNew < vtkRandomPool > () ;}
extern "C" void vtkRandomPool_destructor (vtkNew < vtkRandomPool > sself) {sself . Reset () ; return ;}
extern "C" void * vtkRandomPool_get_ptr (vtkNew < vtkRandomPool > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkReferenceCount > vtkReferenceCount_new () {return vtkNew < vtkReferenceCount > () ;}
extern "C" void vtkReferenceCount_destructor (vtkNew < vtkReferenceCount > sself) {sself . Reset () ; return ;}
extern "C" void * vtkReferenceCount_get_ptr (vtkNew < vtkReferenceCount > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkScalarsToColors > vtkScalarsToColors_new () {return vtkNew < vtkScalarsToColors > () ;}
extern "C" void vtkScalarsToColors_destructor (vtkNew < vtkScalarsToColors > sself) {sself . Reset () ; return ;}
extern "C" void * vtkScalarsToColors_get_ptr (vtkNew < vtkScalarsToColors > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkShortArray > vtkShortArray_new () {return vtkNew < vtkShortArray > () ;}
extern "C" void vtkShortArray_destructor (vtkNew < vtkShortArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkShortArray_get_ptr (vtkNew < vtkShortArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSignedCharArray > vtkSignedCharArray_new () {return vtkNew < vtkSignedCharArray > () ;}
extern "C" void vtkSignedCharArray_destructor (vtkNew < vtkSignedCharArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSignedCharArray_get_ptr (vtkNew < vtkSignedCharArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkSortDataArray > vtkSortDataArray_new () {return vtkNew < vtkSortDataArray > () ;}
extern "C" void vtkSortDataArray_destructor (vtkNew < vtkSortDataArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkSortDataArray_get_ptr (vtkNew < vtkSortDataArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkStringArray > vtkStringArray_new () {return vtkNew < vtkStringArray > () ;}
extern "C" void vtkStringArray_destructor (vtkNew < vtkStringArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkStringArray_get_ptr (vtkNew < vtkStringArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkStringOutputWindow > vtkStringOutputWindow_new () {return vtkNew < vtkStringOutputWindow > () ;}
extern "C" void vtkStringOutputWindow_destructor (vtkNew < vtkStringOutputWindow > sself) {sself . Reset () ; return ;}
extern "C" void * vtkStringOutputWindow_get_ptr (vtkNew < vtkStringOutputWindow > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTimePointUtility > vtkTimePointUtility_new () {return vtkNew < vtkTimePointUtility > () ;}
extern "C" void vtkTimePointUtility_destructor (vtkNew < vtkTimePointUtility > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTimePointUtility_get_ptr (vtkNew < vtkTimePointUtility > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTypeFloat32Array > vtkTypeFloat32Array_new () {return vtkNew < vtkTypeFloat32Array > () ;}
extern "C" void vtkTypeFloat32Array_destructor (vtkNew < vtkTypeFloat32Array > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTypeFloat32Array_get_ptr (vtkNew < vtkTypeFloat32Array > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTypeFloat64Array > vtkTypeFloat64Array_new () {return vtkNew < vtkTypeFloat64Array > () ;}
extern "C" void vtkTypeFloat64Array_destructor (vtkNew < vtkTypeFloat64Array > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTypeFloat64Array_get_ptr (vtkNew < vtkTypeFloat64Array > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTypeInt16Array > vtkTypeInt16Array_new () {return vtkNew < vtkTypeInt16Array > () ;}
extern "C" void vtkTypeInt16Array_destructor (vtkNew < vtkTypeInt16Array > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTypeInt16Array_get_ptr (vtkNew < vtkTypeInt16Array > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTypeInt32Array > vtkTypeInt32Array_new () {return vtkNew < vtkTypeInt32Array > () ;}
extern "C" void vtkTypeInt32Array_destructor (vtkNew < vtkTypeInt32Array > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTypeInt32Array_get_ptr (vtkNew < vtkTypeInt32Array > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTypeInt64Array > vtkTypeInt64Array_new () {return vtkNew < vtkTypeInt64Array > () ;}
extern "C" void vtkTypeInt64Array_destructor (vtkNew < vtkTypeInt64Array > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTypeInt64Array_get_ptr (vtkNew < vtkTypeInt64Array > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTypeInt8Array > vtkTypeInt8Array_new () {return vtkNew < vtkTypeInt8Array > () ;}
extern "C" void vtkTypeInt8Array_destructor (vtkNew < vtkTypeInt8Array > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTypeInt8Array_get_ptr (vtkNew < vtkTypeInt8Array > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTypeUInt16Array > vtkTypeUInt16Array_new () {return vtkNew < vtkTypeUInt16Array > () ;}
extern "C" void vtkTypeUInt16Array_destructor (vtkNew < vtkTypeUInt16Array > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTypeUInt16Array_get_ptr (vtkNew < vtkTypeUInt16Array > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTypeUInt32Array > vtkTypeUInt32Array_new () {return vtkNew < vtkTypeUInt32Array > () ;}
extern "C" void vtkTypeUInt32Array_destructor (vtkNew < vtkTypeUInt32Array > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTypeUInt32Array_get_ptr (vtkNew < vtkTypeUInt32Array > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTypeUInt64Array > vtkTypeUInt64Array_new () {return vtkNew < vtkTypeUInt64Array > () ;}
extern "C" void vtkTypeUInt64Array_destructor (vtkNew < vtkTypeUInt64Array > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTypeUInt64Array_get_ptr (vtkNew < vtkTypeUInt64Array > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkTypeUInt8Array > vtkTypeUInt8Array_new () {return vtkNew < vtkTypeUInt8Array > () ;}
extern "C" void vtkTypeUInt8Array_destructor (vtkNew < vtkTypeUInt8Array > sself) {sself . Reset () ; return ;}
extern "C" void * vtkTypeUInt8Array_get_ptr (vtkNew < vtkTypeUInt8Array > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUnsignedCharArray > vtkUnsignedCharArray_new () {return vtkNew < vtkUnsignedCharArray > () ;}
extern "C" void vtkUnsignedCharArray_destructor (vtkNew < vtkUnsignedCharArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUnsignedCharArray_get_ptr (vtkNew < vtkUnsignedCharArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUnsignedIntArray > vtkUnsignedIntArray_new () {return vtkNew < vtkUnsignedIntArray > () ;}
extern "C" void vtkUnsignedIntArray_destructor (vtkNew < vtkUnsignedIntArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUnsignedIntArray_get_ptr (vtkNew < vtkUnsignedIntArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUnsignedLongArray > vtkUnsignedLongArray_new () {return vtkNew < vtkUnsignedLongArray > () ;}
extern "C" void vtkUnsignedLongArray_destructor (vtkNew < vtkUnsignedLongArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUnsignedLongArray_get_ptr (vtkNew < vtkUnsignedLongArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUnsignedLongLongArray > vtkUnsignedLongLongArray_new () {return vtkNew < vtkUnsignedLongLongArray > () ;}
extern "C" void vtkUnsignedLongLongArray_destructor (vtkNew < vtkUnsignedLongLongArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUnsignedLongLongArray_get_ptr (vtkNew < vtkUnsignedLongLongArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkUnsignedShortArray > vtkUnsignedShortArray_new () {return vtkNew < vtkUnsignedShortArray > () ;}
extern "C" void vtkUnsignedShortArray_destructor (vtkNew < vtkUnsignedShortArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkUnsignedShortArray_get_ptr (vtkNew < vtkUnsignedShortArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkVariantArray > vtkVariantArray_new () {return vtkNew < vtkVariantArray > () ;}
extern "C" void vtkVariantArray_destructor (vtkNew < vtkVariantArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkVariantArray_get_ptr (vtkNew < vtkVariantArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkVersion > vtkVersion_new () {return vtkNew < vtkVersion > () ;}
extern "C" void vtkVersion_destructor (vtkNew < vtkVersion > sself) {sself . Reset () ; return ;}
extern "C" void * vtkVersion_get_ptr (vtkNew < vtkVersion > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkVoidArray > vtkVoidArray_new () {return vtkNew < vtkVoidArray > () ;}
extern "C" void vtkVoidArray_destructor (vtkNew < vtkVoidArray > sself) {sself . Reset () ; return ;}
extern "C" void * vtkVoidArray_get_ptr (vtkNew < vtkVoidArray > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkWeakReference > vtkWeakReference_new () {return vtkNew < vtkWeakReference > () ;}
extern "C" void vtkWeakReference_destructor (vtkNew < vtkWeakReference > sself) {sself . Reset () ; return ;}
extern "C" void * vtkWeakReference_get_ptr (vtkNew < vtkWeakReference > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkXMLFileOutputWindow > vtkXMLFileOutputWindow_new () {return vtkNew < vtkXMLFileOutputWindow > () ;}
extern "C" void vtkXMLFileOutputWindow_destructor (vtkNew < vtkXMLFileOutputWindow > sself) {sself . Reset () ; return ;}
extern "C" void * vtkXMLFileOutputWindow_get_ptr (vtkNew < vtkXMLFileOutputWindow > sself) {return sself . GetPointer () ;}
