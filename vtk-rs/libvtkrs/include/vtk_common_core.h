// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkAOSDataArrayTemplate.h>
#include<vtkAbstractArray.h>
#include<vtkAffineCharArray.h>
#include<vtkAffineDoubleArray.h>
#include<vtkAffineFloatArray.h>
#include<vtkAffineIdTypeArray.h>
#include<vtkAffineIntArray.h>
#include<vtkAffineLongArray.h>
#include<vtkAffineLongLongArray.h>
#include<vtkAffineShortArray.h>
#include<vtkAffineSignedCharArray.h>
#include<vtkAffineUnsignedCharArray.h>
#include<vtkAffineUnsignedIntArray.h>
#include<vtkAffineUnsignedLongArray.h>
#include<vtkAffineUnsignedLongLongArray.h>
#include<vtkAffineUnsignedShortArray.h>
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
#include<vtkBatch.h>
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
#include<vtkCompositeCharArray.h>
#include<vtkCompositeDoubleArray.h>
#include<vtkCompositeFloatArray.h>
#include<vtkCompositeIdTypeArray.h>
#include<vtkCompositeIntArray.h>
#include<vtkCompositeLongArray.h>
#include<vtkCompositeLongLongArray.h>
#include<vtkCompositeShortArray.h>
#include<vtkCompositeSignedCharArray.h>
#include<vtkCompositeUnsignedCharArray.h>
#include<vtkCompositeUnsignedIntArray.h>
#include<vtkCompositeUnsignedLongArray.h>
#include<vtkCompositeUnsignedLongLongArray.h>
#include<vtkCompositeUnsignedShortArray.h>
#include<vtkConstantCharArray.h>
#include<vtkConstantDoubleArray.h>
#include<vtkConstantFloatArray.h>
#include<vtkConstantIdTypeArray.h>
#include<vtkConstantIntArray.h>
#include<vtkConstantLongArray.h>
#include<vtkConstantLongLongArray.h>
#include<vtkConstantShortArray.h>
#include<vtkConstantSignedCharArray.h>
#include<vtkConstantUnsignedCharArray.h>
#include<vtkConstantUnsignedIntArray.h>
#include<vtkConstantUnsignedLongArray.h>
#include<vtkConstantUnsignedLongLongArray.h>
#include<vtkConstantUnsignedShortArray.h>
#include<vtkDataArray.h>
#include<vtkDataArrayCollection.h>
#include<vtkDataArrayCollectionIterator.h>
#include<vtkDataArraySelection.h>
#include<vtkDebugLeaks.h>
#include<vtkDebugLeaksManager.h>
#include<vtkDebugLeaks.h>
#include<vtkDenseArray.h>
#include<vtkDeserializer.h>
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
#include<vtkIndexedCharArray.h>
#include<vtkIndexedDoubleArray.h>
#include<vtkIndexedFloatArray.h>
#include<vtkIndexedIdTypeArray.h>
#include<vtkIndexedIntArray.h>
#include<vtkIndexedLongArray.h>
#include<vtkIndexedLongLongArray.h>
#include<vtkIndexedShortArray.h>
#include<vtkIndexedSignedCharArray.h>
#include<vtkIndexedUnsignedCharArray.h>
#include<vtkIndexedUnsignedIntArray.h>
#include<vtkIndexedUnsignedLongArray.h>
#include<vtkIndexedUnsignedLongLongArray.h>
#include<vtkIndexedUnsignedShortArray.h>
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
#include<vtkInvoker.h>
#include<vtkLargeInteger.h>
#include<vtkLogger.h>
#include<vtkLongArray.h>
#include<vtkLongLongArray.h>
#include<vtkLookupTable.h>
#include<vtkMappedDataArray.h>
#include<vtkMarshalContext.h>
#include<vtkMath.h>
#include<vtkMersenneTwister.h>
#include<vtkMinimalStandardRandomSequence.h>
#include<vtkMultiThreader.h>
#include<vtkNew.h>
#include<vtkNumberToString.h>
#include<vtkOStrStreamWrapper.h>
#include<vtkObject.h>
#include<vtkObjectBase.h>
#include<vtkObjectFactory.h>
#include<vtkObjectFactoryCollection.h>
#include<vtkObjectFactory.h>
#include<vtkOldStyleCallbackCommand.h>
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
#include<vtkSerializer.h>
#include<vtkShortArray.h>
#include<vtkSignedCharArray.h>
#include<vtkSmartPointer.h>
#include<vtkSmartPointerBase.h>
#include<vtkSortDataArray.h>
#include<vtkSparseArray.h>
#include<vtkStdString.h>
#include<vtkStringArray.h>
#include<vtkStringOutputWindow.h>
#include<vtkStringToken.h>
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

// Declare exported functions
extern "C" vtkNew < vtkAffineCharArray > vtkAffineCharArray_new () ;
extern "C" void vtkAffineCharArray_destructor (vtkNew < vtkAffineCharArray > sself) ;
extern "C" void * vtkAffineCharArray_get_ptr (vtkNew < vtkAffineCharArray > sself) ;
extern "C" vtkNew < vtkAffineDoubleArray > vtkAffineDoubleArray_new () ;
extern "C" void vtkAffineDoubleArray_destructor (vtkNew < vtkAffineDoubleArray > sself) ;
extern "C" void * vtkAffineDoubleArray_get_ptr (vtkNew < vtkAffineDoubleArray > sself) ;
extern "C" vtkNew < vtkAffineFloatArray > vtkAffineFloatArray_new () ;
extern "C" void vtkAffineFloatArray_destructor (vtkNew < vtkAffineFloatArray > sself) ;
extern "C" void * vtkAffineFloatArray_get_ptr (vtkNew < vtkAffineFloatArray > sself) ;
extern "C" vtkNew < vtkAffineIdTypeArray > vtkAffineIdTypeArray_new () ;
extern "C" void vtkAffineIdTypeArray_destructor (vtkNew < vtkAffineIdTypeArray > sself) ;
extern "C" void * vtkAffineIdTypeArray_get_ptr (vtkNew < vtkAffineIdTypeArray > sself) ;
extern "C" vtkNew < vtkAffineIntArray > vtkAffineIntArray_new () ;
extern "C" void vtkAffineIntArray_destructor (vtkNew < vtkAffineIntArray > sself) ;
extern "C" void * vtkAffineIntArray_get_ptr (vtkNew < vtkAffineIntArray > sself) ;
extern "C" vtkNew < vtkAffineLongArray > vtkAffineLongArray_new () ;
extern "C" void vtkAffineLongArray_destructor (vtkNew < vtkAffineLongArray > sself) ;
extern "C" void * vtkAffineLongArray_get_ptr (vtkNew < vtkAffineLongArray > sself) ;
extern "C" vtkNew < vtkAffineLongLongArray > vtkAffineLongLongArray_new () ;
extern "C" void vtkAffineLongLongArray_destructor (vtkNew < vtkAffineLongLongArray > sself) ;
extern "C" void * vtkAffineLongLongArray_get_ptr (vtkNew < vtkAffineLongLongArray > sself) ;
extern "C" vtkNew < vtkAffineShortArray > vtkAffineShortArray_new () ;
extern "C" void vtkAffineShortArray_destructor (vtkNew < vtkAffineShortArray > sself) ;
extern "C" void * vtkAffineShortArray_get_ptr (vtkNew < vtkAffineShortArray > sself) ;
extern "C" vtkNew < vtkAffineSignedCharArray > vtkAffineSignedCharArray_new () ;
extern "C" void vtkAffineSignedCharArray_destructor (vtkNew < vtkAffineSignedCharArray > sself) ;
extern "C" void * vtkAffineSignedCharArray_get_ptr (vtkNew < vtkAffineSignedCharArray > sself) ;
extern "C" vtkNew < vtkAffineUnsignedCharArray > vtkAffineUnsignedCharArray_new () ;
extern "C" void vtkAffineUnsignedCharArray_destructor (vtkNew < vtkAffineUnsignedCharArray > sself) ;
extern "C" void * vtkAffineUnsignedCharArray_get_ptr (vtkNew < vtkAffineUnsignedCharArray > sself) ;
extern "C" vtkNew < vtkAffineUnsignedIntArray > vtkAffineUnsignedIntArray_new () ;
extern "C" void vtkAffineUnsignedIntArray_destructor (vtkNew < vtkAffineUnsignedIntArray > sself) ;
extern "C" void * vtkAffineUnsignedIntArray_get_ptr (vtkNew < vtkAffineUnsignedIntArray > sself) ;
extern "C" vtkNew < vtkAffineUnsignedLongArray > vtkAffineUnsignedLongArray_new () ;
extern "C" void vtkAffineUnsignedLongArray_destructor (vtkNew < vtkAffineUnsignedLongArray > sself) ;
extern "C" void * vtkAffineUnsignedLongArray_get_ptr (vtkNew < vtkAffineUnsignedLongArray > sself) ;
extern "C" vtkNew < vtkAffineUnsignedLongLongArray > vtkAffineUnsignedLongLongArray_new () ;
extern "C" void vtkAffineUnsignedLongLongArray_destructor (vtkNew < vtkAffineUnsignedLongLongArray > sself) ;
extern "C" void * vtkAffineUnsignedLongLongArray_get_ptr (vtkNew < vtkAffineUnsignedLongLongArray > sself) ;
extern "C" vtkNew < vtkAffineUnsignedShortArray > vtkAffineUnsignedShortArray_new () ;
extern "C" void vtkAffineUnsignedShortArray_destructor (vtkNew < vtkAffineUnsignedShortArray > sself) ;
extern "C" void * vtkAffineUnsignedShortArray_get_ptr (vtkNew < vtkAffineUnsignedShortArray > sself) ;
extern "C" vtkNew < vtkAnimationCue > vtkAnimationCue_new () ;
extern "C" void vtkAnimationCue_destructor (vtkNew < vtkAnimationCue > sself) ;
extern "C" void * vtkAnimationCue_get_ptr (vtkNew < vtkAnimationCue > sself) ;
extern "C" vtkNew < vtkArchiver > vtkArchiver_new () ;
extern "C" void vtkArchiver_destructor (vtkNew < vtkArchiver > sself) ;
extern "C" void * vtkArchiver_get_ptr (vtkNew < vtkArchiver > sself) ;
extern "C" vtkNew < vtkBitArray > vtkBitArray_new () ;
extern "C" void vtkBitArray_destructor (vtkNew < vtkBitArray > sself) ;
extern "C" void * vtkBitArray_get_ptr (vtkNew < vtkBitArray > sself) ;
extern "C" vtkNew < vtkBitArrayIterator > vtkBitArrayIterator_new () ;
extern "C" void vtkBitArrayIterator_destructor (vtkNew < vtkBitArrayIterator > sself) ;
extern "C" void * vtkBitArrayIterator_get_ptr (vtkNew < vtkBitArrayIterator > sself) ;
extern "C" vtkNew < vtkBoxMuellerRandomSequence > vtkBoxMuellerRandomSequence_new () ;
extern "C" void vtkBoxMuellerRandomSequence_destructor (vtkNew < vtkBoxMuellerRandomSequence > sself) ;
extern "C" void * vtkBoxMuellerRandomSequence_get_ptr (vtkNew < vtkBoxMuellerRandomSequence > sself) ;
extern "C" vtkNew < vtkByteSwap > vtkByteSwap_new () ;
extern "C" void vtkByteSwap_destructor (vtkNew < vtkByteSwap > sself) ;
extern "C" void * vtkByteSwap_get_ptr (vtkNew < vtkByteSwap > sself) ;
extern "C" vtkNew < vtkCallbackCommand > vtkCallbackCommand_new () ;
extern "C" void vtkCallbackCommand_destructor (vtkNew < vtkCallbackCommand > sself) ;
extern "C" void * vtkCallbackCommand_get_ptr (vtkNew < vtkCallbackCommand > sself) ;
extern "C" vtkNew < vtkCharArray > vtkCharArray_new () ;
extern "C" void vtkCharArray_destructor (vtkNew < vtkCharArray > sself) ;
extern "C" void * vtkCharArray_get_ptr (vtkNew < vtkCharArray > sself) ;
extern "C" vtkNew < vtkCollection > vtkCollection_new () ;
extern "C" void vtkCollection_destructor (vtkNew < vtkCollection > sself) ;
extern "C" void * vtkCollection_get_ptr (vtkNew < vtkCollection > sself) ;
extern "C" vtkNew < vtkCollectionIterator > vtkCollectionIterator_new () ;
extern "C" void vtkCollectionIterator_destructor (vtkNew < vtkCollectionIterator > sself) ;
extern "C" void * vtkCollectionIterator_get_ptr (vtkNew < vtkCollectionIterator > sself) ;
extern "C" vtkNew < vtkCompositeCharArray > vtkCompositeCharArray_new () ;
extern "C" void vtkCompositeCharArray_destructor (vtkNew < vtkCompositeCharArray > sself) ;
extern "C" void * vtkCompositeCharArray_get_ptr (vtkNew < vtkCompositeCharArray > sself) ;
extern "C" vtkNew < vtkCompositeDoubleArray > vtkCompositeDoubleArray_new () ;
extern "C" void vtkCompositeDoubleArray_destructor (vtkNew < vtkCompositeDoubleArray > sself) ;
extern "C" void * vtkCompositeDoubleArray_get_ptr (vtkNew < vtkCompositeDoubleArray > sself) ;
extern "C" vtkNew < vtkCompositeFloatArray > vtkCompositeFloatArray_new () ;
extern "C" void vtkCompositeFloatArray_destructor (vtkNew < vtkCompositeFloatArray > sself) ;
extern "C" void * vtkCompositeFloatArray_get_ptr (vtkNew < vtkCompositeFloatArray > sself) ;
extern "C" vtkNew < vtkCompositeIdTypeArray > vtkCompositeIdTypeArray_new () ;
extern "C" void vtkCompositeIdTypeArray_destructor (vtkNew < vtkCompositeIdTypeArray > sself) ;
extern "C" void * vtkCompositeIdTypeArray_get_ptr (vtkNew < vtkCompositeIdTypeArray > sself) ;
extern "C" vtkNew < vtkCompositeIntArray > vtkCompositeIntArray_new () ;
extern "C" void vtkCompositeIntArray_destructor (vtkNew < vtkCompositeIntArray > sself) ;
extern "C" void * vtkCompositeIntArray_get_ptr (vtkNew < vtkCompositeIntArray > sself) ;
extern "C" vtkNew < vtkCompositeLongArray > vtkCompositeLongArray_new () ;
extern "C" void vtkCompositeLongArray_destructor (vtkNew < vtkCompositeLongArray > sself) ;
extern "C" void * vtkCompositeLongArray_get_ptr (vtkNew < vtkCompositeLongArray > sself) ;
extern "C" vtkNew < vtkCompositeLongLongArray > vtkCompositeLongLongArray_new () ;
extern "C" void vtkCompositeLongLongArray_destructor (vtkNew < vtkCompositeLongLongArray > sself) ;
extern "C" void * vtkCompositeLongLongArray_get_ptr (vtkNew < vtkCompositeLongLongArray > sself) ;
extern "C" vtkNew < vtkCompositeShortArray > vtkCompositeShortArray_new () ;
extern "C" void vtkCompositeShortArray_destructor (vtkNew < vtkCompositeShortArray > sself) ;
extern "C" void * vtkCompositeShortArray_get_ptr (vtkNew < vtkCompositeShortArray > sself) ;
extern "C" vtkNew < vtkCompositeSignedCharArray > vtkCompositeSignedCharArray_new () ;
extern "C" void vtkCompositeSignedCharArray_destructor (vtkNew < vtkCompositeSignedCharArray > sself) ;
extern "C" void * vtkCompositeSignedCharArray_get_ptr (vtkNew < vtkCompositeSignedCharArray > sself) ;
extern "C" vtkNew < vtkCompositeUnsignedCharArray > vtkCompositeUnsignedCharArray_new () ;
extern "C" void vtkCompositeUnsignedCharArray_destructor (vtkNew < vtkCompositeUnsignedCharArray > sself) ;
extern "C" void * vtkCompositeUnsignedCharArray_get_ptr (vtkNew < vtkCompositeUnsignedCharArray > sself) ;
extern "C" vtkNew < vtkCompositeUnsignedIntArray > vtkCompositeUnsignedIntArray_new () ;
extern "C" void vtkCompositeUnsignedIntArray_destructor (vtkNew < vtkCompositeUnsignedIntArray > sself) ;
extern "C" void * vtkCompositeUnsignedIntArray_get_ptr (vtkNew < vtkCompositeUnsignedIntArray > sself) ;
extern "C" vtkNew < vtkCompositeUnsignedLongArray > vtkCompositeUnsignedLongArray_new () ;
extern "C" void vtkCompositeUnsignedLongArray_destructor (vtkNew < vtkCompositeUnsignedLongArray > sself) ;
extern "C" void * vtkCompositeUnsignedLongArray_get_ptr (vtkNew < vtkCompositeUnsignedLongArray > sself) ;
extern "C" vtkNew < vtkCompositeUnsignedLongLongArray > vtkCompositeUnsignedLongLongArray_new () ;
extern "C" void vtkCompositeUnsignedLongLongArray_destructor (vtkNew < vtkCompositeUnsignedLongLongArray > sself) ;
extern "C" void * vtkCompositeUnsignedLongLongArray_get_ptr (vtkNew < vtkCompositeUnsignedLongLongArray > sself) ;
extern "C" vtkNew < vtkCompositeUnsignedShortArray > vtkCompositeUnsignedShortArray_new () ;
extern "C" void vtkCompositeUnsignedShortArray_destructor (vtkNew < vtkCompositeUnsignedShortArray > sself) ;
extern "C" void * vtkCompositeUnsignedShortArray_get_ptr (vtkNew < vtkCompositeUnsignedShortArray > sself) ;
extern "C" vtkNew < vtkConstantCharArray > vtkConstantCharArray_new () ;
extern "C" void vtkConstantCharArray_destructor (vtkNew < vtkConstantCharArray > sself) ;
extern "C" void * vtkConstantCharArray_get_ptr (vtkNew < vtkConstantCharArray > sself) ;
extern "C" vtkNew < vtkConstantDoubleArray > vtkConstantDoubleArray_new () ;
extern "C" void vtkConstantDoubleArray_destructor (vtkNew < vtkConstantDoubleArray > sself) ;
extern "C" void * vtkConstantDoubleArray_get_ptr (vtkNew < vtkConstantDoubleArray > sself) ;
extern "C" vtkNew < vtkConstantFloatArray > vtkConstantFloatArray_new () ;
extern "C" void vtkConstantFloatArray_destructor (vtkNew < vtkConstantFloatArray > sself) ;
extern "C" void * vtkConstantFloatArray_get_ptr (vtkNew < vtkConstantFloatArray > sself) ;
extern "C" vtkNew < vtkConstantIdTypeArray > vtkConstantIdTypeArray_new () ;
extern "C" void vtkConstantIdTypeArray_destructor (vtkNew < vtkConstantIdTypeArray > sself) ;
extern "C" void * vtkConstantIdTypeArray_get_ptr (vtkNew < vtkConstantIdTypeArray > sself) ;
extern "C" vtkNew < vtkConstantIntArray > vtkConstantIntArray_new () ;
extern "C" void vtkConstantIntArray_destructor (vtkNew < vtkConstantIntArray > sself) ;
extern "C" void * vtkConstantIntArray_get_ptr (vtkNew < vtkConstantIntArray > sself) ;
extern "C" vtkNew < vtkConstantLongArray > vtkConstantLongArray_new () ;
extern "C" void vtkConstantLongArray_destructor (vtkNew < vtkConstantLongArray > sself) ;
extern "C" void * vtkConstantLongArray_get_ptr (vtkNew < vtkConstantLongArray > sself) ;
extern "C" vtkNew < vtkConstantLongLongArray > vtkConstantLongLongArray_new () ;
extern "C" void vtkConstantLongLongArray_destructor (vtkNew < vtkConstantLongLongArray > sself) ;
extern "C" void * vtkConstantLongLongArray_get_ptr (vtkNew < vtkConstantLongLongArray > sself) ;
extern "C" vtkNew < vtkConstantShortArray > vtkConstantShortArray_new () ;
extern "C" void vtkConstantShortArray_destructor (vtkNew < vtkConstantShortArray > sself) ;
extern "C" void * vtkConstantShortArray_get_ptr (vtkNew < vtkConstantShortArray > sself) ;
extern "C" vtkNew < vtkConstantSignedCharArray > vtkConstantSignedCharArray_new () ;
extern "C" void vtkConstantSignedCharArray_destructor (vtkNew < vtkConstantSignedCharArray > sself) ;
extern "C" void * vtkConstantSignedCharArray_get_ptr (vtkNew < vtkConstantSignedCharArray > sself) ;
extern "C" vtkNew < vtkConstantUnsignedCharArray > vtkConstantUnsignedCharArray_new () ;
extern "C" void vtkConstantUnsignedCharArray_destructor (vtkNew < vtkConstantUnsignedCharArray > sself) ;
extern "C" void * vtkConstantUnsignedCharArray_get_ptr (vtkNew < vtkConstantUnsignedCharArray > sself) ;
extern "C" vtkNew < vtkConstantUnsignedIntArray > vtkConstantUnsignedIntArray_new () ;
extern "C" void vtkConstantUnsignedIntArray_destructor (vtkNew < vtkConstantUnsignedIntArray > sself) ;
extern "C" void * vtkConstantUnsignedIntArray_get_ptr (vtkNew < vtkConstantUnsignedIntArray > sself) ;
extern "C" vtkNew < vtkConstantUnsignedLongArray > vtkConstantUnsignedLongArray_new () ;
extern "C" void vtkConstantUnsignedLongArray_destructor (vtkNew < vtkConstantUnsignedLongArray > sself) ;
extern "C" void * vtkConstantUnsignedLongArray_get_ptr (vtkNew < vtkConstantUnsignedLongArray > sself) ;
extern "C" vtkNew < vtkConstantUnsignedLongLongArray > vtkConstantUnsignedLongLongArray_new () ;
extern "C" void vtkConstantUnsignedLongLongArray_destructor (vtkNew < vtkConstantUnsignedLongLongArray > sself) ;
extern "C" void * vtkConstantUnsignedLongLongArray_get_ptr (vtkNew < vtkConstantUnsignedLongLongArray > sself) ;
extern "C" vtkNew < vtkConstantUnsignedShortArray > vtkConstantUnsignedShortArray_new () ;
extern "C" void vtkConstantUnsignedShortArray_destructor (vtkNew < vtkConstantUnsignedShortArray > sself) ;
extern "C" void * vtkConstantUnsignedShortArray_get_ptr (vtkNew < vtkConstantUnsignedShortArray > sself) ;
extern "C" vtkNew < vtkDataArrayCollection > vtkDataArrayCollection_new () ;
extern "C" void vtkDataArrayCollection_destructor (vtkNew < vtkDataArrayCollection > sself) ;
extern "C" void * vtkDataArrayCollection_get_ptr (vtkNew < vtkDataArrayCollection > sself) ;
extern "C" vtkNew < vtkDataArrayCollectionIterator > vtkDataArrayCollectionIterator_new () ;
extern "C" void vtkDataArrayCollectionIterator_destructor (vtkNew < vtkDataArrayCollectionIterator > sself) ;
extern "C" void * vtkDataArrayCollectionIterator_get_ptr (vtkNew < vtkDataArrayCollectionIterator > sself) ;
extern "C" vtkNew < vtkDataArraySelection > vtkDataArraySelection_new () ;
extern "C" void vtkDataArraySelection_destructor (vtkNew < vtkDataArraySelection > sself) ;
extern "C" void * vtkDataArraySelection_get_ptr (vtkNew < vtkDataArraySelection > sself) ;
extern "C" vtkNew < vtkDebugLeaks > vtkDebugLeaks_new () ;
extern "C" void vtkDebugLeaks_destructor (vtkNew < vtkDebugLeaks > sself) ;
extern "C" void * vtkDebugLeaks_get_ptr (vtkNew < vtkDebugLeaks > sself) ;
extern "C" vtkNew < vtkDeserializer > vtkDeserializer_new () ;
extern "C" void vtkDeserializer_destructor (vtkNew < vtkDeserializer > sself) ;
extern "C" void * vtkDeserializer_get_ptr (vtkNew < vtkDeserializer > sself) ;
extern "C" vtkNew < vtkDoubleArray > vtkDoubleArray_new () ;
extern "C" void vtkDoubleArray_destructor (vtkNew < vtkDoubleArray > sself) ;
extern "C" void * vtkDoubleArray_get_ptr (vtkNew < vtkDoubleArray > sself) ;
extern "C" vtkNew < vtkDynamicLoader > vtkDynamicLoader_new () ;
extern "C" void vtkDynamicLoader_destructor (vtkNew < vtkDynamicLoader > sself) ;
extern "C" void * vtkDynamicLoader_get_ptr (vtkNew < vtkDynamicLoader > sself) ;
extern "C" vtkNew < vtkEventDataDevice3D > vtkEventDataDevice3D_new () ;
extern "C" void vtkEventDataDevice3D_destructor (vtkNew < vtkEventDataDevice3D > sself) ;
extern "C" void * vtkEventDataDevice3D_get_ptr (vtkNew < vtkEventDataDevice3D > sself) ;
extern "C" vtkNew < vtkEventDataForDevice > vtkEventDataForDevice_new () ;
extern "C" void vtkEventDataForDevice_destructor (vtkNew < vtkEventDataForDevice > sself) ;
extern "C" void * vtkEventDataForDevice_get_ptr (vtkNew < vtkEventDataForDevice > sself) ;
extern "C" vtkNew < vtkEventForwarderCommand > vtkEventForwarderCommand_new () ;
extern "C" void vtkEventForwarderCommand_destructor (vtkNew < vtkEventForwarderCommand > sself) ;
extern "C" void * vtkEventForwarderCommand_get_ptr (vtkNew < vtkEventForwarderCommand > sself) ;
extern "C" vtkNew < vtkFileOutputWindow > vtkFileOutputWindow_new () ;
extern "C" void vtkFileOutputWindow_destructor (vtkNew < vtkFileOutputWindow > sself) ;
extern "C" void * vtkFileOutputWindow_get_ptr (vtkNew < vtkFileOutputWindow > sself) ;
extern "C" vtkNew < vtkFloatArray > vtkFloatArray_new () ;
extern "C" void vtkFloatArray_destructor (vtkNew < vtkFloatArray > sself) ;
extern "C" void * vtkFloatArray_get_ptr (vtkNew < vtkFloatArray > sself) ;
extern "C" vtkNew < vtkGarbageCollector > vtkGarbageCollector_new () ;
extern "C" void vtkGarbageCollector_destructor (vtkNew < vtkGarbageCollector > sself) ;
extern "C" void * vtkGarbageCollector_get_ptr (vtkNew < vtkGarbageCollector > sself) ;
extern "C" vtkNew < vtkIdList > vtkIdList_new () ;
extern "C" void vtkIdList_destructor (vtkNew < vtkIdList > sself) ;
extern "C" void * vtkIdList_get_ptr (vtkNew < vtkIdList > sself) ;
extern "C" vtkNew < vtkIdListCollection > vtkIdListCollection_new () ;
extern "C" void vtkIdListCollection_destructor (vtkNew < vtkIdListCollection > sself) ;
extern "C" void * vtkIdListCollection_get_ptr (vtkNew < vtkIdListCollection > sself) ;
extern "C" vtkNew < vtkIdTypeArray > vtkIdTypeArray_new () ;
extern "C" void vtkIdTypeArray_destructor (vtkNew < vtkIdTypeArray > sself) ;
extern "C" void * vtkIdTypeArray_get_ptr (vtkNew < vtkIdTypeArray > sself) ;
extern "C" vtkNew < vtkIndexedCharArray > vtkIndexedCharArray_new () ;
extern "C" void vtkIndexedCharArray_destructor (vtkNew < vtkIndexedCharArray > sself) ;
extern "C" void * vtkIndexedCharArray_get_ptr (vtkNew < vtkIndexedCharArray > sself) ;
extern "C" vtkNew < vtkIndexedDoubleArray > vtkIndexedDoubleArray_new () ;
extern "C" void vtkIndexedDoubleArray_destructor (vtkNew < vtkIndexedDoubleArray > sself) ;
extern "C" void * vtkIndexedDoubleArray_get_ptr (vtkNew < vtkIndexedDoubleArray > sself) ;
extern "C" vtkNew < vtkIndexedFloatArray > vtkIndexedFloatArray_new () ;
extern "C" void vtkIndexedFloatArray_destructor (vtkNew < vtkIndexedFloatArray > sself) ;
extern "C" void * vtkIndexedFloatArray_get_ptr (vtkNew < vtkIndexedFloatArray > sself) ;
extern "C" vtkNew < vtkIndexedIdTypeArray > vtkIndexedIdTypeArray_new () ;
extern "C" void vtkIndexedIdTypeArray_destructor (vtkNew < vtkIndexedIdTypeArray > sself) ;
extern "C" void * vtkIndexedIdTypeArray_get_ptr (vtkNew < vtkIndexedIdTypeArray > sself) ;
extern "C" vtkNew < vtkIndexedIntArray > vtkIndexedIntArray_new () ;
extern "C" void vtkIndexedIntArray_destructor (vtkNew < vtkIndexedIntArray > sself) ;
extern "C" void * vtkIndexedIntArray_get_ptr (vtkNew < vtkIndexedIntArray > sself) ;
extern "C" vtkNew < vtkIndexedLongArray > vtkIndexedLongArray_new () ;
extern "C" void vtkIndexedLongArray_destructor (vtkNew < vtkIndexedLongArray > sself) ;
extern "C" void * vtkIndexedLongArray_get_ptr (vtkNew < vtkIndexedLongArray > sself) ;
extern "C" vtkNew < vtkIndexedLongLongArray > vtkIndexedLongLongArray_new () ;
extern "C" void vtkIndexedLongLongArray_destructor (vtkNew < vtkIndexedLongLongArray > sself) ;
extern "C" void * vtkIndexedLongLongArray_get_ptr (vtkNew < vtkIndexedLongLongArray > sself) ;
extern "C" vtkNew < vtkIndexedShortArray > vtkIndexedShortArray_new () ;
extern "C" void vtkIndexedShortArray_destructor (vtkNew < vtkIndexedShortArray > sself) ;
extern "C" void * vtkIndexedShortArray_get_ptr (vtkNew < vtkIndexedShortArray > sself) ;
extern "C" vtkNew < vtkIndexedSignedCharArray > vtkIndexedSignedCharArray_new () ;
extern "C" void vtkIndexedSignedCharArray_destructor (vtkNew < vtkIndexedSignedCharArray > sself) ;
extern "C" void * vtkIndexedSignedCharArray_get_ptr (vtkNew < vtkIndexedSignedCharArray > sself) ;
extern "C" vtkNew < vtkIndexedUnsignedCharArray > vtkIndexedUnsignedCharArray_new () ;
extern "C" void vtkIndexedUnsignedCharArray_destructor (vtkNew < vtkIndexedUnsignedCharArray > sself) ;
extern "C" void * vtkIndexedUnsignedCharArray_get_ptr (vtkNew < vtkIndexedUnsignedCharArray > sself) ;
extern "C" vtkNew < vtkIndexedUnsignedIntArray > vtkIndexedUnsignedIntArray_new () ;
extern "C" void vtkIndexedUnsignedIntArray_destructor (vtkNew < vtkIndexedUnsignedIntArray > sself) ;
extern "C" void * vtkIndexedUnsignedIntArray_get_ptr (vtkNew < vtkIndexedUnsignedIntArray > sself) ;
extern "C" vtkNew < vtkIndexedUnsignedLongArray > vtkIndexedUnsignedLongArray_new () ;
extern "C" void vtkIndexedUnsignedLongArray_destructor (vtkNew < vtkIndexedUnsignedLongArray > sself) ;
extern "C" void * vtkIndexedUnsignedLongArray_get_ptr (vtkNew < vtkIndexedUnsignedLongArray > sself) ;
extern "C" vtkNew < vtkIndexedUnsignedLongLongArray > vtkIndexedUnsignedLongLongArray_new () ;
extern "C" void vtkIndexedUnsignedLongLongArray_destructor (vtkNew < vtkIndexedUnsignedLongLongArray > sself) ;
extern "C" void * vtkIndexedUnsignedLongLongArray_get_ptr (vtkNew < vtkIndexedUnsignedLongLongArray > sself) ;
extern "C" vtkNew < vtkIndexedUnsignedShortArray > vtkIndexedUnsignedShortArray_new () ;
extern "C" void vtkIndexedUnsignedShortArray_destructor (vtkNew < vtkIndexedUnsignedShortArray > sself) ;
extern "C" void * vtkIndexedUnsignedShortArray_get_ptr (vtkNew < vtkIndexedUnsignedShortArray > sself) ;
extern "C" vtkNew < vtkInformation > vtkInformation_new () ;
extern "C" void vtkInformation_destructor (vtkNew < vtkInformation > sself) ;
extern "C" void * vtkInformation_get_ptr (vtkNew < vtkInformation > sself) ;
extern "C" vtkNew < vtkInformationIterator > vtkInformationIterator_new () ;
extern "C" void vtkInformationIterator_destructor (vtkNew < vtkInformationIterator > sself) ;
extern "C" void * vtkInformationIterator_get_ptr (vtkNew < vtkInformationIterator > sself) ;
extern "C" vtkNew < vtkInformationKeyLookup > vtkInformationKeyLookup_new () ;
extern "C" void vtkInformationKeyLookup_destructor (vtkNew < vtkInformationKeyLookup > sself) ;
extern "C" void * vtkInformationKeyLookup_get_ptr (vtkNew < vtkInformationKeyLookup > sself) ;
extern "C" vtkNew < vtkInformationVector > vtkInformationVector_new () ;
extern "C" void vtkInformationVector_destructor (vtkNew < vtkInformationVector > sself) ;
extern "C" void * vtkInformationVector_get_ptr (vtkNew < vtkInformationVector > sself) ;
extern "C" vtkNew < vtkIntArray > vtkIntArray_new () ;
extern "C" void vtkIntArray_destructor (vtkNew < vtkIntArray > sself) ;
extern "C" void * vtkIntArray_get_ptr (vtkNew < vtkIntArray > sself) ;
extern "C" vtkNew < vtkInvoker > vtkInvoker_new () ;
extern "C" void vtkInvoker_destructor (vtkNew < vtkInvoker > sself) ;
extern "C" void * vtkInvoker_get_ptr (vtkNew < vtkInvoker > sself) ;
extern "C" vtkNew < vtkLongArray > vtkLongArray_new () ;
extern "C" void vtkLongArray_destructor (vtkNew < vtkLongArray > sself) ;
extern "C" void * vtkLongArray_get_ptr (vtkNew < vtkLongArray > sself) ;
extern "C" vtkNew < vtkLongLongArray > vtkLongLongArray_new () ;
extern "C" void vtkLongLongArray_destructor (vtkNew < vtkLongLongArray > sself) ;
extern "C" void * vtkLongLongArray_get_ptr (vtkNew < vtkLongLongArray > sself) ;
extern "C" vtkNew < vtkLookupTable > vtkLookupTable_new () ;
extern "C" void vtkLookupTable_destructor (vtkNew < vtkLookupTable > sself) ;
extern "C" void * vtkLookupTable_get_ptr (vtkNew < vtkLookupTable > sself) ;
extern "C" vtkNew < vtkMarshalContext > vtkMarshalContext_new () ;
extern "C" void vtkMarshalContext_destructor (vtkNew < vtkMarshalContext > sself) ;
extern "C" void * vtkMarshalContext_get_ptr (vtkNew < vtkMarshalContext > sself) ;
extern "C" vtkNew < vtkMath > vtkMath_new () ;
extern "C" void vtkMath_destructor (vtkNew < vtkMath > sself) ;
extern "C" void * vtkMath_get_ptr (vtkNew < vtkMath > sself) ;
extern "C" vtkNew < vtkMersenneTwister > vtkMersenneTwister_new () ;
extern "C" void vtkMersenneTwister_destructor (vtkNew < vtkMersenneTwister > sself) ;
extern "C" void * vtkMersenneTwister_get_ptr (vtkNew < vtkMersenneTwister > sself) ;
extern "C" vtkNew < vtkMinimalStandardRandomSequence > vtkMinimalStandardRandomSequence_new () ;
extern "C" void vtkMinimalStandardRandomSequence_destructor (vtkNew < vtkMinimalStandardRandomSequence > sself) ;
extern "C" void * vtkMinimalStandardRandomSequence_get_ptr (vtkNew < vtkMinimalStandardRandomSequence > sself) ;
extern "C" vtkNew < vtkMultiThreader > vtkMultiThreader_new () ;
extern "C" void vtkMultiThreader_destructor (vtkNew < vtkMultiThreader > sself) ;
extern "C" void * vtkMultiThreader_get_ptr (vtkNew < vtkMultiThreader > sself) ;
extern "C" vtkNew < vtkObject > vtkObject_new () ;
extern "C" void vtkObject_destructor (vtkNew < vtkObject > sself) ;
extern "C" void * vtkObject_get_ptr (vtkNew < vtkObject > sself) ;
extern "C" vtkNew < vtkObjectFactoryCollection > vtkObjectFactoryCollection_new () ;
extern "C" void vtkObjectFactoryCollection_destructor (vtkNew < vtkObjectFactoryCollection > sself) ;
extern "C" void * vtkObjectFactoryCollection_get_ptr (vtkNew < vtkObjectFactoryCollection > sself) ;
extern "C" vtkNew < vtkOldStyleCallbackCommand > vtkOldStyleCallbackCommand_new () ;
extern "C" void vtkOldStyleCallbackCommand_destructor (vtkNew < vtkOldStyleCallbackCommand > sself) ;
extern "C" void * vtkOldStyleCallbackCommand_get_ptr (vtkNew < vtkOldStyleCallbackCommand > sself) ;
extern "C" vtkNew < vtkOutputWindow > vtkOutputWindow_new () ;
extern "C" void vtkOutputWindow_destructor (vtkNew < vtkOutputWindow > sself) ;
extern "C" void * vtkOutputWindow_get_ptr (vtkNew < vtkOutputWindow > sself) ;
extern "C" vtkNew < vtkOverrideInformationCollection > vtkOverrideInformationCollection_new () ;
extern "C" void vtkOverrideInformationCollection_destructor (vtkNew < vtkOverrideInformationCollection > sself) ;
extern "C" void * vtkOverrideInformationCollection_get_ptr (vtkNew < vtkOverrideInformationCollection > sself) ;
extern "C" vtkNew < vtkPoints > vtkPoints_new () ;
extern "C" void vtkPoints_destructor (vtkNew < vtkPoints > sself) ;
extern "C" void * vtkPoints_get_ptr (vtkNew < vtkPoints > sself) ;
extern "C" vtkNew < vtkPoints2D > vtkPoints2D_new () ;
extern "C" void vtkPoints2D_destructor (vtkNew < vtkPoints2D > sself) ;
extern "C" void * vtkPoints2D_get_ptr (vtkNew < vtkPoints2D > sself) ;
extern "C" vtkNew < vtkPriorityQueue > vtkPriorityQueue_new () ;
extern "C" void vtkPriorityQueue_destructor (vtkNew < vtkPriorityQueue > sself) ;
extern "C" void * vtkPriorityQueue_get_ptr (vtkNew < vtkPriorityQueue > sself) ;
extern "C" vtkNew < vtkRandomPool > vtkRandomPool_new () ;
extern "C" void vtkRandomPool_destructor (vtkNew < vtkRandomPool > sself) ;
extern "C" void * vtkRandomPool_get_ptr (vtkNew < vtkRandomPool > sself) ;
extern "C" vtkNew < vtkReferenceCount > vtkReferenceCount_new () ;
extern "C" void vtkReferenceCount_destructor (vtkNew < vtkReferenceCount > sself) ;
extern "C" void * vtkReferenceCount_get_ptr (vtkNew < vtkReferenceCount > sself) ;
extern "C" vtkNew < vtkScalarsToColors > vtkScalarsToColors_new () ;
extern "C" void vtkScalarsToColors_destructor (vtkNew < vtkScalarsToColors > sself) ;
extern "C" void * vtkScalarsToColors_get_ptr (vtkNew < vtkScalarsToColors > sself) ;
extern "C" vtkNew < vtkSerializer > vtkSerializer_new () ;
extern "C" void vtkSerializer_destructor (vtkNew < vtkSerializer > sself) ;
extern "C" void * vtkSerializer_get_ptr (vtkNew < vtkSerializer > sself) ;
extern "C" vtkNew < vtkShortArray > vtkShortArray_new () ;
extern "C" void vtkShortArray_destructor (vtkNew < vtkShortArray > sself) ;
extern "C" void * vtkShortArray_get_ptr (vtkNew < vtkShortArray > sself) ;
extern "C" vtkNew < vtkSignedCharArray > vtkSignedCharArray_new () ;
extern "C" void vtkSignedCharArray_destructor (vtkNew < vtkSignedCharArray > sself) ;
extern "C" void * vtkSignedCharArray_get_ptr (vtkNew < vtkSignedCharArray > sself) ;
extern "C" vtkNew < vtkSortDataArray > vtkSortDataArray_new () ;
extern "C" void vtkSortDataArray_destructor (vtkNew < vtkSortDataArray > sself) ;
extern "C" void * vtkSortDataArray_get_ptr (vtkNew < vtkSortDataArray > sself) ;
extern "C" vtkNew < vtkStringArray > vtkStringArray_new () ;
extern "C" void vtkStringArray_destructor (vtkNew < vtkStringArray > sself) ;
extern "C" void * vtkStringArray_get_ptr (vtkNew < vtkStringArray > sself) ;
extern "C" vtkNew < vtkStringOutputWindow > vtkStringOutputWindow_new () ;
extern "C" void vtkStringOutputWindow_destructor (vtkNew < vtkStringOutputWindow > sself) ;
extern "C" void * vtkStringOutputWindow_get_ptr (vtkNew < vtkStringOutputWindow > sself) ;
extern "C" vtkNew < vtkTimePointUtility > vtkTimePointUtility_new () ;
extern "C" void vtkTimePointUtility_destructor (vtkNew < vtkTimePointUtility > sself) ;
extern "C" void * vtkTimePointUtility_get_ptr (vtkNew < vtkTimePointUtility > sself) ;
extern "C" vtkNew < vtkTypeFloat32Array > vtkTypeFloat32Array_new () ;
extern "C" void vtkTypeFloat32Array_destructor (vtkNew < vtkTypeFloat32Array > sself) ;
extern "C" void * vtkTypeFloat32Array_get_ptr (vtkNew < vtkTypeFloat32Array > sself) ;
extern "C" vtkNew < vtkTypeFloat64Array > vtkTypeFloat64Array_new () ;
extern "C" void vtkTypeFloat64Array_destructor (vtkNew < vtkTypeFloat64Array > sself) ;
extern "C" void * vtkTypeFloat64Array_get_ptr (vtkNew < vtkTypeFloat64Array > sself) ;
extern "C" vtkNew < vtkTypeInt16Array > vtkTypeInt16Array_new () ;
extern "C" void vtkTypeInt16Array_destructor (vtkNew < vtkTypeInt16Array > sself) ;
extern "C" void * vtkTypeInt16Array_get_ptr (vtkNew < vtkTypeInt16Array > sself) ;
extern "C" vtkNew < vtkTypeInt32Array > vtkTypeInt32Array_new () ;
extern "C" void vtkTypeInt32Array_destructor (vtkNew < vtkTypeInt32Array > sself) ;
extern "C" void * vtkTypeInt32Array_get_ptr (vtkNew < vtkTypeInt32Array > sself) ;
extern "C" vtkNew < vtkTypeInt64Array > vtkTypeInt64Array_new () ;
extern "C" void vtkTypeInt64Array_destructor (vtkNew < vtkTypeInt64Array > sself) ;
extern "C" void * vtkTypeInt64Array_get_ptr (vtkNew < vtkTypeInt64Array > sself) ;
extern "C" vtkNew < vtkTypeInt8Array > vtkTypeInt8Array_new () ;
extern "C" void vtkTypeInt8Array_destructor (vtkNew < vtkTypeInt8Array > sself) ;
extern "C" void * vtkTypeInt8Array_get_ptr (vtkNew < vtkTypeInt8Array > sself) ;
extern "C" vtkNew < vtkTypeUInt16Array > vtkTypeUInt16Array_new () ;
extern "C" void vtkTypeUInt16Array_destructor (vtkNew < vtkTypeUInt16Array > sself) ;
extern "C" void * vtkTypeUInt16Array_get_ptr (vtkNew < vtkTypeUInt16Array > sself) ;
extern "C" vtkNew < vtkTypeUInt32Array > vtkTypeUInt32Array_new () ;
extern "C" void vtkTypeUInt32Array_destructor (vtkNew < vtkTypeUInt32Array > sself) ;
extern "C" void * vtkTypeUInt32Array_get_ptr (vtkNew < vtkTypeUInt32Array > sself) ;
extern "C" vtkNew < vtkTypeUInt64Array > vtkTypeUInt64Array_new () ;
extern "C" void vtkTypeUInt64Array_destructor (vtkNew < vtkTypeUInt64Array > sself) ;
extern "C" void * vtkTypeUInt64Array_get_ptr (vtkNew < vtkTypeUInt64Array > sself) ;
extern "C" vtkNew < vtkTypeUInt8Array > vtkTypeUInt8Array_new () ;
extern "C" void vtkTypeUInt8Array_destructor (vtkNew < vtkTypeUInt8Array > sself) ;
extern "C" void * vtkTypeUInt8Array_get_ptr (vtkNew < vtkTypeUInt8Array > sself) ;
extern "C" vtkNew < vtkUnsignedCharArray > vtkUnsignedCharArray_new () ;
extern "C" void vtkUnsignedCharArray_destructor (vtkNew < vtkUnsignedCharArray > sself) ;
extern "C" void * vtkUnsignedCharArray_get_ptr (vtkNew < vtkUnsignedCharArray > sself) ;
extern "C" vtkNew < vtkUnsignedIntArray > vtkUnsignedIntArray_new () ;
extern "C" void vtkUnsignedIntArray_destructor (vtkNew < vtkUnsignedIntArray > sself) ;
extern "C" void * vtkUnsignedIntArray_get_ptr (vtkNew < vtkUnsignedIntArray > sself) ;
extern "C" vtkNew < vtkUnsignedLongArray > vtkUnsignedLongArray_new () ;
extern "C" void vtkUnsignedLongArray_destructor (vtkNew < vtkUnsignedLongArray > sself) ;
extern "C" void * vtkUnsignedLongArray_get_ptr (vtkNew < vtkUnsignedLongArray > sself) ;
extern "C" vtkNew < vtkUnsignedLongLongArray > vtkUnsignedLongLongArray_new () ;
extern "C" void vtkUnsignedLongLongArray_destructor (vtkNew < vtkUnsignedLongLongArray > sself) ;
extern "C" void * vtkUnsignedLongLongArray_get_ptr (vtkNew < vtkUnsignedLongLongArray > sself) ;
extern "C" vtkNew < vtkUnsignedShortArray > vtkUnsignedShortArray_new () ;
extern "C" void vtkUnsignedShortArray_destructor (vtkNew < vtkUnsignedShortArray > sself) ;
extern "C" void * vtkUnsignedShortArray_get_ptr (vtkNew < vtkUnsignedShortArray > sself) ;
extern "C" vtkNew < vtkVariantArray > vtkVariantArray_new () ;
extern "C" void vtkVariantArray_destructor (vtkNew < vtkVariantArray > sself) ;
extern "C" void * vtkVariantArray_get_ptr (vtkNew < vtkVariantArray > sself) ;
extern "C" vtkNew < vtkVersion > vtkVersion_new () ;
extern "C" void vtkVersion_destructor (vtkNew < vtkVersion > sself) ;
extern "C" void * vtkVersion_get_ptr (vtkNew < vtkVersion > sself) ;
extern "C" vtkNew < vtkVoidArray > vtkVoidArray_new () ;
extern "C" void vtkVoidArray_destructor (vtkNew < vtkVoidArray > sself) ;
extern "C" void * vtkVoidArray_get_ptr (vtkNew < vtkVoidArray > sself) ;
extern "C" vtkNew < vtkWeakReference > vtkWeakReference_new () ;
extern "C" void vtkWeakReference_destructor (vtkNew < vtkWeakReference > sself) ;
extern "C" void * vtkWeakReference_get_ptr (vtkNew < vtkWeakReference > sself) ;
extern "C" vtkNew < vtkXMLFileOutputWindow > vtkXMLFileOutputWindow_new () ;
extern "C" void vtkXMLFileOutputWindow_destructor (vtkNew < vtkXMLFileOutputWindow > sself) ;
extern "C" void * vtkXMLFileOutputWindow_get_ptr (vtkNew < vtkXMLFileOutputWindow > sself) ;
